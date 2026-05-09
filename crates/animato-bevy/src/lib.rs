//! # animato-bevy
//!
//! Bevy integration for Animato.
//!
//! The crate keeps the core Animato crates renderer-agnostic by using thin
//! Bevy wrapper components around [`animato_tween::Tween`] and
//! [`animato_spring::SpringN`].
//!
//! ```rust,ignore
//! use animato_bevy::{AnimatoPlugin, AnimatoTween};
//! use animato_tween::Tween;
//! use bevy::prelude::*;
//!
//! App::new()
//!     .add_plugins((DefaultPlugins, AnimatoPlugin))
//!     .add_systems(Startup, |mut commands: Commands| {
//!         commands.spawn((
//!             Transform::default(),
//!             AnimatoTween::translation(
//!                 Tween::new([0.0, 0.0, 0.0], [300.0, 0.0, 0.0])
//!                     .duration(1.0)
//!                     .build(),
//!             ),
//!         ));
//!     });
//! ```

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use animato_core::{Animatable, Update};
use animato_spring::{Decompose, SpringN};
use animato_tween::Tween;
use bevy_app::{App, Plugin, Update as BevyUpdate};
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{IntoScheduleConfigs, SystemSet};
use bevy_time::Time;
use core::marker::PhantomData;

#[cfg(feature = "transform")]
use bevy_math::{Quat, Vec3};
#[cfg(feature = "transform")]
use bevy_transform::components::Transform;

/// Systems registered by [`AnimatoPlugin`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum AnimatoSet {
    /// Advances Animato components using Bevy's [`Time`] resource.
    Tick,
    /// Applies completed tick values into Bevy components such as [`Transform`].
    Apply,
}

/// A named animation marker attached to an entity.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Component)]
pub struct AnimationLabel(pub String);

impl AnimationLabel {
    /// Create a new animation label.
    pub fn new(label: impl Into<String>) -> Self {
        Self(label.into())
    }

    /// Borrow the label as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Which Bevy-facing target a component drives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnimationChannel {
    /// The component only ticks and exposes its value to user systems.
    Value,
    /// Drive [`Transform::translation`] from a `[f32; 3]` value.
    Translation,
    /// Drive [`Transform::scale`] from a `[f32; 3]` value.
    Scale,
    /// Drive [`Transform::rotation`] as a Z-axis angle in radians from an `f32`.
    RotationZ,
}

/// Message written when an [`AnimatoTween`] reaches completion.
#[derive(Clone, Debug, Message)]
pub struct TweenCompleted {
    /// Entity that owns the completed tween.
    pub entity: Entity,
    /// Optional label cloned from [`AnimationLabel`] on the same entity.
    pub label: Option<AnimationLabel>,
    /// Channel driven by the completed tween.
    pub channel: AnimationChannel,
}

/// Message written when an [`AnimatoSpring`] settles.
#[derive(Clone, Debug, Message)]
pub struct SpringSettled {
    /// Entity that owns the settled spring.
    pub entity: Entity,
    /// Optional label cloned from [`AnimationLabel`] on the same entity.
    pub label: Option<AnimationLabel>,
    /// Channel driven by the settled spring.
    pub channel: AnimationChannel,
}

/// Bevy component wrapper around [`Tween<T>`].
#[derive(Clone, Debug, Component)]
pub struct AnimatoTween<T: Animatable + Send + Sync> {
    tween: Tween<T>,
    channel: AnimationChannel,
    completed_reported: bool,
}

impl<T: Animatable + Send + Sync> AnimatoTween<T> {
    /// Create a value-only tween component.
    pub fn new(tween: Tween<T>) -> Self {
        Self {
            tween,
            channel: AnimationChannel::Value,
            completed_reported: false,
        }
    }

    /// Override the Bevy-facing channel.
    pub fn with_channel(mut self, channel: AnimationChannel) -> Self {
        self.channel = channel;
        self
    }

    /// Borrow the inner tween.
    pub fn tween(&self) -> &Tween<T> {
        &self.tween
    }

    /// Mutably borrow the inner tween and clear completion reporting.
    pub fn tween_mut(&mut self) -> &mut Tween<T> {
        self.completed_reported = false;
        &mut self.tween
    }

    /// Current interpolated value.
    pub fn value(&self) -> T {
        self.tween.value()
    }

    /// Channel driven by this tween.
    pub fn channel(&self) -> AnimationChannel {
        self.channel
    }

    /// Pause the inner tween.
    pub fn pause(&mut self) {
        self.tween.pause();
    }

    /// Resume the inner tween.
    pub fn resume(&mut self) {
        self.tween.resume();
    }

    /// Reset the inner tween and allow a future completion message.
    pub fn reset(&mut self) {
        self.tween.reset();
        self.completed_reported = false;
    }
}

impl AnimatoTween<[f32; 3]> {
    /// Create a tween that drives [`Transform::translation`].
    pub fn translation(tween: Tween<[f32; 3]>) -> Self {
        Self::new(tween).with_channel(AnimationChannel::Translation)
    }

    /// Create a tween that drives [`Transform::scale`].
    pub fn scale(tween: Tween<[f32; 3]>) -> Self {
        Self::new(tween).with_channel(AnimationChannel::Scale)
    }
}

impl AnimatoTween<f32> {
    /// Create a tween that drives Z-axis rotation in radians.
    pub fn rotation_z(tween: Tween<f32>) -> Self {
        Self::new(tween).with_channel(AnimationChannel::RotationZ)
    }
}

/// Bevy component wrapper around [`SpringN<T>`].
#[derive(Debug, Component)]
pub struct AnimatoSpring<T: Decompose + Send + Sync> {
    spring: SpringN<T>,
    channel: AnimationChannel,
    settled_reported: bool,
}

impl<T: Decompose + Send + Sync> AnimatoSpring<T> {
    /// Create a value-only spring component.
    pub fn new(spring: SpringN<T>) -> Self {
        let settled_reported = spring.is_settled();
        Self {
            spring,
            channel: AnimationChannel::Value,
            settled_reported,
        }
    }

    /// Override the Bevy-facing channel.
    pub fn with_channel(mut self, channel: AnimationChannel) -> Self {
        self.channel = channel;
        self
    }

    /// Borrow the inner spring.
    pub fn spring(&self) -> &SpringN<T> {
        &self.spring
    }

    /// Mutably borrow the inner spring and clear settled reporting.
    pub fn spring_mut(&mut self) -> &mut SpringN<T> {
        self.settled_reported = false;
        &mut self.spring
    }

    /// Set a new target and allow a future settled message.
    pub fn set_target(&mut self, target: T) {
        self.spring.set_target(target);
        self.settled_reported = false;
    }

    /// Current spring position.
    pub fn position(&self) -> T {
        self.spring.position()
    }

    /// Channel driven by this spring.
    pub fn channel(&self) -> AnimationChannel {
        self.channel
    }
}

impl AnimatoSpring<[f32; 3]> {
    /// Create a spring that drives [`Transform::translation`].
    pub fn translation(spring: SpringN<[f32; 3]>) -> Self {
        Self::new(spring).with_channel(AnimationChannel::Translation)
    }

    /// Create a spring that drives [`Transform::scale`].
    pub fn scale(spring: SpringN<[f32; 3]>) -> Self {
        Self::new(spring).with_channel(AnimationChannel::Scale)
    }
}

impl AnimatoSpring<f32> {
    /// Create a spring that drives Z-axis rotation in radians.
    pub fn rotation_z(spring: SpringN<f32>) -> Self {
        Self::new(spring).with_channel(AnimationChannel::RotationZ)
    }
}

/// Generic plugin that ticks one [`AnimatoTween<T>`] component type.
#[derive(Debug)]
pub struct AnimatoTweenPlugin<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T> Default for AnimatoTweenPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Plugin for AnimatoTweenPlugin<T>
where
    T: Animatable + Send + Sync,
{
    fn build(&self, app: &mut App) {
        app.add_systems(BevyUpdate, tick_tweens::<T>.in_set(AnimatoSet::Tick));
    }
}

/// Generic plugin that ticks one [`AnimatoSpring<T>`] component type.
#[derive(Debug)]
pub struct AnimatoSpringPlugin<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T> Default for AnimatoSpringPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Plugin for AnimatoSpringPlugin<T>
where
    T: Decompose + Send + Sync,
{
    fn build(&self, app: &mut App) {
        app.add_systems(BevyUpdate, tick_springs::<T>.in_set(AnimatoSet::Tick));
    }
}

/// Bevy plugin registering common Animato value types and integration messages.
#[derive(Clone, Copy, Debug, Default)]
pub struct AnimatoPlugin;

impl Plugin for AnimatoPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TweenCompleted>()
            .add_message::<SpringSettled>()
            .configure_sets(BevyUpdate, (AnimatoSet::Tick, AnimatoSet::Apply).chain())
            .add_systems(
                BevyUpdate,
                (
                    tick_tweens::<f32>,
                    tick_tweens::<[f32; 2]>,
                    tick_tweens::<[f32; 3]>,
                    tick_tweens::<[f32; 4]>,
                    tick_springs::<f32>,
                    tick_springs::<[f32; 2]>,
                    tick_springs::<[f32; 3]>,
                    tick_springs::<[f32; 4]>,
                )
                    .in_set(AnimatoSet::Tick),
            );

        #[cfg(feature = "transform")]
        app.add_systems(
            BevyUpdate,
            (
                apply_transform_vec3_tweens,
                apply_transform_f32_tweens,
                apply_transform_vec3_springs,
                apply_transform_f32_springs,
            )
                .in_set(AnimatoSet::Apply),
        );
    }
}

fn tick_tweens<T>(
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimatoTween<T>, Option<&AnimationLabel>)>,
    mut writer: MessageWriter<TweenCompleted>,
) where
    T: Animatable + Send + Sync,
{
    let dt = time.delta_secs();
    for (entity, mut anim, label) in &mut query {
        if anim.completed_reported {
            continue;
        }

        let running = anim.tween.update(dt);
        if !running || anim.tween.is_complete() {
            writer.write(TweenCompleted {
                entity,
                label: label.cloned(),
                channel: anim.channel,
            });
            anim.completed_reported = true;
        }
    }
}

fn tick_springs<T>(
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimatoSpring<T>, Option<&AnimationLabel>)>,
    mut writer: MessageWriter<SpringSettled>,
) where
    T: Decompose + Send + Sync,
{
    let dt = time.delta_secs();
    for (entity, mut anim, label) in &mut query {
        if anim.settled_reported {
            continue;
        }

        let running = anim.spring.update(dt);
        if !running || anim.spring.is_settled() {
            writer.write(SpringSettled {
                entity,
                label: label.cloned(),
                channel: anim.channel,
            });
            anim.settled_reported = true;
        }
    }
}

#[cfg(feature = "transform")]
fn apply_transform_vec3_tweens(mut query: Query<(&mut Transform, &AnimatoTween<[f32; 3]>)>) {
    for (mut transform, anim) in &mut query {
        match anim.channel() {
            AnimationChannel::Translation => transform.translation = to_vec3(anim.value()),
            AnimationChannel::Scale => transform.scale = to_vec3(anim.value()),
            AnimationChannel::Value | AnimationChannel::RotationZ => {}
        }
    }
}

#[cfg(feature = "transform")]
fn apply_transform_f32_tweens(mut query: Query<(&mut Transform, &AnimatoTween<f32>)>) {
    for (mut transform, anim) in &mut query {
        if anim.channel() == AnimationChannel::RotationZ {
            transform.rotation = Quat::from_rotation_z(anim.value());
        }
    }
}

#[cfg(feature = "transform")]
fn apply_transform_vec3_springs(mut query: Query<(&mut Transform, &AnimatoSpring<[f32; 3]>)>) {
    for (mut transform, anim) in &mut query {
        match anim.channel() {
            AnimationChannel::Translation => transform.translation = to_vec3(anim.position()),
            AnimationChannel::Scale => transform.scale = to_vec3(anim.position()),
            AnimationChannel::Value | AnimationChannel::RotationZ => {}
        }
    }
}

#[cfg(feature = "transform")]
fn apply_transform_f32_springs(mut query: Query<(&mut Transform, &AnimatoSpring<f32>)>) {
    for (mut transform, anim) in &mut query {
        if anim.channel() == AnimationChannel::RotationZ {
            transform.rotation = Quat::from_rotation_z(anim.position());
        }
    }
}

#[cfg(feature = "transform")]
#[inline]
fn to_vec3(value: [f32; 3]) -> Vec3 {
    Vec3::new(value[0], value[1], value[2])
}
