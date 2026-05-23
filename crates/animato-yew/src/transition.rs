//! Page transition helpers for Yew Router.

use crate::PresenceAnimation;
use yew::prelude::{Children, Html, Properties, function_component, hook, html};

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

/// Return the current route path as a stable transition key.
#[hook]
pub fn use_route_transition_key() -> String {
    yew_router::prelude::use_location()
        .map(|location| location.path().to_owned())
        .unwrap_or_default()
}

/// Properties for [`PageTransition`].
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PageTransitionProps {
    /// Transition mode.
    #[prop_or_default]
    pub mode: TransitionMode,
    /// Explicit route key. Pass [`route_transition_key`] when rendering under a router.
    #[prop_or_default]
    pub route_key: Option<String>,
    /// Enter animation.
    #[prop_or_default]
    pub enter: Option<PresenceAnimation>,
    /// Exit animation.
    #[prop_or_default]
    pub exit: Option<PresenceAnimation>,
    /// Child route view.
    pub children: Children,
}

/// Route-change transition wrapper.
#[function_component(PageTransition)]
pub fn page_transition(props: &PageTransitionProps) -> Html {
    let enter = props.enter.clone().unwrap_or_else(|| match props.mode {
        TransitionMode::SlideOver => PresenceAnimation::slide_right(),
        TransitionMode::MorphHero => PresenceAnimation::zoom_in(),
        _ => PresenceAnimation::fade(),
    });
    let _exit = props.exit.clone().unwrap_or_else(|| enter.reversed());
    let style = format!(
        "{}{}{}",
        container_css(props.mode),
        enter.to.to_css(),
        crate::presence::transition_css(enter.duration)
    );
    let route_key = props.route_key.clone().unwrap_or_default();

    html! {
        <div
            data-animato-page-transition={format!("{:?}", props.mode)}
            data-animato-route={route_key}
            style={style}
        >
            { for props.children.iter() }
        </div>
    }
}

fn container_css(mode: TransitionMode) -> &'static str {
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
        assert_eq!(
            container_css(TransitionMode::Parallel),
            "display:block; position:relative;"
        );
        assert_eq!(
            container_css(TransitionMode::CrossFade),
            "display:block; position:relative;"
        );
        assert_eq!(
            container_css(TransitionMode::MorphHero),
            "display:block; position:relative;"
        );
    }
}
