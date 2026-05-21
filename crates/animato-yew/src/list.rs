//! FLIP-ready list rendering helpers.

use crate::PresenceAnimation;
use animato_core::Easing;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use yew::prelude::{Callback, Html, Properties, function_component, html};

/// Properties for [`AnimatedFor`].
#[derive(Clone, Properties)]
pub struct AnimatedForProps<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Items to render.
    pub items: Vec<T>,
    /// Stable key extractor.
    pub key_fn: Callback<T, String>,
    /// Child renderer.
    pub render: Callback<T, Html>,
    /// Optional enter animation for inserted rows.
    #[prop_or_default]
    pub enter: Option<PresenceAnimation>,
    /// Optional exit animation for removed rows.
    #[prop_or_default]
    pub exit: Option<PresenceAnimation>,
    /// Move animation duration in seconds.
    #[prop_or(0.25)]
    pub move_duration: f32,
    /// Move animation easing.
    #[prop_or(Easing::EaseOutCubic)]
    pub move_easing: Easing,
    /// Stagger delay between rows.
    #[prop_or(0.0)]
    pub stagger_delay: f32,
}

impl<T> fmt::Debug for AnimatedForProps<T>
where
    T: Clone + PartialEq + fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnimatedForProps")
            .field("items", &self.items)
            .field("enter", &self.enter)
            .field("exit", &self.exit)
            .field("move_duration", &self.move_duration)
            .field("move_easing", &self.move_easing)
            .field("stagger_delay", &self.stagger_delay)
            .finish_non_exhaustive()
    }
}

impl<T> PartialEq for AnimatedForProps<T>
where
    T: Clone + PartialEq + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
            && self.enter == other.enter
            && self.exit == other.exit
            && self.move_duration == other.move_duration
            && self.move_easing == other.move_easing
            && self.stagger_delay == other.stagger_delay
    }
}

/// FLIP-ready keyed list component.
#[function_component(AnimatedFor)]
pub fn animated_for<T>(props: &AnimatedForProps<T>) -> Html
where
    T: Clone + PartialEq + 'static,
{
    let enter = props.enter.clone().unwrap_or_else(PresenceAnimation::fade);
    let _exit = props.exit.clone().unwrap_or_else(|| enter.reversed());
    let transition = format!(
        "transition:transform {:.3}s ease, opacity {:.3}s ease, filter {:.3}s ease; will-change:transform,opacity,filter;",
        props.move_duration.max(0.0),
        props.move_duration.max(0.0),
        props.move_duration.max(0.0)
    );

    html! {
        <div
            data-animato-animated-for="true"
            data-move-duration={props.move_duration.to_string()}
            data-move-easing={format!("{:?}", props.move_easing)}
            data-stagger-delay={props.stagger_delay.to_string()}
            style="display:flex; flex-direction:column; gap:10px;"
        >
            {
                for props.items.iter().enumerate().map(|(index, item)| {
                    let key_value = props.key_fn.emit(item.clone());
                    let child = props.render.emit(item.clone());
                    let delay = (props.stagger_delay.max(0.0) * index as f32).max(0.0);
                    let style = format!(
                        "{}{}transition-delay:{delay:.3}s;",
                        enter.to.to_css(),
                        transition
                    );
                    html! {
                        <div
                            key={key_value.clone()}
                            data-animato-list-item="true"
                            data-animato-key={key_value}
                            style={style}
                        >
                            { child }
                        </div>
                    }
                })
            }
        </div>
    }
}

/// Return a deterministic hash string for a key value.
pub fn stable_key<K: Hash>(key: &K) -> String {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_key_is_deterministic_and_distinguishes_values() {
        assert_eq!(stable_key(&"row-1"), stable_key(&"row-1"));
        assert_ne!(stable_key(&"row-1"), stable_key(&"row-2"));
    }
}
