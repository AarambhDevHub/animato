//! Procedural waveform generators for animation values.

use animato_core::math::sin;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::keyframe::{Keyframe, KeyframeTrack};
#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;
#[cfg(any(feature = "std", feature = "alloc"))]
use animato_core::Easing;
#[cfg(any(feature = "std", feature = "alloc"))]
use animato_core::math::ceil;

const TAU: f32 = core::f32::consts::PI * 2.0;

/// A procedural scalar waveform.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Waveform {
    /// Continuous sine wave.
    Sine {
        /// Cycles per second.
        frequency: f32,
        /// Peak absolute value.
        amplitude: f32,
        /// Phase offset in radians.
        phase: f32,
    },
    /// Linear ramp from `-amplitude` to `amplitude`, then wraps.
    Sawtooth {
        /// Cycles per second.
        frequency: f32,
        /// Peak absolute value.
        amplitude: f32,
    },
    /// Two-state square wave.
    Square {
        /// Cycles per second.
        frequency: f32,
        /// Peak absolute value.
        amplitude: f32,
        /// Fraction of the cycle spent at positive amplitude.
        duty_cycle: f32,
    },
    /// Triangle wave with linear rise and fall.
    Triangle {
        /// Cycles per second.
        frequency: f32,
        /// Peak absolute value.
        amplitude: f32,
    },
    /// Deterministic smoothed noise in `[-1, 1]`.
    Noise {
        /// Deterministic seed.
        seed: u32,
        /// Seconds between random control points.
        smoothness: f32,
    },
}

impl Waveform {
    /// Evaluate the waveform at an absolute time in seconds.
    pub fn sample(&self, time: f32) -> f32 {
        let time = finite_or(time, 0.0);
        match *self {
            Self::Sine {
                frequency,
                amplitude,
                phase,
            } => finite_or(amplitude, 1.0) * sin(TAU * finite_or(frequency, 1.0) * time + phase),
            Self::Sawtooth {
                frequency,
                amplitude,
            } => {
                let cycle = cycle(time, frequency);
                finite_or(amplitude, 1.0) * (cycle * 2.0 - 1.0)
            }
            Self::Square {
                frequency,
                amplitude,
                duty_cycle,
            } => {
                let cycle = cycle(time, frequency);
                let duty = finite_or(duty_cycle, 0.5).clamp(0.0, 1.0);
                let amp = finite_or(amplitude, 1.0);
                if cycle < duty { amp } else { -amp }
            }
            Self::Triangle {
                frequency,
                amplitude,
            } => {
                let cycle = cycle(time, frequency);
                finite_or(amplitude, 1.0) * (1.0 - 4.0 * (cycle - 0.5).abs())
            }
            Self::Noise { seed, smoothness } => {
                let span = finite_or(smoothness, 0.25).max(f32::EPSILON);
                let scaled = time.max(0.0) / span;
                let index = scaled as u32;
                let local = smoothstep(scaled - index as f32);
                let a = hash_noise(seed, index);
                let b = hash_noise(seed, index.saturating_add(1));
                a + (b - a) * local
            }
        }
    }

    /// Convert this waveform into a scalar keyframe track.
    ///
    /// `sample_rate` is in samples per second and is clamped to at least `1`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn to_keyframe_track(&self, duration: f32, sample_rate: f32) -> KeyframeTrack<f32> {
        let duration = finite_or(duration, 0.0).max(0.0);
        let sample_rate = finite_or(sample_rate, 60.0).max(1.0);
        let samples = ceil(duration * sample_rate).max(1.0) as usize;
        let mut frames = Vec::with_capacity(samples + 1);
        for index in 0..=samples {
            let t = duration * index as f32 / samples as f32;
            frames.push(Keyframe::new(t, self.sample(t), Easing::Linear));
        }
        KeyframeTrack::from_sorted_frames(frames)
    }
}

fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

fn cycle(time: f32, frequency: f32) -> f32 {
    let scaled = (time.max(0.0) * finite_or(frequency, 1.0).max(0.0)).max(0.0);
    scaled - scaled as u32 as f32
}

fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn hash_noise(seed: u32, index: u32) -> f32 {
    let mut x = seed ^ index.wrapping_mul(0x9E37_79B9);
    x ^= x >> 16;
    x = x.wrapping_mul(0x7FEB_352D);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846C_A68B);
    x ^= x >> 16;
    let unit = (x as f32) / (u32::MAX as f32);
    unit * 2.0 - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine_respects_frequency_amplitude_and_phase() {
        let wave = Waveform::Sine {
            frequency: 1.0,
            amplitude: 2.0,
            phase: 0.0,
        };
        assert!(wave.sample(0.0).abs() < 0.0001);
        assert!((wave.sample(0.25) - 2.0).abs() < 0.0001);
    }

    #[test]
    fn square_respects_duty_cycle() {
        let wave = Waveform::Square {
            frequency: 1.0,
            amplitude: 3.0,
            duty_cycle: 0.25,
        };
        assert_eq!(wave.sample(0.1), 3.0);
        assert_eq!(wave.sample(0.3), -3.0);
    }

    #[test]
    fn noise_is_deterministic_and_bounded() {
        let wave = Waveform::Noise {
            seed: 42,
            smoothness: 0.25,
        };
        let a = wave.sample(0.123);
        let b = wave.sample(0.123);
        assert_eq!(a, b);
        assert!((-1.0..=1.0).contains(&a));
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn waveform_converts_to_keyframes() {
        let wave = Waveform::Triangle {
            frequency: 1.0,
            amplitude: 1.0,
        };
        let track = wave.to_keyframe_track(1.0, 4.0);
        assert_eq!(track.frames().len(), 5);
        assert_eq!(track.duration(), 1.0);
    }
}
