//! Error helpers for JavaScript-facing APIs.

use wasm_bindgen::JsValue;

pub(crate) fn js_error(message: impl AsRef<str>) -> JsValue {
    #[cfg(target_arch = "wasm32")]
    {
        JsValue::from_str(message.as_ref())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = message;
        JsValue::NULL
    }
}

pub(crate) fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

pub(crate) fn non_negative(value: f32, fallback: f32) -> f32 {
    finite_or(value, fallback).max(0.0)
}

pub(crate) fn require_index(index: u32, len: usize, what: &str) -> Result<usize, JsValue> {
    let index = index as usize;
    if index < len {
        Ok(index)
    } else {
        Err(js_error(format!(
            "{what} index {index} is out of bounds for length {len}"
        )))
    }
}

pub(crate) type JsResult<T> = Result<T, JsValue>;
