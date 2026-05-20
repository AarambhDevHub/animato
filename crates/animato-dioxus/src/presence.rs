//! Presence-style mount and hide animation helpers.

use crate::AnimatedStyle;
use animato_core::Easing;
use animato_spring::SpringConfig;
use dioxus::prelude::*;

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

/// Mount/hide transition wrapper.
#[component]
pub fn AnimatePresence(
    /// Show or hide the children.
    show: Signal<bool>,
    /// Enter animation.
    enter: Option<PresenceAnimation>,
    /// Exit animation.
    exit: Option<PresenceAnimation>,
    /// Keep the node mounted during exit.
    wait_exit: Option<bool>,
    /// Child view.
    children: Element,
) -> Element {
    let enter = enter.unwrap_or_else(PresenceAnimation::fade);
    let exit = exit.unwrap_or_else(|| enter.reversed());
    let wait_exit = wait_exit.unwrap_or(true);
    let transition = transition_css(enter.duration.max(exit.duration));
    let enter_style = format!("{}{}", enter.to.to_css(), transition);
    let exit_style = format!("{}{}", exit.to.to_css(), transition);
    let exit_display = if wait_exit { "" } else { "display:none;" };
    let style = if crate::read_signal(show) {
        enter_style
    } else {
        format!("{exit_style}{exit_display}")
    };

    rsx! {
        div {
            style: "{style}",
            {children}
        }
    }
}

pub(crate) fn transition_css(duration: f32) -> String {
    let duration = duration.max(0.0);
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
    fn spring_presence_keeps_config_and_reverses() {
        let config = SpringConfig::wobbly();
        let spring = PresenceAnimation::spring(config.clone());
        let stored = spring.spring.as_ref().expect("spring config");
        assert_eq!(stored.stiffness, config.stiffness);
        assert_eq!(spring.from.scale, Some(0.8));

        let reversed = spring.reversed();
        assert_eq!(reversed.from.scale, Some(1.0));
        assert_eq!(reversed.to.scale, Some(0.8));
    }

    #[test]
    fn all_presence_presets_are_well_formed() {
        let presets = [
            PresenceAnimation::fade(),
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
            assert_eq!(preset.easing, Easing::EaseOutCubic);
            assert_eq!(preset.from.opacity, Some(0.0));
            assert_eq!(preset.to.opacity, Some(1.0));
            assert!(preset.from.to_css().contains("opacity:0;"));
            assert!(preset.to.to_css().contains("opacity:1;"));
        }
    }

    #[test]
    fn equality_and_transition_css_cover_edge_cases() {
        assert_eq!(PresenceAnimation::fade(), PresenceAnimation::fade());
        assert_ne!(
            PresenceAnimation::spring(SpringConfig::snappy()),
            PresenceAnimation::spring(SpringConfig::wobbly())
        );
        assert_ne!(PresenceAnimation::fade(), PresenceAnimation::slide_up());

        let css = transition_css(-1.0);
        assert!(css.contains("0.000s"));
        assert!(css.contains("will-change:opacity,transform,filter;"));
    }
}
