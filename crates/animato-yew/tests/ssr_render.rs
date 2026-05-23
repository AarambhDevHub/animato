#![cfg(feature = "ssr")]

use animato_core::Easing;
use animato_spring::SpringConfig;
use animato_yew::{
    AgentOutput, AnimatePresence, AnimatedFor, AnimatedStyle, DragConfig, GestureConfig,
    PageTransition, PresenceAnimation, ScrollConfig, ScrollTriggerConfig, SmoothScroll,
    SwipeConfig, TransitionMode, stable_key, use_animation_agent, use_css_spring, use_css_tween,
    use_drag, use_gesture, use_keyframes, use_pinch, use_route_transition_key, use_scroll_progress,
    use_scroll_trigger, use_scroll_velocity, use_spring, use_swipe, use_timeline, use_tween,
};
use yew::prelude::*;

#[function_component(HookShowcase)]
fn hook_showcase() -> Html {
    let (tween_value, tween) = use_tween(0.0_f32, 1.0, |builder| {
        builder.duration(0.2).easing(Easing::EaseOutCubic)
    });
    let (spring_value, spring) = use_spring(0.0_f32, SpringConfig::snappy());
    let timeline = use_timeline(|timeline| timeline);
    let (keyframe_value, keyframes) =
        use_keyframes(|track| track.push(0.0, 0.0_f32).push(1.0, 1.0));
    let tween_css = use_css_tween(
        AnimatedStyle::new().opacity(0.0),
        AnimatedStyle::new().opacity(1.0).translate(10.0, 0.0),
        0.2,
        Easing::EaseOutCubic,
    );
    let spring_css = use_css_spring(AnimatedStyle::new().scale(1.1), SpringConfig::gentle());

    let node = NodeRef::default();
    let scroll_progress = use_scroll_progress(node.clone(), ScrollConfig::default());
    let scroll_trigger = use_scroll_trigger(node.clone(), ScrollTriggerConfig::default());
    let scroll_velocity = use_scroll_velocity();
    let (drag_position, drag) = use_drag(node.clone(), DragConfig::default());
    let gesture = use_gesture(node.clone(), GestureConfig::default());
    let (pinch_scale, pinch) = use_pinch(node.clone());
    let swipe = use_swipe(node, SwipeConfig::default());
    let route_key = use_route_transition_key();
    let agent = use_animation_agent(Callback::from(|_: AgentOutput| ()));

    let debug_summary = format!(
        "{:?}{:?}{:?}{:?}{:?}{:?}{:?}",
        tween, spring, timeline, keyframes, drag, pinch, agent
    );

    let state_summary = format!(
        "{:.1}:{:.1}:{:.1}:{:.1}:{:.1}:{:.1}:{:.1}:{:?}:{:?}:{}:{}:{}:{}:{}:{}:{}:{}",
        *tween_value,
        *spring_value,
        *keyframe_value,
        *scroll_progress,
        *scroll_velocity,
        drag_position[0],
        *pinch_scale,
        *gesture,
        *swipe,
        *tween.progress(),
        *tween.is_complete(),
        format!("{:?}", *tween.state()),
        *spring.is_settled(),
        *timeline.progress(),
        *timeline.is_complete(),
        format!("{:?}", *timeline.state()),
        *keyframes.progress(),
    );

    html! {
        <section data-test="hooks">
            <span data-route={route_key}>{ state_summary }</span>
            <span>{ debug_summary }</span>
            <span>{ (*tween.value()).to_string() }</span>
            <span>{ (*spring.value()).to_string() }</span>
            <span>{ (*keyframes.value()).to_string() }</span>
            <span>{ (*keyframes.is_complete()).to_string() }</span>
            <span>{ (*scroll_trigger.active()).to_string() }</span>
            <span>{ (*scroll_trigger.progress()).to_string() }</span>
            <span>{ (*drag.position())[0].to_string() }</span>
            <span>{ format!("{:?}", *agent.last_output()) }</span>
            <span>{ (*tween_css).clone() }</span>
            <span>{ (*spring_css).clone() }</span>
        </section>
    }
}

#[function_component(UiShowcase)]
fn ui_showcase() -> Html {
    let items = vec![1, 2, 3];
    let key_fn = Callback::from(|value: i32| stable_key(&value));
    let render = Callback::from(|value: i32| html! { <span>{ value }</span> });

    html! {
        <SmoothScroll>
            <AnimatePresence show={true} enter={PresenceAnimation::slide_up()}>
                <PageTransition mode={TransitionMode::SlideOver} route_key={Some("dashboard".to_owned())}>
                    <AnimatedFor<i32>
                        items={items}
                        key_fn={key_fn}
                        render={render}
                        enter={PresenceAnimation::fade()}
                        move_duration={0.2}
                        stagger_delay={0.01}
                    />
                </PageTransition>
            </AnimatePresence>
        </SmoothScroll>
    }
}

#[function_component(AlternateUiShowcase)]
fn alternate_ui_showcase() -> Html {
    html! {
        <>
            <AnimatePresence show={false} wait_exit={false} exit={PresenceAnimation::blur_in()}>
                <span>{ "hidden" }</span>
            </AnimatePresence>
            <PageTransition mode={TransitionMode::MorphHero}>
                <span>{ "hero" }</span>
            </PageTransition>
            <PageTransition mode={TransitionMode::CrossFade}>
                <span>{ "fade" }</span>
            </PageTransition>
        </>
    }
}

#[test]
fn ssr_renders_hooks_without_browser_runtime() {
    let rendered = pollster::block_on(
        yew::LocalServerRenderer::<HookShowcase>::new()
            .hydratable(false)
            .render(),
    );

    assert!(rendered.contains("data-test=\"hooks\""));
    assert!(rendered.contains("TweenHandle"));
    assert!(rendered.contains("opacity:0;"));
}

#[test]
fn ssr_renders_ui_components_with_animation_metadata() {
    let rendered = pollster::block_on(
        yew::LocalServerRenderer::<UiShowcase>::new()
            .hydratable(false)
            .render(),
    );

    assert!(rendered.contains("data-animato-smooth-scroll=\"true\""));
    assert!(rendered.contains("data-animato-presence=\"true\""));
    assert!(rendered.contains("data-animato-page-transition=\"SlideOver\""));
    assert!(rendered.contains("data-animato-list-item=\"true\""));
}

#[test]
fn ssr_renders_hidden_presence_and_alternate_page_modes() {
    let rendered = pollster::block_on(
        yew::LocalServerRenderer::<AlternateUiShowcase>::new()
            .hydratable(false)
            .render(),
    );

    assert!(rendered.contains("display:none;"));
    assert!(rendered.contains("data-animato-page-transition=\"MorphHero\""));
    assert!(rendered.contains("data-animato-page-transition=\"CrossFade\""));
    assert!(rendered.contains("scale(1)"));
}
