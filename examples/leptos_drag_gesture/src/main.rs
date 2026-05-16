use animato::{DragConfig, GestureConfig, SwipeConfig, use_drag, use_gesture, use_pinch, use_swipe};
use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let node = NodeRef::<leptos::html::Div>::new();
    let (position, drag) = use_drag(node, DragConfig::default());
    let (scale, pinch) = use_pinch(node);
    let gesture = use_gesture(node, GestureConfig::default());
    let swipe = use_swipe(node, SwipeConfig::default());
    let pinch_zoom = pinch.clone();
    let pinch_reset = pinch.clone();
    let drag_reset = drag.clone();

    view! {
        <main style=PAGE_STYLE>
            <section style=APP_SHELL>
                <header style=HEADER>
                    <div>
                        <p style=EYEBROW>"Pointer hooks"</p>
                        <h1 style=TITLE>"Drag and gesture"</h1>
                    </div>
                    <div style=POSITION_BADGE>
                        {move || format!("{:.0}, {:.0}", position.get()[0], position.get()[1])}
                    </div>
                </header>

                <div style=WORKSPACE>
                    <div style=GRID_FLOOR>
                        <div
                            node_ref=node
                            style=move || format!(
                                "{DRAG_TARGET} transform:translate({:.1}px,{:.1}px) scale({:.2});",
                                position.get()[0],
                                position.get()[1],
                                scale.get()
                            )
                        >
                            <span style=TARGET_LABEL>"Drag"</span>
                        </div>
                    </div>
                    <aside style=INSPECTOR>
                        <Info label="Scale" value=move || format!("{:.2}", scale.get()) />
                        <Info label="Gesture" value=move || gesture.get().map(|g| format!("{g:?}")).unwrap_or_else(|| "None".to_owned()) />
                        <Info label="Swipe" value=move || swipe.get().map(|s| format!("{:?}", s.direction)).unwrap_or_else(|| "None".to_owned()) />
                    </aside>
                </div>

                <div style=TOOLBAR>
                    <button style=PRIMARY_BUTTON on:click=move |_| {
                        drag.pointer_down(0.0, 0.0, 1);
                        drag.pointer_move(120.0, 32.0, 1, 0.016);
                        drag.pointer_up(120.0, 32.0, 1);
                    }>
                        "Simulate drag"
                    </button>
                    <button style=BUTTON on:click=move |_| pinch_zoom.set_scale(1.28)>"Pinch"</button>
                    <button style=BUTTON on:click=move |_| {
                        drag_reset.pointer_down(position.get()[0], position.get()[1], 1);
                        drag_reset.pointer_move(0.0, 0.0, 1, 0.016);
                        drag_reset.pointer_up(0.0, 0.0, 1);
                    }>
                        "Center"
                    </button>
                    <button style=GHOST_BUTTON on:click=move |_| pinch_reset.reset()>"Reset scale"</button>
                </div>
            </section>
        </main>
    }
}

#[component]
fn Info<F>(label: &'static str, value: F) -> impl IntoView
where
    F: Fn() -> String + Send + Sync + 'static,
{
    view! {
        <div style=INFO_ROW>
            <span>{label}</span>
            <strong>{value}</strong>
        </div>
    }
}

const PAGE_STYLE: &str = "min-height:100vh; display:grid; place-items:center; padding:32px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(135deg,#f8fafc 0%,#ffedd5 45%,#dbeafe 100%); color:#0f172a;";
const APP_SHELL: &str = "width:min(900px, calc(100vw - 32px)); padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:rgba(255,255,255,.86); box-shadow:0 24px 70px rgba(15,23,42,.16);";
const HEADER: &str = "display:flex; align-items:center; justify-content:space-between; gap:16px; margin-bottom:22px;";
const EYEBROW: &str = "margin:0 0 4px; color:#c2410c; font-size:12px; font-weight:800; letter-spacing:.08em; text-transform:uppercase;";
const TITLE: &str = "margin:0; font-size:30px; line-height:1.05; letter-spacing:0;";
const POSITION_BADGE: &str = "min-width:96px; text-align:center; padding:8px 10px; border-radius:8px; background:#0f172a; color:white; font-size:13px; font-weight:800;";
const WORKSPACE: &str = "display:grid; grid-template-columns:minmax(0,1fr) 220px; gap:16px;";
const GRID_FLOOR: &str = "position:relative; min-height:360px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:repeating-linear-gradient(0deg,#e2e8f0 0,#e2e8f0 1px,transparent 1px,transparent 32px), repeating-linear-gradient(90deg,#e2e8f0 0,#e2e8f0 1px,transparent 1px,transparent 32px), #f8fafc; overflow:hidden;";
const DRAG_TARGET: &str = "position:absolute; left:calc(50% - 56px); top:calc(50% - 56px); width:112px; height:112px; display:grid; place-items:center; border-radius:8px; background:linear-gradient(135deg,#f97316,#0ea5e9); color:white; box-shadow:0 22px 44px rgba(249,115,22,.30); touch-action:none; user-select:none; cursor:grab; will-change:transform;";
const TARGET_LABEL: &str = "font-size:15px; font-weight:850;";
const INSPECTOR: &str = "display:grid; align-content:start; gap:10px;";
const INFO_ROW: &str = "display:grid; gap:5px; padding:14px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:white; color:#64748b; font-size:12px; font-weight:750; overflow:hidden;";
const TOOLBAR: &str = "display:flex; flex-wrap:wrap; gap:8px; margin-top:18px;";
const BUTTON: &str = "height:36px; padding:0 13px; border:1px solid rgba(15,23,42,.16); border-radius:8px; background:white; color:#0f172a; font-weight:750; cursor:pointer;";
const PRIMARY_BUTTON: &str = "height:36px; padding:0 14px; border:1px solid #0f172a; border-radius:8px; background:#0f172a; color:white; font-weight:850; cursor:pointer;";
const GHOST_BUTTON: &str = "height:36px; padding:0 13px; border:1px solid transparent; border-radius:8px; background:transparent; color:#475569; font-weight:750; cursor:pointer;";
