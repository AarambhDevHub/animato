//! JavaScript-friendly easing parser and utility exports.

use crate::error::{JsResult, js_error};
use crate::types::{normalize_name, string_array};
use animato_core::Easing;
use js_sys::Array;
use wasm_bindgen::prelude::*;

/// Canonical easing names exposed to JavaScript picker UIs.
pub const EASING_NAMES: &[&str] = &[
    "linear",
    "easeInQuad",
    "easeOutQuad",
    "easeInOutQuad",
    "easeInCubic",
    "easeOutCubic",
    "easeInOutCubic",
    "easeInQuart",
    "easeOutQuart",
    "easeInOutQuart",
    "easeInQuint",
    "easeOutQuint",
    "easeInOutQuint",
    "easeInSine",
    "easeOutSine",
    "easeInOutSine",
    "easeInExpo",
    "easeOutExpo",
    "easeInOutExpo",
    "easeInCirc",
    "easeOutCirc",
    "easeInOutCirc",
    "easeInBack",
    "easeOutBack",
    "easeInOutBack",
    "easeInElastic",
    "easeOutElastic",
    "easeInOutElastic",
    "easeInBounce",
    "easeOutBounce",
    "easeInOutBounce",
    "steps(n)",
    "cubicBezier(x1,y1,x2,y2)",
    "roughEase(strength,points)",
    "slowMo(linearRatio,power)",
    "wiggle(wiggles)",
    "customBounce(strength)",
    "expoScale(start,end)",
];

/// Parse a JavaScript-friendly easing name into Animato's [`Easing`] enum.
pub fn parse_easing(name: &str) -> JsResult<Easing> {
    let trimmed = name.trim();
    if let Some(args) = call_args(trimmed, "steps") {
        let values = parse_numbers(args)?;
        if values.len() != 1 {
            return Err(js_error("steps() expects one numeric argument"));
        }
        return Ok(Easing::Steps(values[0].max(1.0).round() as u32));
    }
    if let Some(args) =
        call_args(trimmed, "cubicBezier").or_else(|| call_args(trimmed, "cubic-bezier"))
    {
        let values = parse_numbers(args)?;
        if values.len() != 4 {
            return Err(js_error("cubicBezier() expects four numeric arguments"));
        }
        return Ok(Easing::CubicBezier(
            values[0], values[1], values[2], values[3],
        ));
    }
    if let Some(args) = call_args(trimmed, "roughEase") {
        let values = parse_numbers(args)?;
        if values.len() != 2 {
            return Err(js_error("roughEase() expects strength and points"));
        }
        return Ok(Easing::RoughEase {
            strength: values[0],
            points: values[1].round().max(1.0) as u32,
        });
    }
    if let Some(args) = call_args(trimmed, "slowMo") {
        let values = parse_numbers(args)?;
        if values.len() != 2 {
            return Err(js_error("slowMo() expects linearRatio and power"));
        }
        return Ok(Easing::SlowMo {
            linear_ratio: values[0],
            power: values[1],
        });
    }
    if let Some(args) = call_args(trimmed, "wiggle") {
        let values = parse_numbers(args)?;
        if values.len() != 1 {
            return Err(js_error("wiggle() expects one numeric argument"));
        }
        return Ok(Easing::Wiggle {
            wiggles: values[0].round().max(1.0) as u32,
        });
    }
    if let Some(args) = call_args(trimmed, "customBounce") {
        let values = parse_numbers(args)?;
        if values.len() != 1 {
            return Err(js_error("customBounce() expects one numeric argument"));
        }
        return Ok(Easing::CustomBounce {
            strength: values[0],
        });
    }
    if let Some(args) = call_args(trimmed, "expoScale") {
        let values = parse_numbers(args)?;
        if values.len() != 2 {
            return Err(js_error("expoScale() expects start and end"));
        }
        return Ok(Easing::ExpoScale {
            start: values[0],
            end: values[1],
        });
    }

    match normalize_name(trimmed).as_str() {
        "linear" => Ok(Easing::Linear),
        "easeinquad" => Ok(Easing::EaseInQuad),
        "easeoutquad" => Ok(Easing::EaseOutQuad),
        "easeinoutquad" => Ok(Easing::EaseInOutQuad),
        "easeincubic" => Ok(Easing::EaseInCubic),
        "easeoutcubic" => Ok(Easing::EaseOutCubic),
        "easeinoutcubic" => Ok(Easing::EaseInOutCubic),
        "easeinquart" => Ok(Easing::EaseInQuart),
        "easeoutquart" => Ok(Easing::EaseOutQuart),
        "easeinoutquart" => Ok(Easing::EaseInOutQuart),
        "easeinquint" => Ok(Easing::EaseInQuint),
        "easeoutquint" => Ok(Easing::EaseOutQuint),
        "easeinoutquint" => Ok(Easing::EaseInOutQuint),
        "easeinsine" => Ok(Easing::EaseInSine),
        "easeoutsine" => Ok(Easing::EaseOutSine),
        "easeinoutsine" => Ok(Easing::EaseInOutSine),
        "easeinexpo" => Ok(Easing::EaseInExpo),
        "easeoutexpo" => Ok(Easing::EaseOutExpo),
        "easeinoutexpo" => Ok(Easing::EaseInOutExpo),
        "easeincirc" => Ok(Easing::EaseInCirc),
        "easeoutcirc" => Ok(Easing::EaseOutCirc),
        "easeinoutcirc" => Ok(Easing::EaseInOutCirc),
        "easeinback" => Ok(Easing::EaseInBack),
        "easeoutback" => Ok(Easing::EaseOutBack),
        "easeinoutback" => Ok(Easing::EaseInOutBack),
        "easeinelastic" => Ok(Easing::EaseInElastic),
        "easeoutelastic" => Ok(Easing::EaseOutElastic),
        "easeinoutelastic" => Ok(Easing::EaseInOutElastic),
        "easeinbounce" => Ok(Easing::EaseInBounce),
        "easeoutbounce" => Ok(Easing::EaseOutBounce),
        "easeinoutbounce" => Ok(Easing::EaseInOutBounce),
        _ => Err(js_error(format!("unknown easing `{name}`"))),
    }
}

/// Return a canonical string for a parsed easing value.
pub fn easing_name(easing: &Easing) -> &'static str {
    match easing {
        Easing::Linear => "linear",
        Easing::EaseInQuad => "easeInQuad",
        Easing::EaseOutQuad => "easeOutQuad",
        Easing::EaseInOutQuad => "easeInOutQuad",
        Easing::EaseInCubic => "easeInCubic",
        Easing::EaseOutCubic => "easeOutCubic",
        Easing::EaseInOutCubic => "easeInOutCubic",
        Easing::EaseInQuart => "easeInQuart",
        Easing::EaseOutQuart => "easeOutQuart",
        Easing::EaseInOutQuart => "easeInOutQuart",
        Easing::EaseInQuint => "easeInQuint",
        Easing::EaseOutQuint => "easeOutQuint",
        Easing::EaseInOutQuint => "easeInOutQuint",
        Easing::EaseInSine => "easeInSine",
        Easing::EaseOutSine => "easeOutSine",
        Easing::EaseInOutSine => "easeInOutSine",
        Easing::EaseInExpo => "easeInExpo",
        Easing::EaseOutExpo => "easeOutExpo",
        Easing::EaseInOutExpo => "easeInOutExpo",
        Easing::EaseInCirc => "easeInCirc",
        Easing::EaseOutCirc => "easeOutCirc",
        Easing::EaseInOutCirc => "easeInOutCirc",
        Easing::EaseInBack => "easeInBack",
        Easing::EaseOutBack => "easeOutBack",
        Easing::EaseInOutBack => "easeInOutBack",
        Easing::EaseInElastic => "easeInElastic",
        Easing::EaseOutElastic => "easeOutElastic",
        Easing::EaseInOutElastic => "easeInOutElastic",
        Easing::EaseInBounce => "easeInBounce",
        Easing::EaseOutBounce => "easeOutBounce",
        Easing::EaseInOutBounce => "easeInOutBounce",
        Easing::CubicBezier(..) => "cubicBezier",
        Easing::Steps(_) => "steps",
        Easing::RoughEase { .. } => "roughEase",
        Easing::SlowMo { .. } => "slowMo",
        Easing::Wiggle { .. } => "wiggle",
        Easing::CustomBounce { .. } => "customBounce",
        Easing::ExpoScale { .. } => "expoScale",
        Easing::Custom(_) => "custom",
    }
}

/// Parse an easing name and return its canonical name.
#[wasm_bindgen(js_name = parseEasing)]
pub fn parse_easing_for_js(name: &str) -> Result<String, JsValue> {
    Ok(easing_name(&parse_easing(name)?).to_owned())
}

/// Apply an easing by name to normalized progress `t`.
#[wasm_bindgen]
pub fn ease(name: &str, t: f32) -> Result<f32, JsValue> {
    Ok(parse_easing(name)?.apply(t))
}

/// Return every available easing name.
#[wasm_bindgen(js_name = availableEasings)]
pub fn available_easings() -> Array {
    string_array(EASING_NAMES)
}

fn call_args<'a>(input: &'a str, function_name: &str) -> Option<&'a str> {
    let input_normalized = normalize_name(input);
    let name_normalized = normalize_name(function_name);
    if !input_normalized.starts_with(&name_normalized) {
        return None;
    }
    let open = input.find('(')?;
    let close = input.rfind(')')?;
    if close <= open {
        return None;
    }
    Some(&input[open + 1..close])
}

fn parse_numbers(input: &str) -> JsResult<Vec<f32>> {
    if input.trim().is_empty() {
        return Ok(Vec::new());
    }
    input
        .split(',')
        .map(|part| {
            let value = part
                .trim()
                .parse::<f32>()
                .map_err(|_| js_error(format!("invalid numeric argument `{}`", part.trim())))?;
            if value.is_finite() {
                Ok(value)
            } else {
                Err(js_error("numeric arguments must be finite"))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_named_easing() {
        assert_eq!(
            parse_easing("ease-out-cubic").unwrap(),
            Easing::EaseOutCubic
        );
        assert_eq!(
            parse_easing("easeInOutBack").unwrap(),
            Easing::EaseInOutBack
        );
    }

    #[test]
    fn parses_parameterized_easing() {
        assert_eq!(parse_easing("steps(5)").unwrap(), Easing::Steps(5));
        assert_eq!(
            parse_easing("cubicBezier(0.4, 0, 0.2, 1)").unwrap(),
            Easing::CubicBezier(0.4, 0.0, 0.2, 1.0)
        );
        assert!(matches!(
            parse_easing("roughEase(0.4, 8)").unwrap(),
            Easing::RoughEase { points: 8, .. }
        ));
    }

    #[test]
    fn rejects_bad_easing() {
        assert!(parse_easing("nope").is_err());
        assert!(parse_easing("steps(1, 2)").is_err());
    }
}
