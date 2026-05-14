//! # animato-gpu
//!
//! Batched `Tween<f32>` evaluation for very large animation sets.
//!
//! `GpuAnimationBatch` uses a wgpu compute shader for supported classic easing
//! variants when a device is available. The deterministic CPU backend is always
//! available and is used automatically when a device cannot be created or a
//! tween uses an easing that the WGSL shader does not support yet.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

mod batch;

pub use batch::{GpuAnimationBatch, GpuBackend, GpuBatchError};
