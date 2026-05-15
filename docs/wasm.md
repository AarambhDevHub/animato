# WASM And Browser Helpers

Features: `wasm`, optional `wasm-dom`.

```toml
[dependencies]
animato = { version = "1.0", features = ["wasm"] }
```

## requestAnimationFrame Driver

```rust,ignore
use animato::{Easing, RafDriver, Tween, Update};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct App {
    tween: Tween<f32>,
    driver: RafDriver,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tween: Tween::new(0.0_f32, 500.0)
                .duration(1.5)
                .easing(Easing::EaseOutBounce)
                .build(),
            driver: RafDriver::new(),
        }
    }

    pub fn tick(&mut self, timestamp_ms: f64) {
        let dt = self.driver.tick(timestamp_ms);
        self.tween.update(dt);
    }
}
```

## DOM Helpers

Enable `wasm-dom` for:

- `FlipAnimation`
- `LayoutAnimator`
- `SharedElementTransition`
- `SplitText`
- `Draggable`
- `Observer`

## Build Example

```sh
cd examples/wasm_counter
wasm-pack build --target web
```

## Related Docs

- [Physics](./physics.md)
- [Driver](./driver.md)
- [Troubleshooting](./troubleshooting.md)
