//! Page transition helpers for Dioxus apps.

use crate::PresenceAnimation;
use dioxus::prelude::*;
use std::fmt;

/// Page transition strategy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TransitionMode {
    /// Old page exits before the new page enters.
    #[default]
    Sequential,
    /// Old and new page animate together.
    Parallel,
    /// Opposing opacity transition.
    CrossFade,
    /// New page slides over the previous page.
    SlideOver,
    /// Shared-element hero morph mode.
    MorphHero,
}

impl fmt::Display for TransitionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Route-change transition wrapper.
#[component]
pub fn PageTransition(
    /// Transition mode.
    mode: Option<TransitionMode>,
    /// Optional route key. Pass the current Dioxus router route string here to
    /// force keyed transition identity.
    route_key: Option<String>,
    /// Enter animation.
    enter: Option<PresenceAnimation>,
    /// Exit animation.
    exit: Option<PresenceAnimation>,
    /// Child route view.
    children: Element,
) -> Element {
    let mode = mode.unwrap_or_default();
    let enter = enter.unwrap_or_else(|| match mode {
        TransitionMode::SlideOver => PresenceAnimation::slide_right(),
        TransitionMode::MorphHero => PresenceAnimation::zoom_in(),
        _ => PresenceAnimation::fade(),
    });
    let _exit = exit.unwrap_or_else(|| enter.reversed());
    let base_style = container_css(mode);
    let style = format!(
        "{base_style}{}{}",
        enter.to.to_css(),
        crate::presence::transition_css(enter.duration)
    );
    let route_key = route_key.unwrap_or_default();

    rsx! {
        div {
            key: "{route_key}",
            style: "{style}",
            {children}
        }
    }
}

/// Return a Dioxus Router route key for use with [`PageTransition`].
#[cfg(feature = "router")]
pub fn route_transition_key<R>() -> String
where
    R: dioxus_router::Routable + Clone + ToString + 'static,
{
    dioxus_router::hooks::use_route::<R>().to_string()
}

/// Return an empty route key when router support is not enabled.
#[cfg(not(feature = "router"))]
pub fn route_transition_key() -> String {
    String::new()
}

pub(crate) fn container_css(mode: TransitionMode) -> &'static str {
    match mode {
        TransitionMode::SlideOver => "display:block; position:relative; overflow:hidden;",
        _ => "display:block; position:relative;",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_css_matches_transition_mode() {
        assert!(container_css(TransitionMode::SlideOver).contains("overflow:hidden"));
        assert_eq!(
            container_css(TransitionMode::Sequential),
            "display:block; position:relative;"
        );
    }

    #[test]
    fn display_matches_debug_label() {
        assert_eq!(TransitionMode::CrossFade.to_string(), "CrossFade");
    }
}
