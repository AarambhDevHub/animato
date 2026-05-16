//! FLIP-ready list rendering helpers.

use crate::PresenceAnimation;
use animato_core::Easing;
use leptos::prelude::*;
use std::hash::Hash;

/// FLIP-ready keyed list component.
#[component]
pub fn AnimatedFor<T, K, KF, CF, IV>(
    /// Reactive list source.
    each: Signal<Vec<T>>,
    /// Stable key extractor.
    key: KF,
    /// Child renderer.
    children: CF,
    /// Optional enter animation for inserted rows.
    #[prop(optional)]
    enter: Option<PresenceAnimation>,
    /// Optional exit animation for removed rows.
    #[prop(optional)]
    exit: Option<PresenceAnimation>,
    /// Move animation duration in seconds.
    #[prop(optional)]
    move_duration: Option<f32>,
    /// Move animation easing.
    #[prop(optional)]
    move_easing: Option<Easing>,
    /// Stagger delay between rows.
    #[prop(optional)]
    stagger_delay: Option<f32>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    K: Eq + Hash + Clone + Send + Sync + 'static,
    KF: Fn(&T) -> K + Clone + Send + Sync + 'static,
    CF: Fn(T) -> IV + Clone + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let enter = enter.unwrap_or_else(PresenceAnimation::fade);
    let _exit = exit.unwrap_or_else(|| enter.reversed());
    let duration = move_duration.unwrap_or(0.25).max(0.0);
    let easing = move_easing.unwrap_or(Easing::EaseOutCubic);
    let stagger = stagger_delay.unwrap_or(0.0).max(0.0);
    let key_fn = key.clone();
    let child_fn = children.clone();

    view! {
        <div
            data-animato-animated-for="true"
            data-move-duration=duration
            data-move-easing=format!("{easing:?}")
            data-stagger-delay=stagger
        >
            <For
                each=move || each.get()
                key=move |item| key_fn(item)
                children=move |item| child_fn(item)
            />
        </div>
    }
}
