//! Recorder controls built on `animato-driver`.

use animato_driver::{AnimationRecorder, RecordedTrack, RecorderError};

/// Start/stop/export controls for animation recording.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RecorderControls {
    recorder: AnimationRecorder,
}

impl RecorderControls {
    /// Create empty recorder controls.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start recording.
    pub fn start(&mut self) {
        self.recorder.start();
    }

    /// Stop recording.
    pub fn stop(&mut self) {
        self.recorder.stop();
    }

    /// Whether the recorder is active.
    pub fn is_recording(&self) -> bool {
        self.recorder.is_recording()
    }

    /// Clear recorded tracks.
    pub fn clear(&mut self) {
        self.recorder.clear();
    }

    /// Record one scalar sample.
    pub fn record(&mut self, label: &str, time: f32, value: f64) {
        self.recorder.record(label, time, value);
    }

    /// Export deterministic JSON.
    pub fn export_json(&self) -> String {
        self.recorder.export_json()
    }

    /// Export deterministic binary data.
    pub fn export_binary(&self) -> Vec<u8> {
        self.recorder.export_binary()
    }

    /// Replace the current recording from JSON.
    pub fn import_json(&mut self, json: &str) -> Result<(), RecorderError> {
        self.recorder = AnimationRecorder::import_json(json)?;
        Ok(())
    }

    /// Replace the current recording from binary data.
    pub fn import_binary(&mut self, bytes: &[u8]) -> Result<(), RecorderError> {
        self.recorder = AnimationRecorder::import_binary(bytes)?;
        Ok(())
    }

    /// Replay a label at seconds.
    pub fn replay(&self, label: &str, time: f32) -> Option<f64> {
        self.recorder.replay(label, time)
    }

    /// Recorded tracks.
    pub fn tracks(&self) -> &[RecordedTrack] {
        self.recorder.tracks()
    }

    /// Access the underlying recorder.
    pub fn recorder(&self) -> &AnimationRecorder {
        &self.recorder
    }

    /// Mutable access to the underlying recorder.
    pub fn recorder_mut(&mut self) -> &mut AnimationRecorder {
        &mut self.recorder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_json_and_replays() {
        let mut controls = RecorderControls::new();
        controls.start();
        controls.record("x", 0.0, 0.0);
        controls.record("x", 1.0, 10.0);
        controls.stop();

        let json = controls.export_json();
        let mut imported = RecorderControls::new();
        imported.import_json(&json).unwrap();
        assert_eq!(imported.replay("x", 0.5), Some(5.0));

        let binary = imported.export_binary();
        let mut from_binary = RecorderControls::new();
        from_binary.import_binary(&binary).unwrap();
        assert_eq!(from_binary.replay("x", 0.5), Some(5.0));
    }
}
