use animato::{DragConfig, DragConstraints, use_drag, use_pinch, use_swipe};
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let target = use_node_ref();
    let (position, drag) = use_drag(
        target.clone(),
        DragConfig {
            constraints: Some(DragConstraints::bounded(-120.0, 120.0, -80.0, 80.0)),
            snap_points: vec![[0.0, 0.0], [100.0, 0.0], [-100.0, 0.0]],
            ..DragConfig::default()
        },
    );
    let (scale, pinch) = use_pinch(target.clone());
    let swipe = use_swipe(target.clone(), Default::default());
    let [x, y] = *position;
    let puck_style = format!(
        "{PUCK} transform:translate({x:.1}px,{y:.1}px) scale({:.2});",
        *scale
    );
    let simulate_drag = {
        let drag = drag.clone();
        Callback::from(move |_| {
            drag.pointer_down(0.0, 0.0, 1);
            drag.pointer_move(86.0, 34.0, 1, 0.08);
            drag.pointer_up(86.0, 34.0, 1);
        })
    };
    let snap_home = {
        let drag = drag.clone();
        Callback::from(move |_| drag.snap_to([0.0, 0.0]))
    };
    let zoom_in = {
        let pinch = pinch.clone();
        Callback::from(move |_| pinch.set_scale(1.18))
    };
    let zoom_reset = Callback::from(move |_| pinch.reset());
    let swipe_label = (*swipe)
        .map(|event| format!("{:?} {:.0}px/s", event.direction, event.velocity))
        .unwrap_or_else(|| "No swipe".to_owned());

    html! {
        <main style={PAGE_STYLE}>
            <section style={APP_SHELL}>
                <header style={HEADER}>
                    <div>
                        <p style={EYEBROW}>{"use_drag"}</p>
                        <h1 style={TITLE}>{"Gesture handle"}</h1>
                    </div>
                    <div style={PROGRESS_BADGE}>{format!("{x:.0}, {y:.0}")}</div>
                </header>

                <div style={STAGE}>
                    <div ref={target} style={SURFACE}>
                        <div style={puck_style}>
                            <span>{"Drag"}</span>
                        </div>
                    </div>
                </div>

                <div style={OPTION_GRID}>
                    <div style={DETAIL_PANEL}>
                        <span>{"Position"}</span>
                        <strong>{format!("{x:.1}px / {y:.1}px")}</strong>
                        <span>{"Scale"}</span>
                        <strong>{format!("{:.2}x", *scale)}</strong>
                        <span>{"Swipe"}</span>
                        <strong>{swipe_label}</strong>
                    </div>
                    <div style={OPTION_PANEL}>
                        <span style={OPTION_LABEL}>{"Controls"}</span>
                        <div style={BUTTON_ROW}>
                            <button style={PRIMARY_BUTTON} onclick={simulate_drag}>{"Simulate"}</button>
                            <button style={BUTTON} onclick={snap_home}>{"Center"}</button>
                            <button style={BUTTON} onclick={zoom_in}>{"Zoom"}</button>
                            <button style={GHOST_BUTTON} onclick={zoom_reset}>{"Reset"}</button>
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
const PROGRESS_BADGE: &str = "min-width:84px; text-align:center; padding:8px 10px; border-radius:8px; background:#0f172a; color:white; font-size:13px; font-weight:800;";
const STAGE: &str = "padding:22px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:linear-gradient(180deg,#ffffff 0%,#f8fafc 100%);";
const SURFACE: &str = "height:260px; display:grid; place-items:center; border-radius:8px; background:repeating-linear-gradient(90deg,#e2e8f0 0,#e2e8f0 1px,transparent 1px,transparent 40px), repeating-linear-gradient(0deg,#e2e8f0 0,#e2e8f0 1px,transparent 1px,transparent 40px), linear-gradient(180deg,#f8fafc,#eef2ff); overflow:hidden; touch-action:none;";
const PUCK: &str = "width:96px; height:96px; display:grid; place-items:center; border-radius:8px; background:linear-gradient(135deg,#16a34a,#0ea5e9); box-shadow:0 20px 38px rgba(14,165,233,.28); color:white; font-size:13px; font-weight:900; user-select:none; will-change:transform;";
const OPTION_GRID: &str = "display:grid; grid-template-columns:repeat(auto-fit,minmax(220px,1fr)); gap:10px; margin-top:14px;";
const OPTION_PANEL: &str = "display:grid; gap:8px; padding:12px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:white;";
const OPTION_LABEL: &str = "color:#475569; font-size:12px; font-weight:850;";
const BUTTON_ROW: &str = "display:flex; flex-wrap:wrap; gap:7px;";
const DETAIL_PANEL: &str = "display:grid; grid-template-columns:1fr auto; gap:8px 12px; align-content:center; padding:12px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:#f8fafc; color:#475569; font-size:12px; font-weight:750;";
const BUTTON: &str = "height:36px; padding:0 13px; border:1px solid rgba(15,23,42,.16); border-radius:8px; background:white; color:#0f172a; font-weight:750; cursor:pointer;";
const PRIMARY_BUTTON: &str = "height:36px; padding:0 14px; border:1px solid #0f172a; border-radius:8px; background:#0f172a; color:white; font-weight:850; cursor:pointer;";
const GHOST_BUTTON: &str = "height:36px; padding:0 13px; border:1px solid transparent; border-radius:8px; background:transparent; color:#475569; font-weight:750; cursor:pointer;";
