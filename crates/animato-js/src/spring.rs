//! Spring bindings.

use crate::error::{JsResult, js_error, non_negative};
use crate::tween::lock;
use crate::types::f32_array;
use animato_core::Update;
use animato_spring::{Spring as CoreSpring, SpringConfig, SpringN};
use js_sys::Float32Array;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

fn preset(name: &str) -> JsResult<SpringConfig> {
    match crate::types::normalize_name(name).as_str() {
        "gentle" => Ok(SpringConfig::gentle()),
        "wobbly" => Ok(SpringConfig::wobbly()),
        "stiff" => Ok(SpringConfig::stiff()),
        "slow" => Ok(SpringConfig::slow()),
        "snappy" => Ok(SpringConfig::snappy()),
        _ => Err(js_error(format!("unknown spring preset `{name}`"))),
    }
}

fn config(stiffness: f32, damping: f32, mass: f32, epsilon: f32) -> SpringConfig {
    SpringConfig {
        stiffness: non_negative(stiffness, 100.0),
        damping: non_negative(damping, 10.0),
        mass: non_negative(mass, 1.0).max(f32::EPSILON),
        epsilon: non_negative(epsilon, 0.001),
    }
}

/// Shared scalar spring update adapter.
#[derive(Clone, Debug)]
pub(crate) struct SharedSpring {
    inner: Arc<Mutex<CoreSpring>>,
}

impl SharedSpring {
    pub(crate) fn new(inner: Arc<Mutex<CoreSpring>>) -> Self {
        Self { inner }
    }
}

impl Update for SharedSpring {
    fn update(&mut self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }
}

macro_rules! shared_spring_n {
    ($name:ident, $value_ty:ty) => {
        /// Shared vector spring update adapter.
        #[derive(Clone, Debug)]
        pub(crate) struct $name {
            inner: Arc<Mutex<SpringN<$value_ty>>>,
        }

        impl $name {
            pub(crate) fn new(inner: Arc<Mutex<SpringN<$value_ty>>>) -> Self {
                Self { inner }
            }
        }

        impl Update for $name {
            fn update(&mut self, dt: f32) -> bool {
                lock(&self.inner).update(dt)
            }
        }
    };
}

shared_spring_n!(SharedSpring2D, [f32; 2]);
shared_spring_n!(SharedSpring3D, [f32; 3]);
shared_spring_n!(SharedSpring4D, [f32; 4]);

/// Scalar damped spring.
#[wasm_bindgen(js_name = Spring)]
#[derive(Clone, Debug)]
pub struct Spring {
    inner: Arc<Mutex<CoreSpring>>,
}

#[wasm_bindgen(js_class = Spring)]
impl Spring {
    /// Create a spring at `initial`, targeting `target`.
    #[wasm_bindgen(constructor)]
    pub fn new(initial: f32, target: f32) -> Self {
        let mut spring = CoreSpring::new(SpringConfig::default());
        spring.snap_to(initial);
        spring.set_target(target);
        Self {
            inner: Arc::new(Mutex::new(spring)),
        }
    }

    /// Create a spring with an initial velocity.
    #[wasm_bindgen(js_name = fromVelocity)]
    pub fn from_velocity(initial: f32, velocity: f32, target: f32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(CoreSpring::from_velocity(
                initial,
                velocity,
                target,
                SpringConfig::default(),
            ))),
        }
    }

    /// Advance by `dt` seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Current position.
    pub fn position(&self) -> f32 {
        lock(&self.inner).position()
    }

    /// Current velocity.
    pub fn velocity(&self) -> f32 {
        lock(&self.inner).velocity()
    }

    /// Current kinetic plus potential energy.
    pub fn energy(&self) -> f32 {
        lock(&self.inner).energy()
    }

    /// Number of target crossings.
    #[wasm_bindgen(js_name = overshootCount)]
    pub fn overshoot_count(&self) -> u32 {
        lock(&self.inner).overshoot_count()
    }

    /// Whether the spring has settled.
    #[wasm_bindgen(js_name = isSettled)]
    pub fn is_settled(&self) -> bool {
        lock(&self.inner).is_settled()
    }

    /// Set a target.
    #[wasm_bindgen(js_name = setTarget)]
    pub fn set_target(&self, target: f32) {
        lock(&self.inner).set_target(target);
    }

    /// Snap instantly to a position.
    #[wasm_bindgen(js_name = snapTo)]
    pub fn snap_to(&self, position: f32) {
        lock(&self.inner).snap_to(position);
    }

    /// Apply a named preset.
    #[wasm_bindgen(js_name = setPreset)]
    pub fn set_preset(&self, name: &str) -> Result<(), JsValue> {
        lock(&self.inner).config = preset(name)?;
        Ok(())
    }

    /// Set custom spring parameters.
    #[wasm_bindgen(js_name = setConfig)]
    pub fn set_config(&self, stiffness: f32, damping: f32, mass: f32, epsilon: f32) {
        lock(&self.inner).config = config(stiffness, damping, mass, epsilon);
    }

    /// Use critically damped configuration.
    #[wasm_bindgen(js_name = setCriticalDamping)]
    pub fn set_critical_damping(&self, stiffness: f32) {
        lock(&self.inner).config = SpringConfig::critically_damped(stiffness);
    }

    /// Use overdamped configuration.
    #[wasm_bindgen(js_name = setOverdamped)]
    pub fn set_overdamped(&self, stiffness: f32, ratio: f32) {
        lock(&self.inner).config = SpringConfig::overdamped(stiffness, ratio);
    }

    /// Use underdamped configuration.
    #[wasm_bindgen(js_name = setUnderdamped)]
    pub fn set_underdamped(&self, stiffness: f32, ratio: f32) {
        lock(&self.inner).config = SpringConfig::underdamped(stiffness, ratio);
    }

    pub(crate) fn shared(&self) -> SharedSpring {
        SharedSpring::new(Arc::clone(&self.inner))
    }
}

macro_rules! vector_spring {
    (
        $class:ident,
        $js_name:ident,
        $shared:ident,
        $value_ty:ty,
        [$($initial:ident),+],
        [$($velocity:ident),+],
        [$($target:ident),+],
        $array_fn:ident
    ) => {
        /// Vector damped spring.
        #[wasm_bindgen(js_name = $js_name)]
        #[derive(Clone, Debug)]
        pub struct $class {
            inner: Arc<Mutex<SpringN<$value_ty>>>,
        }

        #[wasm_bindgen(js_class = $js_name)]
        impl $class {
            /// Create a vector spring.
            #[wasm_bindgen(constructor)]
            #[allow(clippy::too_many_arguments)]
            pub fn new($($initial: f32,)+ $($target: f32),+) -> Self {
                let mut spring = SpringN::new(SpringConfig::default(), [$($initial),+]);
                spring.set_target([$($target),+]);
                Self {
                    inner: Arc::new(Mutex::new(spring)),
                }
            }

            /// Create a vector spring with initial velocity.
            #[wasm_bindgen(js_name = fromVelocity)]
            #[allow(clippy::too_many_arguments)]
            pub fn from_velocity($($initial: f32,)+ $($velocity: f32,)+ $($target: f32),+) -> Self {
                Self {
                    inner: Arc::new(Mutex::new(SpringN::from_velocity(
                        [$($initial),+],
                        [$($velocity),+],
                        [$($target),+],
                        SpringConfig::default(),
                    ))),
                }
            }

            /// Advance by `dt` seconds.
            pub fn update(&self, dt: f32) -> bool {
                lock(&self.inner).update(dt)
            }

            /// Current position as a typed array.
            #[wasm_bindgen(js_name = toArray)]
            pub fn to_array(&self) -> Float32Array {
                let value = lock(&self.inner).position();
                f32_array(&value)
            }

            /// Current velocity as a typed array.
            #[wasm_bindgen(js_name = velocityArray)]
            pub fn velocity_array(&self) -> Float32Array {
                let value = lock(&self.inner).velocity();
                f32_array(&value)
            }

            /// Current kinetic plus potential energy.
            pub fn energy(&self) -> f32 {
                lock(&self.inner).energy()
            }

            /// Total component target crossings.
            #[wasm_bindgen(js_name = overshootCount)]
            pub fn overshoot_count(&self) -> u32 {
                lock(&self.inner).overshoot_count()
            }

            /// Whether the spring has settled.
            #[wasm_bindgen(js_name = isSettled)]
            pub fn is_settled(&self) -> bool {
                lock(&self.inner).is_settled()
            }

            /// Snap instantly to a position.
            #[wasm_bindgen(js_name = snapTo)]
            pub fn snap_to(&self, $($initial: f32),+) {
                lock(&self.inner).snap_to([$($initial),+]);
            }

            /// Set a target.
            #[wasm_bindgen(js_name = setTarget)]
            pub fn set_target(&self, $($target: f32),+) {
                lock(&self.inner).set_target([$($target),+]);
            }

            /// Set custom spring parameters.
            #[wasm_bindgen(js_name = setConfig)]
            pub fn set_config(&self, stiffness: f32, damping: f32, mass: f32, epsilon: f32) {
                lock(&self.inner).set_config(config(stiffness, damping, mass, epsilon));
            }

            /// Use critically damped configuration.
            #[wasm_bindgen(js_name = setCriticalDamping)]
            pub fn set_critical_damping(&self, stiffness: f32) {
                lock(&self.inner).set_config(SpringConfig::critically_damped(stiffness));
            }

            /// Use overdamped configuration.
            #[wasm_bindgen(js_name = setOverdamped)]
            pub fn set_overdamped(&self, stiffness: f32, ratio: f32) {
                lock(&self.inner).set_config(SpringConfig::overdamped(stiffness, ratio));
            }

            /// Use underdamped configuration.
            #[wasm_bindgen(js_name = setUnderdamped)]
            pub fn set_underdamped(&self, stiffness: f32, ratio: f32) {
                lock(&self.inner).set_config(SpringConfig::underdamped(stiffness, ratio));
            }

            pub(crate) fn shared(&self) -> $shared {
                $shared::new(Arc::clone(&self.inner))
            }
        }
    };
}

vector_spring!(
    Spring2D,
    Spring2D,
    SharedSpring2D,
    [f32; 2],
    [x, y],
    [velocity_x, velocity_y],
    [target_x, target_y],
    vec2
);
vector_spring!(
    Spring3D,
    Spring3D,
    SharedSpring3D,
    [f32; 3],
    [x, y, z],
    [velocity_x, velocity_y, velocity_z],
    [target_x, target_y, target_z],
    vec3
);
vector_spring!(
    Spring4D,
    Spring4D,
    SharedSpring4D,
    [f32; 4],
    [x, y, z, w],
    [velocity_x, velocity_y, velocity_z, velocity_w],
    [target_x, target_y, target_z, target_w],
    vec4
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_spring_moves() {
        let spring = Spring::new(0.0, 100.0);
        spring.update(1.0 / 60.0);
        assert!(spring.position() > 0.0);
    }
}
