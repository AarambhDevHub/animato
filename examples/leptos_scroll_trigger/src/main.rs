use animato::{ScrollConfig, ScrollTriggerConfig, use_scroll_progress, use_scroll_trigger};
use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let target = NodeRef::<leptos::html::Div>::new();
    let progress = use_scroll_progress(target, ScrollConfig::default());
    let trigger = use_scroll_trigger(
        target,
        ScrollTriggerConfig {
            threshold: 0.4,
            once: true,
            ..ScrollTriggerConfig::default()
        },
    );

    view! {
        <main style="min-height:220vh; font-family:system-ui, sans-serif; padding:48px;">
            <div style="height:80vh;" />
            <div
                node_ref=target
                style=move || format!(
                    "padding:40px; border-radius:8px; background:#0ea5e9; color:white; opacity:{:.3}; transform:translateY({:.1}px);",
                    progress.get().max(if trigger.active().get() { 1.0 } else { 0.35 }),
                    (1.0 - progress.get()) * 24.0
                )
            >
                <h1>"Scroll-linked section"</h1>
                <p>"Progress: " {move || format!("{:.2}", progress.get())}</p>
            </div>
        </main>
    }
}
