use animato::{
    AnimatedStyle, Easing, PresenceAnimation, ScrollConfig, ScrollTriggerConfig, use_css_tween,
    use_scroll_progress, use_scroll_trigger,
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
            smooth: true,
            smooth_factor: 0.18,
            ..ScrollConfig::default()
        },
    );
    let trigger = use_scroll_trigger(
        target.clone(),
        ScrollTriggerConfig {
            threshold: 0.35,
            once: false,
            scrub: true,
            ..ScrollTriggerConfig::default()
        },
    );
    let accent = use_css_tween(
        AnimatedStyle::new().opacity(0.7).scale(0.98),
        AnimatedStyle::new().opacity(1.0).scale(1.0),
        0.6,
        Easing::EaseOutCubic,
    );
    let progress_value = *progress;
    let trigger_progress = *trigger.progress();
    let active = *trigger.active();
    let meter_style = format!("{METER} width:{:.1}%;", progress_value * 100.0);
    let panel_style = if active {
        format!("{CARD} {}", PresenceAnimation::slide_up().to.to_css())
    } else {
        format!("{CARD} {}", PresenceAnimation::slide_up().from.to_css())
    };

    html! {
        <main style={PAGE_STYLE}>
            <section style={APP_SHELL}>
                <header style={HEADER}>
                    <div>
                        <p style={EYEBROW}>{"use_scroll_trigger"}</p>
                        <h1 style={TITLE}>{"Scroll progress"}</h1>
                    </div>
                    <div style={PROGRESS_BADGE}>{format!("{:.0}%", progress_value * 100.0)}</div>
                </header>
                <div style={INTRO}>
                    {"Scroll this panel until the measured card crosses the viewport threshold."}
                </div>
                <div style={SCROLLER}>
                    <div style={SPACER}>{"Start"}</div>
                    <article ref={target} style={panel_style}>
                        <span style={OPTION_LABEL}>{"Trigger ratio"}</span>
                        <strong style={VALUE}>{format!("{:.0}%", trigger_progress * 100.0)}</strong>
                        <div style={TRACK}><div style={meter_style}></div></div>
                        <div style={format!("{CHIP} {}", &*accent)}>{if active { "Active" } else { "Waiting" }}</div>
                    </article>
                    <div style={SPACER}>{"Finish"}</div>
                </div>
            </section>
        </main>
    }
}

const PAGE_STYLE: &str = "min-height:100vh; display:grid; place-items:center; padding:32px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(135deg,#f8fafc 0%,#dcfce7 48%,#dbeafe 100%); color:#0f172a;";
const APP_SHELL: &str = "width:min(720px, calc(100vw - 32px)); padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:rgba(255,255,255,.86); box-shadow:0 24px 70px rgba(15,23,42,.16);";
const HEADER: &str = "display:flex; align-items:center; justify-content:space-between; gap:16px; margin-bottom:18px;";
const EYEBROW: &str = "margin:0 0 4px; font-size:12px; font-weight:800; letter-spacing:.08em; text-transform:uppercase; color:#15803d;";
const TITLE: &str = "margin:0; font-size:30px; line-height:1.05; letter-spacing:0;";
const PROGRESS_BADGE: &str = "min-width:72px; text-align:center; padding:8px 10px; border-radius:8px; background:#0f172a; color:white; font-size:13px; font-weight:800;";
const INTRO: &str = "padding:12px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:#f8fafc; color:#475569; font-size:13px; font-weight:650; margin-bottom:14px;";
const SCROLLER: &str = "height:440px; overflow:auto; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:linear-gradient(180deg,#ffffff,#eef2ff); padding:18px;";
const SPACER: &str = "height:320px; display:grid; place-items:center; color:#64748b; font-size:12px; font-weight:850; text-transform:uppercase;";
const CARD: &str = "display:grid; gap:12px; padding:18px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:white; box-shadow:0 18px 48px rgba(15,23,42,.14); will-change:transform,opacity;";
const OPTION_LABEL: &str = "color:#475569; font-size:12px; font-weight:850;";
const VALUE: &str = "font-size:34px; line-height:1; letter-spacing:0;";
const TRACK: &str = "height:12px; overflow:hidden; border-radius:8px; background:#e2e8f0;";
const METER: &str = "height:100%; border-radius:8px; background:linear-gradient(90deg,#16a34a,#0ea5e9);";
const CHIP: &str = "justify-self:start; padding:7px 10px; border-radius:8px; background:#0f172a; color:white; font-size:12px; font-weight:850;";
