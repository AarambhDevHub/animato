use animato::{AnimatedFor, Easing, PresenceAnimation};
use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Item {
    id: u32,
    label: &'static str,
    detail: &'static str,
    color: &'static str,
}

#[component]
fn App() -> impl IntoView {
    let initial_items = vec![
        Item {
            id: 1,
            label: "Alpha",
            detail: "Layout pass",
            color: "#2563eb",
        },
        Item {
            id: 2,
            label: "Bravo",
            detail: "Measure",
            color: "#16a34a",
        },
        Item {
            id: 3,
            label: "Charlie",
            detail: "Invert",
            color: "#dc2626",
        },
        Item {
            id: 4,
            label: "Delta",
            detail: "Play",
            color: "#7c3aed",
        },
    ];
    let (items, set_items) = signal(initial_items.clone());
    let reset_items = initial_items.clone();
    let (next_id, set_next_id) = signal(5_u32);

    view! {
        <main style=PAGE_STYLE>
            <section style=APP_SHELL>
                <div style=HEADER>
                    <div>
                        <p style=EYEBROW>"AnimatedFor"</p>
                        <h1 style=TITLE>"FLIP list"</h1>
                    </div>
                    <div style=COUNT>{move || items.get().len()} " rows"</div>
                </div>

                <div style=TOOLBAR>
                    <button style=PRIMARY_BUTTON on:click=move |_| set_items.update(|items| items.reverse())>
                        "Reverse"
                    </button>
                    <button style=BUTTON on:click=move |_| {
                        set_items.update(|items| {
                            if !items.is_empty() {
                                let first = items.remove(0);
                                items.push(first);
                            }
                        });
                    }>
                        "Rotate"
                    </button>
                    <button style=BUTTON on:click=move |_| {
                        let id = next_id.get();
                        set_next_id.set(id + 1);
                        set_items.update(|items| {
                            items.insert(0, Item {
                                id,
                                label: "Echo",
                                detail: "Inserted",
                                color: "#0891b2",
                            });
                        });
                    }>
                        "Add"
                    </button>
                    <button style=BUTTON on:click=move |_| set_items.update(|items| {
                        if items.len() > 1 {
                            items.pop();
                        }
                    })>
                        "Remove"
                    </button>
                    <button style=GHOST_BUTTON on:click=move |_| set_items.set(reset_items.clone())>
                        "Reset"
                    </button>
                </div>

                <AnimatedFor
                    each=items.into()
                    key=|item: &Item| item.id
                    children=|item: Item| {
                        let id = item.id;
                        let label = item.label;
                        let detail = item.detail;
                        let color = item.color;
                        view! {
                        <div style=move || format!("{ROW} border-left-color:{color};")>
                            <div style=ROW_MARKER>
                                {id}
                            </div>
                            <div>
                                <strong style=ROW_TITLE>{label}</strong>
                                <span style=ROW_DETAIL>{detail}</span>
                            </div>
                        </div>
                        }
                    }
                    enter=PresenceAnimation::slide_up()
                    move_duration=0.42
                    move_easing=Easing::EaseOutCubic
                    stagger_delay=0.04
                />
            </section>
        </main>
    }
}

const PAGE_STYLE: &str = "min-height:100vh; display:grid; place-items:center; padding:32px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(135deg,#f8fafc 0%,#e0f2fe 52%,#fef3c7 100%); color:#0f172a;";
const APP_SHELL: &str = "width:min(620px, calc(100vw - 32px)); padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:rgba(255,255,255,.84); box-shadow:0 24px 60px rgba(15,23,42,.14); backdrop-filter:blur(12px);";
const HEADER: &str = "display:flex; align-items:center; justify-content:space-between; gap:16px; margin-bottom:18px;";
const EYEBROW: &str = "margin:0 0 4px; font-size:12px; font-weight:700; letter-spacing:.08em; text-transform:uppercase; color:#0369a1;";
const TITLE: &str = "margin:0; font-size:28px; line-height:1.05; letter-spacing:0; color:#0f172a;";
const COUNT: &str = "min-width:72px; text-align:center; padding:8px 10px; border-radius:8px; background:#0f172a; color:white; font-size:13px; font-weight:700;";
const TOOLBAR: &str = "display:flex; flex-wrap:wrap; gap:8px; margin-bottom:18px;";
const BUTTON: &str = "height:36px; padding:0 13px; border:1px solid rgba(15,23,42,.16); border-radius:8px; background:white; color:#0f172a; font-weight:700; cursor:pointer;";
const PRIMARY_BUTTON: &str = "height:36px; padding:0 14px; border:1px solid #0f172a; border-radius:8px; background:#0f172a; color:white; font-weight:800; cursor:pointer;";
const GHOST_BUTTON: &str = "height:36px; padding:0 13px; border:1px solid transparent; border-radius:8px; background:transparent; color:#475569; font-weight:700; cursor:pointer;";
const ROW: &str = "display:grid; grid-template-columns:44px 1fr; align-items:center; gap:14px; min-height:70px; padding:12px 14px; border:1px solid rgba(15,23,42,.12); border-left:5px solid; border-radius:8px; background:white; box-shadow:0 10px 26px rgba(15,23,42,.08);";
const ROW_MARKER: &str = "width:38px; height:38px; display:grid; place-items:center; border-radius:8px; background:#f1f5f9; color:#0f172a; font-size:14px; font-weight:800;";
const ROW_TITLE: &str = "display:block; font-size:16px; line-height:1.2;";
const ROW_DETAIL: &str = "display:block; margin-top:3px; color:#64748b; font-size:13px; font-weight:650;";
