use animato::{PageTransition, TransitionMode};
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::components::Router;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <PageTransition mode=TransitionMode::CrossFade>
                <main style="min-height:100vh; display:grid; place-items:center; font-family:system-ui, sans-serif;">
                    <section style="width:min(560px, calc(100vw - 32px)); padding:32px; border:1px solid #ddd; border-radius:8px;">
                        <h1>"Route transition shell"</h1>
                        <p>"Wrap router outlets or route content with PageTransition."</p>
                    </section>
                </main>
            </PageTransition>
        </Router>
    }
}
