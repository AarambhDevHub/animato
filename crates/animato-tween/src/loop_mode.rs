//! Loop mode for [`Tween`](crate::Tween).

/// Controls how a tween behaves when it reaches the end of its duration.
///
/// # Example
///
/// ```rust
/// use animato_tween::Loop;
///
/// let mode = Loop::PingPong;
/// ```
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Loop {
    /// Play once and stop. Default.
    Once,
    /// Play exactly `n` times, then stop.
    Times(u32),
    /// Play forward repeatedly, forever.
    Forever,
    /// Play forward, then backward, then forward — forever.
    PingPong,
}
