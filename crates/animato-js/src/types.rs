//! Shared JavaScript binding helpers.

use crate::error::{JsResult, js_error};
use animato_tween::Loop;
use js_sys::{Array, Float32Array};
use wasm_bindgen::JsValue;

pub(crate) fn loop_from_count(count: u32) -> Loop {
    if count == 0 {
        Loop::Once
    } else {
        Loop::Times(count)
    }
}

pub(crate) fn parse_loop_mode(mode: &str) -> JsResult<Loop> {
    let normalized = normalize_name(mode);
    match normalized.as_str() {
        "once" => Ok(Loop::Once),
        "forever" | "infinite" | "loop" => Ok(Loop::Forever),
        "pingpong" | "alternate" | "yoyo" => Ok(Loop::PingPong),
        _ if normalized.starts_with("times") => {
            let n = normalized
                .trim_start_matches("times")
                .parse::<u32>()
                .map_err(|_| js_error(format!("invalid loop mode `{mode}`")))?;
            Ok(Loop::Times(n.max(1)))
        }
        _ => Err(js_error(format!("unknown loop mode `{mode}`"))),
    }
}

pub(crate) fn normalize_name(name: &str) -> String {
    name.chars()
        .filter(|ch| !ch.is_whitespace() && *ch != '-' && *ch != '_')
        .flat_map(char::to_lowercase)
        .collect()
}

pub(crate) fn f32_array(values: &[f32]) -> Float32Array {
    Float32Array::from(values)
}

pub(crate) fn vec2(x: f32, y: f32) -> Float32Array {
    f32_array(&[x, y])
}

pub(crate) fn flat_points(values: &Float32Array) -> JsResult<Vec<[f32; 2]>> {
    if !values.length().is_multiple_of(2) {
        return Err(js_error("point arrays must contain x/y pairs"));
    }
    let mut points = Vec::with_capacity(values.length() as usize / 2);
    let mut index = 0;
    while index < values.length() {
        points.push([values.get_index(index), values.get_index(index + 1)]);
        index += 2;
    }
    Ok(points)
}

pub(crate) fn points_to_array(points: &[[f32; 2]]) -> Float32Array {
    let mut flat = Vec::with_capacity(points.len() * 2);
    for point in points {
        flat.push(point[0]);
        flat.push(point[1]);
    }
    f32_array(&flat)
}

pub(crate) fn string_array(values: &[&str]) -> Array {
    let array = Array::new();
    for value in values {
        array.push(&JsValue::from_str(value));
    }
    array
}
