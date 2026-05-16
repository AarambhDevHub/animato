//! Page transition helpers for Leptos Router.

use crate::PresenceAnimation;
use leptos::prelude::*;

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

/// Route-change transition wrapper.
#[component]
pub fn PageTransition(
    /// Transition mode.
    #[prop(optional)]
    mode: Option<TransitionMode>,
    /// Enter animation.
    #[prop(optional)]
    enter: Option<PresenceAnimation>,
    /// Exit animation.
    #[prop(optional)]
    exit: Option<PresenceAnimation>,
    /// Child route view.
    children: Children,
) -> impl IntoView {
    let _location = leptos_router::hooks::use_location();
    let mode = mode.unwrap_or_default();
    let enter = enter.unwrap_or_else(|| match mode {
        TransitionMode::SlideOver => PresenceAnimation::slide_right(),
        TransitionMode::MorphHero => PresenceAnimation::zoom_in(),
        _ => PresenceAnimation::fade(),
    });
    let _exit = exit.unwrap_or_else(|| enter.reversed());
    let style = match mode {
        TransitionMode::Sequential => enter.to.to_css(),
        TransitionMode::Parallel => enter.to.to_css(),
        TransitionMode::CrossFade => PresenceAnimation::fade().to.to_css(),
        TransitionMode::SlideOver => PresenceAnimation::slide_right().to.to_css(),
        TransitionMode::MorphHero => PresenceAnimation::zoom_in().to.to_css(),
    };
    let child = children();

    view! {
        <div data-animato-page-transition=format!("{mode:?}") style=style>
            {child}
        </div>
    }
}
