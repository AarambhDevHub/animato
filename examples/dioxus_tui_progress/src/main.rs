use animato::{Easing, use_tween};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let (progress, handle) = use_tween(0.0_f32, 1.0, |builder| {
        builder.duration(1.2).easing(Easing::EaseInOutSine)
    });
    let value = format!("{:.3}", *progress.read());

    rsx! {
        main {
            style: "font-family:monospace; padding:24px; display:grid; gap:12px;",
            h1 { style: "font-size:20px; margin:0;", "Dioxus Progress" }
            progress {
                max: "1",
                value: "{value}",
                style: "width:320px;"
            }
            button { onclick: move |_| handle.reset(), "Restart" }
        }
    }
}
