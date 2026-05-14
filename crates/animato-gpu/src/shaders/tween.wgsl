struct TweenInput {
    start: f32,
    end: f32,
    duration: f32,
    elapsed: f32,
    easing_id: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0)
var<storage, read> tweens: array<TweenInput>;

@group(0) @binding(1)
var<storage, read_write> values: array<f32>;

fn ease_out_bounce(t0: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;
    var t = t0;
    if (t < 1.0 / d1) {
        return n1 * t * t;
    }
    if (t < 2.0 / d1) {
        t = t - 1.5 / d1;
        return n1 * t * t + 0.75;
    }
    if (t < 2.5 / d1) {
        t = t - 2.25 / d1;
        return n1 * t * t + 0.9375;
    }
    t = t - 2.625 / d1;
    return n1 * t * t + 0.984375;
}

fn easing_apply(id: u32, t: f32) -> f32 {
    let pi = 3.141592653589793;
    switch id {
        case 1u: { return t * t; }
        case 2u: { return 1.0 - pow(1.0 - t, 2.0); }
        case 3u: {
            if (t < 0.5) { return 2.0 * t * t; }
            return 1.0 - pow(-2.0 * t + 2.0, 2.0) / 2.0;
        }
        case 4u: { return t * t * t; }
        case 5u: { return 1.0 - pow(1.0 - t, 3.0); }
        case 6u: {
            if (t < 0.5) { return 4.0 * t * t * t; }
            return 1.0 - pow(-2.0 * t + 2.0, 3.0) / 2.0;
        }
        case 7u: { return t * t * t * t; }
        case 8u: { return 1.0 - pow(1.0 - t, 4.0); }
        case 9u: {
            if (t < 0.5) { return 8.0 * t * t * t * t; }
            return 1.0 - pow(-2.0 * t + 2.0, 4.0) / 2.0;
        }
        case 10u: { return t * t * t * t * t; }
        case 11u: { return 1.0 - pow(1.0 - t, 5.0); }
        case 12u: {
            if (t < 0.5) { return 16.0 * t * t * t * t * t; }
            return 1.0 - pow(-2.0 * t + 2.0, 5.0) / 2.0;
        }
        case 13u: { return 1.0 - cos(t * pi / 2.0); }
        case 14u: { return sin(t * pi / 2.0); }
        case 15u: { return -(cos(t * pi) - 1.0) / 2.0; }
        case 16u: {
            if (t == 0.0) { return 0.0; }
            return pow(2.0, 10.0 * t - 10.0);
        }
        case 17u: {
            if (t == 1.0) { return 1.0; }
            return 1.0 - pow(2.0, -10.0 * t);
        }
        case 18u: {
            if (t == 0.0) { return 0.0; }
            if (t == 1.0) { return 1.0; }
            if (t < 0.5) { return pow(2.0, 20.0 * t - 10.0) / 2.0; }
            return (2.0 - pow(2.0, -20.0 * t + 10.0)) / 2.0;
        }
        case 19u: { return 1.0 - sqrt(1.0 - t * t); }
        case 20u: { return sqrt(1.0 - (t - 1.0) * (t - 1.0)); }
        case 21u: {
            if (t < 0.5) { return (1.0 - sqrt(1.0 - pow(2.0 * t, 2.0))) / 2.0; }
            return (sqrt(1.0 - pow(-2.0 * t + 2.0, 2.0)) + 1.0) / 2.0;
        }
        case 22u: {
            let c1 = 1.70158;
            let c3 = c1 + 1.0;
            return c3 * t * t * t - c1 * t * t;
        }
        case 23u: {
            let c1 = 1.70158;
            let c3 = c1 + 1.0;
            let u = t - 1.0;
            return 1.0 + c3 * u * u * u + c1 * u * u;
        }
        case 24u: {
            let c1 = 1.70158;
            let c2 = c1 * 1.525;
            if (t < 0.5) {
                return (pow(2.0 * t, 2.0) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0;
            }
            return (pow(2.0 * t - 2.0, 2.0) * ((c2 + 1.0) * (2.0 * t - 2.0) + c2) + 2.0) / 2.0;
        }
        case 25u: {
            if (t == 0.0) { return 0.0; }
            if (t == 1.0) { return 1.0; }
            let c4 = (2.0 * pi) / 3.0;
            return -pow(2.0, 10.0 * t - 10.0) * sin((10.0 * t - 10.75) * c4);
        }
        case 26u: {
            if (t == 0.0) { return 0.0; }
            if (t == 1.0) { return 1.0; }
            let c4 = (2.0 * pi) / 3.0;
            return pow(2.0, -10.0 * t) * sin((10.0 * t - 0.75) * c4) + 1.0;
        }
        case 27u: {
            if (t == 0.0) { return 0.0; }
            if (t == 1.0) { return 1.0; }
            let c5 = (2.0 * pi) / 4.5;
            if (t < 0.5) {
                return -(pow(2.0, 20.0 * t - 10.0) * sin((20.0 * t - 11.125) * c5)) / 2.0;
            }
            return (pow(2.0, -20.0 * t + 10.0) * sin((20.0 * t - 11.125) * c5)) / 2.0 + 1.0;
        }
        case 28u: { return 1.0 - ease_out_bounce(1.0 - t); }
        case 29u: { return ease_out_bounce(t); }
        case 30u: {
            if (t < 0.5) { return (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0; }
            return (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0;
        }
        default: { return t; }
    }
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    if (i >= arrayLength(&values)) {
        return;
    }

    let tween = tweens[i];
    let raw = select(1.0, clamp(tween.elapsed / tween.duration, 0.0, 1.0), tween.duration > 0.0);
    let curved = easing_apply(tween.easing_id, raw);
    values[i] = tween.start + (tween.end - tween.start) * curved;
}
