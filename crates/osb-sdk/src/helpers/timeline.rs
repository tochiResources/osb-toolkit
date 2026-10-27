use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{call, Channel};
use crate::error::Error;

/// A keyframe channel placed at `start` on a timeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineClip {
    /// The clip's start time in milliseconds.
    pub start: f64,
    /// The clip's keyframe channel.
    pub channel: Channel,
}

/// An ordered set of keyframe clips and nested child timelines (a group).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Timeline {
    /// The clips.
    pub clips: Vec<TimelineClip>,
    /// Nested sub-timelines; a parent operation cascades into them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Timeline>,
}

fn run(name: &str, input: Value) -> Result<Timeline, Error> {
    Ok(serde_json::from_value(call(name, input)?)?)
}

/// The timeline's total duration.
pub fn duration(timeline: &Timeline) -> Result<f64, Error> {
    Ok(serde_json::from_value(call(
        "timeline.durationOf",
        json!({ "timeline": timeline }),
    )?)?)
}

/// The timeline's occupied `[start, end]` range, if any.
pub fn time_range(timeline: &Timeline) -> Result<Option<(f64, f64)>, Error> {
    let value: Value = call("timeline.timeRangeOf", json!({ "timeline": timeline }))?;
    if value.is_null() {
        return Ok(None);
    }
    Ok(Some((
        value.get("start").and_then(Value::as_f64).unwrap_or(0.0),
        value.get("end").and_then(Value::as_f64).unwrap_or(0.0),
    )))
}

/// Moves the timeline in time.
pub fn shift(timeline: &Timeline, delta: f64) -> Result<Timeline, Error> {
    run(
        "timeline.shiftTimeline",
        json!({ "timeline": timeline, "delta": delta }),
    )
}

/// Scales the timeline in time about `anchor`.
pub fn scale(timeline: &Timeline, factor: f64, anchor: f64) -> Result<Timeline, Error> {
    run(
        "timeline.scaleTimeline",
        json!({ "timeline": timeline, "factor": factor, "anchor": anchor }),
    )
}

/// Reverses the timeline in time.
pub fn reverse(timeline: &Timeline, start: f64, end: f64) -> Result<Timeline, Error> {
    run(
        "timeline.reverseTimeline",
        json!({ "timeline": timeline, "start": start, "end": end }),
    )
}

/// Tiles the timeline `count` times.
pub fn repeat(timeline: &Timeline, count: usize, ping_pong: bool) -> Result<Timeline, Error> {
    run(
        "timeline.repeatTimeline",
        json!({ "timeline": timeline, "count": count, "pingPong": ping_pong }),
    )
}

/// Offsets each clip by `index * step`.
pub fn stagger(timeline: &Timeline, step: f64) -> Result<Timeline, Error> {
    run(
        "timeline.staggerTimeline",
        json!({ "timeline": timeline, "step": step }),
    )
}

/// A sub-timeline over `[start, end]`, rebased to `0`.
pub fn slice(timeline: &Timeline, start: f64, end: f64) -> Result<Timeline, Error> {
    run(
        "timeline.sliceTimeline",
        json!({ "timeline": timeline, "start": start, "end": end }),
    )
}

/// Resolves the timeline to absolute-time clips.
pub fn flush(timeline: &Timeline) -> Result<Timeline, Error> {
    run("timeline.flushTimeline", json!({ "timeline": timeline }))
}

/// Overlays another timeline's clips.
pub fn merge(a: &Timeline, b: &Timeline) -> Result<Timeline, Error> {
    run(
        "timeline.mergeTimelines",
        json!({ "timeline": a, "other": b }),
    )
}

/// Adds a clip to the timeline.
pub fn add(timeline: &Timeline, channel: Channel, start: f64) -> Result<Timeline, Error> {
    run(
        "timeline.addToTimeline",
        json!({ "timeline": timeline, "channel": channel, "start": start }),
    )
}

/// Adds a nested child timeline (a group).
pub fn group(timeline: &Timeline, child: Timeline) -> Result<Timeline, Error> {
    run(
        "timeline.addGroup",
        json!({ "timeline": timeline, "child": child }),
    )
}

/// Every clip in the tree, in the shared time coordinate.
pub fn all_clips(timeline: &Timeline) -> Result<Vec<TimelineClip>, Error> {
    Ok(serde_json::from_value(call(
        "timeline.allClips",
        json!({ "timeline": timeline }),
    )?)?)
}

/// The total clip count across the tree.
pub fn clip_count(timeline: &Timeline) -> Result<usize, Error> {
    Ok(serde_json::from_value(call(
        "timeline.clipCount",
        json!({ "timeline": timeline }),
    )?)?)
}
