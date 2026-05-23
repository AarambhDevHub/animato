//! Browser DOM helper bindings.

use animato_wasm::ScrollSmoother as CoreScrollSmoother;
use wasm_bindgen::prelude::*;

/// Momentum-style scroll smoothing helper.
#[wasm_bindgen(js_name = ScrollSmoother)]
#[derive(Clone, Debug)]
pub struct ScrollSmoother {
    inner: CoreScrollSmoother,
}

#[wasm_bindgen(js_class = ScrollSmoother)]
impl ScrollSmoother {
    /// Create a scroll smoother.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: CoreScrollSmoother::new(),
        }
    }

    /// Snap instantly to a value.
    #[wasm_bindgen(js_name = snapTo)]
    pub fn snap_to(&mut self, value: f32) {
        self.inner.snap_to(value);
    }

    /// Set target scroll position.
    #[wasm_bindgen(js_name = scrollTo)]
    pub fn scroll_to(&mut self, value: f32) {
        self.inner.scroll_to(value);
    }

    /// Feed a wheel delta.
    #[wasm_bindgen(js_name = onWheel)]
    pub fn on_wheel(&mut self, delta_y: f32) {
        self.inner.on_wheel(delta_y);
    }

    /// Advance by `dt` seconds.
    pub fn update(&mut self, dt: f32) -> bool {
        self.inner.update(dt)
    }

    /// Current smoothed position.
    pub fn current(&self) -> f32 {
        self.inner.current()
    }

    /// Target scroll position.
    pub fn target(&self) -> f32 {
        self.inner.target()
    }

    /// Whether smoothed scroll has settled.
    #[wasm_bindgen(js_name = isSettled)]
    pub fn is_settled(&self) -> bool {
        self.inner.is_settled()
    }
}

impl Default for ScrollSmoother {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::*;
    use crate::easing::parse_easing;
    use crate::error::non_negative;
    use animato_wasm::{
        Draggable as CoreDraggable, FlipAnimation as CoreFlipAnimation, FlipState,
        LayoutAnimator as CoreLayoutAnimator, Observer as CoreObserver, ObserverEvent, SplitMode,
        SplitText as CoreSplitText,
    };
    use web_sys::{Element, PointerEvent, WheelEvent};

    /// FLIP layout transition between two DOM element boxes.
    #[wasm_bindgen(js_name = FlipAnimation)]
    #[derive(Clone, Debug)]
    pub struct FlipAnimation {
        inner: CoreFlipAnimation,
    }

    #[wasm_bindgen(js_class = FlipAnimation)]
    impl FlipAnimation {
        /// Capture a first and last element state.
        #[wasm_bindgen(constructor)]
        pub fn new(first: &Element, last: &Element) -> Self {
            Self {
                inner: CoreFlipAnimation::new(FlipState::capture(first), FlipState::capture(last)),
            }
        }

        /// Set duration.
        pub fn duration(&mut self, seconds: f32) {
            self.inner = self.inner.clone().duration(non_negative(seconds, 0.3));
        }

        /// Set easing.
        #[wasm_bindgen(js_name = setEasing)]
        pub fn set_easing(&mut self, easing: &str) -> Result<(), JsValue> {
            self.inner = self.inner.clone().easing(parse_easing(easing)?);
            Ok(())
        }

        /// Advance by `dt`.
        pub fn update(&mut self, dt: f32) -> bool {
            self.inner.update(dt)
        }

        /// Reset animation.
        pub fn reset(&mut self) {
            self.inner.reset();
        }

        /// Current progress.
        pub fn progress(&self) -> f32 {
            self.inner.progress()
        }

        /// Current CSS transform string.
        #[wasm_bindgen(js_name = cssTransform)]
        pub fn css_transform(&self) -> String {
            self.inner.css_transform()
        }

        /// Apply current transform to an element.
        #[wasm_bindgen(js_name = applyTo)]
        pub fn apply_to(&self, element: &Element) -> Result<(), JsValue> {
            self.inner.apply_to(element)
        }
    }

    /// Multi-element FLIP layout animator.
    #[wasm_bindgen(js_name = LayoutAnimator)]
    #[derive(Debug, Default)]
    pub struct LayoutAnimator {
        inner: CoreLayoutAnimator,
    }

    #[wasm_bindgen(js_class = LayoutAnimator)]
    impl LayoutAnimator {
        /// Create a layout animator.
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        /// Snapshot an element under a key.
        pub fn snapshot(&mut self, key: &str, element: &Element) {
            self.inner.snapshot(key, element);
        }

        /// Compute one transition from the stored snapshot to the current element.
        #[wasm_bindgen(js_name = computeTransition)]
        pub fn compute_transition(
            &mut self,
            key: &str,
            element: &Element,
            duration: f32,
            easing: &str,
        ) -> Result<(), JsValue> {
            self.inner
                .compute_transitions(&[(key, element)], duration, parse_easing(easing)?);
            Ok(())
        }

        /// Advance all transitions.
        pub fn update(&mut self, dt: f32) {
            self.inner.update(dt);
        }

        /// Apply one keyed transition to an element.
        pub fn apply(&self, key: &str, element: &Element) -> Result<(), JsValue> {
            self.inner.apply(&[(key, element)])
        }

        /// Transform string for a key, or an empty string.
        #[wasm_bindgen(js_name = cssTransform)]
        pub fn css_transform(&self, key: &str) -> String {
            self.inner.css_transform(key).unwrap_or_default()
        }

        /// Whether all transitions are complete.
        #[wasm_bindgen(js_name = isComplete)]
        pub fn is_complete(&self) -> bool {
            self.inner.is_complete()
        }

        /// Active animation count.
        #[wasm_bindgen(js_name = animationCount)]
        pub fn animation_count(&self) -> usize {
            self.inner.animation_count()
        }

        /// Clear completed transitions.
        #[wasm_bindgen(js_name = clearCompleted)]
        pub fn clear_completed(&mut self) {
            self.inner.clear_completed();
        }
    }

    /// Text splitting helper.
    #[wasm_bindgen(js_name = SplitText)]
    #[derive(Debug)]
    pub struct SplitText {
        inner: CoreSplitText,
    }

    #[wasm_bindgen(js_class = SplitText)]
    impl SplitText {
        /// Split element text by characters or words.
        #[wasm_bindgen(constructor)]
        pub fn new(element: &Element, mode: &str) -> Result<Self, JsValue> {
            let mode = match crate::types::normalize_name(mode).as_str() {
                "chars" | "characters" => SplitMode::Chars,
                "words" => SplitMode::Words,
                _ => return Err(JsValue::from_str("split mode must be `chars` or `words`")),
            };
            Ok(Self {
                inner: CoreSplitText::split(element, mode)?,
            })
        }

        /// Number of generated spans.
        pub fn len(&self) -> usize {
            self.inner.spans().len()
        }

        /// Whether no spans were generated.
        #[wasm_bindgen(js_name = isEmpty)]
        pub fn is_empty(&self) -> bool {
            self.inner.spans().is_empty()
        }

        /// Restore original text.
        pub fn restore(&self) {
            self.inner.restore();
        }
    }

    /// Pointer drag DOM helper.
    #[wasm_bindgen(js_name = Draggable)]
    #[derive(Debug)]
    pub struct Draggable {
        inner: CoreDraggable,
    }

    #[wasm_bindgen(js_class = Draggable)]
    impl Draggable {
        /// Attach to an element.
        #[wasm_bindgen(constructor)]
        pub fn new(element: Element, x: f32, y: f32) -> Result<Self, JsValue> {
            Ok(Self {
                inner: CoreDraggable::attach(element, [x, y])?,
            })
        }

        /// Current position.
        #[wasm_bindgen(js_name = toArray)]
        pub fn to_array(&self) -> js_sys::Float32Array {
            crate::types::f32_array(&self.inner.position())
        }

        /// Whether the element is being dragged.
        #[wasm_bindgen(js_name = isDragging)]
        pub fn is_dragging(&self) -> bool {
            self.inner.is_dragging()
        }
    }

    /// Low-level DOM event observer helpers.
    #[wasm_bindgen(js_name = Observer)]
    #[derive(Clone, Debug, Default)]
    pub struct Observer;

    #[wasm_bindgen(js_class = Observer)]
    impl Observer {
        /// Create an observer helper.
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self
        }

        /// Convert a pointer-down event to a plain JS object.
        #[wasm_bindgen(js_name = pointerDown)]
        pub fn pointer_down(event: &PointerEvent) -> Result<JsValue, JsValue> {
            observer_event_to_value(CoreObserver::pointer_down(event))
        }

        /// Convert a pointer-move event to a plain JS object.
        #[wasm_bindgen(js_name = pointerMove)]
        pub fn pointer_move(event: &PointerEvent) -> Result<JsValue, JsValue> {
            observer_event_to_value(CoreObserver::pointer_move(event))
        }

        /// Convert a pointer-up event to a plain JS object.
        #[wasm_bindgen(js_name = pointerUp)]
        pub fn pointer_up(event: &PointerEvent) -> Result<JsValue, JsValue> {
            observer_event_to_value(CoreObserver::pointer_up(event))
        }

        /// Convert a wheel event to a plain JS object.
        pub fn wheel(event: &WheelEvent) -> Result<JsValue, JsValue> {
            observer_event_to_value(CoreObserver::wheel(event))
        }
    }

    fn observer_event_to_value(event: ObserverEvent) -> Result<JsValue, JsValue> {
        Ok(match event {
            ObserverEvent::PointerDown { x, y, pointer_id } => object(&[
                ("type", JsValue::from_str("pointerDown")),
                ("x", JsValue::from_f64(x as f64)),
                ("y", JsValue::from_f64(y as f64)),
                ("pointerId", JsValue::from_f64(pointer_id as f64)),
            ]),
            ObserverEvent::PointerMove { x, y, pointer_id } => object(&[
                ("type", JsValue::from_str("pointerMove")),
                ("x", JsValue::from_f64(x as f64)),
                ("y", JsValue::from_f64(y as f64)),
                ("pointerId", JsValue::from_f64(pointer_id as f64)),
            ]),
            ObserverEvent::PointerUp { x, y, pointer_id } => object(&[
                ("type", JsValue::from_str("pointerUp")),
                ("x", JsValue::from_f64(x as f64)),
                ("y", JsValue::from_f64(y as f64)),
                ("pointerId", JsValue::from_f64(pointer_id as f64)),
            ]),
            ObserverEvent::Wheel { delta_x, delta_y } => object(&[
                ("type", JsValue::from_str("wheel")),
                ("deltaX", JsValue::from_f64(delta_x as f64)),
                ("deltaY", JsValue::from_f64(delta_y as f64)),
            ]),
        })
    }

    fn object(entries: &[(&str, JsValue)]) -> JsValue {
        let object = js_sys::Object::new();
        for (key, value) in entries {
            let _ = js_sys::Reflect::set(&object, &JsValue::from_str(key), value);
        }
        object.into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    use super::*;

    macro_rules! stub_class {
        ($name:ident) => {
            #[wasm_bindgen(js_name = $name)]
            /// Non-WASM placeholder for browser-only DOM helpers.
            #[derive(Clone, Debug, Default)]
            pub struct $name;
        };
    }

    stub_class!(FlipAnimation);
    stub_class!(LayoutAnimator);
    stub_class!(SplitText);
    stub_class!(Draggable);
    stub_class!(Observer);
}

pub use dom::{Draggable, FlipAnimation, LayoutAnimator, Observer, SplitText};
