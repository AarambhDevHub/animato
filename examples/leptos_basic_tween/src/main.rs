use animato::{AnimatedStyle, Easing, css_tween, use_tween};
use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (x, handle) = use_tween(0.0_f32, 320.0, |builder| {
        builder.duration(0.9).easing(Easing::EaseOutCubic)
    });
    let (speed, set_speed) = signal(1.0_f32);
    let reset = handle.clone();
    let play = handle.clone();
    let reverse = handle.clone();
    let pause = handle.clone();
    let resume = handle.clone();
    let seek_start = handle.clone();
    let seek_quarter = handle.clone();
    let seek_half = handle.clone();
    let seek_finish = handle.clone();
    let speed_slow = handle.clone();
    let speed_normal = handle.clone();
    let speed_fast = handle.clone();
    let progress_badge = handle.clone();
    let progress_ball = handle.clone();
    let style = css_tween(
        AnimatedStyle::new().opacity(0.35).scale(0.92),
        AnimatedStyle::new().opacity(1.0).scale(1.0),
        0.55,
        Easing::EaseOutCubic,
    );

    view! {
        <main style=PAGE_STYLE>
            <section style=APP_SHELL>
                <header style=HEADER>
                    <div>
                        <p style=EYEBROW>"use_tween"</p>
                        <h1 style=TITLE>"Signal tween"</h1>
                    </div>
                    <div style=PROGRESS_BADGE>
                        {move || format!("{:.0}%", progress_badge.progress().get() * 100.0)}
                    </div>
                </header>

                <div style=STAGE>
                    <div style=TRACK>
                        <div
                            style=move || format!(
                                "{BALL} {} transform:translateX({:.1}px) scale({:.3});",
                                style.get(),
                                x.get(),
                                0.92 + progress_ball.progress().get() * 0.08
                            )
                        />
                    </div>
                    <div style=RULER>
                        <span>"0"</span>
                        <span>"320 px"</span>
                    </div>
                </div>

                <div style=TOOLBAR>
                    <button style=PRIMARY_BUTTON on:click=move |_| play.play()>"Play"</button>
                    <button style=BUTTON on:click=move |_| pause.pause()>"Pause"</button>
                    <button style=BUTTON on:click=move |_| resume.resume()>"Resume"</button>
                    <button style=BUTTON on:click=move |_| reverse.reverse()>"Reverse"</button>
                    <button style=GHOST_BUTTON on:click=move |_| reset.reset()>"Reset"</button>
                </div>

                <div style=OPTION_GRID>
                    <div style=OPTION_PANEL>
                        <span style=OPTION_LABEL>"Seek"</span>
                        <div style=BUTTON_ROW>
                            <button style=BUTTON on:click=move |_| seek_start.seek(0.0)>"0%"</button>
                            <button style=BUTTON on:click=move |_| seek_quarter.seek(0.25)>"25%"</button>
                            <button style=BUTTON on:click=move |_| seek_half.seek(0.5)>"50%"</button>
                            <button style=BUTTON on:click=move |_| seek_finish.seek(1.0)>"100%"</button>
                        </div>
                    </div>

                    <div style=OPTION_PANEL>
                        <span style=OPTION_LABEL>{move || format!("Speed {:.2}x", speed.get())}</span>
                        <div style=BUTTON_ROW>
                            <button style=BUTTON on:click=move |_| {
                                speed_slow.set_time_scale(0.5);
                                set_speed.set(0.5);
                            }>"0.5x"</button>
                            <button style=BUTTON on:click=move |_| {
                                speed_normal.set_time_scale(1.0);
                                set_speed.set(1.0);
                            }>"1x"</button>
                            <button style=BUTTON on:click=move |_| {
                                speed_fast.set_time_scale(1.75);
                                set_speed.set(1.75);
                            }>"1.75x"</button>
                        </div>
                    </div>

                    <div style=DETAIL_PANEL>
                        <span>"Duration"</span>
                        <strong>"0.90s"</strong>
                        <span>"Easing"</span>
                        <strong>"EaseOutCubic"</strong>
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
const BALL: &str = "position:absolute; left:18px; top:26px; width:80px; height:80px; border-radius:8px; background:linear-gradient(135deg,#16a34a,#0ea5e9); box-shadow:0 20px 38px rgba(14,165,233,.28); will-change:transform,opacity;";
const RULER: &str = "display:flex; justify-content:space-between; margin-top:10px; color:#64748b; font-size:12px; font-weight:750;";
const TOOLBAR: &str = "display:flex; flex-wrap:wrap; gap:8px; margin-top:18px;";
const OPTION_GRID: &str = "display:grid; grid-template-columns:repeat(auto-fit,minmax(180px,1fr)); gap:10px; margin-top:14px;";
const OPTION_PANEL: &str = "display:grid; gap:8px; padding:12px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:white;";
const OPTION_LABEL: &str = "color:#475569; font-size:12px; font-weight:850;";
const BUTTON_ROW: &str = "display:flex; flex-wrap:wrap; gap:7px;";
const DETAIL_PANEL: &str = "display:grid; grid-template-columns:1fr auto; gap:8px 12px; align-content:center; padding:12px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:#f8fafc; color:#475569; font-size:12px; font-weight:750;";
const BUTTON: &str = "height:36px; padding:0 13px; border:1px solid rgba(15,23,42,.16); border-radius:8px; background:white; color:#0f172a; font-weight:750; cursor:pointer;";
const PRIMARY_BUTTON: &str = "height:36px; padding:0 14px; border:1px solid #0f172a; border-radius:8px; background:#0f172a; color:white; font-weight:850; cursor:pointer;";
const GHOST_BUTTON: &str = "height:36px; padding:0 13px; border:1px solid transparent; border-radius:8px; background:transparent; color:#475569; font-weight:750; cursor:pointer;";
