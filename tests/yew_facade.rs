use animato::yew::{
    AgentInput, AgentTweenSpec, AnimatedStyle, PresenceAnimation, ScrollConfig, SwipeConfig,
    TransitionMode, stable_key,
};

#[test]
fn yew_facade_exports_core_types() {
    let style = AnimatedStyle::new()
        .opacity(0.5)
        .translate(12.0, 4.0)
        .scale(1.1);
    let css = style.to_css();

    assert!(css.contains("opacity:0.5;"));
    assert!(css.contains("translate(12px,4px)"));
    assert_eq!(PresenceAnimation::fade().to.opacity, Some(1.0));
    assert_eq!(ScrollConfig::default().offset_end, 1.0);
    assert_eq!(TransitionMode::default(), TransitionMode::Sequential);
    assert_eq!(SwipeConfig::default().min_distance, 40.0);
    assert_eq!(stable_key(&"item"), stable_key(&"item"));
}

#[test]
fn yew_facade_exports_agent_messages() {
    let input = AgentInput::Tween(AgentTweenSpec::new("x", 0.0, 1.0).duration(0.2));

    assert!(matches!(input, AgentInput::Tween(spec) if spec.id == "x"));
}
