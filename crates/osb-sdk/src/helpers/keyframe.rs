//! The shared keyframe channel transforms (2D, 3D and color).

use serde_json::{json, Value};

use super::{call, channel_call, Channel};
use crate::error::Error;
use crate::models::Vector3;

/// Collapses an axis (or every axis) to a constant.
pub fn flatten(channel: Channel, axis: Option<&str>, mode: &str) -> Result<Channel, Error> {
    channel_call(
        "keyframe.flatten",
        json!({ "channel": channel, "axis": axis, "mode": mode }),
    )
}

/// Multiplies every component by `factor`.
pub fn scale_keyframes(channel: Channel, factor: f64) -> Result<Channel, Error> {
    channel_call(
        "keyframe.scaleKeyframes",
        json!({ "channel": channel, "factor": factor }),
    )
}

/// Adds a constant `delta` to every component.
pub fn offset_keyframes(channel: Channel, delta: f64) -> Result<Channel, Error> {
    channel_call(
        "keyframe.offsetKeyframes",
        json!({ "channel": channel, "delta": delta }),
    )
}

/// Reflects an axis (or every axis) about `pivot`.
pub fn mirror(channel: Channel, axis: Option<&str>, pivot: f64) -> Result<Channel, Error> {
    channel_call(
        "keyframe.mirror",
        json!({ "channel": channel, "axis": axis, "pivot": pivot }),
    )
}

/// Seeded jitter in `min..max`, preserving keyframe times.
pub fn randomize(
    channel: Channel,
    axis: Option<&str>,
    min: f64,
    max: f64,
    seed: u64,
    step: f64,
) -> Result<Channel, Error> {
    channel_call(
        "keyframe.randomize",
        json!({ "channel": channel, "axis": axis, "min": min, "max": max, "seed": seed, "step": step }),
    )
}

/// Reverses the channel in time about `[start, end]`.
pub fn reverse_keyframes(channel: Channel, start: f64, end: f64) -> Result<Channel, Error> {
    channel_call(
        "keyframe.reverseKeyframes",
        json!({ "channel": channel, "start": start, "end": end }),
    )
}

/// Tiles the `[start, end]` span `count` times.
pub fn repeat_keyframes(
    channel: Channel,
    count: usize,
    start: f64,
    end: f64,
    ping_pong: bool,
) -> Result<Channel, Error> {
    channel_call(
        "keyframe.repeatKeyframes",
        json!({ "channel": channel, "count": count, "start": start, "end": end, "pingPong": ping_pong }),
    )
}

/// Resamples to a fixed `step` (or to exactly `count` samples).
pub fn resample_keyframes(channel: Channel, step: f64, count: usize) -> Result<Channel, Error> {
    channel_call(
        "keyframe.resampleKeyframes",
        json!({ "channel": channel, "step": step, "count": count }),
    )
}

/// Moving-average smoothing over a window of neighbouring keyframes.
pub fn smooth_keyframes(channel: Channel, window: usize) -> Result<Channel, Error> {
    channel_call(
        "keyframe.smoothKeyframes",
        json!({ "channel": channel, "window": window }),
    )
}

/// Douglas-Peucker simplification of each component.
pub fn simplify_keyframes(
    channel: Channel,
    tolerance: f64,
    decimals: u32,
) -> Result<Channel, Error> {
    channel_call(
        "keyframe.simplifyKeyframes",
        json!({ "channel": channel, "tolerance": tolerance, "decimals": decimals }),
    )
}

/// Snaps keyframe times to a beat grid.
pub fn align_keyframes(
    channel: Channel,
    beat: f64,
    offset: f64,
    subdivision: u32,
) -> Result<Channel, Error> {
    channel_call(
        "keyframe.alignKeyframes",
        json!({ "channel": channel, "beat": beat, "offset": offset, "subdivision": subdivision }),
    )
}

/// Re-eases each span (named easing).
pub fn ease_keyframes(channel: Channel, easing: &str, samples: usize) -> Result<Channel, Error> {
    channel_call(
        "keyframe.easeKeyframes",
        json!({ "channel": channel, "easing": easing, "samples": samples }),
    )
}

/// Blends the channel toward `other` by `amount` (`0..=1`).
pub fn merge_keyframes(channel: Channel, other: Channel, amount: f64) -> Result<Channel, Error> {
    channel_call(
        "keyframe.mergeKeyframes",
        json!({ "channel": channel, "other": other, "amount": amount }),
    )
}

/// A stagger delay for element `index`: `index * step`.
pub fn stagger(index: usize, step: f64) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "keyframe.stagger",
        json!({ "index": index, "step": step }),
    )?)?)
}

/// A pulse oscillation channel (2D or 3D, scalar).
pub fn pulse(
    base: f64,
    amplitude: f64,
    frequency: f64,
    phase: f64,
    step: f64,
    duration: f64,
) -> Result<Channel, Error> {
    channel_call(
        "keyframe.pulse",
        json!({ "base": base, "amplitude": amplitude, "frequency": frequency,
                "phase": phase, "step": step, "duration": duration }),
    )
}

/// Seeded random 3D placements in a box.
pub fn scatter(
    seed: u64,
    count: usize,
    min: [f64; 3],
    max: [f64; 3],
    spread: f64,
) -> Result<Vec<Vector3>, Error> {
    Ok(serde_json::from_value(call(
        "keyframe.scatter",
        json!({ "seed": seed, "count": count, "min": min, "max": max, "spread": spread }),
    )?)?)
}

/// Seeded random 2D placements in a box (`z = 0`).
pub fn scatter_2d(
    seed: u64,
    count: usize,
    min: [f64; 2],
    max: [f64; 2],
    spread: f64,
) -> Result<Vec<Vector3>, Error> {
    scatter(
        seed,
        count,
        [min[0], min[1], 0.0],
        [max[0], max[1], 0.0],
        spread,
    )
}

/// Returns the raw helper result as JSON (escape hatch for any helper).
pub fn raw(name: &str, input: Value) -> Result<Value, Error> {
    call(name, input)
}
