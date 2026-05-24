//! Color interpolation bindings.

use crate::easing::parse_easing;
use crate::error::{JsResult, js_error, non_negative};
use crate::types::{f32_array, normalize_name};
use animato_core::Update;
use animato_tween::Tween as CoreTween;
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ColorSpace {
    Rgb,
    Linear,
    Lab,
    Oklch,
}

impl ColorSpace {
    fn parse(input: &str) -> JsResult<Self> {
        match normalize_name(input).as_str() {
            "rgb" | "srgb" => Ok(Self::Rgb),
            "linear" | "linearrgb" => Ok(Self::Linear),
            "lab" => Ok(Self::Lab),
            "oklch" | "oklab" => Ok(Self::Oklch),
            _ => Err(js_error(format!("unknown color space `{input}`"))),
        }
    }
}

/// Perceptual color tween.
#[wasm_bindgen(js_name = ColorTween)]
#[derive(Clone, Debug)]
pub struct ColorTween {
    start: Rgb,
    end: Rgb,
    space: ColorSpace,
    tween: CoreTween<f32>,
}

#[wasm_bindgen(js_class = ColorTween)]
impl ColorTween {
    /// Create a color tween from hex colors.
    #[wasm_bindgen(constructor)]
    pub fn new(from: &str, to: &str, duration: f32, space: &str) -> Result<Self, JsValue> {
        Ok(Self {
            start: parse_color(from)?,
            end: parse_color(to)?,
            space: ColorSpace::parse(space)?,
            tween: CoreTween::new(0.0, 1.0)
                .duration(non_negative(duration, 1.0))
                .build(),
        })
    }

    /// Advance by `dt` seconds.
    pub fn update(&mut self, dt: f32) -> bool {
        self.tween.update(dt)
    }

    /// Current color as hex.
    #[wasm_bindgen(js_name = valueHex)]
    pub fn value_hex(&self) -> String {
        to_hex(interpolate(
            self.start,
            self.end,
            self.tween.value(),
            self.space,
        ))
    }

    /// Current color as `[r, g, b, a]` in 0-1 floats.
    #[wasm_bindgen(js_name = rgbaArray)]
    pub fn rgba_array(&self) -> Float32Array {
        let color = interpolate(self.start, self.end, self.tween.value(), self.space);
        f32_array(&[color.r, color.g, color.b, 1.0])
    }

    /// Current normalized progress.
    pub fn progress(&self) -> f32 {
        self.tween.progress()
    }

    /// Whether playback is complete.
    #[wasm_bindgen(js_name = isComplete)]
    pub fn is_complete(&self) -> bool {
        self.tween.is_complete()
    }

    /// Reset playback.
    pub fn reset(&mut self) {
        self.tween.reset();
    }

    /// Set easing by name.
    #[wasm_bindgen(js_name = setEasing)]
    pub fn set_easing(&mut self, easing: &str) -> Result<(), JsValue> {
        self.tween.easing = parse_easing(easing)?;
        Ok(())
    }
}

/// Interpolate two colors immediately.
#[wasm_bindgen(js_name = interpolateColor)]
pub fn interpolate_color(from: &str, to: &str, t: f32, space: &str) -> Result<String, JsValue> {
    Ok(to_hex(interpolate(
        parse_color(from)?,
        parse_color(to)?,
        t,
        ColorSpace::parse(space)?,
    )))
}

fn interpolate(from: Rgb, to: Rgb, t: f32, space: ColorSpace) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    match space {
        ColorSpace::Rgb => lerp_rgb(from, to, t),
        ColorSpace::Linear => linear_to_srgb(lerp_rgb(srgb_to_linear(from), srgb_to_linear(to), t)),
        ColorSpace::Lab => xyz_to_srgb(lab_to_xyz(lerp3(srgb_to_lab(from), srgb_to_lab(to), t))),
        ColorSpace::Oklch => oklab_to_srgb(oklch_to_oklab(lerp_oklch(
            oklab_to_oklch(srgb_to_oklab(from)),
            oklab_to_oklch(srgb_to_oklab(to)),
            t,
        ))),
    }
}

fn lerp_rgb(a: Rgb, b: Rgb, t: f32) -> Rgb {
    Rgb {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
    }
}

fn parse_color(input: &str) -> JsResult<Rgb> {
    let hex = input.trim().trim_start_matches('#');
    let (r, g, b) = match hex.len() {
        3 => (
            u8::from_str_radix(&hex[0..1].repeat(2), 16),
            u8::from_str_radix(&hex[1..2].repeat(2), 16),
            u8::from_str_radix(&hex[2..3].repeat(2), 16),
        ),
        6 => (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ),
        _ => {
            return Err(js_error(format!(
                "expected #rgb or #rrggbb color, got `{input}`"
            )));
        }
    };
    Ok(Rgb {
        r: r.map_err(|_| js_error(format!("invalid red channel in `{input}`")))? as f32 / 255.0,
        g: g.map_err(|_| js_error(format!("invalid green channel in `{input}`")))? as f32 / 255.0,
        b: b.map_err(|_| js_error(format!("invalid blue channel in `{input}`")))? as f32 / 255.0,
    })
}

fn to_hex(color: Rgb) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        channel(color.r),
        channel(color.g),
        channel(color.b)
    )
}

fn channel(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn srgb_channel_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_channel_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn srgb_to_linear(c: Rgb) -> Rgb {
    Rgb {
        r: srgb_channel_to_linear(c.r),
        g: srgb_channel_to_linear(c.g),
        b: srgb_channel_to_linear(c.b),
    }
}

fn linear_to_srgb(c: Rgb) -> Rgb {
    Rgb {
        r: linear_channel_to_srgb(c.r),
        g: linear_channel_to_srgb(c.g),
        b: linear_channel_to_srgb(c.b),
    }
}

fn srgb_to_xyz(c: Rgb) -> [f32; 3] {
    let c = srgb_to_linear(c);
    [
        c.r * 0.4124564 + c.g * 0.3575761 + c.b * 0.1804375,
        c.r * 0.2126729 + c.g * 0.7151522 + c.b * 0.0721750,
        c.r * 0.0193339 + c.g * 0.119_192 + c.b * 0.9503041,
    ]
}

fn xyz_to_srgb(xyz: [f32; 3]) -> Rgb {
    linear_to_srgb(Rgb {
        r: xyz[0] * 3.2404542 + xyz[1] * -1.5371385 + xyz[2] * -0.4985314,
        g: xyz[0] * -0.969_266 + xyz[1] * 1.8760108 + xyz[2] * 0.0415560,
        b: xyz[0] * 0.0556434 + xyz[1] * -0.2040259 + xyz[2] * 1.0572252,
    })
}

fn srgb_to_lab(c: Rgb) -> [f32; 3] {
    let xyz = srgb_to_xyz(c);
    let x = lab_f(xyz[0] / 0.95047);
    let y = lab_f(xyz[1]);
    let z = lab_f(xyz[2] / 1.08883);
    [116.0 * y - 16.0, 500.0 * (x - y), 200.0 * (y - z)]
}

fn lab_to_xyz(lab: [f32; 3]) -> [f32; 3] {
    let y = (lab[0] + 16.0) / 116.0;
    let x = lab[1] / 500.0 + y;
    let z = y - lab[2] / 200.0;
    [0.95047 * lab_f_inv(x), lab_f_inv(y), 1.08883 * lab_f_inv(z)]
}

fn lab_f(t: f32) -> f32 {
    if t > 0.008856 {
        t.cbrt()
    } else {
        7.787 * t + 16.0 / 116.0
    }
}

fn lab_f_inv(t: f32) -> f32 {
    let t3 = t * t * t;
    if t3 > 0.008856 {
        t3
    } else {
        (t - 16.0 / 116.0) / 7.787
    }
}

fn srgb_to_oklab(c: Rgb) -> [f32; 3] {
    let c = srgb_to_linear(c);
    let l = 0.41222146 * c.r + 0.53633255 * c.g + 0.051445995 * c.b;
    let m = 0.2119035 * c.r + 0.6806995 * c.g + 0.10739696 * c.b;
    let s = 0.08830246 * c.r + 0.28171884 * c.g + 0.6299787 * c.b;
    let l = l.cbrt();
    let m = m.cbrt();
    let s = s.cbrt();
    [
        0.21045426 * l + 0.7936178 * m - 0.004072047 * s,
        1.9779985 * l - 2.4285922 * m + 0.4505937 * s,
        0.025904037 * l + 0.78277177 * m - 0.80867577 * s,
    ]
}

fn oklab_to_srgb(oklab: [f32; 3]) -> Rgb {
    let l = oklab[0] + 0.39633778 * oklab[1] + 0.21580376 * oklab[2];
    let m = oklab[0] - 0.105561346 * oklab[1] - 0.06385417 * oklab[2];
    let s = oklab[0] - 0.08948418 * oklab[1] - 1.2914855 * oklab[2];
    let l = l * l * l;
    let m = m * m * m;
    let s = s * s * s;
    linear_to_srgb(Rgb {
        r: 4.0767417 * l - 3.3077116 * m + 0.23096994 * s,
        g: -1.268438 * l + 2.6097574 * m - 0.34131938 * s,
        b: -0.0041960863 * l - 0.7034186 * m + 1.7076147 * s,
    })
}

fn oklab_to_oklch(oklab: [f32; 3]) -> [f32; 3] {
    [
        oklab[0],
        (oklab[1] * oklab[1] + oklab[2] * oklab[2]).sqrt(),
        oklab[2].atan2(oklab[1]),
    ]
}

fn oklch_to_oklab(oklch: [f32; 3]) -> [f32; 3] {
    [
        oklch[0],
        oklch[1] * oklch[2].cos(),
        oklch[1] * oklch[2].sin(),
    ]
}

fn lerp3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn lerp_oklch(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let mut dh = b[2] - a[2];
    if dh > core::f32::consts::PI {
        dh -= core::f32::consts::TAU;
    } else if dh < -core::f32::consts::PI {
        dh += core::f32::consts::TAU;
    }
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + dh * t,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hex() {
        assert_eq!(to_hex(parse_color("#0f0").unwrap()), "#00ff00");
    }

    #[test]
    fn interpolates_rgb() {
        assert_eq!(
            interpolate_color("#000", "#fff", 0.5, "rgb").unwrap(),
            "#808080"
        );
    }
}
