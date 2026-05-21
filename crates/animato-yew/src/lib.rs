//! # animato-yew
//!
//! First-class Yew integration for Animato.
//!
//! This crate provides `UseStateHandle`-backed animation hooks, CSS style
//! interpolation, scroll helpers, presence wrappers, route transitions, keyed
//! list scaffolding, gesture bindings, and a serializable `f32` coordination
//! runtime for browser agents. Hooks keep Animato's renderer-agnostic engines
//! local to the component and schedule `requestAnimationFrame` only while an
//! animation is active.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(feature = "agent")]
mod agent;
#[cfg(feature = "css")]
mod css;
#[cfg(feature = "gesture")]
mod gesture;
mod hooks;
#[cfg(feature = "list")]
mod list;
#[cfg(feature = "presence")]
mod presence;
mod raf;
#[cfg(feature = "scroll")]
mod scroll;
#[cfg(feature = "transition")]
mod transition;

#[cfg(feature = "agent")]
pub use agent::{
    AgentInput, AgentOutput, AgentSpringSpec, AgentTweenSpec, AnimationAgent, AnimationAgentHandle,
    use_animation_agent,
};
#[cfg(feature = "css")]
pub use css::{
    AnimatedStyle, use_css_spring, use_css_spring as css_spring, use_css_tween,
    use_css_tween as css_tween,
};
#[cfg(feature = "gesture")]
pub use gesture::{
    DragAxis, DragConfig, DragConstraints, DragHandle, Gesture, GestureConfig, PinchHandle,
    SwipeConfig, SwipeDirection, SwipeEvent, use_drag, use_gesture, use_pinch, use_swipe,
};
pub use hooks::{
    KeyframeHandle, SpringHandle, TimelineHandle, TweenHandle, use_keyframes, use_spring,
    use_timeline, use_tween,
};
#[cfg(feature = "list")]
pub use list::{AnimatedFor, AnimatedForProps, stable_key};
#[cfg(feature = "presence")]
pub use presence::{AnimatePresence, PresenceAnimation};
#[cfg(feature = "scroll")]
pub use scroll::{
    ScrollAxis, ScrollConfig, ScrollProgressCalculator, ScrollTriggerConfig, ScrollTriggerHandle,
    SmoothScroll, use_scroll_progress, use_scroll_trigger, use_scroll_velocity,
};
#[cfg(feature = "transition")]
pub use transition::{
    PageTransition, TransitionMode, use_route_transition_key,
    use_route_transition_key as route_transition_key,
};

pub(crate) fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn now_ms() -> f64 {
    0.0
}
