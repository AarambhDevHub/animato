use animato::{DragConfig, use_drag, use_pinch};
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
    let pinch_zoom = pinch.clone();
    let pinch_reset = pinch.clone();

    view! {
        <main style="min-height:100vh; display:grid; place-items:center; font-family:system-ui, sans-serif;">
            <section>
                <div
                    node_ref=node
                    style=move || format!(
                        "width:112px; height:112px; border-radius:8px; background:#f97316; transform:translate({:.1}px,{:.1}px) scale({:.2});",
                        position.get()[0],
                        position.get()[1],
                        scale.get()
                    )
                />
                <div style="display:flex; gap:8px; margin-top:20px;">
                    <button on:click=move |_| {
                        drag.pointer_down(0.0, 0.0, 1);
                        drag.pointer_move(80.0, 24.0, 1, 0.016);
                        drag.pointer_up(80.0, 24.0, 1);
                    }>"Simulate drag"</button>
                    <button on:click=move |_| pinch_zoom.set_scale(1.25)>"Pinch"</button>
                    <button on:click=move |_| pinch_reset.reset()>"Reset scale"</button>
                </div>
            </section>
        </main>
    }
}
