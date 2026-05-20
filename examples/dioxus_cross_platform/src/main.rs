use animato::{AnimationBackend, Easing, MotionConfig, PlatformAdapter, SpringConfig, use_motion};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let motion = use_motion(0.0_f32);
    let backend = PlatformAdapter::detect();
    let backend_label = match backend {
        AnimationBackend::WebRaf => "web rAF",
        AnimationBackend::NativeClock => "native clock",
        AnimationBackend::TerminalPoll => "terminal poll",
    };
    let platform_badge = match backend {
        AnimationBackend::WebRaf => "Browser",
        AnimationBackend::NativeClock => "Desktop",
        AnimationBackend::TerminalPoll => "Terminal",
    };
    let value = motion.value().clamp(0.0, 1.0);
    let value_text = format!("{:.3}", value);
    let percent_text = format!("{:.0}%", value * 100.0);
    let progress_style = format!("{PROGRESS_FILL} width:{:.2}%;", value * 100.0);
    let orb_style = format!(
        "{ORBIT_DOT} transform:translateX({:.1}px) rotate({:.1}deg);",
        value * 296.0,
        value * 180.0
    );

    let animate = motion.clone();
    let spring = motion.clone();
    let snap = motion.clone();
    let stop = motion.clone();

    rsx! {
        main { style: "{PAGE_STYLE}",
            section { style: "{APP_SHELL}",
                header { style: "{HEADER}",
                    div {
                        p { style: "{EYEBROW}", "cross platform" }
                        h1 { style: "{TITLE}", "One motion handle" }
                    }
                    div { style: "{BACKEND_BADGE}", "{platform_badge}" }
                }

                div { style: "{HERO_GRID}",
                    div { style: "{STAGE}",
                        div { style: "{ORBIT_TRACK}",
                            div { style: "{orb_style}" }
                        }
                        div { style: "{PROGRESS_TRACK}",
                            div { style: "{progress_style}" }
                        }
                    }

                    aside { style: "{INFO_PANEL}",
                        h2 { style: "{PANEL_TITLE}", "Runtime" }
                        InfoRow { label: "Backend", value: backend_label.to_owned() }
                        InfoRow { label: "Progress", value: percent_text.clone() }
                        InfoRow { label: "Signal", value: value_text.clone() }
                        InfoRow { label: "Feature", value: "dioxus-web / dioxus-desktop".to_owned() }
                    }
                }

                div { style: "{TOOLBAR}",
                    button {
                        style: "{PRIMARY_BUTTON}",
                        onclick: move |_| {
                            animate.animate_to(
                                1.0,
                                MotionConfig::Tween {
                                    duration: 0.7,
                                    easing: Easing::EaseOutCubic,
                                    delay: 0.0,
                                },
                            );
                        },
                        "Tween to 100%"
                    }
                    button {
                        style: "{BUTTON}",
                        onclick: move |_| spring.spring_to(0.35, SpringConfig::wobbly()),
                        "Spring back"
                    }
                    button {
                        style: "{BUTTON}",
                        onclick: move |_| snap.snap_to(0.0),
                        "Snap zero"
                    }
                    button {
                        style: "{GHOST_BUTTON}",
                        onclick: move |_| stop.stop(),
                        "Stop"
                    }
                }
            }
        }
    }
}

#[component]
fn InfoRow(label: &'static str, value: String) -> Element {
    rsx! {
        div { style: "{INFO_ROW}",
            span { "{label}" }
            strong { "{value}" }
        }
    }
}

const PAGE_STYLE: &str = "min-height:100vh; display:grid; place-items:center; padding:32px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(135deg,#f8fafc 0%,#dbeafe 48%,#f0fdf4 100%); color:#0f172a;";
const APP_SHELL: &str = "width:min(860px, calc(100vw - 32px)); padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:rgba(255,255,255,.88); box-shadow:0 24px 70px rgba(15,23,42,.16);";
const HEADER: &str =
    "display:flex; align-items:center; justify-content:space-between; gap:18px; margin-bottom:22px;";
const EYEBROW: &str = "margin:0 0 4px; font-size:12px; font-weight:800; letter-spacing:.08em; text-transform:uppercase; color:#1d4ed8;";
const TITLE: &str = "margin:0; font-size:30px; line-height:1.05; letter-spacing:0;";
const BACKEND_BADGE: &str = "min-width:96px; text-align:center; padding:8px 10px; border-radius:8px; background:#0f172a; color:white; font-size:13px; font-weight:850;";
const HERO_GRID: &str = "display:grid; grid-template-columns:minmax(0,1.3fr) minmax(260px,.7fr); gap:16px;";
const STAGE: &str = "display:grid; gap:16px; padding:18px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:linear-gradient(180deg,#ffffff,#f8fafc);";
const ORBIT_TRACK: &str = "height:260px; position:relative; overflow:hidden; border-radius:8px; background:repeating-linear-gradient(90deg,#e2e8f0 0,#e2e8f0 1px,transparent 1px,transparent 37px), linear-gradient(135deg,#eef2ff,#f0fdf4);";
const ORBIT_DOT: &str = "position:absolute; left:24px; top:88px; width:88px; height:88px; border-radius:8px; background:linear-gradient(135deg,#2563eb,#16a34a); box-shadow:0 24px 46px rgba(37,99,235,.30); will-change:transform;";
const PROGRESS_TRACK: &str = "height:14px; border-radius:999px; background:#e2e8f0; overflow:hidden;";
const PROGRESS_FILL: &str = "height:100%; border-radius:999px; background:linear-gradient(90deg,#2563eb,#16a34a);";
const INFO_PANEL: &str =
    "display:grid; align-content:start; gap:10px; padding:16px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:#f8fafc;";
const PANEL_TITLE: &str = "margin:0 0 4px; font-size:16px; letter-spacing:0;";
const INFO_ROW: &str = "display:flex; justify-content:space-between; align-items:center; gap:16px; padding:11px 12px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:white; color:#475569; font-size:13px; font-weight:750;";
const TOOLBAR: &str = "display:flex; flex-wrap:wrap; gap:8px; margin-top:18px;";
const BUTTON: &str = "height:36px; padding:0 13px; border:1px solid rgba(15,23,42,.16); border-radius:8px; background:white; color:#0f172a; font-weight:750; cursor:pointer;";
const PRIMARY_BUTTON: &str = "height:36px; padding:0 14px; border:1px solid #0f172a; border-radius:8px; background:#0f172a; color:white; font-weight:850; cursor:pointer;";
const GHOST_BUTTON: &str = "height:36px; padding:0 13px; border:1px solid transparent; border-radius:8px; background:transparent; color:#475569; font-weight:750; cursor:pointer;";
