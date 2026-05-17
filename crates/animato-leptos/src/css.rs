//! CSS property helpers for Leptos animation hooks.

use animato_core::Easing;
use animato_spring::SpringConfig;
use leptos::prelude::{Effect, Get, ReadSignal, Set, signal};

/// CSS property bag used by Animato Leptos helpers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AnimatedStyle {
    /// CSS `opacity`.
    pub opacity: Option<f32>,
    /// Raw CSS `transform` string appended after generated transform parts.
    pub transform: Option<String>,
    /// Uniform CSS scale.
    pub scale: Option<f32>,
    /// Translation on the x axis in CSS pixels.
    pub translate_x: Option<f32>,
    /// Translation on the y axis in CSS pixels.
    pub translate_y: Option<f32>,
    /// Rotation in degrees.
    pub rotate: Option<f32>,
    /// Skew on the x axis in degrees.
    pub skew_x: Option<f32>,
    /// Skew on the y axis in degrees.
    pub skew_y: Option<f32>,
    /// CSS blur radius in pixels.
    pub blur: Option<f32>,
    /// RGBA background color with components in `[0.0, 1.0]`.
    pub background_color: Option<[f32; 4]>,
    /// CSS border radius in pixels.
    pub border_radius: Option<f32>,
    /// CSS width in pixels.
    pub width: Option<f32>,
    /// CSS height in pixels.
    pub height: Option<f32>,
    /// Raw CSS `clip-path` value.
    pub clip_path: Option<String>,
    /// Additional raw CSS property/value pairs.
    pub custom: Vec<(String, String)>,
}

impl AnimatedStyle {
    /// Create an empty style bag.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `opacity`.
    pub fn opacity(mut self, value: f32) -> Self {
        self.opacity = Some(value.clamp(0.0, 1.0));
        self
    }

    /// Set translation in CSS pixels.
    pub fn translate(mut self, x: f32, y: f32) -> Self {
        self.translate_x = Some(crate::finite_or(x, 0.0));
        self.translate_y = Some(crate::finite_or(y, 0.0));
        self
    }

    /// Set uniform scale.
    pub fn scale(mut self, value: f32) -> Self {
        self.scale = Some(crate::finite_or(value, 1.0).max(0.0));
        self
    }

    /// Set rotation in degrees.
    pub fn rotate(mut self, degrees: f32) -> Self {
        self.rotate = Some(crate::finite_or(degrees, 0.0));
        self
    }

    /// Set blur in CSS pixels.
    pub fn blur(mut self, px: f32) -> Self {
        self.blur = Some(crate::finite_or(px, 0.0).max(0.0));
        self
    }

    /// Set width in CSS pixels.
    pub fn width(mut self, px: f32) -> Self {
        self.width = Some(crate::finite_or(px, 0.0).max(0.0));
        self
    }

    /// Set height in CSS pixels.
    pub fn height(mut self, px: f32) -> Self {
        self.height = Some(crate::finite_or(px, 0.0).max(0.0));
        self
    }

    /// Set background color from RGBA components in `[0.0, 1.0]`.
    pub fn background_color(mut self, rgba: [f32; 4]) -> Self {
        self.background_color = Some(rgba.map(|v| v.clamp(0.0, 1.0)));
        self
    }

    /// Set border radius in CSS pixels.
    pub fn border_radius(mut self, px: f32) -> Self {
        self.border_radius = Some(crate::finite_or(px, 0.0).max(0.0));
        self
    }

    /// Set raw `clip-path`.
    pub fn clip_path(mut self, value: impl Into<String>) -> Self {
        self.clip_path = Some(value.into());
        self
    }

    /// Set raw `transform`.
    pub fn transform(mut self, value: impl Into<String>) -> Self {
        self.transform = Some(value.into());
        self
    }

    /// Add a custom raw CSS property.
    pub fn custom(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom.push((name.into(), value.into()));
        self
    }

    /// Interpolate two style bags.
    pub fn interpolate(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            opacity: lerp_option(self.opacity, other.opacity, t),
            transform: choose_string(self.transform.as_ref(), other.transform.as_ref(), t),
            scale: lerp_option(self.scale, other.scale, t),
            translate_x: lerp_option(self.translate_x, other.translate_x, t),
            translate_y: lerp_option(self.translate_y, other.translate_y, t),
            rotate: lerp_option(self.rotate, other.rotate, t),
            skew_x: lerp_option(self.skew_x, other.skew_x, t),
            skew_y: lerp_option(self.skew_y, other.skew_y, t),
            blur: lerp_option(self.blur, other.blur, t),
            background_color: lerp_color(self.background_color, other.background_color, t),
            border_radius: lerp_option(self.border_radius, other.border_radius, t),
            width: lerp_option(self.width, other.width, t),
            height: lerp_option(self.height, other.height, t),
            clip_path: choose_string(self.clip_path.as_ref(), other.clip_path.as_ref(), t),
            custom: if t >= 1.0 {
                other.custom.clone()
            } else {
                self.custom.clone()
            },
        }
    }

    /// Convert the property bag into a CSS style string.
    pub fn to_css(&self) -> String {
        let mut style = String::new();

        if let Some(opacity) = self.opacity {
            push_prop(&mut style, "opacity", &format_num(opacity));
        }

        let transform = self.transform_string();
        if !transform.is_empty() {
            push_prop(&mut style, "transform", &transform);
        }

        if let Some(blur) = self.blur {
            push_prop(&mut style, "filter", &format!("blur({})", format_px(blur)));
        }
        if let Some(color) = self.background_color {
            push_prop(&mut style, "background-color", &rgba_to_css(color));
        }
        if let Some(radius) = self.border_radius {
            push_prop(&mut style, "border-radius", &format_px(radius));
        }
        if let Some(width) = self.width {
            push_prop(&mut style, "width", &format_px(width));
        }
        if let Some(height) = self.height {
            push_prop(&mut style, "height", &format_px(height));
        }
        if let Some(clip_path) = &self.clip_path {
            push_prop(&mut style, "clip-path", clip_path);
        }
        for (name, value) in &self.custom {
            push_prop(&mut style, name, value);
        }

        style
    }

    /// Return only the generated CSS transform string.
    pub fn transform_string(&self) -> String {
        let mut parts = Vec::new();
        if let Some(x) = self.translate_x {
            let y = self.translate_y.unwrap_or(0.0);
            parts.push(format!("translate({},{})", format_px(x), format_px(y)));
        } else if let Some(y) = self.translate_y {
            parts.push(format!("translateY({})", format_px(y)));
        }
        if let Some(scale) = self.scale {
            parts.push(format!("scale({})", format_num(scale)));
        }
        if let Some(rotate) = self.rotate {
            parts.push(format!("rotate({}deg)", format_num(rotate)));
        }
        if let Some(skew_x) = self.skew_x {
            parts.push(format!("skewX({}deg)", format_num(skew_x)));
        }
        if let Some(skew_y) = self.skew_y {
            parts.push(format!("skewY({}deg)", format_num(skew_y)));
        }
        if let Some(raw) = &self.transform {
            parts.push(raw.clone());
        }
        parts.join(" ")
    }
}

/// Animate CSS properties with a tween and return a style-string signal.
pub fn css_tween(
    from: AnimatedStyle,
    to: AnimatedStyle,
    duration: f32,
    easing: Easing,
) -> ReadSignal<String> {
    let initial = if crate::ssr::is_hydrating() {
        to.to_css()
    } else {
        from.to_css()
    };
    let (style, set_style) = signal(initial);
    let (progress, _handle) = crate::hooks::use_tween(0.0_f32, 1.0, move |builder| {
        builder.duration(duration).easing(easing)
    });

    Effect::new(move || {
        let p = progress.get();
        set_style.set(from.interpolate(&to, p).to_css());
    });

    style
}

/// Animate CSS properties with a spring and return a style-string signal.
pub fn css_spring(target: AnimatedStyle, config: SpringConfig) -> ReadSignal<String> {
    let (style, set_style) = signal(if crate::ssr::is_hydrating() {
        target.to_css()
    } else {
        AnimatedStyle::default().to_css()
    });
    let (progress, handle) = crate::hooks::use_spring(0.0_f32, config);
    handle.set_target(1.0);

    Effect::new(move || {
        let p = progress.get().clamp(0.0, 1.0);
        set_style.set(AnimatedStyle::default().interpolate(&target, p).to_css());
    });

    style
}

fn lerp_option(a: Option<f32>, b: Option<f32>, t: f32) -> Option<f32> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a + (b - a) * t),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b * t),
        (None, None) => None,
    }
}

fn lerp_color(a: Option<[f32; 4]>, b: Option<[f32; 4]>, t: f32) -> Option<[f32; 4]> {
    match (a, b) {
        (Some(a), Some(b)) => Some([
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
            a[3] + (b[3] - a[3]) * t,
        ]),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some([b[0] * t, b[1] * t, b[2] * t, b[3] * t]),
        (None, None) => None,
    }
}

fn choose_string(a: Option<&String>, b: Option<&String>, t: f32) -> Option<String> {
    match (a, b) {
        (_, Some(b)) if t >= 1.0 => Some(b.clone()),
        (Some(a), _) => Some(a.clone()),
        (None, Some(b)) => Some(b.clone()),
        (None, None) => None,
    }
}

fn push_prop(style: &mut String, name: &str, value: &str) {
    if !style.is_empty() {
        style.push(' ');
    }
    style.push_str(name);
    style.push(':');
    style.push_str(value);
    style.push(';');
}

fn format_px(value: f32) -> String {
    format!("{}px", format_num(value))
}

fn format_num(value: f32) -> String {
    let value = crate::finite_or(value, 0.0);
    let rounded = (value * 1000.0).round() / 1000.0;
    if (rounded - rounded.round()).abs() < 0.0001 {
        format!("{}", rounded.round() as i32)
    } else {
        format!("{rounded:.3}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned()
    }
}

fn rgba_to_css(color: [f32; 4]) -> String {
    let r = (color[0].clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (color[1].clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (color[2].clamp(0.0, 1.0) * 255.0).round() as u8;
    let a = format_num(color[3].clamp(0.0, 1.0));
    format!("rgba({r},{g},{b},{a})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_transform_and_properties() {
        let css = AnimatedStyle::new()
            .opacity(0.5)
            .translate(10.0, 20.0)
            .scale(1.25)
            .blur(4.0)
            .background_color([1.0, 0.0, 0.5, 0.75])
            .to_css();

        assert!(css.contains("opacity:0.5;"));
        assert!(css.contains("transform:translate(10px,20px) scale(1.25);"));
        assert!(css.contains("filter:blur(4px);"));
        assert!(css.contains("background-color:rgba(255,0,128,0.75);"));
    }

    #[test]
    fn interpolates_numeric_fields() {
        let from = AnimatedStyle::new().opacity(0.0).translate(0.0, 0.0);
        let to = AnimatedStyle::new().opacity(1.0).translate(100.0, 50.0);
        let mid = from.interpolate(&to, 0.5);

        assert_eq!(mid.opacity, Some(0.5));
        assert_eq!(mid.translate_x, Some(50.0));
        assert_eq!(mid.translate_y, Some(25.0));
    }

    #[test]
    fn serializes_extended_fields_and_clamps_inputs() {
        let mut style = AnimatedStyle::new()
            .opacity(2.0)
            .translate(f32::NAN, 12.25)
            .scale(-1.0)
            .rotate(45.5)
            .blur(-4.0)
            .width(120.0)
            .height(80.5)
            .background_color([2.0, -1.0, 0.25, 1.5])
            .border_radius(6.0)
            .clip_path("inset(10%)")
            .transform("translateZ(0)")
            .custom("pointer-events", "none");
        style.skew_x = Some(10.0);
        style.skew_y = Some(-5.0);

        let css = style.to_css();
        assert!(css.contains("opacity:1;"));
        assert!(css.contains("translate(0px,12.25px)"));
        assert!(css.contains("scale(0)"));
        assert!(css.contains("rotate(45.5deg)"));
        assert!(css.contains("skewX(10deg)"));
        assert!(css.contains("skewY(-5deg)"));
        assert!(css.contains("filter:blur(0px);"));
        assert!(css.contains("background-color:rgba(255,0,64,1);"));
        assert!(css.contains("border-radius:6px;"));
        assert!(css.contains("width:120px;"));
        assert!(css.contains("height:80.5px;"));
        assert!(css.contains("clip-path:inset(10%);"));
        assert!(css.contains("pointer-events:none;"));
    }

    #[test]
    fn interpolation_handles_missing_fields_and_string_switches() {
        let from = AnimatedStyle::new()
            .opacity(0.4)
            .transform("rotate(0deg)")
            .custom("color", "red");
        let to = AnimatedStyle::new()
            .scale(2.0)
            .clip_path("circle(50%)")
            .transform("rotate(90deg)")
            .custom("color", "blue");

        let mid = from.interpolate(&to, 0.5);
        assert_eq!(mid.opacity, Some(0.4));
        assert_eq!(mid.scale, Some(1.0));
        assert_eq!(mid.transform.as_deref(), Some("rotate(0deg)"));
        assert_eq!(mid.clip_path.as_deref(), Some("circle(50%)"));
        assert_eq!(mid.custom[0].1, "red");

        let end = from.interpolate(&to, 1.0);
        assert_eq!(end.transform.as_deref(), Some("rotate(90deg)"));
        assert_eq!(end.custom[0].1, "blue");
    }

    #[test]
    fn formatting_helpers_are_stable() {
        assert_eq!(format_num(f32::NAN), "0");
        assert_eq!(format_num(2.0), "2");
        assert_eq!(format_num(2.125), "2.125");
        assert_eq!(format_px(3.5), "3.5px");
        assert_eq!(rgba_to_css([-1.0, 0.5, 2.0, f32::NAN]), "rgba(0,128,255,0)");
    }
}
