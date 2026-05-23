use animato::{
    ScrollConfig, ScrollTriggerConfig, use_scroll_progress, use_scroll_trigger, use_scroll_velocity,
};
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let target = use_node_ref();
    let progress = use_scroll_progress(
        target.clone(),
        ScrollConfig {
            smooth: false,
            ..ScrollConfig::default()
        },
    );
    let trigger = use_scroll_trigger(
        target.clone(),
        ScrollTriggerConfig {
            threshold: 0.42,
            once: false,
            ..ScrollTriggerConfig::default()
        },
    );
    let velocity = use_scroll_velocity();

    let progress_value = *progress;
    let trigger_progress = *trigger.progress();
    let active = *trigger.active();
    let target_style = format!(
        "{TARGET} opacity:{:.3}; transform:translateY({:.1}px) scale({:.3}); border-color:{};",
        0.42 + progress_value * 0.58,
        (1.0 - progress_value) * 42.0,
        0.94 + progress_value * 0.06,
        if active { "#16a34a" } else { "rgba(15,23,42,.12)" }
    );
    let fill_style = format!("{PROGRESS_FILL} width:{:.2}%;", progress_value * 100.0);

    html! {
        <main style={PAGE_STYLE}>
            <section style={INTRO}>
                <p style={EYEBROW}>{"Scroll hooks"}</p>
                <h1 style={TITLE}>{"Scroll-linked section"}</h1>
                <div style={STATUS_GRID}>
                    <Stat label="Progress" value={format!("{:.0}%", progress_value * 100.0)} />
                    <Stat label="Trigger" value={if active { "Active".to_owned() } else { "Idle".to_owned() }} />
                    <Stat label="Velocity" value={format!("{:.0}px/s", *velocity)} />
                    <Stat label="Viewport" value={format!("{:.0}%", trigger_progress * 100.0)} />
                </div>
            </section>

            <div style={SPACER}>
                <span style={SCROLL_MARKER}>{"Scroll"}</span>
            </div>

            <div ref={target} style={target_style}>
                <div style={TARGET_HEADER}>
                    <span style={BADGE}>{if active { "Triggered" } else { "Watching" }}</span>
                    <strong>{format!("{:.0}%", progress_value * 100.0)}</strong>
                </div>
                <h2 style={CARD_TITLE}>{"Viewport progress"}</h2>
                <div style={PROGRESS_TRACK}>
                    <div style={fill_style}></div>
                </div>
            </div>

            <div style={TAIL_SPACE}></div>
        </main>
    }
}

#[derive(Properties, PartialEq)]
struct StatProps {
    label: &'static str,
    value: String,
}

#[function_component(Stat)]
fn stat(props: &StatProps) -> Html {
    html! {
        <div style={STAT_CARD}>
            <span style={STAT_LABEL}>{props.label}</span>
            <strong style={STAT_VALUE}>{props.value.clone()}</strong>
        </div>
    }
}

const PAGE_STYLE: &str = "min-height:230vh; padding:36px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(180deg,#f8fafc 0%,#e0f2fe 40%,#f0fdf4 100%); color:#0f172a;";
const INTRO: &str = "position:sticky; top:24px; z-index:1; width:min(760px, calc(100vw - 32px)); margin:0 auto; padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:rgba(255,255,255,.88); box-shadow:0 20px 60px rgba(15,23,42,.12); backdrop-filter:blur(12px);";
const EYEBROW: &str = "margin:0 0 4px; color:#0369a1; font-size:12px; font-weight:800; letter-spacing:.08em; text-transform:uppercase;";
const TITLE: &str = "margin:0 0 18px; font-size:30px; line-height:1.05; letter-spacing:0;";
const STATUS_GRID: &str = "display:grid; grid-template-columns:repeat(auto-fit,minmax(130px,1fr)); gap:10px;";
const STAT_CARD: &str = "padding:13px 14px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:white;";
const STAT_LABEL: &str = "display:block; color:#64748b; font-size:12px; font-weight:750;";
const STAT_VALUE: &str = "display:block; margin-top:5px; color:#0f172a; font-size:20px; line-height:1;";
const SPACER: &str = "height:76vh; display:grid; place-items:center;";
const SCROLL_MARKER: &str = "display:inline-grid; place-items:center; height:36px; padding:0 14px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:white; color:#475569; font-size:13px; font-weight:800;";
const TARGET: &str = "width:min(760px, calc(100vw - 32px)); margin:0 auto; min-height:330px; padding:24px; border:2px solid; border-radius:8px; background:linear-gradient(135deg,#0f172a 0%,#1e3a8a 52%,#065f46 100%); color:white; box-shadow:0 28px 80px rgba(15,23,42,.24); will-change:transform,opacity;";
const TARGET_HEADER: &str = "display:flex; align-items:center; justify-content:space-between; gap:12px; margin-bottom:48px;";
const BADGE: &str = "display:inline-grid; place-items:center; height:30px; padding:0 10px; border-radius:8px; background:rgba(255,255,255,.16); color:white; font-size:12px; font-weight:850;";
const CARD_TITLE: &str = "margin:0 0 20px; font-size:32px; letter-spacing:0;";
const PROGRESS_TRACK: &str = "height:14px; border-radius:999px; background:rgba(255,255,255,.18); overflow:hidden;";
const PROGRESS_FILL: &str = "height:100%; border-radius:999px; background:linear-gradient(90deg,#38bdf8,#86efac);";
const TAIL_SPACE: &str = "height:120vh;";
