use animato::{PageTransition, PresenceAnimation, TransitionMode, use_route_transition_key};
use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/details")]
    Details,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Shell />
        </BrowserRouter>
    }
}

#[function_component(Shell)]
fn shell() -> Html {
    let route_key = use_route_transition_key();

    html! {
        <main style={PAGE_STYLE}>
            <section style={APP_SHELL}>
                <header style={HEADER}>
                    <div>
                        <p style={EYEBROW}>{"PageTransition"}</p>
                        <h1 style={TITLE}>{"Router motion"}</h1>
                    </div>
                    <nav style={TOOLBAR}>
                        <Link<Route> to={Route::Home} classes="nav-link">{"Home"}</Link<Route>>
                        <Link<Route> to={Route::Details} classes="nav-link">{"Details"}</Link<Route>>
                    </nav>
                </header>

                <style>
                    {".nav-link{height:36px;display:inline-flex;align-items:center;padding:0 13px;border:1px solid rgba(15,23,42,.16);border-radius:8px;background:white;color:#0f172a;font-weight:750;text-decoration:none}.nav-link:hover{border-color:#0f172a}"}
                </style>

                <PageTransition
                    mode={TransitionMode::SlideOver}
                    route_key={Some(route_key)}
                    enter={Some(PresenceAnimation::slide_right())}
                >
                    <Switch<Route> render={switch} />
                </PageTransition>
            </section>
        </main>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <article style={CARD}>
                <span style={OPTION_LABEL}>{"Home route"}</span>
                <strong style={VALUE}>{"Dashboard"}</strong>
                <p style={COPY}>{"A dense Yew route surface using the same page transition wrapper as application screens."}</p>
            </article>
        },
        Route::Details => html! {
            <article style={CARD}>
                <span style={OPTION_LABEL}>{"Details route"}</span>
                <strong style={VALUE}>{"Inspection"}</strong>
                <p style={COPY}>{"The route key changes under Yew Router and the wrapper keeps transform and opacity styles predictable."}</p>
            </article>
        },
        Route::NotFound => html! {
            <article style={CARD}>
                <span style={OPTION_LABEL}>{"Missing route"}</span>
                <strong style={VALUE}>{"404"}</strong>
            </article>
        },
    }
}

const PAGE_STYLE: &str = "min-height:100vh; display:grid; place-items:center; padding:32px; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background:linear-gradient(135deg,#f8fafc 0%,#dcfce7 48%,#dbeafe 100%); color:#0f172a;";
const APP_SHELL: &str = "width:min(720px, calc(100vw - 32px)); padding:24px; border:1px solid rgba(15,23,42,.12); border-radius:8px; background:rgba(255,255,255,.86); box-shadow:0 24px 70px rgba(15,23,42,.16);";
const HEADER: &str = "display:flex; align-items:center; justify-content:space-between; flex-wrap:wrap; gap:16px; margin-bottom:24px;";
const EYEBROW: &str = "margin:0 0 4px; font-size:12px; font-weight:800; letter-spacing:.08em; text-transform:uppercase; color:#15803d;";
const TITLE: &str = "margin:0; font-size:30px; line-height:1.05; letter-spacing:0;";
const TOOLBAR: &str = "display:flex; flex-wrap:wrap; gap:8px;";
const CARD: &str = "display:grid; gap:12px; min-height:260px; padding:22px; border:1px solid rgba(15,23,42,.10); border-radius:8px; background:linear-gradient(180deg,#ffffff,#f8fafc); box-shadow:0 18px 48px rgba(15,23,42,.14);";
const OPTION_LABEL: &str = "color:#475569; font-size:12px; font-weight:850;";
const VALUE: &str = "font-size:42px; line-height:1; letter-spacing:0;";
const COPY: &str = "max-width:54ch; margin:0; color:#475569; line-height:1.55; font-size:14px; font-weight:650;";
