//! Presence-style mount and hide animation helpers.

use crate::AnimatedStyle;
use animato_core::Easing;
use animato_spring::SpringConfig;
use leptos::prelude::*;

/// Style transition used by [`AnimatePresence`] and page/list helpers.
#[derive(Clone, Debug)]
pub struct PresenceAnimation {
    /// Duration in seconds for tween-based presence transitions.
    pub duration: f32,
    /// Easing curve for tween-based presence transitions.
    pub easing: Easing,
    /// Starting style.
    pub from: AnimatedStyle,
    /// Ending style.
    pub to: AnimatedStyle,
    /// Optional spring config for spring-driven presence transitions.
    pub spring: Option<SpringConfig>,
}

impl PresenceAnimation {
    /// Fade from transparent to opaque.
    pub fn fade() -> Self {
        Self::new(
            AnimatedStyle::new().opacity(0.0),
            AnimatedStyle::new().opacity(1.0),
        )
    }

    /// Slide up while fading in.
    pub fn slide_up() -> Self {
        Self::new(
            AnimatedStyle::new().opacity(0.0).translate(0.0, 20.0),
            AnimatedStyle::new().opacity(1.0).translate(0.0, 0.0),
        )
    }

    /// Slide down while fading in.
    pub fn slide_down() -> Self {
        Self::new(
            AnimatedStyle::new().opacity(0.0).translate(0.0, -20.0),
            AnimatedStyle::new().opacity(1.0).translate(0.0, 0.0),
        )
    }

    /// Slide from the left while fading in.
    pub fn slide_left() -> Self {
        Self::new(
            AnimatedStyle::new().opacity(0.0).translate(-20.0, 0.0),
            AnimatedStyle::new().opacity(1.0).translate(0.0, 0.0),
        )
    }

    /// Slide from the right while fading in.
    pub fn slide_right() -> Self {
        Self::new(
            AnimatedStyle::new().opacity(0.0).translate(20.0, 0.0),
            AnimatedStyle::new().opacity(1.0).translate(0.0, 0.0),
        )
    }

    /// Zoom in while fading in.
    pub fn zoom_in() -> Self {
        Self::new(
            AnimatedStyle::new().opacity(0.0).scale(0.8),
            AnimatedStyle::new().opacity(1.0).scale(1.0),
        )
    }

    /// Zoom out while fading in.
    pub fn zoom_out() -> Self {
        Self::new(
            AnimatedStyle::new().opacity(0.0).scale(1.2),
            AnimatedStyle::new().opacity(1.0).scale(1.0),
        )
    }

    /// Rotate on the x axis while fading in.
    pub fn flip_x() -> Self {
        Self::new(
            AnimatedStyle::new()
                .opacity(0.0)
                .transform("rotateX(90deg)"),
            AnimatedStyle::new().opacity(1.0).transform("rotateX(0deg)"),
        )
    }

    /// Rotate on the y axis while fading in.
    pub fn flip_y() -> Self {
        Self::new(
            AnimatedStyle::new()
                .opacity(0.0)
                .transform("rotateY(90deg)"),
            AnimatedStyle::new().opacity(1.0).transform("rotateY(0deg)"),
        )
    }

    /// Blur in while fading in.
    pub fn blur_in() -> Self {
        Self::new(
            AnimatedStyle::new().opacity(0.0).blur(10.0),
            AnimatedStyle::new().opacity(1.0).blur(0.0),
        )
    }

    /// Spring presence transition.
    pub fn spring(config: SpringConfig) -> Self {
        let mut animation = Self::zoom_in();
        animation.spring = Some(config);
        animation
    }

    /// Build a presence animation from two styles.
    pub fn new(from: AnimatedStyle, to: AnimatedStyle) -> Self {
        Self {
            duration: 0.25,
            easing: Easing::EaseOutCubic,
            from,
            to,
            spring: None,
        }
    }

    /// Return a reversed version of the animation.
    pub fn reversed(&self) -> Self {
        Self {
            duration: self.duration,
            easing: self.easing.clone(),
            from: self.to.clone(),
            to: self.from.clone(),
            spring: self.spring.clone(),
        }
    }
}

/// Mount/hide transition wrapper.
#[component]
pub fn AnimatePresence(
    /// Show or hide the children.
    show: ReadSignal<bool>,
    /// Enter animation.
    #[prop(optional)]
    enter: Option<PresenceAnimation>,
    /// Exit animation.
    #[prop(optional)]
    exit: Option<PresenceAnimation>,
    /// Keep the node mounted during exit.
    #[prop(default = true)]
    wait_exit: bool,
    /// Child view.
    children: Children,
) -> impl IntoView {
    let enter = enter.unwrap_or_else(PresenceAnimation::fade);
    let exit = exit.unwrap_or_else(|| enter.reversed());
    let enter_style = enter.to.to_css();
    let exit_style = exit.to.to_css();
    let exit_display = if wait_exit { "" } else { "display:none;" };
    let child = children();

    view! {
        <div
            data-animato-presence="true"
            style=move || {
                if show.get() {
                    enter_style.clone()
                } else {
                    format!("{exit_style}{exit_display}")
                }
            }
        >
            {child}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_have_expected_styles() {
        let fade = PresenceAnimation::fade();
        assert_eq!(fade.from.opacity, Some(0.0));
        assert_eq!(fade.to.opacity, Some(1.0));

        let slide = PresenceAnimation::slide_up();
        assert_eq!(slide.from.translate_y, Some(20.0));
        assert_eq!(slide.to.translate_y, Some(0.0));
    }
}
