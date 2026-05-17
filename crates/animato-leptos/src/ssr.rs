//! SSR and hydration guards for animation hooks.

use animato_core::Animatable;
use leptos::prelude::*;

/// Returns `true` when hooks should avoid browser-only animation work.
///
/// This is `true` while rendering on the server and also on non-wasm test
/// targets, where there is no browser rAF loop to drive animations.
pub fn is_hydrating() -> bool {
    is_server() || !is_browser()
}

/// Create a signal that is safe to read during SSR.
///
/// On the server and during non-browser tests this returns the provided static
/// value. Browser callers can use it as a hydration-safe seed before starting
/// an animated hook.
pub fn use_client_only<T>(server_value: T) -> ReadSignal<T>
where
    T: Animatable + Send + Sync + 'static,
{
    let (value, _set_value) = signal(server_value);
    value
}

/// Render a static fallback during SSR and animated children in the browser.
#[component]
pub fn SsrFallback(
    /// Static fallback view.
    fallback: AnyView,
    /// Browser-side animated view.
    children: Children,
) -> AnyView {
    if is_hydrating() { fallback } else { children() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::{Get, Owner};

    #[test]
    fn client_only_returns_seed_value() {
        Owner::new().with(|| {
            let value = use_client_only(42_i32);
            assert_eq!(value.get(), 42);
        });
    }
}
