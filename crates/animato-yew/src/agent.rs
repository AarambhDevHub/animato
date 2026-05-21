//! Serializable `f32` animation coordination for Yew.

use crate::raf::RafLoop;
use animato_core::{Easing, Update};
use animato_spring::{Spring, SpringConfig};
use animato_tween::Tween;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use yew::prelude::{Callback, UseStateHandle, hook, use_state_eq};

/// Marker type for the Yew animation agent integration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AnimationAgent;

/// Serializable `f32` tween request.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "agent", derive(serde::Serialize, serde::Deserialize))]
pub struct AgentTweenSpec {
    /// Unique channel id.
    pub id: String,
    /// Start value.
    pub from: f32,
    /// End value.
    pub to: f32,
    /// Duration in seconds.
    pub duration: f32,
    /// Easing curve.
    pub easing: Easing,
}

impl AgentTweenSpec {
    /// Create a tween request.
    pub fn new(id: impl Into<String>, from: f32, to: f32) -> Self {
        Self {
            id: id.into(),
            from,
            to,
            duration: 0.3,
            easing: Easing::EaseOutCubic,
        }
    }

    /// Set duration in seconds.
    pub fn duration(mut self, duration: f32) -> Self {
        self.duration = crate::finite_or(duration, 0.3).max(0.0);
        self
    }

    /// Set easing.
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}

/// Serializable `f32` spring request.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "agent", derive(serde::Serialize, serde::Deserialize))]
pub struct AgentSpringSpec {
    /// Unique channel id.
    pub id: String,
    /// Initial/current value.
    pub from: f32,
    /// Target value.
    pub to: f32,
    /// Spring parameters.
    pub config: SpringConfig,
}

impl PartialEq for AgentSpringSpec {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.from == other.from
            && self.to == other.to
            && spring_config_eq(&self.config, &other.config)
    }
}

impl AgentSpringSpec {
    /// Create a spring request.
    pub fn new(id: impl Into<String>, from: f32, to: f32) -> Self {
        Self {
            id: id.into(),
            from,
            to,
            config: SpringConfig::snappy(),
        }
    }

    /// Set spring parameters.
    pub fn config(mut self, config: SpringConfig) -> Self {
        self.config = config;
        self
    }
}

/// Input message for [`AnimationAgentHandle`].
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "agent", derive(serde::Serialize, serde::Deserialize))]
pub enum AgentInput {
    /// Start or replace a tween channel.
    Tween(AgentTweenSpec),
    /// Start or replace a spring channel.
    Spring(AgentSpringSpec),
    /// Stop a channel.
    Stop {
        /// Channel id.
        id: String,
    },
    /// Stop all channels.
    Reset,
}

/// Output message from [`AnimationAgentHandle`].
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "agent", derive(serde::Serialize, serde::Deserialize))]
pub enum AgentOutput {
    /// Channel started.
    Started {
        /// Channel id.
        id: String,
        /// Initial value.
        value: f32,
    },
    /// Channel advanced.
    Tick {
        /// Channel id.
        id: String,
        /// Current value.
        value: f32,
        /// Normalized progress.
        progress: f32,
    },
    /// Channel completed.
    Completed {
        /// Channel id.
        id: String,
        /// Final value.
        value: f32,
    },
    /// Channel stopped.
    Stopped {
        /// Channel id.
        id: String,
    },
    /// All channels reset.
    Reset,
}

/// Handle returned by [`use_animation_agent`].
#[derive(Clone)]
pub struct AnimationAgentHandle {
    runtime: Rc<RefCell<AgentRuntime>>,
    last_output: UseStateHandle<Option<AgentOutput>>,
    callback: Callback<AgentOutput>,
    loop_handle: RafLoop,
}

impl fmt::Debug for AnimationAgentHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnimationAgentHandle")
            .field("last_output", &*self.last_output)
            .finish_non_exhaustive()
    }
}

impl AnimationAgentHandle {
    /// Send a message to the agent runtime.
    pub fn send(&self, input: AgentInput) {
        for output in self.runtime.borrow_mut().handle(input) {
            self.emit(output);
        }
        if self.runtime.borrow().is_active() {
            self.loop_handle.kick();
        }
    }

    /// Last emitted output state handle.
    pub fn last_output(&self) -> UseStateHandle<Option<AgentOutput>> {
        self.last_output.clone()
    }

    /// Deterministically advance all active channels by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        let outputs = self.runtime.borrow_mut().tick(dt.max(0.0));
        let active = self.runtime.borrow().is_active();
        for output in outputs {
            self.emit(output);
        }
        active
    }

    fn emit(&self, output: AgentOutput) {
        set_if_changed(&self.last_output, Some(output.clone()));
        self.callback.emit(output);
    }
}

/// Create an animation agent handle.
#[hook]
pub fn use_animation_agent(callback: Callback<AgentOutput>) -> AnimationAgentHandle {
    let runtime = Rc::new(RefCell::new(AgentRuntime::default()));
    let last_output = use_state_eq(|| None);
    let loop_handle = RafLoop::new({
        let runtime = Rc::clone(&runtime);
        let last_output = last_output.clone();
        let callback = callback.clone();
        move |dt| {
            let outputs = runtime.borrow_mut().tick(dt.max(0.0));
            let active = runtime.borrow().is_active();
            for output in outputs {
                set_if_changed(&last_output, Some(output.clone()));
                callback.emit(output);
            }
            active
        }
    });

    AnimationAgentHandle {
        runtime,
        last_output,
        callback,
        loop_handle,
    }
}

#[derive(Default)]
struct AgentRuntime {
    channels: HashMap<String, AgentChannel>,
}

impl fmt::Debug for AgentRuntime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AgentRuntime")
            .field("channel_count", &self.channels.len())
            .finish()
    }
}

impl AgentRuntime {
    fn handle(&mut self, input: AgentInput) -> Vec<AgentOutput> {
        match input {
            AgentInput::Tween(spec) => {
                let id = spec.id.clone();
                let tween = Tween::new(spec.from, spec.to)
                    .duration(spec.duration)
                    .easing(spec.easing)
                    .build();
                let value = tween.value();
                self.channels.insert(id.clone(), AgentChannel::Tween(tween));
                vec![AgentOutput::Started { id, value }]
            }
            AgentInput::Spring(spec) => {
                let id = spec.id.clone();
                let mut spring = Spring::new(spec.config);
                spring.snap_to(spec.from);
                spring.set_target(spec.to);
                let value = spring.position();
                self.channels
                    .insert(id.clone(), AgentChannel::Spring(spring));
                vec![AgentOutput::Started { id, value }]
            }
            AgentInput::Stop { id } => {
                self.channels.remove(&id);
                vec![AgentOutput::Stopped { id }]
            }
            AgentInput::Reset => {
                self.channels.clear();
                vec![AgentOutput::Reset]
            }
        }
    }

    fn tick(&mut self, dt: f32) -> Vec<AgentOutput> {
        let mut outputs = Vec::new();
        let mut complete = Vec::new();

        for (id, channel) in &mut self.channels {
            let running = channel.update(dt);
            let value = channel.value();
            let progress = channel.progress();
            outputs.push(AgentOutput::Tick {
                id: id.clone(),
                value,
                progress,
            });
            if !running {
                outputs.push(AgentOutput::Completed {
                    id: id.clone(),
                    value,
                });
                complete.push(id.clone());
            }
        }

        for id in complete {
            self.channels.remove(&id);
        }

        outputs
    }

    fn is_active(&self) -> bool {
        !self.channels.is_empty()
    }
}

enum AgentChannel {
    Tween(Tween<f32>),
    Spring(Spring),
}

impl fmt::Debug for AgentChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tween(_) => f.debug_tuple("Tween").finish(),
            Self::Spring(_) => f.debug_tuple("Spring").finish(),
        }
    }
}

impl AgentChannel {
    fn update(&mut self, dt: f32) -> bool {
        match self {
            Self::Tween(tween) => tween.update(dt),
            Self::Spring(spring) => spring.update(dt),
        }
    }

    fn value(&self) -> f32 {
        match self {
            Self::Tween(tween) => tween.value(),
            Self::Spring(spring) => spring.position(),
        }
    }

    fn progress(&self) -> f32 {
        match self {
            Self::Tween(tween) => tween.progress(),
            Self::Spring(spring) => {
                if spring.is_settled() {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
}

fn spring_config_eq(a: &SpringConfig, b: &SpringConfig) -> bool {
    a.stiffness == b.stiffness
        && a.damping == b.damping
        && a.mass == b.mass
        && a.epsilon == b.epsilon
}

fn set_if_changed<T>(state: &UseStateHandle<T>, next: T)
where
    T: PartialEq + 'static,
{
    if **state != next {
        state.set(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_emits_tween_flow() {
        let mut runtime = AgentRuntime::default();
        let outputs = runtime.handle(AgentInput::Tween(
            AgentTweenSpec::new("x", 0.0, 10.0).duration(1.0),
        ));
        assert_eq!(
            outputs,
            vec![AgentOutput::Started {
                id: "x".to_owned(),
                value: 0.0,
            }]
        );

        let outputs = runtime.tick(0.5);
        assert!(outputs.iter().any(|output| matches!(
            output,
            AgentOutput::Tick { id, value, .. } if id == "x" && *value >= 5.0
        )));

        let outputs = runtime.tick(0.5);
        assert!(outputs.iter().any(|output| matches!(
            output,
            AgentOutput::Completed { id, value } if id == "x" && *value == 10.0
        )));
    }

    #[test]
    fn runtime_resets_channels() {
        let mut runtime = AgentRuntime::default();
        runtime.handle(AgentInput::Spring(AgentSpringSpec::new("s", 0.0, 1.0)));
        assert!(runtime.is_active());
        assert_eq!(runtime.handle(AgentInput::Reset), vec![AgentOutput::Reset]);
        assert!(!runtime.is_active());
    }
}
