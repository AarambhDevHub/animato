//! Gesture recognition for pointer input.

use crate::drag::PointerData;

const RAD_TO_DEG: f32 = 180.0 / core::f32::consts::PI;
const PINCH_EPSILON: f32 = 0.01;
const ROTATION_EPSILON_DEG: f32 = 1.0;

/// Gesture recognition thresholds.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GestureConfig {
    /// Maximum pointer travel for a tap or long press.
    pub tap_max_distance: f32,
    /// Maximum duration, in seconds, for a tap.
    pub tap_max_duration: f32,
    /// Minimum pointer travel for a swipe.
    pub swipe_min_distance: f32,
    /// Minimum duration, in seconds, for a long press.
    pub long_press_duration: f32,
    /// Maximum interval, in seconds, between taps for a double tap.
    pub double_tap_max_interval: f32,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            tap_max_distance: 8.0,
            tap_max_duration: 0.25,
            swipe_min_distance: 40.0,
            long_press_duration: 0.5,
            double_tap_max_interval: 0.3,
        }
    }
}

/// Direction of a recognized swipe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SwipeDirection {
    /// Swipe toward negative y.
    Up,
    /// Swipe toward positive y.
    Down,
    /// Swipe toward negative x.
    Left,
    /// Swipe toward positive x.
    Right,
}

/// Gesture emitted by [`GestureRecognizer`].
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Gesture {
    /// Single tap at a position.
    Tap {
        /// Tap position.
        position: [f32; 2],
    },
    /// Two taps close together.
    DoubleTap {
        /// Position of the second tap.
        position: [f32; 2],
    },
    /// Pointer held in place long enough.
    LongPress {
        /// Release position.
        position: [f32; 2],
        /// Press duration in seconds.
        duration: f32,
    },
    /// Fast directional pointer movement.
    Swipe {
        /// Dominant swipe direction.
        direction: SwipeDirection,
        /// Average swipe velocity in units per second.
        velocity: f32,
        /// Swipe distance in units.
        distance: f32,
    },
    /// Two-pointer pinch with scale relative to the start distance.
    Pinch {
        /// Scale relative to the initial two-pointer distance.
        scale: f32,
        /// Center point between the two pointers.
        center: [f32; 2],
    },
    /// Two-pointer rotation in degrees.
    Rotation {
        /// Angle delta in degrees.
        angle_delta: f32,
        /// Center point between the two pointers.
        center: [f32; 2],
    },
}

/// Pointer gesture recognizer.
///
/// Feed pointer down, move, and up samples with monotonically increasing
/// timestamps in seconds. Single-pointer gestures emit on pointer up. Pinch and
/// rotation emit on the first pointer up after a two-pointer interaction.
#[derive(Clone, Debug)]
pub struct GestureRecognizer {
    config: GestureConfig,
    active: [Option<PointerTrack>; 2],
    two_start_distance: f32,
    two_start_angle: f32,
    last_tap: Option<TapRecord>,
}

impl Default for GestureRecognizer {
    fn default() -> Self {
        Self::new(GestureConfig::default())
    }
}

impl GestureRecognizer {
    /// Create a recognizer with custom thresholds.
    pub fn new(config: GestureConfig) -> Self {
        Self {
            config,
            active: [None, None],
            two_start_distance: 0.0,
            two_start_angle: 0.0,
            last_tap: None,
        }
    }

    /// Return the current configuration.
    pub fn config(&self) -> GestureConfig {
        self.config
    }

    /// Start tracking a pointer.
    pub fn on_pointer_down(&mut self, data: PointerData, time_seconds: f32) {
        let time_seconds = time_seconds.max(0.0);
        if let Some(index) = self.find_index(data.pointer_id) {
            self.active[index] = Some(PointerTrack::new(data, time_seconds));
            self.refresh_two_pointer_start();
            return;
        }

        if let Some(slot) = self.active.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(PointerTrack::new(data, time_seconds));
            self.refresh_two_pointer_start();
        }
    }

    /// Update an active pointer.
    pub fn on_pointer_move(&mut self, data: PointerData, time_seconds: f32) {
        if let Some(index) = self.find_index(data.pointer_id)
            && let Some(track) = &mut self.active[index]
        {
            track.last = data.position();
            track.last_time = time_seconds.max(track.start_time);
        }
    }

    /// Stop tracking a pointer and emit a gesture when one is recognized.
    pub fn on_pointer_up(&mut self, data: PointerData, time_seconds: f32) -> Option<Gesture> {
        let index = self.find_index(data.pointer_id)?;
        let mut released = self.active[index]?;
        released.last = data.position();
        released.last_time = time_seconds.max(released.start_time);

        if self.active_count() == 2 {
            let gesture = self.two_pointer_gesture(index, released);
            self.active[index] = None;
            self.refresh_two_pointer_start();
            return gesture;
        }

        self.active[index] = None;
        self.single_pointer_gesture(released)
    }

    fn single_pointer_gesture(&mut self, track: PointerTrack) -> Option<Gesture> {
        let duration = (track.last_time - track.start_time).max(0.0);
        let delta = [
            track.last[0] - track.start[0],
            track.last[1] - track.start[1],
        ];
        let distance = length(delta);

        if duration >= self.config.long_press_duration && distance <= self.config.tap_max_distance {
            return Some(Gesture::LongPress {
                position: track.last,
                duration,
            });
        }

        if distance >= self.config.swipe_min_distance {
            return Some(Gesture::Swipe {
                direction: swipe_direction(delta),
                velocity: if duration > 0.0 {
                    distance / duration
                } else {
                    0.0
                },
                distance,
            });
        }

        if duration <= self.config.tap_max_duration && distance <= self.config.tap_max_distance {
            let position = track.last;
            if let Some(last) = self.last_tap
                && track.last_time - last.time <= self.config.double_tap_max_interval
                && length([
                    position[0] - last.position[0],
                    position[1] - last.position[1],
                ]) <= self.config.tap_max_distance
            {
                self.last_tap = None;
                return Some(Gesture::DoubleTap { position });
            }

            self.last_tap = Some(TapRecord {
                position,
                time: track.last_time,
            });
            return Some(Gesture::Tap { position });
        }

        None
    }

    fn two_pointer_gesture(
        &self,
        released_index: usize,
        released: PointerTrack,
    ) -> Option<Gesture> {
        let other = self.active.iter().enumerate().find_map(|(index, track)| {
            if index != released_index {
                *track
            } else {
                None
            }
        })?;

        if self.two_start_distance <= f32::EPSILON {
            return None;
        }

        let current_distance = distance(released.last, other.last);
        let scale = current_distance / self.two_start_distance;
        let current_angle = angle_between(released.last, other.last);
        let angle_delta = normalize_degrees(current_angle - self.two_start_angle);
        let center = midpoint(released.last, other.last);
        let scale_delta = (scale - 1.0).abs();

        if angle_delta.abs() >= ROTATION_EPSILON_DEG && angle_delta.abs() / 45.0 >= scale_delta {
            Some(Gesture::Rotation {
                angle_delta,
                center,
            })
        } else if scale_delta >= PINCH_EPSILON {
            Some(Gesture::Pinch { scale, center })
        } else {
            None
        }
    }

    fn find_index(&self, pointer_id: u64) -> Option<usize> {
        self.active.iter().position(|track| {
            track
                .map(|track| track.pointer_id == pointer_id)
                .unwrap_or(false)
        })
    }

    fn active_count(&self) -> usize {
        self.active.iter().filter(|track| track.is_some()).count()
    }

    fn refresh_two_pointer_start(&mut self) {
        if self.active_count() != 2 {
            self.two_start_distance = 0.0;
            self.two_start_angle = 0.0;
            return;
        }

        let first = self.active.iter().flatten().next().unwrap().last;
        let second = self.active.iter().flatten().nth(1).unwrap().last;
        self.two_start_distance = distance(first, second);
        self.two_start_angle = angle_between(first, second);
    }
}

#[derive(Clone, Copy, Debug)]
struct PointerTrack {
    pointer_id: u64,
    start: [f32; 2],
    last: [f32; 2],
    start_time: f32,
    last_time: f32,
}

impl PointerTrack {
    fn new(data: PointerData, time: f32) -> Self {
        Self {
            pointer_id: data.pointer_id,
            start: data.position(),
            last: data.position(),
            start_time: time,
            last_time: time,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct TapRecord {
    position: [f32; 2],
    time: f32,
}

#[inline]
fn distance(a: [f32; 2], b: [f32; 2]) -> f32 {
    length([b[0] - a[0], b[1] - a[1]])
}

#[inline]
fn length(vector: [f32; 2]) -> f32 {
    libm::sqrtf(vector[0] * vector[0] + vector[1] * vector[1])
}

#[inline]
fn angle_between(a: [f32; 2], b: [f32; 2]) -> f32 {
    libm::atan2f(b[1] - a[1], b[0] - a[0]) * RAD_TO_DEG
}

#[inline]
fn midpoint(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
    [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5]
}

#[inline]
fn normalize_degrees(mut angle: f32) -> f32 {
    while angle > 180.0 {
        angle -= 360.0;
    }
    while angle < -180.0 {
        angle += 360.0;
    }
    angle
}

#[inline]
fn swipe_direction(delta: [f32; 2]) -> SwipeDirection {
    if delta[0].abs() >= delta[1].abs() {
        if delta[0] >= 0.0 {
            SwipeDirection::Right
        } else {
            SwipeDirection::Left
        }
    } else if delta[1] >= 0.0 {
        SwipeDirection::Down
    } else {
        SwipeDirection::Up
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_tap() {
        let mut recognizer = GestureRecognizer::default();
        recognizer.on_pointer_down(PointerData::new(10.0, 20.0, 1), 0.0);
        let gesture = recognizer.on_pointer_up(PointerData::new(12.0, 20.0, 1), 0.1);
        assert_eq!(
            gesture,
            Some(Gesture::Tap {
                position: [12.0, 20.0]
            })
        );
    }

    #[test]
    fn recognizes_double_tap() {
        let mut recognizer = GestureRecognizer::default();
        recognizer.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
        recognizer.on_pointer_up(PointerData::new(0.0, 0.0, 1), 0.1);
        recognizer.on_pointer_down(PointerData::new(1.0, 1.0, 1), 0.2);
        let gesture = recognizer.on_pointer_up(PointerData::new(1.0, 1.0, 1), 0.25);
        assert_eq!(
            gesture,
            Some(Gesture::DoubleTap {
                position: [1.0, 1.0]
            })
        );
    }

    #[test]
    fn recognizes_long_press() {
        let mut recognizer = GestureRecognizer::default();
        recognizer.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
        let gesture = recognizer.on_pointer_up(PointerData::new(1.0, 1.0, 1), 0.7);
        assert!(matches!(gesture, Some(Gesture::LongPress { .. })));
    }

    #[test]
    fn recognizes_swipe() {
        let mut recognizer = GestureRecognizer::default();
        recognizer.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
        let gesture = recognizer.on_pointer_up(PointerData::new(80.0, 10.0, 1), 0.2);
        assert!(matches!(
            gesture,
            Some(Gesture::Swipe {
                direction: SwipeDirection::Right,
                ..
            })
        ));
    }

    #[test]
    fn recognizes_pinch() {
        let mut recognizer = GestureRecognizer::default();
        recognizer.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
        recognizer.on_pointer_down(PointerData::new(10.0, 0.0, 2), 0.0);
        recognizer.on_pointer_move(PointerData::new(20.0, 0.0, 2), 0.1);
        let gesture = recognizer.on_pointer_up(PointerData::new(0.0, 0.0, 1), 0.2);
        assert_eq!(
            gesture,
            Some(Gesture::Pinch {
                scale: 2.0,
                center: [10.0, 0.0]
            })
        );
    }

    #[test]
    fn recognizes_rotation() {
        let mut recognizer = GestureRecognizer::default();
        recognizer.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
        recognizer.on_pointer_down(PointerData::new(10.0, 0.0, 2), 0.0);
        recognizer.on_pointer_move(PointerData::new(0.0, 10.0, 2), 0.1);
        let gesture = recognizer.on_pointer_up(PointerData::new(0.0, 0.0, 1), 0.2);
        assert!(matches!(
            gesture,
            Some(Gesture::Rotation {
                angle_delta,
                ..
            }) if (angle_delta - 90.0).abs() < 0.01
        ));
    }
}
