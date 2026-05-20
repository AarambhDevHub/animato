use animato::{Easing, use_tween};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let (progress, handle) = use_tween(0.0_f32, 1.0, |builder| {
        builder.duration(1.2).easing(Easing::EaseInOutSine)
    });
    let progress_value = (*progress.read()).clamp(0.0, 1.0);
    let percent_text = format!("{:.0}%", progress_value * 100.0);
    let decimal_text = format!("{:.3}", progress_value);
    let bar = terminal_bar(progress_value, 34);
    let fill_style = format!("{BAR_FILL} width:{:.2}%;", progress_value * 100.0);

    let restart = handle.clone();
    let finish = handle.clone();
    let pause = handle.clone();
    let resume = handle.clone();

    rsx! {
        main { style: "{PAGE_STYLE}",
            section { style: "{TERMINAL}",
                header { style: "{TITLE_ROW}",
                    div {
                        p { style: "{PROMPT}", "$ animato dioxus-tui-progress" }
                        h1 { style: "{TITLE}", "Progress monitor" }
                    }
                    div { style: "{STATUS}", "{percent_text}" }
                }

                div { style: "{SCREEN}",
                    div { style: "{LINE}",
                        span { style: "{KEY}", "task" }
                        span { "compile dioxus renderer" }
                    }
                    div { style: "{LINE}",
                        span { style: "{KEY}", "easing" }
                        span { "EaseInOutSine" }
                    }
                    div { style: "{LINE}",
                        span { style: "{KEY}", "value" }
                        span { "{decimal_text}" }
                    }
                    pre { style: "{ASCII_BAR}", "{bar}" }
                    div { style: "{BAR_TRACK}",
                        div { style: "{fill_style}" }
                    }
                }

                div { style: "{LOG_PANEL}",
                    LogLine { time: "00:00", text: "driver attached" }
                    LogLine { time: "00:01", text: "signal updated" }
                    LogLine { time: "00:02", text: "terminal frame painted" }
                }

                div { style: "{TOOLBAR}",
                    button { style: "{PRIMARY_BUTTON}", onclick: move |_| restart.reset(), "Restart" }
                    button { style: "{BUTTON}", onclick: move |_| pause.pause(), "Pause" }
                    button { style: "{BUTTON}", onclick: move |_| resume.resume(), "Resume" }
                    button { style: "{GHOST_BUTTON}", onclick: move |_| finish.seek(1.0), "Finish" }
                }
            }
        }
    }
}

#[component]
fn LogLine(time: &'static str, text: &'static str) -> Element {
    rsx! {
        div { style: "{LOG_LINE}",
            span { style: "{LOG_TIME}", "{time}" }
            span { "{text}" }
        }
    }
}

fn terminal_bar(progress: f32, width: usize) -> String {
    let filled = (progress.clamp(0.0, 1.0) * width as f32).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "#".repeat(filled), ".".repeat(empty))
}

const PAGE_STYLE: &str = "min-height:100vh; display:grid; place-items:center; padding:28px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(135deg,#0f172a 0%,#164e63 52%,#14532d 100%); color:#d1fae5;";
const TERMINAL: &str = "width:min(760px, calc(100vw - 32px)); padding:22px; border:1px solid rgba(187,247,208,.28); border-radius:8px; background:rgba(2,6,23,.88); box-shadow:0 28px 80px rgba(0,0,0,.34);";
const TITLE_ROW: &str =
    "display:flex; align-items:center; justify-content:space-between; gap:18px; margin-bottom:18px;";
const PROMPT: &str =
    "margin:0 0 5px; color:#86efac; font-family:ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size:12px;";
const TITLE: &str = "margin:0; color:#f8fafc; font-size:28px; line-height:1.05; letter-spacing:0;";
const STATUS: &str = "min-width:76px; text-align:center; padding:8px 10px; border:1px solid rgba(134,239,172,.45); border-radius:8px; background:rgba(20,83,45,.72); color:#bbf7d0; font-family:ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size:13px; font-weight:850;";
const SCREEN: &str = "display:grid; gap:10px; padding:18px; border:1px solid rgba(148,163,184,.24); border-radius:8px; background:#020617; font-family:ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;";
const LINE: &str = "display:grid; grid-template-columns:86px 1fr; gap:12px; color:#cbd5e1; font-size:13px;";
const KEY: &str = "color:#67e8f9;";
const ASCII_BAR: &str = "margin:8px 0 0; color:#86efac; font-size:15px; line-height:1.35; white-space:pre-wrap;";
const BAR_TRACK: &str = "height:12px; border-radius:999px; background:#1e293b; overflow:hidden;";
const BAR_FILL: &str = "height:100%; border-radius:999px; background:linear-gradient(90deg,#22c55e,#67e8f9);";
const LOG_PANEL: &str = "display:grid; gap:8px; margin-top:14px; padding:13px; border:1px solid rgba(148,163,184,.20); border-radius:8px; background:rgba(15,23,42,.72);";
const LOG_LINE: &str =
    "display:grid; grid-template-columns:64px 1fr; gap:10px; color:#cbd5e1; font-family:ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size:12px;";
const LOG_TIME: &str = "color:#94a3b8;";
const TOOLBAR: &str = "display:flex; flex-wrap:wrap; gap:8px; margin-top:16px;";
const BUTTON: &str = "height:36px; padding:0 13px; border:1px solid rgba(134,239,172,.28); border-radius:8px; background:#0f172a; color:#d1fae5; font-weight:750; cursor:pointer;";
const PRIMARY_BUTTON: &str = "height:36px; padding:0 14px; border:1px solid #86efac; border-radius:8px; background:#86efac; color:#052e16; font-weight:850; cursor:pointer;";
const GHOST_BUTTON: &str = "height:36px; padding:0 13px; border:1px solid transparent; border-radius:8px; background:transparent; color:#a7f3d0; font-weight:750; cursor:pointer;";
