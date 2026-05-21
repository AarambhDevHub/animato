use animato::{AnimatedFor, Easing, PresenceAnimation};
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Clone, Debug, PartialEq)]
struct Row {
    id: usize,
    label: &'static str,
    tone: &'static str,
}

const ROWS: [Row; 5] = [
    Row {
        id: 1,
        label: "Capture",
        tone: "#16a34a",
    },
    Row {
        id: 2,
        label: "Layout",
        tone: "#0ea5e9",
    },
    Row {
        id: 3,
        label: "Measure",
        tone: "#f59e0b",
    },
    Row {
        id: 4,
        label: "Transform",
        tone: "#e11d48",
    },
    Row {
        id: 5,
        label: "Commit",
        tone: "#7c3aed",
    },
];

#[function_component(App)]
fn app() -> Html {
    let rows = use_state_eq(|| ROWS[..3].to_vec());
    let add_rows = {
        let rows = rows.clone();
        Callback::from(move |_| {
            let mut next = (*rows).clone();
            if let Some(row) = ROWS.iter().find(|row| !next.iter().any(|item| item.id == row.id)) {
                next.push(row.clone());
            }
            rows.set(next);
        })
    };
    let rotate_rows = {
        let rows = rows.clone();
        Callback::from(move |_| {
            let mut next = (*rows).clone();
            if !next.is_empty() {
                next.rotate_left(1);
            }
            rows.set(next);
        })
    };
    let remove_rows = {
        let rows = rows.clone();
        Callback::from(move |_| {
            let mut next = (*rows).clone();
            next.pop();
            rows.set(next);
        })
    };
    let reset_rows = {
        let rows = rows.clone();
        Callback::from(move |_| rows.set(ROWS[..3].to_vec()))
    };

    html! {
        <main style={PAGE_STYLE}>
            <section style={APP_SHELL}>
                <header style={HEADER}>
                    <div>
                        <p style={EYEBROW}>{"AnimatedFor"}</p>
                        <h1 style={TITLE}>{"FLIP list"}</h1>
                    </div>
                    <div style={PROGRESS_BADGE}>{format!("{} rows", rows.len())}</div>
                </header>

                <AnimatedFor<Row>
                    items={(*rows).clone()}
                    key_fn={Callback::from(|row: Row| row.id.to_string())}
                    render={Callback::from(|row: Row| {
                        html! {
                            <div style={ROW}>
                                <span style={format!("{DOT} background:{};", row.tone)}></span>
                                <strong>{row.label}</strong>
                                <span style={ROW_META}>{format!("#{}", row.id)}</span>
                            </div>
                        }
                    })}
                    enter={Some(PresenceAnimation::slide_up())}
                    move_duration={0.28}
                    move_easing={Easing::EaseOutCubic}
                    stagger_delay={0.035}
                />

                <div style={TOOLBAR}>
                    <button style={PRIMARY_BUTTON} onclick={add_rows}>{"Add"}</button>
                    <button style={BUTTON} onclick={rotate_rows}>{"Rotate"}</button>
                    <button style={BUTTON} onclick={remove_rows}>{"Remove"}</button>
                    <button style={GHOST_BUTTON} onclick={reset_rows}>{"Reset"}</button>
                </div>
            </section>
        </main>
    }
}

const PAGE_STYLE: &str = "min-height:100vh; display:grid; place-items:center; padding:32px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(135deg,#f8fafc 0%,#dcfce7 48%,#dbeafe 100%); color:#0f172a;";
const APP_SHELL: &str = "width:min(640px, calc(100vw - 32px)); padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:rgba(255,255,255,.86); box-shadow:0 24px 70px rgba(15,23,42,.16);";
const HEADER: &str = "display:flex; align-items:center; justify-content:space-between; gap:16px; margin-bottom:20px;";
const EYEBROW: &str = "margin:0 0 4px; font-size:12px; font-weight:800; letter-spacing:.08em; text-transform:uppercase; color:#15803d;";
const TITLE: &str = "margin:0; font-size:30px; line-height:1.05; letter-spacing:0;";
const PROGRESS_BADGE: &str = "min-width:72px; text-align:center; padding:8px 10px; border-radius:8px; background:#0f172a; color:white; font-size:13px; font-weight:800;";
const ROW: &str = "min-height:54px; display:grid; grid-template-columns:auto 1fr auto; align-items:center; gap:12px; padding:12px 14px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:white; color:#0f172a; box-shadow:0 10px 24px rgba(15,23,42,.08);";
const DOT: &str = "width:12px; height:12px; border-radius:99px;";
const ROW_META: &str = "color:#64748b; font-size:12px; font-weight:800;";
const TOOLBAR: &str = "display:flex; flex-wrap:wrap; gap:8px; margin-top:18px;";
const BUTTON: &str = "height:36px; padding:0 13px; border:1px solid rgba(15,23,42,.16); border-radius:8px; background:white; color:#0f172a; font-weight:750; cursor:pointer;";
const PRIMARY_BUTTON: &str = "height:36px; padding:0 14px; border:1px solid #0f172a; border-radius:8px; background:#0f172a; color:white; font-weight:850; cursor:pointer;";
const GHOST_BUTTON: &str = "height:36px; padding:0 13px; border:1px solid transparent; border-radius:8px; background:transparent; color:#475569; font-weight:750; cursor:pointer;";
