//! Page transition helpers for Leptos Router.

use crate::PresenceAnimation;
use leptos::prelude::*;
use std::sync::{Arc, Mutex};

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
    let location = leptos_router::hooks::use_location();
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
    let transition = transition_css(&enter);
    let (style, set_style) = signal(format!("{style}{transition}"));
    let previous_path = Arc::new(Mutex::new(None::<String>));
    let child = children();

    Effect::new(move || {
        let path = location.pathname.get();
        let mut previous = previous_path
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let changed = previous.as_ref().is_some_and(|current| current != &path);
        *previous = Some(path);
        drop(previous);

        if changed {
            let from = format!("{}transition:none;", enter.from.to_css());
            let to = format!("{}{transition}", enter.to.to_css());
            set_style.set(from);
            schedule_style(set_style, to);
        } else {
            set_style.set(format!("{}{transition}", enter.to.to_css()));
        }
    });

    view! {
        <div data-animato-page-transition=format!("{mode:?}") style=move || style.get()>
            {child}
        </div>
    }
}

fn transition_css(animation: &PresenceAnimation) -> String {
    format!(
        "transition:opacity {:.3}s ease, transform {:.3}s ease, filter {:.3}s ease; will-change:opacity,transform,filter;",
        animation.duration.max(0.0),
        animation.duration.max(0.0),
        animation.duration.max(0.0)
    )
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn schedule_style(set_style: WriteSignal<String>, style: String) {
    let _ = leptos::prelude::request_animation_frame_with_handle(move || {
        set_style.set(style);
    });
}

#[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
fn schedule_style(set_style: WriteSignal<String>, style: String) {
    set_style.set(style);
}
