//! FLIP-ready list rendering helpers.

use crate::PresenceAnimation;
use animato_core::Easing;
use leptos::prelude::*;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::cell::{Cell, RefCell};
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
    /// Delay before existing rows move to their new positions, in seconds.
    #[prop(optional)]
    move_delay: Option<f32>,
    /// Stagger delay between rows.
    #[prop(optional)]
    stagger_delay: Option<f32>,
    /// Gap between generated row wrappers in pixels. Defaults to zero.
    #[prop(optional)]
    gap: Option<f32>,
    /// CSS class applied to each generated row wrapper.
    ///
    /// This is useful for stacking-context rules such as
    /// `item_class="relative z-0 hover:z-50"`.
    #[prop(optional, into)]
    item_class: Option<String>,
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
    let move_delay = move_delay.unwrap_or(0.0).max(0.0);
    let stagger = stagger_delay.unwrap_or(0.0).max(0.0);
    let gap = gap.unwrap_or(0.0).max(0.0);
    let container_style = format!("display:flex; flex-direction:column; gap:{gap:.3}px;");
    let item_class = item_class.unwrap_or_default();
    let container = NodeRef::<leptos::html::Div>::new();
    let key_for_key = key.clone();
    let key_for_child = key.clone();
    let each_for_render = each;
    let child_fn = children.clone();
    let item_class_for_child = item_class.clone();

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    {
        let previous_rects = Rc::new(RefCell::new(HashMap::new()));
        let effect_initialized = Rc::new(Cell::new(false));
        let animation = enter.clone();
        let each_for_effect = each;
        let key_for_effect = key.clone();
        let easing_for_effect = easing.clone();
        let initial_rects = Rc::clone(&previous_rects);
        let initial_effect = Rc::clone(&effect_initialized);
        let initial_animation = animation.clone();
        container.on_load(move |container| {
            animate_flip(
                &container,
                Rc::clone(&initial_rects),
                duration,
                css_timing_function(&easing),
                move_delay,
                stagger,
                initial_animation.clone(),
            );
        });

        Effect::new(move || {
            let list = each_for_effect.get();
            let _keys = list
                .iter()
                .map(|item| stable_key(&key_for_effect(item)))
                .collect::<Vec<_>>();

            // The mount hook owns the initial enter pass. Skipping the effect's
            // first run prevents duplicate scheduling regardless of whether the
            // effect or node load callback runs first.
            if !initial_effect.replace(true) {
                return;
            }
            let Some(container) = container.get_untracked() else {
                return;
            };
            animate_flip(
                &container,
                Rc::clone(&previous_rects),
                duration,
                css_timing_function(&easing_for_effect),
                move_delay,
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
            data-move-delay=move_delay
            data-stagger-delay=stagger
            data-gap=gap
            style=container_style
        >
            <For
                each=move || each_for_render.get()
                key=move |item| key_for_key(item)
                children=move |item| {
                    let key_value = stable_key(&key_for_child(&item));
                    let child = child_fn(item);
                    let item_class = item_class_for_child.clone();
                    view! {
                        <div
                            class=item_class
                            data-animato-list-item="true"
                            data-animato-key=key_value
                            style="will-change:transform,opacity,filter;"
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
    container: &web_sys::Element,
    previous_rects: Rc<RefCell<HashMap<String, ItemRect>>>,
    move_duration: f32,
    move_easing: &'static str,
    move_delay: f32,
    stagger: f32,
    enter: PresenceAnimation,
) {
    if crate::ssr::is_hydrating() {
        return;
    }

    let elements = list_item_elements(container);
    let previous = previous_rects.borrow().clone();
    let mut next = HashMap::new();
    let mut targets = Vec::with_capacity(elements.len());

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
        let entering = !previous.contains_key(&key);
        let style = html.style();
        let _ = style.set_property("transition", "none");

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

        targets.push((element.clone(), index, entering));
    }

    *previous_rects.borrow_mut() = next;

    let target_transform = {
        let transform = enter.to.transform_string();
        if transform.is_empty() {
            "none".to_owned()
        } else {
            transform
        }
    };
    let target_opacity = enter.to.opacity.unwrap_or(1.0).to_string();
    let target_filter = enter
        .to
        .blur
        .map(|blur| format!("blur({blur}px)"))
        .unwrap_or_else(|| "none".to_owned());
    let enter_duration = enter.duration.max(0.0);
    let enter_easing = css_timing_function(&enter.easing);

    let _ = leptos::prelude::request_animation_frame_with_handle(move || {
        let _ = leptos::prelude::request_animation_frame_with_handle(move || {
            for (element, index, entering) in targets {
                let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() else {
                    continue;
                };
                let transition = item_transition(
                    move_duration,
                    move_easing,
                    move_delay,
                    enter_duration,
                    enter_easing,
                    stagger,
                    index,
                    entering,
                );
                let style = html.style();
                let _ = style.set_property("transition", &transition);
                let _ = style.set_property("transform", &target_transform);
                let _ = style.set_property("opacity", &target_opacity);
                let _ = style.set_property("filter", &target_filter);
            }
        });
    });
}

#[allow(clippy::too_many_arguments)]
#[cfg(any(
    test,
    all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
))]
fn item_transition(
    move_duration: f32,
    move_easing: &str,
    move_delay: f32,
    enter_duration: f32,
    enter_easing: &str,
    stagger: f32,
    index: usize,
    entering: bool,
) -> String {
    let stagger_delay = stagger.max(0.0) * index as f32;
    if entering {
        format!(
            "transform {enter_duration:.3}s {enter_easing} {stagger_delay:.3}s, \
             opacity {enter_duration:.3}s {enter_easing} {stagger_delay:.3}s, \
             filter {enter_duration:.3}s {enter_easing} {stagger_delay:.3}s"
        )
    } else {
        let transform_delay = move_delay.max(0.0) + stagger_delay;
        format!(
            "transform {move_duration:.3}s {move_easing} {transform_delay:.3}s, \
             opacity {move_duration:.3}s {move_easing} {stagger_delay:.3}s, \
             filter {move_duration:.3}s {move_easing} {stagger_delay:.3}s"
        )
    }
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

    #[test]
    fn entering_rows_use_presence_timing_without_move_delay() {
        let transition = item_transition(0.6, "linear", 0.3, 0.25, "ease-in", 0.02, 1, true);
        assert!(transition.contains("transform 0.250s ease-in 0.020s"));
        assert!(transition.contains("opacity 0.250s ease-in 0.020s"));
        assert!(!transition.contains("0.320s"));
    }

    #[test]
    fn existing_rows_delay_only_the_move_transform() {
        let transition = item_transition(0.6, "linear", 0.3, 0.25, "ease-in", 0.02, 1, false);
        assert!(transition.contains("transform 0.600s linear 0.320s"));
        assert!(transition.contains("opacity 0.600s linear 0.020s"));
    }
}
