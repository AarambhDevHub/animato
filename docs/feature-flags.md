# Feature Flags

The facade crate is intentionally feature-gated so users can avoid unnecessary
dependencies.

```toml
[dependencies]
animato = { version = "1.2", features = ["path", "physics"] }
```

## Facade Features

| Feature | Enables |
|---------|---------|
| `default` | `std`, `tween`, `timeline`, `spring`, `driver` |
| `std` | Hosted functionality and std forwarding |
| `tween` | `animato-tween` and allocation-backed keyframes |
| `timeline` | `animato-timeline` |
| `spring` | `animato-spring` and `SpringN<T>` allocation support |
| `path` | `animato-path` with `std` paths, SVG parser, morphing |
| `physics` | `animato-physics` with allocation support |
| `color` | `animato-color` and `palette` re-export |
| `driver` | `animato-driver` |
| `gpu` | `animato-gpu`, `tween`, `std` |
| `bevy` | `animato-bevy`, `tween`, `spring`, `std` |
| `wasm` | `animato-wasm`, `driver` |
| `wasm-dom` | `wasm` plus DOM helpers |
| `leptos` | `animato-leptos` hooks/components without forcing an app mode |
| `leptos-csr` | `leptos` plus Leptos CSR mode |
| `leptos-hydrate` | `leptos` plus Leptos hydration mode |
| `leptos-ssr` | `leptos` plus Leptos SSR mode |
| `dioxus` | `animato-dioxus` hooks/components without forcing a renderer |
| `dioxus-web` | `dioxus` plus Dioxus web renderer support |
| `dioxus-desktop` | `dioxus` plus Dioxus desktop renderer support |
| `dioxus-router` | `dioxus` plus route transition helpers |
| `dioxus-native` | `dioxus` plus portable native window animation handles |
| `serde` | Serde derives and re-exports on supported types |
| `tokio` | `Timeline::wait()` |

## no_std

Prefer focused crates for no_std:

```toml
animato-core = { version = "1.2", default-features = false }
animato-tween = { version = "1.2", default-features = false }
```

See [no-std.md](./no-std.md).

## Common Mistakes

- `MotionPathTween` requires `path`.
- `Inertia` and `DragState` require `physics`.
- `GpuAnimationBatch` requires `gpu`.
- `RafDriver` requires `wasm`.
- DOM helpers require `wasm-dom` and `wasm32`.
- Bevy transform helpers require `bevy`.
- Leptos apps should choose exactly one app mode feature: `leptos-csr`,
  `leptos-hydrate`, or `leptos-ssr`.
- Dioxus apps should enable a renderer feature in the app crate, such as
  `dioxus-web` with `dioxus/web` or `dioxus-desktop` with `dioxus/desktop`.
- When both `leptos` and `dioxus` are enabled, use `animato::leptos::*` or
  `animato::dioxus::*` to avoid intentionally duplicated hook/component names.

## Related Docs

- [Installation](./installation.md)
- [Troubleshooting](./troubleshooting.md)
