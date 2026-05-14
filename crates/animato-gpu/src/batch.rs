//! Batched tween evaluation.

use animato_core::{Easing, Update};
use animato_tween::Tween;
use core::fmt;
use std::sync::mpsc;

const SHADER_SOURCE: &str = include_str!("shaders/tween.wgsl");

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuTweenInput {
    start: f32,
    end: f32,
    duration: f32,
    elapsed: f32,
    easing_id: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

/// The backend currently used by a [`GpuAnimationBatch`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuBackend {
    /// Deterministic CPU fallback.
    Cpu,
    /// A wgpu device and queue were supplied.
    Gpu,
}

/// Error returned by GPU initialization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GpuBatchError {
    /// No suitable wgpu adapter was found.
    AdapterUnavailable,
    /// Requesting the wgpu device failed.
    RequestDevice(String),
}

struct GpuResources {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl fmt::Debug for GpuResources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GpuResources")
            .field("device", &"wgpu::Device")
            .field("queue", &"wgpu::Queue")
            .field("pipeline", &"tween.wgsl::main")
            .field("bind_group_layout", &"tween storage buffers")
            .finish()
    }
}

/// Batch of `Tween<f32>` values evaluated together.
///
/// The public API is intentionally small: push tweens, tick the batch, then
/// read the current values. The CPU fallback preserves exact `Tween<f32>`
/// behavior, including delays, looping, and advanced/custom easing.
#[derive(Debug)]
pub struct GpuAnimationBatch {
    tweens: Vec<Tween<f32>>,
    values: Vec<f32>,
    inputs: Vec<GpuTweenInput>,
    resources: Option<GpuResources>,
    force_cpu: bool,
}

impl Default for GpuAnimationBatch {
    fn default() -> Self {
        Self::new_cpu()
    }
}

impl GpuAnimationBatch {
    /// Create a CPU-only batch.
    pub fn new_cpu() -> Self {
        Self {
            tweens: Vec::new(),
            values: Vec::new(),
            inputs: Vec::new(),
            resources: None,
            force_cpu: false,
        }
    }

    /// Create a batch from an existing wgpu device and queue.
    ///
    /// If an unsupported easing is pushed later, [`backend`](Self::backend)
    /// reports [`GpuBackend::Cpu`] and ticks continue through the exact CPU
    /// fallback path.
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Result<Self, GpuBatchError> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("animato-gpu tween.wgsl"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("animato-gpu tween bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("animato-gpu tween pipeline layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("animato-gpu tween pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
        Ok(Self {
            tweens: Vec::new(),
            values: Vec::new(),
            inputs: Vec::new(),
            resources: Some(GpuResources {
                device,
                queue,
                pipeline,
                bind_group_layout,
            }),
            force_cpu: false,
        })
    }

    /// Try to create a GPU-backed batch using the default wgpu adapter.
    pub fn try_new_auto() -> Result<Self, GpuBatchError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .map_err(|_| GpuBatchError::AdapterUnavailable)?;

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("animato-gpu device"),
            ..Default::default()
        }))
        .map_err(|err| GpuBatchError::RequestDevice(err.to_string()))?;

        Self::new(device, queue)
    }

    /// Create a GPU-backed batch when possible, otherwise return CPU fallback.
    ///
    /// This function never panics because GPU availability is environmental.
    pub fn new_auto() -> Self {
        Self::try_new_auto().unwrap_or_else(|_| Self::new_cpu())
    }

    /// Push a tween and return its batch index.
    pub fn push(&mut self, tween: Tween<f32>) -> usize {
        if classic_easing_id(&tween.easing).is_none() {
            self.force_cpu = true;
        }
        let index = self.tweens.len();
        self.values.push(tween.value());
        self.tweens.push(tween);
        index
    }

    /// Advance every tween by `dt` seconds and refresh the output buffer.
    pub fn tick(&mut self, dt: f32) {
        for tween in &mut self.tweens {
            tween.update(dt);
        }

        if self.backend() == GpuBackend::Gpu {
            self.prepare_gpu_inputs();
            match self.dispatch_gpu() {
                Ok(()) => return,
                Err(_) => {
                    self.force_cpu = true;
                }
            }
        }

        self.refresh_cpu_values();
    }

    /// Current output values, in insertion order.
    pub fn read_back(&self) -> &[f32] {
        &self.values
    }

    /// Currently active backend.
    pub fn backend(&self) -> GpuBackend {
        if self.resources.is_some() && !self.force_cpu {
            GpuBackend::Gpu
        } else {
            GpuBackend::Cpu
        }
    }

    /// Number of tweens in the batch.
    pub fn len(&self) -> usize {
        self.tweens.len()
    }

    /// `true` when the batch contains no tweens.
    pub fn is_empty(&self) -> bool {
        self.tweens.is_empty()
    }

    /// Remove all tweens and output values.
    pub fn clear(&mut self) {
        self.tweens.clear();
        self.values.clear();
        self.inputs.clear();
        self.force_cpu = false;
    }

    /// WGSL shader source used by the GPU backend.
    pub fn shader_source() -> &'static str {
        SHADER_SOURCE
    }

    fn prepare_gpu_inputs(&mut self) {
        self.inputs.clear();
        self.inputs.reserve(self.tweens.len());
        for tween in &self.tweens {
            let easing_id = classic_easing_id(&tween.easing).unwrap_or(0);
            let (start, end) = if tween.is_ping_pong_reversed() {
                (tween.end, tween.start)
            } else {
                (tween.start, tween.end)
            };
            self.inputs.push(GpuTweenInput {
                start,
                end,
                duration: tween.duration,
                elapsed: tween.elapsed(),
                easing_id,
                _pad0: 0,
                _pad1: 0,
                _pad2: 0,
            });
        }
    }

    fn dispatch_gpu(&mut self) -> Result<(), GpuBatchError> {
        let resources = self
            .resources
            .as_ref()
            .ok_or(GpuBatchError::AdapterUnavailable)?;
        if self.inputs.is_empty() {
            return Ok(());
        }

        let input_bytes = bytemuck::cast_slice(&self.inputs);
        let output_size = (self.values.len() * core::mem::size_of::<f32>()) as wgpu::BufferAddress;

        let input_buffer = resources.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("animato-gpu tween input"),
            size: input_bytes.len() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        resources.queue.write_buffer(&input_buffer, 0, input_bytes);

        let output_buffer = resources.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("animato-gpu tween output"),
            size: output_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let readback_buffer = resources.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("animato-gpu tween readback"),
            size: output_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let bind_group = resources
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("animato-gpu tween bind group"),
                layout: &resources.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: input_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: output_buffer.as_entire_binding(),
                    },
                ],
            });

        let mut encoder =
            resources
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("animato-gpu tween encoder"),
                });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("animato-gpu tween pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&resources.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(self.inputs.len().div_ceil(64) as u32, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output_buffer, 0, &readback_buffer, 0, output_size);
        resources.queue.submit(Some(encoder.finish()));

        let slice = readback_buffer.slice(..);
        let (sender, receiver) = mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        resources
            .device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|err| GpuBatchError::RequestDevice(err.to_string()))?;
        receiver
            .recv()
            .map_err(|err| GpuBatchError::RequestDevice(err.to_string()))?
            .map_err(|err| GpuBatchError::RequestDevice(err.to_string()))?;

        {
            let mapped = slice.get_mapped_range();
            let values: &[f32] = bytemuck::cast_slice(&mapped);
            self.values.copy_from_slice(values);
        }
        readback_buffer.unmap();

        Ok(())
    }

    fn refresh_cpu_values(&mut self) {
        for (tween, value) in self.tweens.iter().zip(self.values.iter_mut()) {
            *value = tween.value();
        }
    }
}

#[inline]
fn classic_easing_id(easing: &Easing) -> Option<u32> {
    Some(match easing {
        Easing::Linear => 0,
        Easing::EaseInQuad => 1,
        Easing::EaseOutQuad => 2,
        Easing::EaseInOutQuad => 3,
        Easing::EaseInCubic => 4,
        Easing::EaseOutCubic => 5,
        Easing::EaseInOutCubic => 6,
        Easing::EaseInQuart => 7,
        Easing::EaseOutQuart => 8,
        Easing::EaseInOutQuart => 9,
        Easing::EaseInQuint => 10,
        Easing::EaseOutQuint => 11,
        Easing::EaseInOutQuint => 12,
        Easing::EaseInSine => 13,
        Easing::EaseOutSine => 14,
        Easing::EaseInOutSine => 15,
        Easing::EaseInExpo => 16,
        Easing::EaseOutExpo => 17,
        Easing::EaseInOutExpo => 18,
        Easing::EaseInCirc => 19,
        Easing::EaseOutCirc => 20,
        Easing::EaseInOutCirc => 21,
        Easing::EaseInBack => 22,
        Easing::EaseOutBack => 23,
        Easing::EaseInOutBack => 24,
        Easing::EaseInElastic => 25,
        Easing::EaseOutElastic => 26,
        Easing::EaseInOutElastic => 27,
        Easing::EaseInBounce => 28,
        Easing::EaseOutBounce => 29,
        Easing::EaseInOutBounce => 30,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use animato_core::Easing;

    #[test]
    fn cpu_batch_matches_regular_tween_values() {
        let mut expected = Tween::new(0.0_f32, 100.0)
            .duration(1.0)
            .easing(Easing::EaseOutCubic)
            .build();
        let mut batch = GpuAnimationBatch::new_cpu();
        batch.push(
            Tween::new(0.0_f32, 100.0)
                .duration(1.0)
                .easing(Easing::EaseOutCubic)
                .build(),
        );

        expected.update(0.25);
        batch.tick(0.25);

        assert!((batch.read_back()[0] - expected.value()).abs() < 0.0001);
    }

    #[test]
    fn unsupported_easing_keeps_cpu_backend() {
        let mut batch = GpuAnimationBatch::new_cpu();
        batch.push(
            Tween::new(0.0_f32, 1.0)
                .easing(Easing::CubicBezier(0.25, 0.1, 0.25, 1.0))
                .build(),
        );
        assert_eq!(batch.backend(), GpuBackend::Cpu);
    }

    #[test]
    fn shader_source_is_embedded() {
        assert!(GpuAnimationBatch::shader_source().contains("@compute"));
        assert!(GpuAnimationBatch::shader_source().contains("ease_out_bounce"));
    }

    #[test]
    fn default_len_clear_and_empty_tick_are_cpu_safe() {
        let mut batch = GpuAnimationBatch::default();

        assert_eq!(batch.backend(), GpuBackend::Cpu);
        assert!(batch.is_empty());
        assert_eq!(batch.len(), 0);
        batch.tick(0.25);
        assert!(batch.read_back().is_empty());

        let index = batch.push(Tween::new(1.0_f32, 3.0).duration(1.0).build());
        assert_eq!(index, 0);
        assert_eq!(batch.len(), 1);
        assert_eq!(batch.read_back(), &[1.0]);

        batch.clear();
        assert!(batch.is_empty());
        assert!(batch.read_back().is_empty());
        assert_eq!(batch.backend(), GpuBackend::Cpu);
    }

    #[test]
    fn supported_easing_ids_cover_all_shader_variants() {
        let supported = [
            Easing::Linear,
            Easing::EaseInQuad,
            Easing::EaseOutQuad,
            Easing::EaseInOutQuad,
            Easing::EaseInCubic,
            Easing::EaseOutCubic,
            Easing::EaseInOutCubic,
            Easing::EaseInQuart,
            Easing::EaseOutQuart,
            Easing::EaseInOutQuart,
            Easing::EaseInQuint,
            Easing::EaseOutQuint,
            Easing::EaseInOutQuint,
            Easing::EaseInSine,
            Easing::EaseOutSine,
            Easing::EaseInOutSine,
            Easing::EaseInExpo,
            Easing::EaseOutExpo,
            Easing::EaseInOutExpo,
            Easing::EaseInCirc,
            Easing::EaseOutCirc,
            Easing::EaseInOutCirc,
            Easing::EaseInBack,
            Easing::EaseOutBack,
            Easing::EaseInOutBack,
            Easing::EaseInElastic,
            Easing::EaseOutElastic,
            Easing::EaseInOutElastic,
            Easing::EaseInBounce,
            Easing::EaseOutBounce,
            Easing::EaseInOutBounce,
        ];

        for (index, easing) in supported.iter().enumerate() {
            assert_eq!(classic_easing_id(easing), Some(index as u32));
        }
        assert_eq!(classic_easing_id(&Easing::Steps(4)), None);
    }

    #[test]
    fn cpu_fallback_handles_multiple_tweens_and_loops() {
        let mut batch = GpuAnimationBatch::new_cpu();
        batch.push(
            Tween::new(0.0_f32, 10.0)
                .duration(1.0)
                .looping(animato_tween::Loop::Forever)
                .build(),
        );
        batch.push(
            Tween::new(10.0_f32, 0.0)
                .duration(2.0)
                .easing(Easing::EaseInOutQuad)
                .build(),
        );

        batch.tick(1.25);

        assert!((batch.read_back()[0] - 2.5).abs() < 0.001);
        assert!(batch.read_back()[1] < 5.0);
    }

    #[test]
    fn auto_constructor_falls_back_or_reports_gpu_without_panicking() {
        let mut batch = GpuAnimationBatch::new_auto();

        assert!(matches!(batch.backend(), GpuBackend::Cpu | GpuBackend::Gpu));
        batch.push(Tween::new(0.0_f32, 1.0).duration(0.1).build());
        batch.tick(0.1);
        assert_eq!(batch.read_back().len(), 1);
    }

    #[test]
    fn gpu_error_debug_and_equality_are_stable() {
        let adapter = GpuBatchError::AdapterUnavailable;
        let device = GpuBatchError::RequestDevice("lost".to_owned());

        assert_eq!(adapter, GpuBatchError::AdapterUnavailable);
        assert_ne!(adapter, device);
        assert!(format!("{device:?}").contains("lost"));
    }
}
