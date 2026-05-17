use animato::{AnimationBackend, Easing, MotionConfig, PlatformAdapter, use_motion};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let motion = use_motion(0.0_f32);
    let backend = match PlatformAdapter::detect() {
        AnimationBackend::WebRaf => "web rAF",
        AnimationBackend::NativeClock => "native clock",
        AnimationBackend::TerminalPoll => "terminal poll",
    };
    let value = format!("{:.3}", motion.value());

    rsx! {
        main {
            style: "min-height:100vh; display:grid; place-items:center; font-family:system-ui, sans-serif;",
            section {
                style: "display:grid; gap:14px; min-width:320px;",
                h1 { style: "font-size:24px; margin:0;", "Cross Platform Motion" }
                p { style: "margin:0; color:#475569;", "Backend: {backend}" }
                progress {
                    max: "1",
                    value: "{value}",
                    style: "width:100%; height:20px;"
                }
                button {
                    onclick: move |_| {
                        motion.animate_to(
                            1.0,
                            MotionConfig::Tween {
                                duration: 0.6,
                                easing: Easing::EaseOutCubic,
                                delay: 0.0,
                            },
                        );
                    },
                    "Animate"
                }
            }
        }
    }
}
