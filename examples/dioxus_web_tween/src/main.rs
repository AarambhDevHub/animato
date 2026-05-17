use animato::{Easing, use_tween};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let (x, handle) = use_tween(0.0_f32, 240.0, |builder| {
        builder.duration(0.8).easing(Easing::EaseOutCubic)
    });
    let box_style = format!(
        "width:56px; height:56px; position:absolute; left:24px; top:20px; background:#1f7a5c; transform:translateX({:.1}px);",
        *x.read()
    );

    rsx! {
        main {
            style: "min-height:100vh; display:grid; place-items:center; font-family:system-ui, sans-serif; background:#f7f7f5;",
            section {
                style: "width:min(520px, calc(100vw - 32px)); display:grid; gap:18px;",
                h1 { style: "font-size:28px; margin:0;", "Animato Dioxus Tween" }
                div {
                    style: "height:96px; border:1px solid #d0d0ca; background:white; overflow:hidden; position:relative;",
                    div {
                        style: "{box_style}"
                    }
                }
                button {
                    style: "width:max-content; padding:10px 14px; border:1px solid #1f2937; background:#1f2937; color:white;",
                    onclick: move |_| handle.reverse(),
                    "Reverse"
                }
            }
        }
    }
}
