//! Presence-style mount and hide animation helpers.

use crate::AnimatedStyle;
use animato_core::Easing;
use animato_spring::SpringConfig;
use yew::prelude::{Children, Html, Properties, function_component, html};

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

impl PartialEq for PresenceAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration
            && self.easing == other.easing
            && self.from == other.from
            && self.to == other.to
            && spring_eq(self.spring.as_ref(), other.spring.as_ref())
    }
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

/// Properties for [`AnimatePresence`].
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AnimatePresenceProps {
    /// Show or hide the children.
    pub show: bool,
    /// Enter animation.
    #[prop_or_default]
    pub enter: Option<PresenceAnimation>,
    /// Exit animation.
    #[prop_or_default]
    pub exit: Option<PresenceAnimation>,
    /// Keep the node mounted during exit.
    #[prop_or(true)]
    pub wait_exit: bool,
    /// Child nodes.
    pub children: Children,
}

/// Mount/hide transition wrapper.
#[function_component(AnimatePresence)]
pub fn animate_presence(props: &AnimatePresenceProps) -> Html {
    let enter = props.enter.clone().unwrap_or_else(PresenceAnimation::fade);
    let exit = props.exit.clone().unwrap_or_else(|| enter.reversed());
    let transition = transition_css(enter.duration.max(exit.duration));
    let style = if props.show {
        format!("{}{}", enter.to.to_css(), transition)
    } else {
        let exit_display = if props.wait_exit { "" } else { "display:none;" };
        format!("{}{}{}", exit.to.to_css(), transition, exit_display)
    };

    html! {
        <div data-animato-presence="true" style={style}>
            { for props.children.iter() }
        </div>
    }
}

pub(crate) fn transition_css(duration: f32) -> String {
    let duration = crate::finite_or(duration, 0.0).max(0.0);
    format!(
        "transition:opacity {duration:.3}s ease, transform {duration:.3}s ease, filter {duration:.3}s ease; will-change:opacity,transform,filter;"
    )
}

fn spring_eq(a: Option<&SpringConfig>, b: Option<&SpringConfig>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => {
            a.stiffness == b.stiffness
                && a.damping == b.damping
                && a.mass == b.mass
                && a.epsilon == b.epsilon
        }
        (None, None) => true,
        _ => false,
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

    #[test]
    fn all_presence_presets_are_well_formed() {
        let presets = [
            PresenceAnimation::slide_down(),
            PresenceAnimation::slide_left(),
            PresenceAnimation::slide_right(),
            PresenceAnimation::zoom_in(),
            PresenceAnimation::zoom_out(),
            PresenceAnimation::flip_x(),
            PresenceAnimation::flip_y(),
            PresenceAnimation::blur_in(),
        ];

        for preset in presets {
            assert!(preset.duration > 0.0);
            assert!(preset.to.to_css().contains("opacity:1;"));
            assert_eq!(preset.reversed().from, preset.to);
        }
    }

    #[test]
    fn transition_clamps_negative_duration() {
        assert!(transition_css(-1.0).contains("opacity 0.000s"));
    }
}
