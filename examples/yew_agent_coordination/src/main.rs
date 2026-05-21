use animato::{
    AgentInput, AgentOutput, AgentSpringSpec, AgentTweenSpec, Easing, use_animation_agent,
};
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let value = use_state_eq(|| 0.0_f32);
    let status = use_state_eq(|| "Idle".to_owned());
    let callback = {
        let value = value.clone();
        let status = status.clone();
        Callback::from(move |output: AgentOutput| match output {
            AgentOutput::Started { id, value: next } => {
                value.set(next);
                status.set(format!("{id} started"));
            }
            AgentOutput::Tick { id, value: next, .. } => {
                value.set(next);
                status.set(format!("{id} ticking"));
            }
            AgentOutput::Completed { id, value: next } => {
                value.set(next);
                status.set(format!("{id} complete"));
            }
            AgentOutput::Stopped { id } => status.set(format!("{id} stopped")),
            AgentOutput::Reset => {
                value.set(0.0);
                status.set("Reset".to_owned());
            }
        })
    };
    let agent = use_animation_agent(callback);
    let x = *value;
    let marker_style = format!("{MARKER} transform:translateX({x:.1}px);");
    let start_tween = {
        let agent = agent.clone();
        Callback::from(move |_| {
            agent.send(AgentInput::Tween(
                AgentTweenSpec::new("shared-x", 0.0, 320.0)
                    .duration(0.9)
                    .easing(Easing::EaseOutCubic),
            ));
        })
    };
    let start_spring = {
        let agent = agent.clone();
        Callback::from(move |_| {
            agent.send(AgentInput::Spring(AgentSpringSpec::new(
                "shared-x", x, 180.0,
            )));
        })
    };
    let reset = {
        let agent = agent.clone();
        Callback::from(move |_| agent.send(AgentInput::Reset))
    };

    html! {
        <main style={PAGE_STYLE}>
            <section style={APP_SHELL}>
                <header style={HEADER}>
                    <div>
                        <p style={EYEBROW}>{"use_animation_agent"}</p>
                        <h1 style={TITLE}>{"Agent coordination"}</h1>
                    </div>
                    <div style={PROGRESS_BADGE}>{format!("{x:.0}px")}</div>
                </header>

                <div style={STAGE}>
                    <div style={TRACK}>
                        <div style={marker_style}></div>
                    </div>
                    <div style={RULER}>
                        <span>{"0"}</span>
                        <span>{"320 px"}</span>
                    </div>
                </div>

                <div style={OPTION_GRID}>
                    <div style={DETAIL_PANEL}>
                        <span>{"Channel"}</span>
                        <strong>{"shared-x"}</strong>
                        <span>{"Status"}</span>
                        <strong>{(*status).clone()}</strong>
                    </div>
                    <div style={OPTION_PANEL}>
                        <span style={OPTION_LABEL}>{"Messages"}</span>
                        <div style={BUTTON_ROW}>
                            <button style={PRIMARY_BUTTON} onclick={start_tween}>{"Tween"}</button>
                            <button style={BUTTON} onclick={start_spring}>{"Spring"}</button>
                            <button style={GHOST_BUTTON} onclick={reset}>{"Reset"}</button>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}

const PAGE_STYLE: &str = "min-height:100vh; display:grid; place-items:center; padding:32px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(135deg,#f8fafc 0%,#dcfce7 48%,#dbeafe 100%); color:#0f172a;";
const APP_SHELL: &str = "width:min(720px, calc(100vw - 32px)); padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:rgba(255,255,255,.86); box-shadow:0 24px 70px rgba(15,23,42,.16);";
const HEADER: &str = "display:flex; align-items:center; justify-content:space-between; gap:16px; margin-bottom:24px;";
const EYEBROW: &str = "margin:0 0 4px; font-size:12px; font-weight:800; letter-spacing:.08em; text-transform:uppercase; color:#15803d;";
const TITLE: &str = "margin:0; font-size:30px; line-height:1.05; letter-spacing:0;";
const PROGRESS_BADGE: &str = "min-width:72px; text-align:center; padding:8px 10px; border-radius:8px; background:#0f172a; color:white; font-size:13px; font-weight:800;";
const STAGE: &str = "padding:22px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:linear-gradient(180deg,#ffffff 0%,#f8fafc 100%);";
const TRACK: &str = "position:relative; height:132px; border-radius:8px; background:repeating-linear-gradient(90deg,#e2e8f0 0,#e2e8f0 1px,transparent 1px,transparent 40px), linear-gradient(180deg,#f8fafc,#eef2ff); overflow:hidden;";
const MARKER: &str = "position:absolute; left:18px; top:26px; width:80px; height:80px; border-radius:8px; background:linear-gradient(135deg,#16a34a,#0ea5e9); box-shadow:0 20px 38px rgba(14,165,233,.28); will-change:transform;";
const RULER: &str = "display:flex; justify-content:space-between; margin-top:10px; color:#64748b; font-size:12px; font-weight:750;";
const OPTION_GRID: &str = "display:grid; grid-template-columns:repeat(auto-fit,minmax(220px,1fr)); gap:10px; margin-top:14px;";
const OPTION_PANEL: &str = "display:grid; gap:8px; padding:12px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:white;";
const OPTION_LABEL: &str = "color:#475569; font-size:12px; font-weight:850;";
const BUTTON_ROW: &str = "display:flex; flex-wrap:wrap; gap:7px;";
const DETAIL_PANEL: &str = "display:grid; grid-template-columns:1fr auto; gap:8px 12px; align-content:center; padding:12px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:#f8fafc; color:#475569; font-size:12px; font-weight:750;";
const BUTTON: &str = "height:36px; padding:0 13px; border:1px solid rgba(15,23,42,.16); border-radius:8px; background:white; color:#0f172a; font-weight:750; cursor:pointer;";
const PRIMARY_BUTTON: &str = "height:36px; padding:0 14px; border:1px solid #0f172a; border-radius:8px; background:#0f172a; color:white; font-weight:850; cursor:pointer;";
const GHOST_BUTTON: &str = "height:36px; padding:0 13px; border:1px solid transparent; border-radius:8px; background:transparent; color:#475569; font-weight:750; cursor:pointer;";
