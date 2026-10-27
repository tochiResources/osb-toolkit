//! The shared motion-graphics helpers (M38/M40): one surface for 2D, 3D and color.
//!
//! The canonical logic lives in Rust (`osb_runtime::animations::keyframes` and
//! `osb_runtime::animations::color_ops` for keyframes/colors,
//! `osb_runtime::storyboarding::helpers` for command bridging,
//! `osb_runtime::storyboarding::origin` for origins, and
//! `osb_runtime::storyboarding::scene` for the 2D scene graph). These facades
//! drive the single `helper` bridge op, so a caller gets the same behaviour in
//! every language.
//!
//! The surface is code-split by library into [`keyframe`], [`scene3d`],
//! [`origin`], [`easing`], [`timeline`], [`color`] and [`scene2d`]; this module
//! holds the shared [`Channel`] type and the raw [`call`] entry point.
//!
//! Because a [`Channel`] is dimension-agnostic, **the Z axis is optional**: pass
//! a scalar or 2-vector channel for 2D, a 3-vector channel for 3D, or a
//! 4-component channel for a color. The keyframe transforms accept **either** a
//! channel or a 2D command channel (`transform_commands`), so 2D commands use
//! exactly the same helpers as 3D nodes.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::Error;
use crate::models::{HelperResult, StoryboardCommand, StoryboardCommandKind};

pub mod color;
pub mod easing;
pub mod keyframe;
pub mod origin;
pub mod scene2d;
pub mod scene3d;
pub mod timeline;

/// A single keyframe: a time, a value of one, two, three or four components, and
/// the easing that governs the segment from this keyframe to the next.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChannelKey {
    /// The keyframe time in milliseconds.
    pub time: f64,
    /// The value components (1, 2, 3 or 4).
    pub value: Vec<f64>,
    /// The outgoing segment's easing name (omitted means linear).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub easing: Option<String>,
}

/// A keyframed channel: a list of `(time, value)` samples. The component count
/// decides whether it is a 2D (1-2), 3D (3) or color (4) channel; the Z axis is
/// always optional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    /// The channel's keyframes.
    pub keys: Vec<ChannelKey>,
}

impl Channel {
    /// Builds a channel from keyframes.
    pub fn new(keys: Vec<ChannelKey>) -> Self {
        Channel { keys }
    }
}

/// Calls any shared helper by name with a raw input object.
pub fn call(name: &str, input: Value) -> Result<Value, Error> {
    let result: HelperResult = crate::client::call(json!({
        "op": "Helpers.call",
        "name": name,
        "input": input,
    }))?;
    Ok(result.value.unwrap_or(Value::Null))
}

pub(crate) fn channel_call(name: &str, input: Value) -> Result<Channel, Error> {
    Ok(serde_json::from_value(call(name, input)?)?)
}

/// Bridges any keyframe transform onto a 2D command channel: extract `kind`'s
/// channel, apply the named helper with `options`, and write it back as commands.
pub fn transform_commands(
    commands: Vec<StoryboardCommand>,
    kind: StoryboardCommandKind,
    name: &str,
    options: Value,
) -> Result<Vec<StoryboardCommand>, Error> {
    let mut input = options;
    if !input.is_object() {
        input = json!({});
    }
    let object = input.as_object_mut().expect("object");
    object.insert("commands".to_string(), serde_json::to_value(commands)?);
    object.insert("kind".to_string(), serde_json::to_value(kind)?);
    Ok(serde_json::from_value(call(name, input)?)?)
}
