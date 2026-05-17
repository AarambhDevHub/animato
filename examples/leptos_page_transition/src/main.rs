use animato::{PageTransition, TransitionMode};
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main style=PAGE_STYLE>
                <section style=APP_SHELL>
                    <header style=HEADER>
                        <div>
                            <p style=EYEBROW>"PageTransition"</p>
                            <h1 style=TITLE>"Route cards"</h1>
                        </div>
                        <nav style=NAV>
                            <A href="/" exact=true>
                                <span style=NAV_ITEM>"Overview"</span>
                            </A>
                            <A href="/reports">
                                <span style=NAV_ITEM>"Reports"</span>
                            </A>
                            <A href="/settings">
                                <span style=NAV_ITEM>"Settings"</span>
                            </A>
                        </nav>
                    </header>

                    <PageTransition mode=TransitionMode::SlideOver>
                        <Routes fallback=NotFound>
                            <Route path=path!("") view=Overview />
                            <Route path=path!("reports") view=Reports />
                            <Route path=path!("settings") view=Settings />
                        </Routes>
                    </PageTransition>
                </section>
            </main>
        </Router>
    }
}

#[component]
fn Overview() -> impl IntoView {
    view! {
        <article style=ROUTE_CARD>
            <div style=ROUTE_TOPLINE>
                <span style=BADGE_BLUE>"Live"</span>
                <span style=META>"24 active timelines"</span>
            </div>
            <h2 style=ROUTE_TITLE>"Motion dashboard"</h2>
            <div style=METRIC_GRID>
                <Metric label="Tweens" value="128" color="#2563eb" />
                <Metric label="Springs" value="42" color="#16a34a" />
                <Metric label="Routes" value="3" color="#7c3aed" />
            </div>
        </article>
    }
}

#[component]
fn Reports() -> impl IntoView {
    view! {
        <article style=ROUTE_CARD>
            <div style=ROUTE_TOPLINE>
                <span style=BADGE_GREEN>"Stable"</span>
                <span style=META>"Frame budget: 8.1 ms"</span>
            </div>
            <h2 style=ROUTE_TITLE>"Transition report"</h2>
            <div style=BAR_LIST>
                <Bar label="Opacity" value=82 color="#2563eb" />
                <Bar label="Transform" value=94 color="#16a34a" />
                <Bar label="Scroll" value=68 color="#dc2626" />
            </div>
        </article>
    }
}

#[component]
fn Settings() -> impl IntoView {
    view! {
        <article style=ROUTE_CARD>
            <div style=ROUTE_TOPLINE>
                <span style=BADGE_PURPLE>"Config"</span>
                <span style=META>"SlideOver mode"</span>
            </div>
            <h2 style=ROUTE_TITLE>"Animation profile"</h2>
            <div style=SETTING_LIST>
                <Setting label="Duration" value="260 ms" />
                <Setting label="Easing" value="EaseOutCubic" />
                <Setting label="Hydration guard" value="Enabled" />
            </div>
        </article>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <article style=ROUTE_CARD>
            <h2 style=ROUTE_TITLE>"Missing route"</h2>
            <p style=META>"Choose a route from the header."</p>
        </article>
    }
}

#[component]
fn Metric(label: &'static str, value: &'static str, color: &'static str) -> impl IntoView {
    view! {
        <div style=METRIC_CARD>
            <span style=move || format!("{METRIC_DOT} background:{color};")></span>
            <strong style=METRIC_VALUE>{value}</strong>
            <span style=METRIC_LABEL>{label}</span>
        </div>
    }
}

#[component]
fn Bar(label: &'static str, value: u32, color: &'static str) -> impl IntoView {
    view! {
        <div style=BAR_ROW>
            <div style=BAR_LABEL>
                <span>{label}</span>
                <strong>{value}"%"</strong>
            </div>
            <div style=BAR_TRACK>
                <div style=move || format!("{BAR_FILL} width:{value}%; background:{color};")></div>
            </div>
        </div>
    }
}

#[component]
fn Setting(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div style=SETTING_ROW>
            <span>{label}</span>
            <strong>{value}</strong>
        </div>
    }
}

const PAGE_STYLE: &str = "min-height:100vh; display:grid; place-items:center; padding:32px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(135deg,#f8fafc 0%,#dbeafe 46%,#dcfce7 100%); color:#0f172a;";
const APP_SHELL: &str = "width:min(860px, calc(100vw - 32px)); min-height:560px; padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:rgba(255,255,255,.86); box-shadow:0 24px 70px rgba(15,23,42,.16); overflow:hidden;";
const HEADER: &str = "display:flex; align-items:flex-start; justify-content:space-between; gap:20px; margin-bottom:22px;";
const EYEBROW: &str = "margin:0 0 4px; font-size:12px; font-weight:800; letter-spacing:.08em; text-transform:uppercase; color:#1d4ed8;";
const TITLE: &str = "margin:0; font-size:30px; line-height:1.05; letter-spacing:0;";
const NAV: &str = "display:flex; flex-wrap:wrap; gap:8px; justify-content:flex-end;";
const NAV_ITEM: &str = "display:inline-grid; place-items:center; height:36px; padding:0 13px; border:1px solid rgba(15,23,42,.14); border-radius:8px; background:white; color:#0f172a; font-size:14px; font-weight:800; text-decoration:none;";
const ROUTE_CARD: &str = "min-height:370px; padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:linear-gradient(180deg,#ffffff 0%,#f8fafc 100%); box-shadow:0 16px 44px rgba(15,23,42,.10);";
const ROUTE_TOPLINE: &str = "display:flex; align-items:center; justify-content:space-between; gap:12px; margin-bottom:24px;";
const BADGE_BLUE: &str = "display:inline-flex; align-items:center; height:28px; padding:0 10px; border-radius:8px; background:#dbeafe; color:#1d4ed8; font-size:12px; font-weight:800;";
const BADGE_GREEN: &str = "display:inline-flex; align-items:center; height:28px; padding:0 10px; border-radius:8px; background:#dcfce7; color:#15803d; font-size:12px; font-weight:800;";
const BADGE_PURPLE: &str = "display:inline-flex; align-items:center; height:28px; padding:0 10px; border-radius:8px; background:#ede9fe; color:#6d28d9; font-size:12px; font-weight:800;";
const META: &str = "margin:0; color:#64748b; font-size:14px; font-weight:650;";
const ROUTE_TITLE: &str = "margin:0 0 22px; font-size:26px; letter-spacing:0; color:#0f172a;";
const METRIC_GRID: &str = "display:grid; grid-template-columns:repeat(3,minmax(0,1fr)); gap:12px;";
const METRIC_CARD: &str = "min-height:132px; padding:16px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:white;";
const METRIC_DOT: &str = "display:block; width:10px; height:10px; border-radius:999px; margin-bottom:18px;";
const METRIC_VALUE: &str = "display:block; font-size:32px; line-height:1; color:#0f172a;";
const METRIC_LABEL: &str = "display:block; margin-top:8px; color:#64748b; font-size:13px; font-weight:700;";
const BAR_LIST: &str = "display:grid; gap:18px;";
const BAR_ROW: &str = "display:grid; gap:8px;";
const BAR_LABEL: &str = "display:flex; justify-content:space-between; color:#334155; font-size:14px; font-weight:750;";
const BAR_TRACK: &str = "height:12px; border-radius:999px; background:#e2e8f0; overflow:hidden;";
const BAR_FILL: &str = "height:100%; border-radius:999px;";
const SETTING_LIST: &str = "display:grid; gap:10px;";
const SETTING_ROW: &str = "display:flex; align-items:center; justify-content:space-between; gap:16px; padding:15px 16px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:white; color:#475569; font-size:14px;";
