use animato::AnimatedFor;
use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Item {
    id: u32,
    label: &'static str,
}

#[component]
fn App() -> impl IntoView {
    let (items, set_items) = signal(vec![
        Item { id: 1, label: "Alpha" },
        Item { id: 2, label: "Bravo" },
        Item { id: 3, label: "Charlie" },
    ]);

    view! {
        <main style="min-height:100vh; display:grid; place-items:center; font-family:system-ui, sans-serif;">
            <section style="width:min(420px, calc(100vw - 32px));">
                <button on:click=move |_| set_items.update(|items| items.reverse())>"Reverse"</button>
                <AnimatedFor
                    each=items.into()
                    key=|item: &Item| item.id
                    children=|item: Item| view! {
                        <div style="margin-top:8px; padding:14px 16px; border:1px solid #ddd; border-radius:8px;">
                            {item.label}
                        </div>
                    }
                    stagger_delay=0.04
                />
            </section>
        </main>
    }
}
