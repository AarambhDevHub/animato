//! Builder for [`Tween<T>`].

use crate::loop_mode::Loop;
use crate::tween::Tween;
use motus_core::{Animatable, Easing};

/// Consuming builder for [`Tween<T>`].
///
/// Obtained via [`Tween::new`]. All fields have sensible defaults —
/// only `start` and `end` are required.
///
/// # Example
///
/// ```rust
/// use motus_tween::Tween;
/// use motus_core::Easing;
///
/// let tween = Tween::new(0.0_f32, 100.0)
///     .duration(1.5)
///     .easing(Easing::EaseOutBack)
///     .delay(0.1)
///     .time_scale(1.0)
///     .build();
/// ```
pub struct TweenBuilder<T: Animatable> {
    start: T,
    end: T,
    duration: f32,
    easing: Easing,
    delay: f32,
    time_scale: f32,
    looping: Loop,
}

impl<T: Animatable + core::fmt::Debug> core::fmt::Debug for TweenBuilder<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TweenBuilder")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("duration", &self.duration)
            .field("easing", &self.easing)
            .field("delay", &self.delay)
            .field("time_scale", &self.time_scale)
            .field("looping", &self.looping)
            .finish()
    }
}

impl<T: Animatable> TweenBuilder<T> {
    /// Create a builder animating from `start` to `end`.
    #[inline]
    pub fn new(start: T, end: T) -> Self {
        Self {
            start,
            end,
            duration: 1.0,
            easing: Easing::Linear,
            delay: 0.0,
            time_scale: 1.0,
            looping: Loop::Once,
        }
    }

    /// Set the animation duration in seconds. Negative values are clamped to `0`.
    #[inline]
    pub fn duration(mut self, secs: f32) -> Self {
        self.duration = secs.max(0.0);
        self
    }

    /// Set the easing curve.
    #[inline]
    pub fn easing(mut self, e: Easing) -> Self {
        self.easing = e;
        self
    }

    /// Set a pre-animation delay in seconds. The value is held at `start` during this period.
    #[inline]
    pub fn delay(mut self, secs: f32) -> Self {
        self.delay = secs.max(0.0);
        self
    }

    /// Set the time scale multiplier (`1.0` = normal speed, `2.0` = double speed).
    #[inline]
    pub fn time_scale(mut self, s: f32) -> Self {
        self.time_scale = s.max(0.0);
        self
    }

    /// Set the looping mode.
    #[inline]
    pub fn looping(mut self, mode: Loop) -> Self {
        self.looping = mode;
        self
    }

    /// Consume the builder and return the configured [`Tween<T>`].
    #[inline]
    pub fn build(self) -> Tween<T> {
        Tween::from_builder(
            self.start,
            self.end,
            self.duration,
            self.easing,
            self.delay,
            self.time_scale,
            self.looping,
        )
    }
}

impl<T: Animatable> Tween<T> {
    /// Begin building a tween from `start` to `end`.
    ///
    /// ```rust
    /// use motus_tween::Tween;
    /// use motus_core::Easing;
    ///
    /// let t = Tween::new(0.0_f32, 1.0)
    ///     .duration(2.0)
    ///     .easing(Easing::EaseInOutSine)
    ///     .build();
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new(start: T, end: T) -> TweenBuilder<T> {
        TweenBuilder::new(start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use motus_core::Update;

    #[test]
    fn builder_defaults() {
        let t = Tween::new(0.0_f32, 1.0).build();
        assert_eq!(t.duration, 1.0);
        assert_eq!(t.delay, 0.0);
        assert_eq!(t.time_scale, 1.0);
        assert_eq!(t.looping, Loop::Once);
    }

    #[test]
    fn builder_chains() {
        let mut t = Tween::new(0.0_f32, 100.0)
            .duration(2.0)
            .delay(0.5)
            .time_scale(2.0)
            .looping(Loop::Forever)
            .build();
        assert_eq!(t.duration, 2.0);
        // time_scale=2.0: 0.5s of dt advances 1.0s of animation time
        // delay=0.5, so after 0.5s of dt → running starts
        t.update(0.5); // exhausts delay
        t.update(0.5); // 0.5 * time_scale(2) = 1.0s of anim time
        assert!(t.value() > 0.0);
    }

    #[test]
    fn negative_duration_clamped() {
        let t = Tween::new(0.0_f32, 1.0).duration(-5.0).build();
        assert_eq!(t.duration, 0.0);
    }

    #[test]
    fn negative_delay_clamped() {
        let t = Tween::new(0.0_f32, 1.0).delay(-1.0).build();
        assert_eq!(t.delay, 0.0);
    }
}
