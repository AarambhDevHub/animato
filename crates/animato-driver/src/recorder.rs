//! Frame-value recording and replay helpers.

use std::fmt;
use std::string::String;
use std::vec::Vec;

/// A recorded scalar sample.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RecordedSample {
    /// Absolute time in seconds.
    pub time: f32,
    /// Recorded scalar value.
    pub value: f64,
}

/// Samples for one recorded animation label.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordedTrack {
    /// Track label.
    pub label: String,
    /// Time-ordered samples.
    pub samples: Vec<RecordedSample>,
}

/// Captures scalar animation values for later replay or DevTools export.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AnimationRecorder {
    active: bool,
    tracks: Vec<RecordedTrack>,
}

impl AnimationRecorder {
    /// Create an inactive empty recorder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start accepting samples.
    pub fn start(&mut self) {
        self.active = true;
    }

    /// Stop accepting samples.
    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Return whether recording is active.
    pub fn is_recording(&self) -> bool {
        self.active
    }

    /// Remove all recorded tracks.
    pub fn clear(&mut self) {
        self.tracks.clear();
    }

    /// Recorded tracks.
    pub fn tracks(&self) -> &[RecordedTrack] {
        &self.tracks
    }

    /// Record one scalar sample if the recorder is active.
    pub fn record(&mut self, label: impl AsRef<str>, time: f32, value: f64) {
        if !self.active || !time.is_finite() || !value.is_finite() {
            return;
        }
        let label = label.as_ref();
        let sample = RecordedSample {
            time: time.max(0.0),
            value,
        };
        let track = match self.tracks.iter_mut().find(|track| track.label == label) {
            Some(track) => track,
            None => {
                self.tracks.push(RecordedTrack {
                    label: label.to_owned(),
                    samples: Vec::new(),
                });
                self.tracks.last_mut().expect("track just pushed")
            }
        };

        match track
            .samples
            .binary_search_by(|existing| existing.time.total_cmp(&sample.time))
        {
            Ok(index) => track.samples[index] = sample,
            Err(index) => track.samples.insert(index, sample),
        }
    }

    /// Export recorded data as deterministic JSON.
    pub fn export_json(&self) -> String {
        let mut out = String::from("{\"tracks\":[");
        for (track_index, track) in self.tracks.iter().enumerate() {
            if track_index > 0 {
                out.push(',');
            }
            out.push_str("{\"label\":\"");
            push_escaped(&mut out, &track.label);
            out.push_str("\",\"frames\":[");
            for (sample_index, sample) in track.samples.iter().enumerate() {
                if sample_index > 0 {
                    out.push(',');
                }
                out.push('[');
                push_float(&mut out, sample.time as f64);
                out.push(',');
                push_float(&mut out, sample.value);
                out.push(']');
            }
            out.push_str("]}");
        }
        out.push_str("]}");
        out
    }

    /// Import data previously produced by [`export_json`](Self::export_json).
    pub fn import_json(json: &str) -> Result<Self, RecorderError> {
        let mut cursor = JsonCursor::new(json);
        cursor.seek("\"tracks\"")?;
        cursor.seek("[")?;
        let mut recorder = Self::new();
        loop {
            cursor.skip_ws();
            if cursor.consume(']') {
                break;
            }
            cursor.expect('{')?;
            cursor.seek("\"label\"")?;
            cursor.seek(":")?;
            let label = cursor.string()?;
            cursor.seek("\"frames\"")?;
            cursor.seek("[")?;
            let mut samples = Vec::new();
            loop {
                cursor.skip_ws();
                if cursor.consume(']') {
                    break;
                }
                cursor.expect('[')?;
                let time = cursor.number()? as f32;
                cursor.expect(',')?;
                let value = cursor.number()?;
                cursor.expect(']')?;
                samples.push(RecordedSample { time, value });
                cursor.skip_ws();
                cursor.consume(',');
            }
            samples.sort_by(|a, b| a.time.total_cmp(&b.time));
            recorder.tracks.push(RecordedTrack { label, samples });
            cursor.skip_ws();
            cursor.expect('}')?;
            cursor.skip_ws();
            cursor.consume(',');
        }
        Ok(recorder)
    }

    /// Export recorded data as a compact deterministic binary format.
    pub fn export_binary(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"ANIMREC1");
        out.extend_from_slice(&(self.tracks.len() as u32).to_le_bytes());
        for track in &self.tracks {
            let label = track.label.as_bytes();
            out.extend_from_slice(&(label.len() as u32).to_le_bytes());
            out.extend_from_slice(label);
            out.extend_from_slice(&(track.samples.len() as u32).to_le_bytes());
            for sample in &track.samples {
                out.extend_from_slice(&sample.time.to_le_bytes());
                out.extend_from_slice(&sample.value.to_le_bytes());
            }
        }
        out
    }

    /// Import data previously produced by [`export_binary`](Self::export_binary).
    pub fn import_binary(bytes: &[u8]) -> Result<Self, RecorderError> {
        let mut reader = BinaryReader::new(bytes);
        reader.expect_magic(b"ANIMREC1")?;
        let track_count = reader.u32()? as usize;
        let mut tracks = Vec::with_capacity(track_count);
        for _ in 0..track_count {
            let label_len = reader.u32()? as usize;
            let label = String::from_utf8(reader.bytes(label_len)?.to_vec())
                .map_err(|_| RecorderError::InvalidUtf8)?;
            let sample_count = reader.u32()? as usize;
            let mut samples = Vec::with_capacity(sample_count);
            for _ in 0..sample_count {
                samples.push(RecordedSample {
                    time: reader.f32()?,
                    value: reader.f64()?,
                });
            }
            tracks.push(RecordedTrack { label, samples });
        }
        Ok(Self {
            active: false,
            tracks,
        })
    }

    /// Replay a recorded track at `time`, linearly interpolating between samples.
    pub fn replay(&self, label: &str, time: f32) -> Option<f64> {
        let track = self.tracks.iter().find(|track| track.label == label)?;
        match track.samples.as_slice() {
            [] => None,
            [only] => Some(only.value),
            samples => {
                let time = time.max(0.0);
                if time <= samples[0].time {
                    return Some(samples[0].value);
                }
                let last = samples.len() - 1;
                if time >= samples[last].time {
                    return Some(samples[last].value);
                }
                let upper = samples.partition_point(|sample| sample.time <= time);
                let a = samples[upper - 1];
                let b = samples[upper];
                let span = (b.time - a.time).max(f32::EPSILON) as f64;
                let t = ((time - a.time) as f64 / span).clamp(0.0, 1.0);
                Some(a.value + (b.value - a.value) * t)
            }
        }
    }
}

/// Error returned when recorder import fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecorderError {
    /// JSON input does not match the recorder export shape.
    InvalidJson,
    /// Binary input has invalid magic or is truncated.
    InvalidBinary,
    /// A binary label was not valid UTF-8.
    InvalidUtf8,
}

impl fmt::Display for RecorderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson => f.write_str("invalid recorder JSON"),
            Self::InvalidBinary => f.write_str("invalid recorder binary"),
            Self::InvalidUtf8 => f.write_str("invalid recorder UTF-8"),
        }
    }
}

impl std::error::Error for RecorderError {}

fn push_escaped(out: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
}

fn push_float(out: &mut String, value: f64) {
    if !value.is_finite() {
        out.push_str("0.000000");
        return;
    }
    let mut value = value;
    if value < 0.0 {
        out.push('-');
        value = -value;
    }
    let scaled = (value * 1_000_000.0 + 0.5) as u64;
    push_u64(out, scaled / 1_000_000);
    out.push('.');
    let frac = scaled % 1_000_000;
    let mut place = 100_000;
    while place > 0 {
        out.push(char::from(b'0' + ((frac / place) % 10) as u8));
        place /= 10;
    }
}

fn push_u64(out: &mut String, mut value: u64) {
    if value == 0 {
        out.push('0');
        return;
    }
    let mut digits = [0_u8; 20];
    let mut len = 0;
    while value > 0 {
        digits[len] = (value % 10) as u8;
        value /= 10;
        len += 1;
    }
    for digit in digits[..len].iter().rev() {
        out.push(char::from(b'0' + *digit));
    }
}

struct JsonCursor<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> JsonCursor<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn seek(&mut self, needle: &str) -> Result<(), RecorderError> {
        let offset = self.input[self.pos..]
            .find(needle)
            .ok_or(RecorderError::InvalidJson)?;
        self.pos += offset + needle.len();
        Ok(())
    }

    fn skip_ws(&mut self) {
        while self
            .input
            .as_bytes()
            .get(self.pos)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.pos += 1;
        }
    }

    fn consume(&mut self, ch: char) -> bool {
        self.skip_ws();
        if self.input[self.pos..].starts_with(ch) {
            self.pos += ch.len_utf8();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, ch: char) -> Result<(), RecorderError> {
        if self.consume(ch) {
            Ok(())
        } else {
            Err(RecorderError::InvalidJson)
        }
    }

    fn string(&mut self) -> Result<String, RecorderError> {
        self.skip_ws();
        self.expect('"')?;
        let mut out = String::new();
        while let Some(ch) = self.input[self.pos..].chars().next() {
            self.pos += ch.len_utf8();
            match ch {
                '"' => return Ok(out),
                '\\' => {
                    let escaped = self.input[self.pos..]
                        .chars()
                        .next()
                        .ok_or(RecorderError::InvalidJson)?;
                    self.pos += escaped.len_utf8();
                    match escaped {
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        'n' => out.push('\n'),
                        'r' => out.push('\r'),
                        't' => out.push('\t'),
                        _ => return Err(RecorderError::InvalidJson),
                    }
                }
                _ => out.push(ch),
            }
        }
        Err(RecorderError::InvalidJson)
    }

    fn number(&mut self) -> Result<f64, RecorderError> {
        self.skip_ws();
        let start = self.pos;
        while let Some(byte) = self.input.as_bytes().get(self.pos) {
            if byte.is_ascii_digit() || matches!(*byte, b'-' | b'+' | b'.' | b'e' | b'E') {
                self.pos += 1;
            } else {
                break;
            }
        }
        self.input[start..self.pos]
            .parse::<f64>()
            .map_err(|_| RecorderError::InvalidJson)
    }
}

struct BinaryReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> BinaryReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn expect_magic(&mut self, magic: &[u8]) -> Result<(), RecorderError> {
        if self.bytes(magic.len())? == magic {
            Ok(())
        } else {
            Err(RecorderError::InvalidBinary)
        }
    }

    fn bytes(&mut self, len: usize) -> Result<&'a [u8], RecorderError> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or(RecorderError::InvalidBinary)?;
        if end > self.bytes.len() {
            return Err(RecorderError::InvalidBinary);
        }
        let out = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(out)
    }

    fn u32(&mut self) -> Result<u32, RecorderError> {
        let bytes: [u8; 4] = self
            .bytes(4)?
            .try_into()
            .map_err(|_| RecorderError::InvalidBinary)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn f32(&mut self) -> Result<f32, RecorderError> {
        let bytes: [u8; 4] = self
            .bytes(4)?
            .try_into()
            .map_err(|_| RecorderError::InvalidBinary)?;
        Ok(f32::from_le_bytes(bytes))
    }

    fn f64(&mut self) -> Result<f64, RecorderError> {
        let bytes: [u8; 8] = self
            .bytes(8)?
            .try_into()
            .map_err(|_| RecorderError::InvalidBinary)?;
        Ok(f64::from_le_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recorder_round_trips_json_and_replays() {
        let mut recorder = AnimationRecorder::new();
        recorder.start();
        recorder.record("x", 0.0, 0.0);
        recorder.record("x", 1.0, 10.0);
        let json = recorder.export_json();
        let imported = AnimationRecorder::import_json(&json).expect("json import");
        assert_eq!(imported.replay("x", 0.5), Some(5.0));
    }

    #[test]
    fn recorder_round_trips_binary() {
        let mut recorder = AnimationRecorder::new();
        recorder.start();
        recorder.record("scale", 0.0, 1.0);
        recorder.record("scale", 1.0, 2.0);
        let binary = recorder.export_binary();
        let imported = AnimationRecorder::import_binary(&binary).expect("binary import");
        assert_eq!(imported.replay("scale", 0.25), Some(1.25));
    }
}
