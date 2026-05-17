use animato::{
    AnimatedStyle, AnimationBackend, PlatformAdapter, PresenceAnimation, ScrollConfig,
    ScrollProgressCalculator,
};

#[test]
fn dioxus_facade_reexports_core_types() {
    let css = AnimatedStyle::new()
        .opacity(0.5)
        .translate(10.0, 20.0)
        .to_css();
    assert!(css.contains("opacity:0.5;"));

    let fade = PresenceAnimation::fade();
    assert_eq!(fade.from.opacity, Some(0.0));
    assert_eq!(fade.to.opacity, Some(1.0));
}

#[test]
fn dioxus_facade_reexports_scroll_and_platform_helpers() {
    let mut calc = ScrollProgressCalculator::new(ScrollConfig {
        smooth: false,
        ..ScrollConfig::default()
    });
    assert_eq!(calc.calculate(100.0, 100.0, 100.0, 300.0), 1.0);

    assert!(matches!(
        PlatformAdapter::detect(),
        AnimationBackend::WebRaf | AnimationBackend::NativeClock | AnimationBackend::TerminalPoll
    ));
}
