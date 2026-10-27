use serde_json::{json, Value};

use super::call;
use crate::error::Error;
use crate::models::Easing;

/// Applies `easing` to `t`, remapped onto `[from, to]` (stretch over a timeframe).
pub fn stretch(easing: Easing, t: f64, from: f64, to: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.stretchEasing",
        json!({ "easing": easing, "t": t, "from": from, "to": to }),
    )?)?)
}

/// Amplifies an easing about linear by `amount` (`>1` stronger).
pub fn amplify(easing: Easing, t: f64, amount: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.amplifyEasing",
        json!({ "easing": easing, "t": t, "amount": amount }),
    )?)?)
}

/// Time-reverses an easing.
pub fn reverse(easing: Easing, t: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.reverseEasing",
        json!({ "easing": easing, "t": t }),
    )?)?)
}

/// Mirrors an easing about `t = 0.5`.
pub fn mirror(easing: Easing, t: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.mirrorEasing",
        json!({ "easing": easing, "t": t }),
    )?)?)
}

/// Ping-pongs an easing.
pub fn ping_pong(easing: Easing, t: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.pingPongEasing",
        json!({ "easing": easing, "t": t }),
    )?)?)
}

/// Tiles an easing `count` times.
pub fn repeat(easing: Easing, t: f64, count: usize) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.repeatEasing",
        json!({ "easing": easing, "t": t, "count": count }),
    )?)?)
}

/// Composes two easings: `outer(inner(t))`.
pub fn compose(outer: Easing, inner: Easing, t: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.composeEasing",
        json!({ "outer": outer, "inner": inner, "t": t }),
    )?)?)
}

/// Blends two easings by `amount`.
pub fn blend(a: Easing, b: Easing, t: f64, amount: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.blendEasing",
        json!({ "a": a, "b": b, "t": t, "amount": amount }),
    )?)?)
}

/// Evaluates a cubic-bezier easing.
pub fn bezier(x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.bezierValue",
        json!({ "x1": x1, "y1": y1, "x2": x2, "y2": y2, "t": t }),
    )?)?)
}

/// Amplifies a cubic-bezier easing about linear.
pub fn amplify_bezier(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    t: f64,
    amount: f64,
) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.amplifyBezier",
        json!({ "x1": x1, "y1": y1, "x2": x2, "y2": y2, "t": t, "amount": amount }),
    )?)?)
}

/// Samples an easing at `count` points in `[0, 1]`.
pub fn sample(easing: Easing, count: usize) -> Result<Vec<f64>, Error> {
    Ok(serde_json::from_value(call(
        "easing.sampleEasing",
        json!({ "easing": easing, "count": count }),
    )?)?)
}

/// Evaluates a piecewise easing remap.
pub fn remap(segments: Value, t: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.remapEasing",
        json!({ "segments": segments, "t": t }),
    )?)?)
}

/// A stagger amount for element `index` (`index * step`).
pub fn stagger(index: usize, step: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "easing.staggerEasing",
        json!({ "index": index, "step": step }),
    )?)?)
}
