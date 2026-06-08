//! Animation recorder JavaScript bindings.

use crate::error::js_error;
use animato_driver::AnimationRecorder as CoreAnimationRecorder;
use js_sys::Uint8Array;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

/// Scalar animation recorder for DevTools-style capture and replay.
#[wasm_bindgen(js_name = AnimationRecorder)]
#[derive(Clone, Debug)]
pub struct AnimationRecorder {
    inner: Arc<Mutex<CoreAnimationRecorder>>,
}

#[wasm_bindgen(js_class = AnimationRecorder)]
impl AnimationRecorder {
    /// Create an empty inactive recorder.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CoreAnimationRecorder::new())),
        }
    }

    /// Import a recorder from JSON.
    #[wasm_bindgen(js_name = importJson)]
    pub fn import_json(json: &str) -> Result<Self, JsValue> {
        Ok(Self {
            inner: Arc::new(Mutex::new(
                CoreAnimationRecorder::import_json(json)
                    .map_err(|error| js_error(error.to_string()))?,
            )),
        })
    }

    /// Import a recorder from binary bytes.
    #[wasm_bindgen(js_name = importBinary)]
    pub fn import_binary(bytes: &Uint8Array) -> Result<Self, JsValue> {
        let mut data = vec![0_u8; bytes.length() as usize];
        bytes.copy_to(&mut data);
        Ok(Self {
            inner: Arc::new(Mutex::new(
                CoreAnimationRecorder::import_binary(&data)
                    .map_err(|error| js_error(error.to_string()))?,
            )),
        })
    }

    /// Start recording.
    pub fn start(&self) {
        lock(&self.inner).start();
    }

    /// Stop recording.
    pub fn stop(&self) {
        lock(&self.inner).stop();
    }

    /// Whether recording is active.
    #[wasm_bindgen(js_name = isRecording)]
    pub fn is_recording(&self) -> bool {
        lock(&self.inner).is_recording()
    }

    /// Clear all tracks.
    pub fn clear(&self) {
        lock(&self.inner).clear();
    }

    /// Record one scalar sample.
    pub fn record(&self, label: &str, time: f32, value: f64) {
        lock(&self.inner).record(label, time, value);
    }

    /// Export deterministic JSON.
    #[wasm_bindgen(js_name = exportJson)]
    pub fn export_json(&self) -> String {
        lock(&self.inner).export_json()
    }

    /// Export deterministic binary data.
    #[wasm_bindgen(js_name = exportBinary)]
    pub fn export_binary(&self) -> Uint8Array {
        Uint8Array::from(lock(&self.inner).export_binary().as_slice())
    }

    /// Replay a label at seconds. Returns `NaN` when the label has no data.
    pub fn replay(&self, label: &str, time: f32) -> f64 {
        lock(&self.inner).replay(label, time).unwrap_or(f64::NAN)
    }
}

impl Default for AnimationRecorder {
    fn default() -> Self {
        Self::new()
    }
}

fn lock<T>(shared: &Arc<Mutex<T>>) -> std::sync::MutexGuard<'_, T> {
    shared.lock().expect("animato-js recorder lock poisoned")
}
