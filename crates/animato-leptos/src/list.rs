//! FLIP-ready list rendering helpers.

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use crate::AnimatedStyle;
use crate::PresenceAnimation;
use animato_core::Easing;
use leptos::prelude::*;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::cell::{Cell, RefCell};
use std::collections::hash_map::DefaultHasher;
#[cfg(any(
    test,
    all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
))]
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::rc::Rc;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use wasm_bindgen::{JsCast, closure::Closure};

#[cfg(any(
    test,
    all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
))]
const EXIT_REMOVAL_GRACE_SECONDS: f32 = 0.10;

#[derive(Clone, Debug)]
struct RenderedItem<T, K> {
    key: K,
    dom_key: String,
    item: T,
    #[cfg(any(
        test,
        all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
    ))]
    exiting: bool,
    #[cfg(any(
        test,
        all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
    ))]
    exit_generation: u64,
}

#[cfg(any(
    test,
    all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
))]
#[derive(Clone, Debug, PartialEq, Eq)]
struct ExitSchedule<K> {
    key: K,
    generation: u64,
    index: usize,
}

/// FLIP-ready keyed list component.
///
/// Removed rows remain mounted for the configured exit transition. Once the
/// exit duration and stagger delay have elapsed, the row is removed and the
/// surviving rows animate to their new positions using FLIP transforms.
#[component]
pub fn AnimatedFor<T, K, KF, CF, IV>(
    /// Reactive list source.
    each: Signal<Vec<T>>,
    /// Stable key extractor.
    key: KF,
    /// Child renderer.
    children: CF,
    /// Optional enter animation for initial and inserted rows.
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
    /// Delay before surviving rows move after an exiting row is removed.
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
    let exit = exit.unwrap_or_else(|| enter.reversed());
    let duration = move_duration.unwrap_or(0.25).max(0.0);
    let easing = move_easing.unwrap_or(Easing::EaseOutCubic);
    let easing_label = format!("{easing:?}");
    let move_delay = move_delay.unwrap_or(0.0).max(0.0);
    let stagger = stagger_delay.unwrap_or(0.0).max(0.0);
    let gap = gap.unwrap_or(0.0).max(0.0);
    let container_style = format!("display:flex; flex-direction:column; gap:{gap:.3}px;");
    let item_class = item_class.unwrap_or_default();
    let container = NodeRef::<leptos::html::Div>::new();

    let initial_items = each
        .get_untracked()
        .into_iter()
        .map(|item| {
            let item_key = key(&item);
            RenderedItem {
                dom_key: stable_key(&item_key),
                key: item_key,
                item,
                #[cfg(any(
                    test,
                    all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
                ))]
                exiting: false,
                #[cfg(any(
                    test,
                    all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
                ))]
                exit_generation: 0,
            }
        })
        .collect::<Vec<_>>();
    let (rendered_items, set_rendered_items) = signal(initial_items);
    #[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
    let _set_rendered_items = set_rendered_items;
    let child_fn = children;
    let item_class_for_child = item_class;

    #[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
    let _exit_for_native = &exit;

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    {
        let source_effect_initialized = Rc::new(Cell::new(false));
        let each_for_source = each;
        let key_for_source = key.clone();
        let exit_duration = exit.duration.max(0.0);
        let component_disposed = Arc::new(AtomicBool::new(false));
        let component_disposed_for_cleanup = Arc::clone(&component_disposed);
        on_cleanup(move || {
            component_disposed_for_cleanup.store(true, Ordering::Relaxed);
        });

        Effect::new(move || {
            let source = each_for_source.get();

            // The retained render list is initialized synchronously from the
            // source. Skip the effect's first pass to avoid replacing it before
            // the mount-owned initial enter animation runs.
            if !source_effect_initialized.replace(true) {
                return;
            }

            let current = rendered_items.get_untracked();
            let (next, exits) = reconcile_rendered_items(&current, source, &key_for_source);
            set_rendered_items.set(next);

            for schedule in exits {
                let key = schedule.key;
                let generation = schedule.generation;
                let delay_ms = exit_removal_delay_ms(exit_duration, stagger, schedule.index);
                let set_rendered_items_for_exit = set_rendered_items;
                let component_disposed = Arc::clone(&component_disposed);
                let callback = Closure::once_into_js(move || {
                    if component_disposed.load(Ordering::Relaxed) {
                        return;
                    }
                    set_rendered_items_for_exit.update(|items| {
                        items.retain(|entry| !matches_exit(entry, &key, generation));
                    });
                });

                if let Some(window) = web_sys::window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        callback.as_ref().unchecked_ref(),
                        delay_ms,
                    );
                }
            }
        });

        let previous_rects = Rc::new(RefCell::new(HashMap::new()));
        let animation_effect_initialized = Rc::new(Cell::new(false));
        let initial_rects = Rc::clone(&previous_rects);
        let initial_enter = enter.clone();
        let initial_exit = exit.clone();
        let initial_easing = easing.clone();

        container.on_load(move |container| {
            animate_flip(
                &container,
                Rc::clone(&initial_rects),
                duration,
                css_timing_function(&initial_easing),
                move_delay,
                stagger,
                initial_enter.clone(),
                initial_exit.clone(),
                &HashSet::new(),
            );
        });

        let enter_for_effect = enter.clone();
        let exit_for_effect = exit.clone();
        let easing_for_effect = easing.clone();
        Effect::new(move || {
            let items = rendered_items.get();
            let exiting_keys = items
                .iter()
                .filter(|entry| entry.exiting)
                .map(|entry| entry.dom_key.clone())
                .collect::<HashSet<_>>();

            // The node-load callback owns the first enter pass.
            if !animation_effect_initialized.replace(true) {
                return;
            }

            let previous_rects = Rc::clone(&previous_rects);
            let enter = enter_for_effect.clone();
            let exit = exit_for_effect.clone();
            let easing = easing_for_effect.clone();
            let _ = request_animation_frame_with_handle(move || {
                let Some(container) = container.get_untracked() else {
                    return;
                };
                animate_flip(
                    &container,
                    previous_rects,
                    duration,
                    css_timing_function(&easing),
                    move_delay,
                    stagger,
                    enter,
                    exit,
                    &exiting_keys,
                );
            });
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
                each=move || rendered_items.get()
                key=move |entry| entry.key.clone()
                children=move |entry| {
                    let key_value = entry.dom_key;
                    let child = child_fn(entry.item);
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

#[cfg(any(
    test,
    all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
))]
fn reconcile_rendered_items<T, K, KF>(
    current: &[RenderedItem<T, K>],
    source: Vec<T>,
    key_fn: &KF,
) -> (Vec<RenderedItem<T, K>>, Vec<ExitSchedule<K>>)
where
    T: Clone,
    K: Eq + Hash + Clone,
    KF: Fn(&T) -> K,
{
    let source_entries = source
        .into_iter()
        .map(|item| (key_fn(&item), item))
        .collect::<Vec<_>>();
    let source_keys = source_entries
        .iter()
        .map(|(key, _)| key.clone())
        .collect::<HashSet<_>>();
    let current_by_key = current
        .iter()
        .map(|entry| (entry.key.clone(), entry))
        .collect::<HashMap<_, _>>();

    let mut next = source_entries
        .into_iter()
        .map(|(key, item)| {
            if let Some(existing) = current_by_key.get(&key) {
                RenderedItem {
                    key,
                    dom_key: existing.dom_key.clone(),
                    item,
                    exiting: false,
                    exit_generation: existing.exit_generation,
                }
            } else {
                RenderedItem {
                    dom_key: stable_key(&key),
                    key,
                    item,
                    exiting: false,
                    exit_generation: 0,
                }
            }
        })
        .collect::<Vec<_>>();

    let mut retained_exits = Vec::new();
    let mut schedules = Vec::new();
    for (index, entry) in current.iter().enumerate() {
        if source_keys.contains(&entry.key) {
            continue;
        }

        let mut retained = entry.clone();
        if !retained.exiting {
            retained.exiting = true;
            retained.exit_generation = retained.exit_generation.saturating_add(1);
            schedules.push(ExitSchedule {
                key: retained.key.clone(),
                generation: retained.exit_generation,
                index,
            });
        }
        retained_exits.push((index, retained));
    }

    for (index, entry) in retained_exits {
        next.insert(index.min(next.len()), entry);
    }

    (next, schedules)
}

#[cfg(any(
    test,
    all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
))]
fn matches_exit<T, K: Eq>(entry: &RenderedItem<T, K>, key: &K, generation: u64) -> bool {
    entry.key.eq(key) && entry.exiting && entry.exit_generation == generation
}

#[cfg(any(
    test,
    all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
))]
fn exit_removal_delay_ms(exit_duration: f32, stagger: f32, index: usize) -> i32 {
    let seconds =
        exit_duration.max(0.0) + stagger.max(0.0) * index as f32 + EXIT_REMOVAL_GRACE_SECONDS;
    (seconds * 1000.0).round().clamp(0.0, i32::MAX as f32) as i32
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
#[derive(Clone, Copy, Debug)]
struct ItemRect {
    left: f64,
    top: f64,
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ItemPhase {
    Entering,
    Moving,
    Exiting,
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
#[allow(clippy::too_many_arguments)]
fn animate_flip(
    container: &web_sys::Element,
    previous_rects: Rc<RefCell<HashMap<String, ItemRect>>>,
    move_duration: f32,
    move_easing: &'static str,
    move_delay: f32,
    stagger: f32,
    enter: PresenceAnimation,
    exit: PresenceAnimation,
    exiting_keys: &HashSet<String>,
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
        let exiting = exiting_keys.contains(&key);
        let entering = !exiting && !previous.contains_key(&key);
        let phase = if exiting {
            ItemPhase::Exiting
        } else if entering {
            ItemPhase::Entering
        } else {
            ItemPhase::Moving
        };
        let style = html.style();
        let _ = style.set_property("transition", "none");

        match phase {
            ItemPhase::Entering => {
                apply_animated_style(&style, &enter.from);
                let _ = style.remove_property("pointer-events");
            }
            ItemPhase::Exiting => {
                apply_animated_style(&style, &exit.from);
                let _ = style.set_property("pointer-events", "none");
            }
            ItemPhase::Moving => {
                let normal_transform = resolved_transform(&enter.to);
                if let Some(before) = previous.get(&key) {
                    let dx = before.left - rect.left();
                    let dy = before.top - rect.top();
                    if dx.abs() > 0.5 || dy.abs() > 0.5 {
                        let inverse = if normal_transform == "none" {
                            format!("translate({dx:.1}px,{dy:.1}px)")
                        } else {
                            format!("translate({dx:.1}px,{dy:.1}px) {normal_transform}")
                        };
                        let _ = style.set_property("transform", &inverse);
                    } else {
                        let _ = style.set_property("transform", &normal_transform);
                    }
                }
                let _ = style.set_property("opacity", &resolved_opacity(&enter.to));
                let _ = style.set_property("filter", &resolved_filter(&enter.to));
                let _ = style.remove_property("pointer-events");
            }
        }

        targets.push((element.clone(), index, phase));
    }

    *previous_rects.borrow_mut() = next;

    let enter_duration = enter.duration.max(0.0);
    let enter_easing = css_timing_function(&enter.easing);
    let exit_duration = exit.duration.max(0.0);
    let exit_easing = css_timing_function(&exit.easing);
    let enter_target = resolved_style(&enter.to);
    let exit_target = resolved_style(&exit.to);

    let _ = request_animation_frame_with_handle(move || {
        let _ = request_animation_frame_with_handle(move || {
            for (element, index, phase) in targets {
                let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() else {
                    continue;
                };
                let transition = match phase {
                    ItemPhase::Entering => {
                        presence_transition(enter_duration, enter_easing, stagger, index)
                    }
                    ItemPhase::Moving => item_transition(
                        move_duration,
                        move_easing,
                        move_delay,
                        enter_duration,
                        enter_easing,
                        stagger,
                        index,
                        false,
                    ),
                    ItemPhase::Exiting => {
                        presence_transition(exit_duration, exit_easing, stagger, index)
                    }
                };
                let target = if phase == ItemPhase::Exiting {
                    &exit_target
                } else {
                    &enter_target
                };
                let style = html.style();
                let _ = style.set_property("transition", &transition);
                let _ = style.set_property("transform", &target.0);
                let _ = style.set_property("opacity", &target.1);
                let _ = style.set_property("filter", &target.2);
            }
        });
    });
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn apply_animated_style(style: &web_sys::CssStyleDeclaration, value: &AnimatedStyle) {
    let _ = style.set_property("transform", &resolved_transform(value));
    let _ = style.set_property("opacity", &resolved_opacity(value));
    let _ = style.set_property("filter", &resolved_filter(value));
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn resolved_style(value: &AnimatedStyle) -> (String, String, String) {
    (
        resolved_transform(value),
        resolved_opacity(value),
        resolved_filter(value),
    )
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn resolved_transform(value: &AnimatedStyle) -> String {
    let transform = value.transform_string();
    if transform.is_empty() {
        "none".to_owned()
    } else {
        transform
    }
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn resolved_opacity(value: &AnimatedStyle) -> String {
    value.opacity.unwrap_or(1.0).to_string()
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn resolved_filter(value: &AnimatedStyle) -> String {
    value
        .blur
        .map(|blur| format!("blur({blur}px)"))
        .unwrap_or_else(|| "none".to_owned())
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
    if entering {
        presence_transition(enter_duration, enter_easing, stagger, index)
    } else {
        let stagger_delay = stagger.max(0.0) * index as f32;
        let transform_delay = move_delay.max(0.0) + stagger_delay;
        format!(
            "transform {move_duration:.3}s {move_easing} {transform_delay:.3}s, \
             opacity {move_duration:.3}s {move_easing} {stagger_delay:.3}s, \
             filter {move_duration:.3}s {move_easing} {stagger_delay:.3}s"
        )
    }
}

#[cfg(any(
    test,
    all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))
))]
fn presence_transition(duration: f32, easing: &str, stagger: f32, index: usize) -> String {
    let delay = stagger.max(0.0) * index as f32;
    format!(
        "transform {duration:.3}s {easing} {delay:.3}s, \
         opacity {duration:.3}s {easing} {delay:.3}s, \
         filter {duration:.3}s {easing} {delay:.3}s"
    )
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

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TestItem {
        id: u32,
        label: &'static str,
    }

    fn rendered(id: u32) -> RenderedItem<TestItem, u32> {
        RenderedItem {
            key: id,
            dom_key: stable_key(&id),
            item: TestItem { id, label: "item" },
            exiting: false,
            exit_generation: 0,
        }
    }

    #[test]
    fn stable_key_is_deterministic_and_distinguishes_values() {
        assert_eq!(stable_key(&"row-1"), stable_key(&"row-1"));
        assert_ne!(stable_key(&"row-1"), stable_key(&"row-2"));
    }

    #[test]
    fn removed_rows_are_retained_and_scheduled_once() {
        let current = vec![rendered(1), rendered(2), rendered(3)];
        let source = vec![
            TestItem {
                id: 1,
                label: "one",
            },
            TestItem {
                id: 3,
                label: "three",
            },
        ];

        let (next, schedules) = reconcile_rendered_items(&current, source, &|item| item.id);
        assert_eq!(
            next.iter().map(|entry| entry.key).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert!(next[1].exiting);
        assert_eq!(
            schedules,
            vec![ExitSchedule {
                key: 2,
                generation: 1,
                index: 1
            }]
        );

        let source = vec![
            TestItem {
                id: 1,
                label: "one",
            },
            TestItem {
                id: 3,
                label: "three",
            },
        ];
        let (_, repeated_schedules) = reconcile_rendered_items(&next, source, &|item| item.id);
        assert!(repeated_schedules.is_empty());
    }

    #[test]
    fn returning_key_cancels_the_exit_state_without_changing_generation() {
        let mut exiting = rendered(2);
        exiting.exiting = true;
        exiting.exit_generation = 4;
        let source = vec![TestItem {
            id: 2,
            label: "returned",
        }];

        let (next, schedules) = reconcile_rendered_items(&[exiting], source, &|item| item.id);
        assert!(!next[0].exiting);
        assert_eq!(next[0].exit_generation, 4);
        assert_eq!(next[0].item.label, "returned");
        assert!(schedules.is_empty());
    }

    #[test]
    fn multiple_removed_rows_keep_their_previous_order() {
        let current = vec![rendered(1), rendered(2), rendered(3), rendered(4)];
        let source = vec![
            TestItem {
                id: 2,
                label: "two",
            },
            TestItem {
                id: 4,
                label: "four",
            },
        ];

        let (next, schedules) = reconcile_rendered_items(&current, source, &|item| item.id);
        assert_eq!(
            next.iter().map(|entry| entry.key).collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
        assert_eq!(schedules.len(), 2);
        assert!(next[0].exiting);
        assert!(next[2].exiting);
    }

    #[test]
    fn stale_exit_generation_cannot_remove_a_newer_exit() {
        let mut entry = rendered(2);
        entry.exiting = true;
        entry.exit_generation = 2;

        assert!(!matches_exit(&entry, &2, 1));
        assert!(matches_exit(&entry, &2, 2));
        entry.exiting = false;
        assert!(!matches_exit(&entry, &2, 2));
    }

    #[test]
    fn exit_removal_waits_for_duration_stagger_and_frame_grace() {
        assert_eq!(exit_removal_delay_ms(0.25, 0.05, 2), 450);
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

    #[test]
    fn presence_transition_applies_stagger_to_all_properties() {
        let transition = presence_transition(0.4, "ease-out", 0.03, 2);
        assert!(transition.contains("transform 0.400s ease-out 0.060s"));
        assert!(transition.contains("filter 0.400s ease-out 0.060s"));
    }
}
