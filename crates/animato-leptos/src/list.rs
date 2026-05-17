//! FLIP-ready list rendering helpers.

use crate::PresenceAnimation;
use animato_core::Easing;
use leptos::prelude::*;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::cell::RefCell;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::rc::Rc;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use wasm_bindgen::JsCast;

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
    let easing_label = format!("{easing:?}");
    let stagger = stagger_delay.unwrap_or(0.0).max(0.0);
    let container = NodeRef::<leptos::html::Div>::new();
    let key_for_key = key.clone();
    let key_for_child = key.clone();
    let each_for_render = each;
    let child_fn = children.clone();

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    {
        let previous_rects = Rc::new(RefCell::new(HashMap::new()));
        let animation = enter.clone();
        let each_for_effect = each;
        let key_for_effect = key.clone();
        let easing_for_effect = easing.clone();
        let initial_rects = Rc::clone(&previous_rects);
        container.on_load(move |container| {
            let elements = list_item_elements(&container);
            *initial_rects.borrow_mut() = collect_rects(&elements);
        });

        Effect::new(move || {
            let list = each_for_effect.get();
            let _keys = list
                .iter()
                .map(|item| stable_key(&key_for_effect(item)))
                .collect::<Vec<_>>();
            animate_flip(
                container,
                Rc::clone(&previous_rects),
                duration,
                css_timing_function(&easing_for_effect),
                stagger,
                animation.clone(),
            );
        });
    }

    view! {
        <div
            node_ref=container
            data-animato-animated-for="true"
            data-move-duration=duration
            data-move-easing=easing_label
            data-stagger-delay=stagger
            style="display:flex; flex-direction:column; gap:10px;"
        >
            <For
                each=move || each_for_render.get()
                key=move |item| key_for_key(item)
                children=move |item| {
                    let key_value = stable_key(&key_for_child(&item));
                    let child = child_fn(item);
                    view! {
                        <div
                            data-animato-list-item="true"
                            data-animato-key=key_value
                            style="will-change:transform,opacity;"
                        >
                            {child}
                        </div>
                    }
                }
            />
        </div>
    }
}

fn stable_key<K: Hash>(key: &K) -> String {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish().to_string()
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
#[derive(Clone, Copy, Debug)]
struct ItemRect {
    left: f64,
    top: f64,
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn animate_flip(
    container: NodeRef<leptos::html::Div>,
    previous_rects: Rc<RefCell<HashMap<String, ItemRect>>>,
    duration: f32,
    easing: &'static str,
    stagger: f32,
    enter: PresenceAnimation,
) {
    if crate::ssr::is_hydrating() {
        return;
    }

    let Some(container) = container.get_untracked() else {
        return;
    };

    let elements = list_item_elements(&container);
    let previous = previous_rects.borrow().clone();
    let mut next = HashMap::new();
    let transition = format!(
        "transform {:.3}s {}, opacity {:.3}s {}, filter {:.3}s {}",
        duration, easing, duration, easing, duration, easing
    );

    for (index, element) in elements.iter().enumerate() {
        let Some(key) = element.get_attribute("data-animato-key") else {
            continue;
        };
        let rect = element.get_bounding_client_rect();
        next.insert(
            key.clone(),
            ItemRect {
                left: rect.left(),
                top: rect.top(),
            },
        );

        let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() else {
            continue;
        };
        let delay = format!("{:.3}s", stagger * index as f32);
        let style = html.style();
        let _ = style.set_property("transition", "none");
        let _ = style.set_property("transition-delay", &delay);

        if let Some(before) = previous.get(&key) {
            let dx = before.left - rect.left();
            let dy = before.top - rect.top();
            if dx.abs() > 0.5 || dy.abs() > 0.5 {
                let _ = style.set_property("transform", &format!("translate({dx:.1}px,{dy:.1}px)"));
                let _ = style.set_property("opacity", "1");
            }
        } else {
            let from_transform = enter.from.transform_string();
            if !from_transform.is_empty() {
                let _ = style.set_property("transform", &from_transform);
            }
            if let Some(opacity) = enter.from.opacity {
                let _ = style.set_property("opacity", &opacity.to_string());
            }
            if let Some(blur) = enter.from.blur {
                let _ = style.set_property("filter", &format!("blur({blur}px)"));
            }
        }
    }

    *previous_rects.borrow_mut() = next;

    let target_transform = enter.to.transform_string();
    let target_opacity = enter.to.opacity.unwrap_or(1.0).to_string();
    let target_filter = enter
        .to
        .blur
        .map(|blur| format!("blur({blur}px)"))
        .unwrap_or_else(|| "none".to_owned());

    let _ = leptos::prelude::request_animation_frame_with_handle(move || {
        let _ = leptos::prelude::request_animation_frame_with_handle(move || {
            for element in elements {
                let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() else {
                    continue;
                };
                let style = html.style();
                let _ = style.set_property("transition", &transition);
                let _ = style.set_property("transform", &target_transform);
                let _ = style.set_property("opacity", &target_opacity);
                let _ = style.set_property("filter", &target_filter);
            }
        });
    });
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn list_item_elements(container: &web_sys::Element) -> Vec<web_sys::Element> {
    let children = container.children();
    (0..children.length())
        .filter_map(|index| children.item(index))
        .filter(|element| element.has_attribute("data-animato-list-item"))
        .collect()
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn collect_rects(elements: &[web_sys::Element]) -> HashMap<String, ItemRect> {
    elements
        .iter()
        .filter_map(|element| {
            let key = element.get_attribute("data-animato-key")?;
            let rect = element.get_bounding_client_rect();
            Some((
                key,
                ItemRect {
                    left: rect.left(),
                    top: rect.top(),
                },
            ))
        })
        .collect()
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn css_timing_function(easing: &Easing) -> &'static str {
    match easing {
        Easing::Linear => "linear",
        Easing::EaseInQuad | Easing::EaseInCubic | Easing::EaseInQuart | Easing::EaseInQuint => {
            "cubic-bezier(.55,.06,.68,.19)"
        }
        Easing::EaseOutQuad
        | Easing::EaseOutCubic
        | Easing::EaseOutQuart
        | Easing::EaseOutQuint => "cubic-bezier(.22,1,.36,1)",
        Easing::EaseInOutQuad
        | Easing::EaseInOutCubic
        | Easing::EaseInOutQuart
        | Easing::EaseInOutQuint => "cubic-bezier(.65,0,.35,1)",
        Easing::CubicBezier(_, _, _, _) => "cubic-bezier(.22,1,.36,1)",
        Easing::Steps(_) => "steps(6, jump-end)",
        _ => "ease",
    }
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
