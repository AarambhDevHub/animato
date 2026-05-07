//! Core [`Tween<T>`] type and [`TweenState`] enum.

use crate::loop_mode::Loop;
use animato_core::{Animatable, Easing, Playable, Update};

/// The current execution state of a [`Tween`].
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TweenState {
    /// Waiting for the delay period to expire before starting.
    Idle,
    /// Actively animating.
    Running,
    /// Paused mid-animation — `update()` calls are no-ops.
    Paused,
    /// Finished. Further `update()` calls return `false` immediately.
    Completed,
}

/// A single-value animation from `start` to `end` over `duration` seconds.
///
/// Build with [`Tween::new`] and the consuming builder chain:
///
/// ```rust
/// use animato_tween::Tween;
/// use animato_core::Easing;
///
/// let mut t = Tween::new(0.0_f32, 100.0)
///     .duration(1.5)
///     .easing(Easing::EaseOutCubic)
///     .delay(0.2)
///     .build();
/// ```
///
/// # `no_std`
///
/// `Tween<T>` is stack-allocated — no heap allocation occurs in `update()` or `value()`.
#[derive(Clone, Debug)]
pub struct Tween<T: Animatable> {
    /// The value at `t = 0`.
    pub start: T,
    /// The value at `t = duration`.
    pub end: T,
    /// Total animation duration in seconds. Clamped to `≥ 0`.
    pub duration: f32,
    /// Easing curve applied to the normalised progress.
    pub easing: Easing,
    /// Delay in seconds before the animation begins.
    pub delay: f32,
    /// Time scale multiplier. `1.0` = normal, `2.0` = double speed.
    pub time_scale: f32,
    /// Looping behaviour.
    pub looping: Loop,

    // ── private state ────────────────────────────────────────────────────────
    elapsed: f32,
    delay_elapsed: f32,
    state: TweenState,
    loop_count: u32,
    /// When PingPong is active and we're in the backward pass.
    ping_pong_reverse: bool,
}

impl<T: Animatable> Tween<T> {
    // ── Construction (called by TweenBuilder) ────────────────────────────────

    /// Create via [`TweenBuilder`](crate::TweenBuilder) — use `Tween::new(start, end)`.
    #[doc(hidden)]
    pub(crate) fn from_builder(
        start: T,
        end: T,
        duration: f32,
        easing: Easing,
        delay: f32,
        time_scale: f32,
        looping: Loop,
    ) -> Self {
        let initial_state = if delay > 0.0 {
            TweenState::Idle
        } else {
            TweenState::Running
        };
        Self {
            start,
            end,
            duration: duration.max(0.0),
            easing,
            delay: delay.max(0.0),
            time_scale: time_scale.max(0.0),
            looping,
            elapsed: 0.0,
            delay_elapsed: 0.0,
            state: initial_state,
            loop_count: 0,
            ping_pong_reverse: false,
        }
    }

    // ── Public API ───────────────────────────────────────────────────────────

    /// The current interpolated value.
    ///
    /// This is the hot path — no allocation, just a lerp.
    ///
    /// ```rust
    /// use animato_tween::Tween;
    /// use animato_core::Easing;
    ///
    /// let t = Tween::new(0.0_f32, 100.0).duration(1.0).build();
    /// assert_eq!(t.value(), 0.0); // hasn't started yet
    /// ```
    pub fn value(&self) -> T {
        if self.duration == 0.0 {
            return self.end.clone();
        }
        let raw_t = (self.elapsed / self.duration).clamp(0.0, 1.0);
        let curved_t = self.easing.apply(raw_t);
        if self.ping_pong_reverse {
            self.end.lerp(&self.start, curved_t)
        } else {
            self.start.lerp(&self.end, curved_t)
        }
    }

    /// Normalised progress in `[0.0, 1.0]` — raw, before easing.
    pub fn progress(&self) -> f32 {
        if self.duration == 0.0 {
            return 1.0;
        }
        (self.elapsed / self.duration).clamp(0.0, 1.0)
    }

    /// Normalised progress after easing is applied.
    pub fn eased_progress(&self) -> f32 {
        self.easing.apply(self.progress())
    }

    /// `true` when the tween has finished all its loops.
    pub fn is_complete(&self) -> bool {
        self.state == TweenState::Completed
    }

    /// Current execution state.
    pub fn state(&self) -> &TweenState {
        &self.state
    }

    /// Reset to the very beginning, including delay and loop counter.
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.delay_elapsed = 0.0;
        self.loop_count = 0;
        self.ping_pong_reverse = false;
        self.state = if self.delay > 0.0 {
            TweenState::Idle
        } else {
            TweenState::Running
        };
    }

    /// Jump to a normalised time `t ∈ [0, 1]` within the current pass.
    ///
    /// Does not affect loop count or ping-pong direction.
    pub fn seek(&mut self, t: f32) {
        self.elapsed = (t.clamp(0.0, 1.0) * self.duration).max(0.0);
        if self.state == TweenState::Completed {
            self.state = TweenState::Running;
        }
    }

    /// Swap `start` and `end` in place.
    ///
    /// The animation immediately plays toward the new `end`.
    ///
    /// When called mid-animation, the current visual position is preserved —
    /// `elapsed` is mirrored so the object appears to continue from where it is.
    ///
    /// # Example
    ///
    /// ```rust
    /// use animato_tween::Tween;
    /// use animato_core::{Easing, Update};
    ///
    /// let mut t = Tween::new(0.0_f32, 100.0)
    ///     .duration(1.0)
    ///     .easing(Easing::Linear)
    ///     .build();
    /// // Advance 30% of the way
    /// t.update(0.3);
    /// assert!((t.value() - 30.0).abs() < 1.0);
    /// // Reverse — now at 70% of the backward journey (100→0)
    /// t.reverse();
    /// assert!((t.value() - 30.0).abs() < 1.0); // same visual position
    /// ```
    pub fn reverse(&mut self) {
        core::mem::swap(&mut self.start, &mut self.end);
        // Mirror elapsed: preserves the current visual position after swap.
        // e.g. 30% forward → 70% backward, same screen position.
        self.elapsed = (self.duration - self.elapsed).clamp(0.0, self.duration);
        if self.state == TweenState::Completed {
            self.state = TweenState::Running;
        }
    }

    /// Pause — `update()` calls become no-ops until [`resume`](Self::resume).
    pub fn pause(&mut self) {
        if self.state == TweenState::Running {
            self.state = TweenState::Paused;
        }
    }

    /// Resume from a paused state.
    pub fn resume(&mut self) {
        if self.state == TweenState::Paused {
            self.state = TweenState::Running;
        }
    }

    #[inline]
    fn playback_duration(&self) -> f32 {
        match self.looping {
            Loop::Once => self.delay + self.duration,
            Loop::Times(n) => self.delay + self.duration * n.max(1) as f32,
            Loop::Forever | Loop::PingPong => f32::INFINITY,
        }
    }
}

impl<T: Animatable> Update for Tween<T> {
    /// Advance the tween by `dt` seconds.
    ///
    /// Returns `true` while still running, `false` when complete.
    /// Negative `dt` is treated as `0.0`.
    fn update(&mut self, dt: f32) -> bool {
        let dt = dt.max(0.0);

        match self.state {
            TweenState::Completed => return false,
            TweenState::Paused => return true,
            TweenState::Idle => {
                // Drain delay bucket
                self.delay_elapsed += dt;
                if self.delay_elapsed < self.delay {
                    return true;
                }
                // Carry the overflow into running time
                let overflow = self.delay_elapsed - self.delay;
                self.state = TweenState::Running;
                self.elapsed += overflow * self.time_scale;
            }
            TweenState::Running => {
                self.elapsed += dt * self.time_scale;
            }
        }

        // Zero-duration tween completes immediately
        if self.duration == 0.0 {
            self.state = TweenState::Completed;
            return false;
        }

        // Handle loop overflow
        while self.elapsed >= self.duration {
            match &self.looping {
                Loop::Once => {
                    self.elapsed = self.duration;
                    self.state = TweenState::Completed;
                    return false;
                }
                Loop::Times(n) => {
                    self.loop_count += 1;
                    if self.loop_count >= *n {
                        self.elapsed = self.duration;
                        self.state = TweenState::Completed;
                        return false;
                    }
                    self.elapsed -= self.duration;
                }
                Loop::Forever => {
                    self.elapsed -= self.duration;
                }
                Loop::PingPong => {
                    self.elapsed -= self.duration;
                    self.ping_pong_reverse = !self.ping_pong_reverse;
                }
            }
        }

        true
    }
}

impl<T: Animatable> Playable for Tween<T> {
    fn duration(&self) -> f32 {
        self.playback_duration()
    }

    fn reset(&mut self) {
        Tween::reset(self);
    }

    fn seek_to(&mut self, progress: f32) {
        let progress = progress.clamp(0.0, 1.0);
        let total = self.playback_duration();
        let finite_total = if total.is_finite() {
            total
        } else {
            self.delay + self.duration
        };

        Tween::reset(self);

        if finite_total == 0.0 {
            self.elapsed = self.duration;
            self.state = TweenState::Completed;
            return;
        }

        let secs = finite_total * progress;
        if secs < self.delay {
            self.delay_elapsed = secs;
            self.state = if self.delay > 0.0 {
                TweenState::Idle
            } else {
                TweenState::Running
            };
            return;
        }

        let anim_secs = (secs - self.delay).max(0.0);
        if self.duration == 0.0 {
            self.elapsed = 0.0;
            self.state = TweenState::Completed;
            return;
        }

        match self.looping {
            Loop::Once => {
                self.elapsed = anim_secs.min(self.duration);
                self.state = if progress >= 1.0 {
                    TweenState::Completed
                } else {
                    TweenState::Running
                };
            }
            Loop::Times(n) => {
                let plays = n.max(1);
                let total_anim = self.duration * plays as f32;
                if anim_secs >= total_anim || progress >= 1.0 {
                    self.loop_count = plays;
                    self.elapsed = self.duration;
                    self.state = TweenState::Completed;
                } else {
                    self.loop_count = (anim_secs / self.duration) as u32;
                    self.elapsed = anim_secs - self.duration * self.loop_count as f32;
                    self.state = TweenState::Running;
                }
            }
            Loop::Forever => {
                self.elapsed = anim_secs % self.duration;
                self.state = TweenState::Running;
            }
            Loop::PingPong => {
                let cycle = anim_secs % (self.duration * 2.0);
                self.ping_pong_reverse = cycle >= self.duration;
                self.elapsed = if self.ping_pong_reverse {
                    cycle - self.duration
                } else {
                    cycle
                };
                self.state = TweenState::Running;
            }
        }
    }

    fn is_complete(&self) -> bool {
        Tween::is_complete(self)
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loop_mode::Loop;
    use animato_core::Easing;

    fn make(start: f32, end: f32, duration: f32) -> Tween<f32> {
        Tween::new(start, end).duration(duration).build()
    }

    #[test]
    fn value_at_start_equals_start() {
        let t = make(10.0, 90.0, 2.0);
        assert_eq!(t.value(), 10.0);
    }

    #[test]
    fn value_at_end_equals_end() {
        let mut t = make(10.0, 90.0, 1.0);
        t.update(1.0);
        assert_eq!(t.value(), 90.0);
    }

    #[test]
    fn is_complete_after_full_duration() {
        let mut t = make(0.0, 1.0, 1.0);
        t.update(1.0);
        assert!(t.is_complete());
    }

    #[test]
    fn large_dt_completes_cleanly() {
        let mut t = make(0.0, 1.0, 1.0);
        t.update(100.0);
        assert!(t.is_complete());
        assert_eq!(t.value(), 1.0);
    }

    #[test]
    fn no_update_after_complete() {
        let mut t = make(0.0, 1.0, 0.5);
        t.update(1.0);
        assert!(!t.update(1.0)); // still returns false, no panic
    }

    #[test]
    fn delay_holds_at_start() {
        let mut t = Tween::new(0.0_f32, 100.0).duration(1.0).delay(0.5).build();
        t.update(0.25); // still in delay
        assert_eq!(t.value(), 0.0);
        assert_eq!(t.state(), &TweenState::Idle);
    }

    #[test]
    fn delay_transitions_to_running() {
        let mut t = Tween::new(0.0_f32, 100.0).duration(1.0).delay(0.5).build();
        t.update(0.5); // exactly at delay end → now Running
        assert_eq!(t.state(), &TweenState::Running);
    }

    #[test]
    fn seek_jumps_to_midpoint() {
        let mut t = make(0.0, 100.0, 1.0);
        t.seek(0.5);
        // Linear easing: midpoint = 50.0
        let t2 = Tween::new(0.0_f32, 100.0)
            .duration(1.0)
            .easing(Easing::Linear)
            .build();
        let mut t2 = t2;
        t2.seek(0.5);
        assert!((t2.value() - 50.0).abs() < 0.01);
    }

    #[test]
    fn reverse_swaps_direction() {
        let mut t = Tween::new(0.0_f32, 100.0)
            .duration(1.0)
            .easing(Easing::Linear)
            .build();
        // Advance 40% forward
        t.update(0.4);
        let before = t.value(); // ~40.0
        // Reverse: visual position preserved, now animating toward 0
        t.reverse();
        // Immediately after reverse, same visual position
        assert!(
            (t.value() - before).abs() < 1.0,
            "visual position should be preserved: before={} after={}",
            before,
            t.value()
        );
        // One more step: value should decrease
        t.update(0.1);
        assert!(t.value() < before, "value should decrease after reverse");
    }

    #[test]
    fn pause_stops_progress() {
        let mut t = make(0.0, 1.0, 2.0);
        t.update(0.5);
        let v_before = t.value();
        t.pause();
        t.update(0.5); // should not advance
        assert_eq!(t.value(), v_before);
    }

    #[test]
    fn resume_continues_progress() {
        let mut t = make(0.0, 1.0, 2.0);
        t.update(0.5);
        t.pause();
        t.update(0.5); // no-op while paused
        let v_paused = t.value();
        t.resume();
        t.update(0.5); // should advance
        assert!(
            t.value() > v_paused,
            "resumed tween must advance past v_paused={}",
            v_paused
        );
    }

    #[test]
    fn loop_times_completes_after_n() {
        let mut t = Tween::new(0.0_f32, 1.0)
            .duration(1.0)
            .looping(Loop::Times(3))
            .build();
        // 3 × 1.0s + small epsilon to push past the 3rd boundary
        t.update(3.0 + f32::EPSILON);
        assert!(t.is_complete());
    }

    #[test]
    fn loop_forever_never_completes() {
        let mut t = Tween::new(0.0_f32, 1.0)
            .duration(1.0)
            .looping(Loop::Forever)
            .build();
        for _ in 0..1000 {
            t.update(0.1);
        }
        assert!(!t.is_complete());
    }

    #[test]
    fn pingpong_reverses_direction() {
        let mut t = Tween::new(0.0_f32, 100.0)
            .duration(1.0)
            .easing(Easing::Linear)
            .looping(Loop::PingPong)
            .build();
        // Forward pass → value should be 100 at t=1.0
        t.update(1.0);
        // Now in reverse: at halfway through backward pass value should be ~50
        t.update(0.5);
        let v = t.value();
        assert!(v > 40.0 && v < 60.0, "pingpong mid-reverse = {}", v);
    }

    #[test]
    fn reset_returns_to_idle_with_delay() {
        let mut t = Tween::new(0.0_f32, 1.0).duration(1.0).delay(0.5).build();
        t.update(2.0); // complete
        t.reset();
        assert_eq!(t.state(), &TweenState::Idle);
        assert_eq!(t.value(), 0.0);
    }

    #[test]
    fn zero_duration_completes_immediately() {
        let mut t = make(0.0, 100.0, 0.0);
        t.update(0.0);
        assert!(t.is_complete());
        assert_eq!(t.value(), 100.0);
    }

    #[test]
    fn negative_dt_is_noop() {
        let mut t = make(0.0, 100.0, 1.0);
        t.update(-5.0);
        assert_eq!(t.value(), 0.0);
    }
}
