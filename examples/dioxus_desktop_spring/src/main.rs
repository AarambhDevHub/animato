use animato::{SpringConfig, use_spring, use_window_spring};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let (scale, spring) = use_spring(1.0_f32, SpringConfig::snappy());
    let window = use_window_spring(SpringConfig::stiff());
    let panel_style = format!(
        "height:140px; display:grid; place-items:center; background:white; border:1px solid #cbd5e1; transform:scale({:.3});",
        *scale.read()
    );

    rsx! {
        main {
            style: "min-height:100vh; display:grid; place-items:center; font-family:system-ui, sans-serif; background:#f4f7fb;",
            section {
                style: "display:grid; gap:16px; width:min(460px, calc(100vw - 32px));",
                h1 { style: "font-size:26px; margin:0;", "Desktop Spring" }
                div {
                    style: "{panel_style}",
                    "Spring value"
                }
                div {
                    style: "display:flex; gap:10px;",
                    button { onclick: move |_| spring.set_target(1.12), "Grow" }
                    button { onclick: move |_| spring.set_target(1.0), "Reset" }
                    button { onclick: move |_| window.move_to(80.0, 80.0), "Track Window Move" }
                }
            }
        }
    }
}
