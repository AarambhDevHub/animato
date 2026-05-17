//! FLIP-ready list rendering helpers.

use crate::PresenceAnimation;
use animato_core::Easing;
use dioxus::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Render a keyed list with animation metadata.
///
/// This helper keeps Dioxus ownership simple: callers pass a `Signal<Vec<T>>`,
/// a stable key extractor, and a child renderer returning an `Element`.
#[allow(non_snake_case)]
pub fn AnimatedFor<T, K, KF, CF>(
    each: Signal<Vec<T>>,
    key: KF,
    children: CF,
    enter: Option<PresenceAnimation>,
    exit: Option<PresenceAnimation>,
    move_duration: Option<f32>,
    move_easing: Option<Easing>,
    stagger_delay: Option<f32>,
) -> Element
where
    T: Clone + 'static,
    K: Eq + Hash + Clone + 'static,
    KF: Fn(&T) -> K + Clone + 'static,
    CF: Fn(T) -> Element + Clone + 'static,
{
    let _enter = enter.unwrap_or_else(PresenceAnimation::fade);
    let _exit = exit.unwrap_or_else(|| _enter.reversed());
    let duration = move_duration.unwrap_or(0.25).max(0.0);
    let easing = move_easing.unwrap_or(Easing::EaseOutCubic);
    let stagger = stagger_delay.unwrap_or(0.0).max(0.0);
    let list = crate::read_signal(each);

    rsx! {
        div {
            style: "display:flex; flex-direction:column; gap:10px;",
            for (index, item) in list.into_iter().enumerate() {
                {
                    let key_value = stable_key(&key(&item));
                    let child = children(item);
                    let delay = stagger * index as f32;
                    let style = format!(
                        "will-change:transform,opacity; transition:transform {duration:.3}s ease, opacity {duration:.3}s ease; transition-delay:{delay:.3}s;"
                    );
                    rsx! {
                        div {
                            key: "{key_value}",
                            style: "{style}",
                            title: "{easing:?}",
                            {child}
                        }
                    }
                }
            }
        }
    }
}

/// Convert any hashable key into a deterministic string key.
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
