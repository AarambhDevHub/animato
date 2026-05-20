use animato::{NativeWindowState, SpringConfig, use_spring, use_window_spring};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let (scale, spring) = use_spring(1.0_f32, SpringConfig::snappy());
    let (offset, offset_spring) = use_spring(0.0_f32, SpringConfig::wobbly());
    let window_handle = use_signal(|| use_window_spring(SpringConfig::stiff()));
    let mut window_state = use_signal(NativeWindowState::default);

    let grow = spring.clone();
    let settle = spring.clone();
    let pulse = spring.clone();
    let drift = offset_spring.clone();
    let center = offset_spring.clone();
    let move_window = window_handle;
    let snap_window = window_handle;

    let preview_style = format!(
        "{PREVIEW} transform:translateX({:.1}px) scale({:.3});",
        *offset.read(),
        *scale.read()
    );
    let current = *window_state.read();
    let position = format!("{:.0}, {:.0}", current.position[0], current.position[1]);
    let size = format!("{:.0} x {:.0}", current.size[0], current.size[1]);
    let opacity = format!("{:.0}%", current.opacity * 100.0);
    let scale_text = format!("{:.3}", *scale.read());
    let offset_text = format!("{:.1}px", *offset.read());
    let window_preview_style = format!(
        "width:54%; height:48%; border-radius:8px; background:linear-gradient(135deg,#2563eb,#16a34a); box-shadow:0 14px 30px rgba(37,99,235,.28); transform:translate({:.1}px,{:.1}px); opacity:{:.3};",
        current.position[0] / 18.0,
        current.position[1] / 18.0,
        current.opacity
    );

    rsx! {
        main { style: "{PAGE_STYLE}",
            section { style: "{APP_SHELL}",
                header { style: "{HEADER}",
                    div {
                        p { style: "{EYEBROW}", "dioxus desktop" }
                        h1 { style: "{TITLE}", "Spring system panel" }
                    }
                    div { style: "{STATUS_PILL}", "NativeClock" }
                }

                div { style: "{LAYOUT}",
                    div { style: "{STAGE}",
                        div { style: "{TRACK}",
                            div { style: "{preview_style}",
                                span { style: "{PREVIEW_LABEL}", "spring" }
                            }
                        }
                        div { style: "{METER_ROW}",
                            span { "Scale" }
                            strong { "{scale_text}" }
                            span { "Offset" }
                            strong { "{offset_text}" }
                        }
                    }

                    aside { style: "{SYSTEM_PANEL}",
                        h2 { style: "{PANEL_TITLE}", "Portable window state" }
                        div { style: "{STAT_GRID}",
                            Stat { label: "Position", value: position }
                            Stat { label: "Size", value: size }
                            Stat { label: "Opacity", value: opacity }
                        }
                        div { style: "{WINDOW_PREVIEW}",
                            div { style: "{window_preview_style}" }
                        }
                    }
                }

                div { style: "{TOOLBAR}",
                    button { style: "{PRIMARY_BUTTON}", onclick: move |_| grow.set_target(1.12), "Grow" }
                    button { style: "{BUTTON}", onclick: move |_| pulse.set_target(0.92), "Compress" }
                    button { style: "{BUTTON}", onclick: move |_| settle.set_target(1.0), "Settle" }
                    button { style: "{BUTTON}", onclick: move |_| drift.set_target(82.0), "Drift" }
                    button { style: "{BUTTON}", onclick: move |_| center.set_target(0.0), "Center" }
                    button {
                        style: "{BUTTON}",
                        onclick: move |_| {
                            let handle = move_window.read().clone();
                            handle.move_to(120.0, 80.0);
                            for _ in 0..28 {
                                handle.tick(1.0 / 60.0);
                            }
                            window_state.set(handle.state());
                        },
                        "Track move"
                    }
                    button {
                        style: "{GHOST_BUTTON}",
                        onclick: move |_| {
                            let handle = snap_window.read().clone();
                            handle.snap_to(0.0, 0.0);
                            window_state.set(handle.state());
                        },
                        "Snap origin"
                    }
                }
            }
        }
    }
}

#[component]
fn Stat(label: &'static str, value: String) -> Element {
    rsx! {
        div { style: "{STAT_CARD}",
            span { style: "{STAT_LABEL}", "{label}" }
            strong { style: "{STAT_VALUE}", "{value}" }
        }
    }
}

const PAGE_STYLE: &str = "min-height:100vh; display:grid; place-items:center; padding:30px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(135deg,#f8fafc 0%,#e0f2fe 45%,#dcfce7 100%); color:#0f172a;";
const APP_SHELL: &str = "width:min(920px, calc(100vw - 32px)); padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:rgba(255,255,255,.88); box-shadow:0 24px 70px rgba(15,23,42,.16);";
const HEADER: &str =
    "display:flex; align-items:center; justify-content:space-between; gap:18px; margin-bottom:20px;";
const EYEBROW: &str = "margin:0 0 4px; font-size:12px; font-weight:800; letter-spacing:.08em; text-transform:uppercase; color:#0369a1;";
const TITLE: &str = "margin:0; font-size:30px; line-height:1.05; letter-spacing:0;";
const STATUS_PILL: &str = "min-width:104px; text-align:center; padding:8px 10px; border-radius:8px; background:#0f172a; color:white; font-size:13px; font-weight:850;";
const LAYOUT: &str = "display:grid; grid-template-columns:minmax(0,1.35fr) minmax(260px,.65fr); gap:16px;";
const STAGE: &str = "padding:18px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:linear-gradient(180deg,#ffffff,#f8fafc);";
const TRACK: &str = "height:250px; position:relative; overflow:hidden; border-radius:8px; background:repeating-linear-gradient(90deg,#e2e8f0 0,#e2e8f0 1px,transparent 1px,transparent 42px), linear-gradient(180deg,#eef2ff,#f8fafc);";
const PREVIEW: &str = "position:absolute; left:50%; top:50%; width:148px; height:112px; margin-left:-74px; margin-top:-56px; display:grid; place-items:center; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:linear-gradient(135deg,#16a34a,#0ea5e9); color:white; box-shadow:0 24px 50px rgba(14,165,233,.28); will-change:transform;";
const PREVIEW_LABEL: &str = "font-size:13px; font-weight:850; letter-spacing:.08em; text-transform:uppercase;";
const METER_ROW: &str = "display:grid; grid-template-columns:auto 1fr auto 1fr; gap:8px 12px; margin-top:12px; color:#475569; font-size:13px; font-weight:750;";
const SYSTEM_PANEL: &str =
    "display:grid; gap:12px; padding:16px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:#f8fafc;";
const PANEL_TITLE: &str = "margin:0; font-size:16px; letter-spacing:0;";
const STAT_GRID: &str = "display:grid; gap:9px;";
const STAT_CARD: &str = "display:flex; align-items:center; justify-content:space-between; gap:12px; padding:11px 12px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:white;";
const STAT_LABEL: &str = "color:#64748b; font-size:12px; font-weight:800;";
const STAT_VALUE: &str = "color:#0f172a; font-size:14px;";
const WINDOW_PREVIEW: &str =
    "height:128px; display:grid; place-items:center; border-radius:8px; background:#e2e8f0; overflow:hidden;";
const TOOLBAR: &str = "display:flex; flex-wrap:wrap; gap:8px; margin-top:18px;";
const BUTTON: &str = "height:36px; padding:0 13px; border:1px solid rgba(15,23,42,.16); border-radius:8px; background:white; color:#0f172a; font-weight:750; cursor:pointer;";
const PRIMARY_BUTTON: &str = "height:36px; padding:0 14px; border:1px solid #0f172a; border-radius:8px; background:#0f172a; color:white; font-weight:850; cursor:pointer;";
const GHOST_BUTTON: &str = "height:36px; padding:0 13px; border:1px solid transparent; border-radius:8px; background:transparent; color:#475569; font-weight:750; cursor:pointer;";
