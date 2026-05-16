use animato::{AnimatedStyle, Easing, css_tween, use_tween};
use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (x, handle) = use_tween(0.0_f32, 260.0, |builder| {
        builder.duration(0.9).easing(Easing::EaseOutCubic)
    });
    let reset = handle.clone();
    let play = handle.clone();
    let reverse = handle.clone();
    let style = css_tween(
        AnimatedStyle::new().opacity(0.2).scale(0.9),
        AnimatedStyle::new().opacity(1.0).scale(1.0),
        0.6,
        Easing::EaseOutCubic,
    );

    view! {
        <main style="min-height:100vh; display:grid; place-items:center; font-family:system-ui, sans-serif;">
            <section style="width:min(520px, calc(100vw - 32px));">
                <div
                    style=move || format!(
                        "{} width:96px; height:96px; border-radius:8px; background:#22c55e; transform:translateX({:.1}px);",
                        style.get(),
                        x.get()
                    )
                />
                <div style="display:flex; gap:8px; margin-top:20px;">
                    <button on:click=move |_| reset.reset()>"Reset"</button>
                    <button on:click=move |_| play.play()>"Play"</button>
                    <button on:click=move |_| reverse.reverse()>"Reverse"</button>
                </div>
            </section>
        </main>
    }
}
