//! The shared motion-graphics `helper` bridge.
//!
//! One op, `helper`, dispatches by a namespaced `name` to the canonical Rust
//! libraries:
//!
//! - `keyframe.*` - the shared keyframe transforms in
//!   `osb_runtime::animations::keyframes`. They accept **either** a `channel`
//!   (`{ keys: [{ time, value: [...] }] }`, 1/2/3 components, so Z is optional)
//!   **or** a 2D command set (`{ commands, kind }`), which is bridged to a
//!   channel and back - so the same helpers work for 2D commands and 3D nodes.
//! - `scene3d.*` - the strictly 3D constructors (`orbit`/`spin`/`tumble`/`pathFollow`).
//! - `command.*` - the 2D command-list utilities.
//! - `origin.*` - the shared origin/anchor helpers.
//!
//! The legacy `storyboarding3dHelper`/`storyboardHelper` ops are aliases that
//! prefix an unprefixed name for the matching library.

use osb_api::helper::{HelperRequest, HelperResult};
use osb_api::storyboard::{
    CommandValue as ApiCommandValue, StoryboardCommand, StoryboardCommandKind,
};
use osb_runtime::animations::easing_ops;
use osb_runtime::animations::keyframes::Channel;
use osb_runtime::animations::timeline::{Timeline, TimelineClip};
use osb_runtime::animations::{color_ops, ColorSpace, ColorStop, Easing, KeyframedValue};
use osb_runtime::context::Rng;
use osb_runtime::math::{Color4, Vec2, Vec3, Vec4};
use osb_runtime::storyboarding::helpers as sb;
use osb_runtime::storyboarding::{
    Camera2d, Command, CommandType, CommandValue, Node2d, Object2d, ParameterTypeValue,
    ResolvedSprite2d, Scene2d, Scene2dOptions, Sprite2d, TransformTimeline2d,
};
use osb_runtime::storyboarding3d::helpers as h3;
use osb_runtime::storyboarding3d::helpers::{Axis, FlattenMode};
use serde_json::{json, Value};

use crate::{error_json, to_json};

/// Runs a shared `helper` request.
pub fn helper(value: &Value) -> String {
    let request: HelperRequest = match serde_json::from_value(value.clone()) {
        Ok(request) => request,
        Err(error) => return error_json(&error.to_string()),
    };
    match dispatch(&normalize(&request.name), &request.input) {
        Ok(result) => to_json(&HelperResult { value: result }),
        Err(message) => error_json(&message),
    }
}

/// The legacy `storyboarding3dHelper` op: an unprefixed name is treated as a
/// `keyframe.*`/`scene3d.*` helper.
pub fn storyboarding3d_helper(value: &Value) -> String {
    helper(value)
}

/// The legacy `storyboardHelper` op: an unprefixed name is treated as a
/// `command.*` helper.
pub fn storyboard_helper(value: &Value) -> String {
    let request: HelperRequest = match serde_json::from_value(value.clone()) {
        Ok(request) => request,
        Err(error) => return error_json(&error.to_string()),
    };
    let name = if request.name.contains('.') {
        request.name.clone()
    } else {
        format!("command.{}", request.name)
    };
    match dispatch(&name, &request.input) {
        Ok(result) => to_json(&HelperResult { value: result }),
        Err(message) => error_json(&message),
    }
}

/// Adds the library prefix to an unprefixed (legacy) helper name.
fn normalize(name: &str) -> String {
    if name.contains('.') {
        return name.to_string();
    }
    const SCENE3D: [&str; 4] = ["orbit", "spin", "tumble", "pathFollow"];
    const COMMAND: [&str; 14] = [
        "selectCommands",
        "shiftCommands",
        "scaleCommands",
        "reverseCommands",
        "mergeCommands",
        "splitCommands",
        "flattenCommands",
        "randomizeCommands",
        "filterCommands",
        "removeCommands",
        "mapCommands",
        "duplicateCommands",
        "alignCommands",
        "quantizeCommands",
    ];
    if SCENE3D.contains(&name) {
        format!("scene3d.{name}")
    } else if COMMAND.contains(&name) {
        format!("command.{name}")
    } else {
        format!("keyframe.{name}")
    }
}

fn dispatch(name: &str, input: &Value) -> Result<Value, String> {
    let (library, function) = name
        .split_once('.')
        .ok_or_else(|| format!("helper name needs a `library.function` form: {name}"))?;
    match library {
        "keyframe" => dispatch_keyframe(function, input),
        "scene3d" => dispatch_scene3d(function, input),
        "command" => dispatch_command(function, input),
        "origin" => dispatch_origin(function, input),
        "easing" => dispatch_easing(function, input),
        "timeline" => dispatch_timeline(function, input),
        "color" => dispatch_color(function, input),
        "scene2d" => dispatch_scene2d(function, input),
        other => Err(format!("unknown helper library: {other}")),
    }
}

// -- argument helpers -------------------------------------------------------

fn number(input: &Value, key: &str, default: f64) -> f64 {
    input.get(key).and_then(Value::as_f64).unwrap_or(default)
}

fn opt_number(input: &Value, key: &str) -> Option<f64> {
    input.get(key).and_then(Value::as_f64)
}

fn usize_field(input: &Value, key: &str, default: usize) -> usize {
    input
        .get(key)
        .and_then(Value::as_u64)
        .unwrap_or(default as u64) as usize
}

fn bool_field(input: &Value, key: &str, default: bool) -> bool {
    input.get(key).and_then(Value::as_bool).unwrap_or(default)
}

fn axis_field(input: &Value) -> Option<Axis> {
    input
        .get("axis")
        .and_then(Value::as_str)
        .and_then(Axis::parse)
}

fn flatten_mode(input: &Value) -> FlattenMode {
    match input.get("mode").and_then(Value::as_str).unwrap_or("") {
        "first" => FlattenMode::First,
        "last" => FlattenMode::Last,
        "value" => FlattenMode::Value(number(input, "value", 0.0)),
        _ => FlattenMode::Mean,
    }
}

fn vec3_field(input: &Value, key: &str, default: Vec3) -> Vec3 {
    let Some(values) = input.get(key).and_then(Value::as_array) else {
        return default;
    };
    let component = |index: usize, fallback: f64| {
        values
            .get(index)
            .and_then(Value::as_f64)
            .unwrap_or(fallback)
    };
    Vec3::new(
        component(0, default.x),
        component(1, default.y),
        component(2, default.z),
    )
}

// -- channel <-> JSON -------------------------------------------------------

fn parse_channel(value: &Value) -> Result<Channel, String> {
    let keys = value
        .get("keys")
        .and_then(Value::as_array)
        .ok_or_else(|| "a channel needs a `keys` array".to_string())?;
    let dimension = keys
        .first()
        .and_then(|key| key.get("value"))
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(1);
    let component = |key: &Value, index: usize| {
        key.get("value")
            .and_then(Value::as_array)
            .and_then(|values| values.get(index))
            .and_then(Value::as_f64)
            .unwrap_or(0.0)
    };
    let easing = |key: &Value| -> Easing {
        crate::json::enum_field::<osb_api::Easing>(key, "easing").unwrap_or(Easing::None)
    };
    match dimension {
        2 => {
            let mut kv = KeyframedValue::new();
            for key in keys {
                let time = key.get("time").and_then(Value::as_f64).unwrap_or(0.0);
                kv.add_eased(
                    time,
                    Vec2::new(component(key, 0), component(key, 1)),
                    easing(key),
                );
            }
            Ok(Channel::Vector2(kv))
        }
        3 => {
            let mut kv = KeyframedValue::new();
            for key in keys {
                let time = key.get("time").and_then(Value::as_f64).unwrap_or(0.0);
                kv.add_eased(
                    time,
                    Vec3::new(component(key, 0), component(key, 1), component(key, 2)),
                    easing(key),
                );
            }
            Ok(Channel::Vector3(kv))
        }
        4 => {
            let mut kv = KeyframedValue::new();
            for key in keys {
                let time = key.get("time").and_then(Value::as_f64).unwrap_or(0.0);
                kv.add_eased(
                    time,
                    Color4::new(
                        component(key, 0),
                        component(key, 1),
                        component(key, 2),
                        component(key, 3),
                    ),
                    easing(key),
                );
            }
            Ok(Channel::Color(kv))
        }
        _ => {
            let mut kv = KeyframedValue::new();
            for key in keys {
                let time = key.get("time").and_then(Value::as_f64).unwrap_or(0.0);
                kv.add_eased(time, component(key, 0), easing(key));
            }
            Ok(Channel::Scalar(kv))
        }
    }
}

/// A keyframe JSON object, including its easing only when it is not linear.
fn channel_key_json(time: f64, value: Value, easing: Easing) -> Value {
    if easing == Easing::None {
        json!({ "time": time, "value": value })
    } else {
        json!({ "time": time, "value": value, "easing": easing })
    }
}

fn channel_json(channel: &Channel) -> Value {
    let keys: Vec<Value> = match channel {
        Channel::Scalar(kv) => kv
            .keyframes()
            .iter()
            .map(|key| channel_key_json(key.time, json!([key.value]), key.easing))
            .collect(),
        Channel::Vector2(kv) => kv
            .keyframes()
            .iter()
            .map(|key| channel_key_json(key.time, json!([key.value.x, key.value.y]), key.easing))
            .collect(),
        Channel::Vector3(kv) => kv
            .keyframes()
            .iter()
            .map(|key| {
                channel_key_json(
                    key.time,
                    json!([key.value.x, key.value.y, key.value.z]),
                    key.easing,
                )
            })
            .collect(),
        Channel::Color(kv) => kv
            .keyframes()
            .iter()
            .map(|key| {
                channel_key_json(
                    key.time,
                    json!([key.value.r, key.value.g, key.value.b, key.value.a]),
                    key.easing,
                )
            })
            .collect(),
    };
    json!({ "keys": keys })
}

/// The shape a keyframe helper's input (and output) takes.
enum KeyframeTarget {
    Channel,
    Commands(CommandType),
}

fn read_keyframe_target(input: &Value) -> Result<(Channel, KeyframeTarget), String> {
    if input.get("commands").is_some() {
        let commands = parse_commands(input)?;
        let kind = kind_field(input)
            .ok_or_else(|| "`kind` is required when transforming a command channel".to_string())?;
        let channel = sb::command_channel(&commands, kind).ok_or_else(|| {
            "no commands of that kind, or the kind is not a supported keyframe channel".to_string()
        })?;
        Ok((channel, KeyframeTarget::Commands(kind)))
    } else {
        let channel = input
            .get("channel")
            .ok_or_else(|| "this helper needs a `channel` or `commands`".to_string())?;
        Ok((parse_channel(channel)?, KeyframeTarget::Channel))
    }
}

fn write_keyframe_target(
    input: &Value,
    target: KeyframeTarget,
    channel: &Channel,
) -> Result<Value, String> {
    match target {
        KeyframeTarget::Channel => Ok(channel_json(channel)),
        KeyframeTarget::Commands(kind) => {
            let commands = parse_commands(input)?;
            // Easing preservation: the resolved segments keep the input easings.
            let easings = sb::command_easings(&commands, kind);
            Ok(commands_json(&sb::apply_command_channel_eased(
                &commands, kind, channel, &easings,
            )))
        }
    }
}

// -- keyframe dispatch ------------------------------------------------------

fn dispatch_keyframe(name: &str, input: &Value) -> Result<Value, String> {
    // Constructors that do not consume a channel.
    match name {
        "stagger" => {
            return Ok(json!(h3::stagger(
                usize_field(input, "index", 0),
                number(input, "step", 0.0)
            )))
        }
        "pulse" => {
            return Ok(channel_json(&h3::pulse(
                number(input, "base", 1.0),
                number(input, "amplitude", 0.5),
                number(input, "frequency", 1.0),
                number(input, "phase", 0.0),
                number(input, "step", 0.0),
                number(input, "duration", 1000.0),
            )))
        }
        "scatter" => {
            let seed = input.get("seed").and_then(Value::as_u64).unwrap_or(0);
            let points = h3::scatter(
                seed,
                usize_field(input, "count", 8),
                vec3_field(input, "min", Vec3::new(-100.0, -100.0, 0.0)),
                vec3_field(input, "max", Vec3::new(100.0, 100.0, 0.0)),
                number(input, "spread", 0.0),
            );
            return Ok(Value::Array(
                points
                    .into_iter()
                    .map(|p| json!({ "x": p.x, "y": p.y, "z": p.z }))
                    .collect(),
            ));
        }
        _ => {}
    }

    let (channel, target) = read_keyframe_target(input)?;
    let transformed = match name {
        "flatten" => channel.flatten(axis_field(input), flatten_mode(input)),
        "scaleKeyframes" => channel.scale(number(input, "factor", 1.0)),
        "offsetKeyframes" => {
            if let Some(delta) = input.get("deltaChannel") {
                let delta = parse_channel(delta)?;
                let delta = delta.component(Axis::X).unwrap_or_default();
                channel.offset_by(&delta)
            } else {
                channel.offset(number(input, "delta", 0.0))
            }
        }
        "mirror" => channel.mirror(axis_field(input), number(input, "pivot", 0.0)),
        "randomize" => {
            let seed = input.get("seed").and_then(Value::as_u64).unwrap_or(0);
            channel.randomize(
                axis_field(input),
                number(input, "min", -1.0),
                number(input, "max", 1.0),
                seed,
                number(input, "step", 0.0),
            )
        }
        "reverseKeyframes" => {
            let (start, end) = channel
                .time_range()
                .ok_or_else(|| "the channel is empty".to_string())?;
            channel.reverse(
                opt_number(input, "start").unwrap_or(start),
                opt_number(input, "end").unwrap_or(end),
            )
        }
        "repeatKeyframes" | "loopKeyframes" => channel.repeat(
            usize_field(input, "count", 1),
            bool_field(input, "pingPong", false),
            number(input, "start", 0.0),
            number(input, "end", 0.0),
        ),
        "resampleKeyframes" => {
            channel.resample(number(input, "step", 0.0), usize_field(input, "count", 0))
        }
        "smoothKeyframes" => channel.smooth(usize_field(input, "window", 3)),
        "simplifyKeyframes" => channel.simplify(
            number(input, "tolerance", 0.1),
            usize_field(input, "decimals", 3) as u32,
        ),
        "alignKeyframes" => channel.align(
            number(input, "beat", 500.0),
            number(input, "offset", 0.0),
            usize_field(input, "subdivision", 1) as u32,
        ),
        "easeKeyframes" => {
            channel.ease(crate::json::easing(input), usize_field(input, "samples", 8))
        }
        "mergeKeyframes" => {
            let other = input
                .get("other")
                .ok_or_else(|| "mergeKeyframes needs `other`".to_string())?;
            channel.merge(&parse_channel(other)?, number(input, "amount", 0.5))
        }
        other => return Err(format!("unknown keyframe helper: {other}")),
    };
    write_keyframe_target(input, target, &transformed)
}

// -- scene3d dispatch -------------------------------------------------------

fn dispatch_scene3d(name: &str, input: &Value) -> Result<Value, String> {
    Ok(match name {
        "orbit" => channel_json(&h3::orbit(
            vec3_field(input, "center", Vec3::ZERO),
            number(input, "radiusX", number(input, "radius", 100.0)),
            number(input, "radiusY", number(input, "radius", 100.0)),
            number(input, "startAngle", 0.0),
            number(input, "sweep", std::f64::consts::TAU),
            usize_field(input, "count", 32),
        )),
        "spin" => channel_json(&h3::spin(
            axis_field(input).unwrap_or(Axis::Z),
            number(input, "turns", 1.0),
            number(input, "duration", 1000.0),
            usize_field(input, "count", 16),
        )),
        "tumble" => channel_json(&h3::tumble(
            vec3_field(input, "turns", Vec3::ONE),
            number(input, "duration", 1000.0),
            usize_field(input, "count", 16),
        )),
        "pathFollow" => {
            let points: Vec<Vec3> = input
                .get("points")
                .and_then(Value::as_array)
                .map(|list| {
                    list.iter()
                        .map(|p| {
                            let values = p.as_array().cloned().unwrap_or_default();
                            let at =
                                |i: usize| values.get(i).and_then(Value::as_f64).unwrap_or(0.0);
                            Vec3::new(at(0), at(1), at(2))
                        })
                        .collect()
                })
                .unwrap_or_default();
            channel_json(&h3::path_follow(
                &points,
                usize_field(input, "count", 16),
                bool_field(input, "byArcLength", false),
            ))
        }
        "bakeWorld" => {
            let world_value = input.get("world").unwrap_or(input);
            let bake_input: osb_runtime::storyboarding3d::input::Bake3dInput =
                serde_json::from_value(world_value.clone()).map_err(|error| error.to_string())?;
            let (root, camera, options) =
                osb_runtime::storyboarding3d::input::build_scene(&bake_input);
            let world = osb_runtime::storyboarding3d::World3d {
                root,
                camera,
                ..Default::default()
            };
            let options2d = scene2d_options_from(input.get("options2d").unwrap_or(&Value::Null));
            storyboard_json(&world.bake(&options, &options2d))
        }
        "bakeScene" => {
            let (scene, options2d, options3d) = build_storyboard_scene(input)?;
            storyboard_json(&scene.bake(&options2d, &options3d))
        }
        other => return Err(format!("unknown scene3d helper: {other}")),
    })
}

/// Builds a [`StoryboardScene`](osb_runtime::storyboarding3d::StoryboardScene) from
/// a `{ worlds2d, worlds3d, options, options2d }` request, with its 2D/3D bake
/// options. Shared by the `scene3d.bakeScene` helper and the `bakeScene` runtime op.
pub(crate) fn build_storyboard_scene(
    input: &Value,
) -> Result<
    (
        osb_runtime::storyboarding3d::StoryboardScene,
        Scene2dOptions,
        osb_runtime::storyboarding3d::BakeOptions,
    ),
    String,
> {
    let options2d = scene2d_options_from(input.get("options2d").unwrap_or(&Value::Null));
    let options3d = input
        .get("options")
        .map(|options| {
            let bake_input: osb_runtime::storyboarding3d::input::Bake3dInput =
                serde_json::from_value(json!({ "options": options })).unwrap_or_default();
            osb_runtime::storyboarding3d::input::build_scene(&bake_input).2
        })
        .unwrap_or_default();
    let mut scene = osb_runtime::storyboarding3d::StoryboardScene::new();
    if let Some(worlds) = input.get("worlds2d").and_then(Value::as_array) {
        for world in worlds {
            scene.worlds2d.push(parse_scene2d(world)?);
        }
    }
    if let Some(worlds) = input.get("worlds3d").and_then(Value::as_array) {
        for world in worlds {
            let bake_input: osb_runtime::storyboarding3d::input::Bake3dInput =
                serde_json::from_value(world.clone()).map_err(|error| error.to_string())?;
            let (root, camera, _) = osb_runtime::storyboarding3d::input::build_scene(&bake_input);
            scene.worlds3d.push(osb_runtime::storyboarding3d::World3d {
                root,
                camera,
                ..Default::default()
            });
        }
    }
    Ok((scene, options2d, options3d))
}

// -- origin dispatch --------------------------------------------------------

fn dispatch_origin(name: &str, input: &Value) -> Result<Value, String> {
    let point = |key: &str| {
        let value = input.get(key);
        Vec2::new(
            value
                .and_then(|v| v.get("x"))
                .and_then(Value::as_f64)
                .unwrap_or(0.0),
            value
                .and_then(|v| v.get("y"))
                .and_then(Value::as_f64)
                .unwrap_or(0.0),
        )
    };
    if name == "inScreenBounds" {
        // `origin` here is the resolved anchor vector, not the enum.
        return Ok(json!(osb_runtime::storyboarding::in_screen_bounds(
            point("position"),
            point("size"),
            number(input, "rotation", 0.0),
            point("origin"),
        )));
    }
    let api_origin: osb_api::OsbOrigin = input
        .get("origin")
        .cloned()
        .map(serde_json::from_value)
        .transpose()
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "an origin id/name is required".to_string())?;
    let origin: osb_runtime::storyboarding::OsbOrigin = api_origin;
    let width = number(input, "width", 0.0);
    let height = number(input, "height", 0.0);
    if name == "resolveAnchor3d" {
        // A `Vec3` anchor (`{x, y, z}`) whose `z` anchors the node's depth (z-index).
        let anchor_value = input.get("anchor");
        let component = |key: &str| {
            anchor_value
                .and_then(|value| value.get(key))
                .and_then(Value::as_f64)
                .unwrap_or(0.0)
        };
        let anchor = Vec3::new(component("x"), component("y"), component("z"));
        let vector = osb_runtime::storyboarding::resolve_anchor_3d(origin, anchor, width, height);
        return Ok(serde_json::to_value(vector).unwrap_or(Value::Null));
    }
    let vector = match name {
        "getOriginVector" => osb_runtime::storyboarding::origin_vector(origin, width, height),
        "resolveAnchor" => {
            osb_runtime::storyboarding::resolve_anchor(origin, point("anchor"), width, height)
        }
        other => return Err(format!("unknown origin helper: {other}")),
    };
    Ok(serde_json::to_value(vector).unwrap_or(Value::Null))
}

// -- color dispatch ---------------------------------------------------------

fn color_field(input: &Value, key: &str, default: Color4) -> Color4 {
    input.get(key).map(parse_color_value).unwrap_or(default)
}

/// Parses a color from `{ r, g, b, a }` or `[r, g, b, a]` (alpha defaults to 1).
fn parse_color_value(value: &Value) -> Color4 {
    if let Some(array) = value.as_array() {
        let at = |index: usize, fallback: f64| {
            array.get(index).and_then(Value::as_f64).unwrap_or(fallback)
        };
        return Color4::new(at(0, 0.0), at(1, 0.0), at(2, 0.0), at(3, 1.0));
    }
    let component =
        |key: &str, fallback: f64| value.get(key).and_then(Value::as_f64).unwrap_or(fallback);
    Color4::new(
        component("r", 0.0),
        component("g", 0.0),
        component("b", 0.0),
        component("a", 1.0),
    )
}

fn color_json(color: Color4) -> Value {
    json!({ "r": color.r, "g": color.g, "b": color.b, "a": color.a })
}

/// Parses a `Vec4` from `[x, y, z, w]` or `{ x, y, z, w }`.
fn parse_vec4_value(value: &Value, default: Vec4) -> Vec4 {
    if let Some(array) = value.as_array() {
        let at = |index: usize, fallback: f64| {
            array.get(index).and_then(Value::as_f64).unwrap_or(fallback)
        };
        return Vec4::new(
            at(0, default.x),
            at(1, default.y),
            at(2, default.z),
            at(3, default.w),
        );
    }
    let component =
        |key: &str, fallback: f64| value.get(key).and_then(Value::as_f64).unwrap_or(fallback);
    Vec4::new(
        component("x", default.x),
        component("y", default.y),
        component("z", default.z),
        component("w", default.w),
    )
}

fn color_space(input: &Value) -> ColorSpace {
    input
        .get("space")
        .and_then(Value::as_str)
        .and_then(ColorSpace::parse)
        .unwrap_or(ColorSpace::Rgb)
}

fn color_stops(input: &Value) -> Vec<ColorStop> {
    input
        .get("stops")
        .and_then(Value::as_array)
        .map(|list| {
            list.iter()
                .map(|stop| ColorStop {
                    offset: stop.get("offset").and_then(Value::as_f64).unwrap_or(0.0),
                    color: color_field(stop, "color", Color4::WHITE),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Applies a named CSS/Sass-style single-color helper.
fn apply_named_color(color: Color4, op: &str, input: &Value) -> Result<Color4, String> {
    Ok(match op {
        "lighten" => color_ops::lighten(color, number(input, "amount", 0.1)),
        "darken" => color_ops::darken(color, number(input, "amount", 0.1)),
        "saturate" => color_ops::saturate(color, number(input, "amount", 0.1)),
        "desaturate" => color_ops::desaturate(color, number(input, "amount", 0.1)),
        "adjustHue" | "rotateHue" => color_ops::adjust_hue(color, number(input, "degrees", 0.0)),
        "complement" => color_ops::complement(color),
        "grayscale" => color_ops::grayscale(color),
        "invert" => color_ops::invert(color),
        "mix" => color_ops::mix(
            color,
            color_field(input, "other", Color4::BLACK),
            number(input, "weight", 0.5),
        ),
        "tint" => color_ops::tint(color, number(input, "weight", 0.5)),
        "shade" => color_ops::shade(color, number(input, "weight", 0.5)),
        "opacify" | "fadeIn" => color_ops::opacify(color, number(input, "amount", 0.1)),
        "transparentize" | "fadeOut" => {
            color_ops::transparentize(color, number(input, "amount", 0.1))
        }
        "withAlpha" | "fade" => color_ops::with_alpha(color, number(input, "alpha", 1.0)),
        "setComponent" => color_ops::set_component(
            color,
            color_space(input),
            usize_field(input, "component", 0),
            number(input, "value", 0.0),
        ),
        other => return Err(format!("unknown color op: {other}")),
    })
}

fn dispatch_color(name: &str, input: &Value) -> Result<Value, String> {
    let space = color_space(input);

    // Channel-based color helpers operate on a color keyframe channel.
    if let Some(channel_value) = input.get("channel") {
        let channel = parse_channel(channel_value)?;
        let result = match name {
            "offsetChannel" => color_ops::offset_channel(
                &channel,
                space,
                parse_vec4_value(input.get("delta").unwrap_or(&Value::Null), Vec4::ZERO),
            ),
            "scaleChannel" => color_ops::scale_channel(
                &channel,
                space,
                parse_vec4_value(
                    input.get("factor").unwrap_or(&Value::Null),
                    Vec4::new(1.0, 1.0, 1.0, 1.0),
                ),
            ),
            "randomizeChannel" => {
                let seed = input.get("seed").and_then(Value::as_u64).unwrap_or(0);
                let min = parse_vec4_value(
                    input.get("min").unwrap_or(&Value::Null),
                    Vec4::new(-0.1, -0.1, -0.1, 0.0),
                );
                let max = parse_vec4_value(
                    input.get("max").unwrap_or(&Value::Null),
                    Vec4::new(0.1, 0.1, 0.1, 0.0),
                );
                color_ops::randomize_channel(&channel, space, seed, min, max)
            }
            "applyChannel" => {
                let op = input
                    .get("op")
                    .and_then(Value::as_str)
                    .ok_or_else(|| "applyChannel needs an `op`".to_string())?;
                color_ops::transform_channel(&channel, |color| {
                    apply_named_color(color, op, input).unwrap_or(color)
                })
            }
            other => return Err(format!("unknown color channel helper: {other}")),
        };
        return Ok(channel_json(&result));
    }

    // Value helpers.
    Ok(match name {
        "toSpace" => {
            let value = space.to_space(color_field(input, "color", Color4::WHITE));
            json!([value.x, value.y, value.z, value.w])
        }
        "fromSpace" => color_json(space.from_space(parse_vec4_value(
            input.get("value").unwrap_or(&Value::Null),
            Vec4::ZERO,
        ))),
        "offset" => color_json(color_ops::offset_color(
            color_field(input, "color", Color4::WHITE),
            space,
            parse_vec4_value(input.get("delta").unwrap_or(&Value::Null), Vec4::ZERO),
        )),
        "scale" => color_json(color_ops::scale_color(
            color_field(input, "color", Color4::WHITE),
            space,
            parse_vec4_value(
                input.get("factor").unwrap_or(&Value::Null),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
            ),
        )),
        "randomize" => {
            let seed = input.get("seed").and_then(Value::as_u64).unwrap_or(0);
            let mut rng = Rng::new(seed);
            let min = parse_vec4_value(
                input.get("min").unwrap_or(&Value::Null),
                Vec4::new(-0.1, -0.1, -0.1, 0.0),
            );
            let max = parse_vec4_value(
                input.get("max").unwrap_or(&Value::Null),
                Vec4::new(0.1, 0.1, 0.1, 0.0),
            );
            color_json(color_ops::randomize_color(
                color_field(input, "color", Color4::WHITE),
                space,
                &mut rng,
                min,
                max,
            ))
        }
        "apply" => {
            let op = input
                .get("op")
                .and_then(Value::as_str)
                .ok_or_else(|| "apply needs an `op`".to_string())?;
            color_json(apply_named_color(
                color_field(input, "color", Color4::WHITE),
                op,
                input,
            )?)
        }
        "gradient" => color_json(color_ops::sample_gradient(
            &color_stops(input),
            number(input, "t", 0.0),
            space,
            bool_field(input, "repeat", false),
        )),
        "gradientChannel" => channel_json(&color_ops::gradient_channel(
            &color_stops(input),
            space,
            usize_field(input, "count", 16),
            number(input, "start", 0.0),
            number(input, "end", 1000.0),
            bool_field(input, "repeat", false),
        )),
        other => return Err(format!("unknown color helper: {other}")),
    })
}

// -- scene2d dispatch -------------------------------------------------------

fn parse_vec2_field(input: &Value, key: &str) -> Vec2 {
    if let Some(values) = input.get(key).and_then(Value::as_array) {
        return Vec2::new(
            values.first().and_then(Value::as_f64).unwrap_or(0.0),
            values.get(1).and_then(Value::as_f64).unwrap_or(0.0),
        );
    }
    if let Some(object) = input.get(key) {
        return Vec2::new(
            object.get("x").and_then(Value::as_f64).unwrap_or(0.0),
            object.get("y").and_then(Value::as_f64).unwrap_or(0.0),
        );
    }
    Vec2::ZERO
}

fn parse_origin(value: Option<&Value>) -> Result<osb_runtime::storyboarding::OsbOrigin, String> {
    let api_origin: osb_api::OsbOrigin = value
        .cloned()
        .map(serde_json::from_value)
        .transpose()
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "an origin id/name is required".to_string())?;
    Ok(api_origin)
}

/// Applies a `transform` object's channels onto a 2D node.
fn apply_transform(node: &mut Node2d, value: Option<&Value>) -> Result<(), String> {
    let Some(value) = value else {
        return Ok(());
    };
    if let Some(channel) = value.get("position") {
        node.position = parse_channel(channel)?;
    }
    if let Some(channel) = value.get("scale") {
        node.scale = parse_channel(channel)?;
    }
    if let Some(channel) = value.get("rotation") {
        node.rotation = parse_channel(channel)?;
    }
    if let Some(channel) = value.get("opacity") {
        node.opacity = parse_channel(channel)?;
    }
    if let Some(channel) = value.get("color") {
        node.color = parse_channel(channel)?;
    }
    if let Some(channel) = value.get("anchor") {
        node.anchor = parse_channel(channel)?;
    }
    Ok(())
}

fn transform_json(node: &Node2d) -> Value {
    json!({
        "position": channel_json(&node.position),
        "scale": channel_json(&node.scale),
        "rotation": channel_json(&node.rotation),
        "opacity": channel_json(&node.opacity),
        "color": channel_json(&node.color),
        "anchor": channel_json(&node.anchor),
    })
}

/// Parses a node's per-field transform timeline: an object whose `position`/
/// `scale`/`rotation`/`opacity`/`color`/`anchor` fields are each a `Timeline`
/// (`{ clips: [{ start, channel }], children: [...] }`).
fn parse_transform_timeline2d(value: &Value) -> Result<TransformTimeline2d, String> {
    let mut timeline = TransformTimeline2d::new();
    if let Some(field) = value.get("position") {
        timeline.position = parse_timeline(field)?;
    }
    if let Some(field) = value.get("scale") {
        timeline.scale = parse_timeline(field)?;
    }
    if let Some(field) = value.get("rotation") {
        timeline.rotation = parse_timeline(field)?;
    }
    if let Some(field) = value.get("opacity") {
        timeline.opacity = parse_timeline(field)?;
    }
    if let Some(field) = value.get("color") {
        timeline.color = parse_timeline(field)?;
    }
    if let Some(field) = value.get("anchor") {
        timeline.anchor = parse_timeline(field)?;
    }
    Ok(timeline)
}

fn transform_timeline2d_json(timeline: &TransformTimeline2d) -> Value {
    json!({
        "position": timeline_json(&timeline.position),
        "scale": timeline_json(&timeline.scale),
        "rotation": timeline_json(&timeline.rotation),
        "opacity": timeline_json(&timeline.opacity),
        "color": timeline_json(&timeline.color),
        "anchor": timeline_json(&timeline.anchor),
    })
}

/// A `path` of child indices addressing a node in a scene tree (`[]` = the root).
fn usize_path(input: &Value, key: &str) -> Vec<usize> {
    input
        .get(key)
        .and_then(Value::as_array)
        .map(|list| {
            list.iter()
                .filter_map(Value::as_u64)
                .map(|value| value as usize)
                .collect()
        })
        .unwrap_or_default()
}

fn node2d_at_path<'a>(node: &'a Node2d, path: &[usize]) -> Option<&'a Node2d> {
    let mut current = node;
    for &index in path {
        current = current.children.get(index)?.node()?;
    }
    Some(current)
}

fn node2d_at_path_mut<'a>(node: &'a mut Node2d, path: &[usize]) -> Option<&'a mut Node2d> {
    let mut current = node;
    for &index in path {
        current = current.children.get_mut(index)?.node_mut()?;
    }
    Some(current)
}

fn sprite2d_json(sprite: &Sprite2d) -> Value {
    json!({
        "texturePath": sprite.texture_path,
        "origin": sprite.origin,
        "additive": sprite.additive,
        "commands": commands_json(&sprite.commands),
    })
}

/// Parses a node's common fields into a [`Node2d`] (transform, timeline, flags
/// and its children), without attaching a sprite leaf.
fn parse_node2d_base(value: &Value) -> Result<Node2d, String> {
    let mut node = Node2d::new(value.get("name").and_then(Value::as_str).unwrap_or(""));
    node.layer = value
        .get("layer")
        .and_then(Value::as_str)
        .and_then(osb_runtime::storyboarding::OsbLayer::from_name);
    apply_transform(&mut node, value.get("transform"))?;
    if let Some(timeline) = value.get("timeline").filter(|value| !value.is_null()) {
        node.timeline = Some(parse_transform_timeline2d(timeline)?);
    }
    node.inherits_opacity = bool_field(value, "inheritsOpacity", true);
    node.inherits_color = bool_field(value, "inheritsColor", true);
    node.inherits_transform = bool_field(value, "inheritsTransform", true);
    if let Some(children) = value.get("children").and_then(Value::as_array) {
        for child in children {
            node.add(parse_node2d(child)?);
        }
    }
    Ok(node)
}

/// Parses a scene-tree object: a [`Node2d`] group, or a [`Sprite2d`] when the
/// object carries a `sprite` leaf.
fn parse_node2d(value: &Value) -> Result<Box<dyn Object2d>, String> {
    let node = parse_node2d_base(value)?;
    match value.get("sprite").filter(|value| !value.is_null()) {
        Some(sprite) => {
            let texture_path = sprite
                .get("texturePath")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            let origin = parse_origin(sprite.get("origin"))?;
            let mut sprite2d = Sprite2d::new(texture_path, origin);
            sprite2d.node = node;
            sprite2d.additive = bool_field(sprite, "additive", false);
            if let Some(commands) = sprite.get("commands").and_then(Value::as_array) {
                sprite2d.commands = parse_command_list(commands)?;
            }
            Ok(Box::new(sprite2d))
        }
        None => Ok(Box::new(node)),
    }
}

fn node2d_json(object: &dyn Object2d) -> Value {
    let node = object.node();
    let transform = node.map(transform_json).unwrap_or(Value::Null);
    let timeline = node
        .and_then(|node| node.timeline.as_ref())
        .map(transform_timeline2d_json);
    json!({
        "name": object.name(),
        "layer": object.layer(),
        "transform": transform,
        "timeline": timeline,
        "sprite": object.sprite().map(sprite2d_json),
        "children": object.children().iter().map(|child| node2d_json(child.as_ref())).collect::<Vec<Value>>(),
        "inheritsOpacity": node.map(|node| node.inherits_opacity).unwrap_or(true),
        "inheritsColor": node.map(|node| node.inherits_color).unwrap_or(true),
        "inheritsTransform": node.map(|node| node.inherits_transform).unwrap_or(true),
    })
}

fn parse_camera2d(value: Option<&Value>) -> Result<Camera2d, String> {
    let mut camera = Camera2d::identity();
    let Some(value) = value else {
        return Ok(camera);
    };
    if let Some(channel) = value.get("position") {
        camera.position = parse_channel(channel)?;
    }
    if let Some(channel) = value.get("zoom") {
        camera.zoom = parse_channel(channel)?;
    }
    if let Some(channel) = value.get("rotation") {
        camera.rotation = parse_channel(channel)?;
    }
    if let Some(channel) = value.get("anchor") {
        camera.anchor = parse_channel(channel)?;
    }
    Ok(camera)
}

fn camera2d_json(camera: &Camera2d) -> Value {
    json!({
        "position": channel_json(&camera.position),
        "zoom": channel_json(&camera.zoom),
        "rotation": channel_json(&camera.rotation),
        "anchor": channel_json(&camera.anchor),
    })
}

fn parse_scene2d(value: &Value) -> Result<Scene2d, String> {
    let root = value
        .get("root")
        .map(parse_node2d_base)
        .transpose()?
        .unwrap_or_default();
    Ok(Scene2d {
        name: value
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        root,
        camera: parse_camera2d(value.get("camera"))?,
        start_time: number(value, "startTime", 0.0),
        end_time: number(value, "endTime", 0.0),
    })
}

fn scene2d_json(scene: &Scene2d) -> Value {
    json!({
        "name": scene.name,
        "root": node2d_json(&scene.root),
        "camera": camera2d_json(&scene.camera),
        "startTime": scene.start_time,
        "endTime": scene.end_time,
    })
}

fn scene2d_options_from(options: &Value) -> Scene2dOptions {
    Scene2dOptions {
        start_time: number(options, "startTime", 0.0),
        end_time: number(options, "endTime", 0.0),
        time_step: number(options, "timeStep", 16.0),
        tolerance_2d: number(options, "tolerance2d", 0.1),
        tolerance_1d: number(options, "tolerance1d", 0.001),
        decimals: usize_field(options, "decimals", 3) as u32,
    }
}

fn parse_scene2d_options(input: &Value) -> Scene2dOptions {
    let options = input.get("options").unwrap_or(input);
    scene2d_options_from(options)
}

/// Serializes a runtime storyboard as the wire model (for `*.bake` helpers).
pub(crate) fn storyboard_json(storyboard: &osb_runtime::storyboarding::Storyboard) -> Value {
    serde_json::to_value(osb_api::storyboard::Storyboard::from(storyboard)).unwrap_or(Value::Null)
}

fn resolved_sprite_json(sprite: &ResolvedSprite2d) -> Value {
    json!({
        "texturePath": sprite.texture_path,
        "origin": sprite.origin,
        "layer": sprite.layer,
        "additive": sprite.additive,
        "commands": commands_json(&sprite.commands),
        "timeRange": { "start": sprite.time_range.0, "end": sprite.time_range.1 },
    })
}

fn dispatch_scene2d(name: &str, input: &Value) -> Result<Value, String> {
    let scene_value = input
        .get("scene")
        .ok_or_else(|| "scene2d helpers need a `scene`".to_string())?;
    let mut scene = parse_scene2d(scene_value)?;
    Ok(match name {
        "resolve" => {
            let options = parse_scene2d_options(input);
            Value::Array(
                scene
                    .resolve(&options)
                    .iter()
                    .map(resolved_sprite_json)
                    .collect(),
            )
        }
        "bake" => {
            let options = parse_scene2d_options(input);
            storyboard_json(&scene.bake(&options))
        }
        "timeRange" => match scene.time_range() {
            Some((start, end)) => json!({ "start": start, "end": end }),
            None => Value::Null,
        },
        "translate" => {
            scene.root.translate(parse_vec2_field(input, "delta"));
            scene2d_json(&scene)
        }
        "scale" | "scaleBy" => {
            scene.root.scale_by(number(input, "factor", 1.0));
            scene2d_json(&scene)
        }
        "rotate" => {
            scene.root.rotate(number(input, "radians", 0.0));
            scene2d_json(&scene)
        }
        "setOpacity" => {
            scene.root.set_opacity(number(input, "value", 1.0));
            scene2d_json(&scene)
        }
        "setColor" => {
            scene
                .root
                .set_color(color_field(input, "color", Color4::WHITE));
            scene2d_json(&scene)
        }
        "flatten" => {
            scene.root.flatten(axis_field(input), flatten_mode(input));
            scene2d_json(&scene)
        }
        "ease" => {
            scene
                .root
                .ease(crate::json::easing(input), usize_field(input, "samples", 8));
            scene2d_json(&scene)
        }
        "shift" => {
            scene.root.shift(number(input, "delta", 0.0));
            scene2d_json(&scene)
        }
        "scaleTime" => {
            scene
                .root
                .scale_time(number(input, "factor", 1.0), number(input, "anchor", 0.0));
            scene2d_json(&scene)
        }
        "reverse" => {
            scene
                .root
                .reverse(number(input, "start", 0.0), number(input, "end", 0.0));
            scene2d_json(&scene)
        }
        "timeline" => {
            let path = usize_path(input, "path");
            node2d_at_path(&scene.root, &path)
                .and_then(|node| node.timeline.as_ref())
                .map(transform_timeline2d_json)
                .unwrap_or(Value::Null)
        }
        "setTimeline" | "clearTimeline" => {
            let path = usize_path(input, "path");
            let timeline = match input.get("timeline") {
                Some(value) if name != "clearTimeline" && !value.is_null() => {
                    Some(parse_transform_timeline2d(value)?)
                }
                _ => None,
            };
            let node = node2d_at_path_mut(&mut scene.root, &path)
                .ok_or_else(|| "setTimeline: no node at `path`".to_string())?;
            node.timeline = timeline;
            scene2d_json(&scene)
        }
        other => return Err(format!("unknown scene2d helper: {other}")),
    })
}

// -- command conversion -----------------------------------------------------

fn parse_kind(name: &str) -> Option<CommandType> {
    Some(match name {
        "move" => CommandType::Move,
        "moveX" => CommandType::MoveX,
        "moveY" => CommandType::MoveY,
        "scale" => CommandType::Scale,
        "scaleVec" => CommandType::ScaleVec,
        "rotate" => CommandType::Rotate,
        "fade" => CommandType::Fade,
        "color" => CommandType::Color,
        "colorHsb" => CommandType::ColorHsb,
        "parameter" => CommandType::Parameter,
        "anchor" => CommandType::Anchor,
        _ => return None,
    })
}

fn kind_field(input: &Value) -> Option<CommandType> {
    input
        .get("kind")
        .and_then(Value::as_str)
        .and_then(parse_kind)
}

fn api_kind(kind: StoryboardCommandKind) -> CommandType {
    match kind {
        StoryboardCommandKind::Move => CommandType::Move,
        StoryboardCommandKind::MoveX => CommandType::MoveX,
        StoryboardCommandKind::MoveY => CommandType::MoveY,
        StoryboardCommandKind::Scale => CommandType::Scale,
        StoryboardCommandKind::ScaleVec => CommandType::ScaleVec,
        StoryboardCommandKind::Rotate => CommandType::Rotate,
        StoryboardCommandKind::Fade => CommandType::Fade,
        StoryboardCommandKind::Color => CommandType::Color,
        StoryboardCommandKind::ColorHsb => CommandType::ColorHsb,
        StoryboardCommandKind::Parameter => CommandType::Parameter,
        StoryboardCommandKind::Anchor => CommandType::Anchor,
    }
}

fn api_value_to_runtime(value: &ApiCommandValue) -> CommandValue {
    match value {
        ApiCommandValue::Scalar(v) => CommandValue::Scalar(*v),
        ApiCommandValue::Vector2(v) => CommandValue::Vector2(Vec2::new(v.x, v.y)),
        ApiCommandValue::Color(c) => {
            CommandValue::Color(osb_runtime::math::Color4::new(c.r, c.g, c.b, c.a))
        }
        ApiCommandValue::Parameter(p) => CommandValue::Parameter(match p {
            osb_api::storyboard::ParameterValue::FlipHorizontal(v) => {
                ParameterTypeValue::FlipHorizontal(*v)
            }
            osb_api::storyboard::ParameterValue::FlipVertical(v) => {
                ParameterTypeValue::FlipVertical(*v)
            }
            osb_api::storyboard::ParameterValue::Additive(v) => ParameterTypeValue::Additive(*v),
        }),
        ApiCommandValue::None => CommandValue::None,
    }
}

fn command_to_api(kind: CommandType, command: Command) -> StoryboardCommand {
    StoryboardCommand {
        kind: kind.into(),
        start_time: command.start_time,
        end_time: command.end_time,
        easing: command.easing,
        start: command.start_value.into(),
        end: command.end_value.into(),
    }
}

fn parse_commands(input: &Value) -> Result<Vec<sb::KindedCommand>, String> {
    let list = input
        .get("commands")
        .and_then(Value::as_array)
        .ok_or_else(|| "this helper needs a `commands` array".to_string())?;
    parse_command_list(list)
}

fn parse_command_list(list: &[Value]) -> Result<Vec<sb::KindedCommand>, String> {
    let mut out = Vec::with_capacity(list.len());
    for entry in list {
        let command: StoryboardCommand = serde_json::from_value(entry.clone())
            .map_err(|error| format!("invalid command: {error}"))?;
        out.push((
            api_kind(command.kind),
            Command {
                start_time: command.start_time,
                end_time: command.end_time,
                easing: command.easing,
                start_value: api_value_to_runtime(&command.start),
                end_value: api_value_to_runtime(&command.end),
            },
        ));
    }
    Ok(out)
}

fn commands_json(commands: &[sb::KindedCommand]) -> Value {
    Value::Array(
        commands
            .iter()
            .map(|(kind, command)| {
                serde_json::to_value(command_to_api(*kind, *command)).unwrap_or(Value::Null)
            })
            .collect(),
    )
}

// -- command dispatch -------------------------------------------------------

fn dispatch_command(name: &str, input: &Value) -> Result<Value, String> {
    let commands = parse_commands(input)?;
    Ok(match name {
        "selectCommands" => commands_json(&sb::select_commands(
            &commands,
            kind_field(input),
            opt_number(input, "start"),
            opt_number(input, "end"),
        )),
        "shiftCommands" => {
            let mut shifted = commands.clone();
            sb::shift_commands(&mut shifted, number(input, "delta", 0.0));
            commands_json(&shifted)
        }
        "scaleCommands" => {
            let mut scaled = commands.clone();
            sb::scale_time_commands(
                &mut scaled,
                number(input, "factor", 1.0),
                number(input, "anchor", 0.0),
            );
            commands_json(&scaled)
        }
        "reverseCommands" => commands_json(&sb::reverse_commands(
            &commands,
            number(input, "start", 0.0),
            number(input, "end", 0.0),
        )),
        "mergeCommands" => commands_json(&sb::merge_commands(
            &commands,
            number(input, "tolerance", 0.0),
        )),
        "splitCommands" => commands_json(&sb::split_commands(&commands, number(input, "at", 0.0))),
        "flattenCommands" => commands_json(&sb::flatten_commands(
            &commands,
            kind_field(input),
            flatten_mode(input),
        )),
        "randomizeCommands" => {
            let seed = input.get("seed").and_then(Value::as_u64).unwrap_or(0);
            commands_json(&sb::randomize_commands(
                &commands,
                number(input, "min", -1.0),
                number(input, "max", 1.0),
                seed,
            ))
        }
        "filterCommands" | "removeCommands" => {
            let kind = kind_field(input);
            let start = opt_number(input, "start");
            let end = opt_number(input, "end");
            let matches = |(command_kind, command): &sb::KindedCommand| {
                let kind_matches = kind.map(|k| k == *command_kind).unwrap_or(true);
                let after_start = start.map(|s| command.start_time >= s).unwrap_or(true);
                let before_end = end.map(|e| command.end_time <= e).unwrap_or(true);
                kind_matches && after_start && before_end
            };
            if name == "filterCommands" {
                commands_json(&sb::filter_commands(&commands, matches))
            } else {
                commands_json(&sb::remove_commands(&commands, matches))
            }
        }
        "mapCommands" => commands_json(&sb::transform_command_values(
            &commands,
            number(input, "valueScale", 1.0),
            number(input, "valueOffset", 0.0),
        )),
        "duplicateCommands" => commands_json(&sb::duplicate_commands(
            &commands,
            number(input, "offset", 0.0),
            usize_field(input, "count", 1),
        )),
        "alignCommands" => commands_json(&sb::align_commands(
            &commands,
            number(input, "beat", 500.0),
            number(input, "offset", 0.0),
            usize_field(input, "subdivision", 1) as u32,
            number(input, "valueStep", 0.0),
        )),
        "quantizeCommands" => commands_json(&sb::quantize_commands(
            &commands,
            number(input, "timeStep", 0.0),
            number(input, "valueStep", 0.0),
        )),
        "setEasing" => commands_json(&sb::set_easing(&commands, crate::json::easing(input))),
        "applyEasing" => {
            let kind = kind_field(input).ok_or_else(|| "applyEasing needs a `kind`".to_string())?;
            let result = sb::retime_easing(
                &commands,
                kind,
                crate::json::easing(input),
                usize_field(input, "samples", 8),
            )
            .ok_or_else(|| "no commands of that kind, or not a numeric channel".to_string())?;
            commands_json(&result)
        }
        "applyBezier" => {
            let kind = kind_field(input).ok_or_else(|| "applyBezier needs a `kind`".to_string())?;
            let result = sb::retime_bezier(
                &commands,
                kind,
                number(input, "x1", 0.0),
                number(input, "y1", 0.0),
                number(input, "x2", 1.0),
                number(input, "y2", 1.0),
                usize_field(input, "samples", 8),
            )
            .ok_or_else(|| "no commands of that kind, or not a numeric channel".to_string())?;
            commands_json(&result)
        }
        other => return Err(format!("unknown command helper: {other}")),
    })
}

// -- easing dispatch --------------------------------------------------------

fn easing_field(input: &Value, key: &str) -> Easing {
    crate::json::enum_field::<osb_api::Easing>(input, key).unwrap_or(Easing::None)
}

fn dispatch_easing(name: &str, input: &Value) -> Result<Value, String> {
    let easing = easing_field(input, "easing");
    let t = number(input, "t", 0.0);
    Ok(match name {
        "stretchEasing" => json!(easing_ops::stretch_easing(
            easing,
            t,
            number(input, "from", 0.0),
            number(input, "to", 1.0)
        )),
        "amplifyEasing" => json!(easing_ops::amplify_easing(
            easing,
            t,
            number(input, "amount", 2.0)
        )),
        "reverseEasing" => json!(easing_ops::reverse_easing(easing, t)),
        "mirrorEasing" => json!(easing_ops::mirror_easing(easing, t)),
        "pingPongEasing" => json!(easing_ops::ping_pong_easing(easing, t)),
        "repeatEasing" => json!(easing_ops::repeat_easing(
            easing,
            t,
            usize_field(input, "count", 2)
        )),
        "composeEasing" => json!(easing_ops::compose_easing(
            easing_field(input, "outer"),
            easing_field(input, "inner"),
            t
        )),
        "blendEasing" => json!(easing_ops::blend_easing(
            easing_field(input, "a"),
            easing_field(input, "b"),
            t,
            number(input, "amount", 0.5)
        )),
        "bezierValue" => json!(easing_ops::bezier_easing(
            number(input, "x1", 0.0),
            number(input, "y1", 0.0),
            number(input, "x2", 1.0),
            number(input, "y2", 1.0),
            t
        )),
        "amplifyBezier" => json!(easing_ops::amplify_bezier(
            number(input, "x1", 0.0),
            number(input, "y1", 0.0),
            number(input, "x2", 1.0),
            number(input, "y2", 1.0),
            t,
            number(input, "amount", 2.0)
        )),
        "sampleEasing" => Value::Array(
            easing_ops::sample_easing(easing, usize_field(input, "count", 8))
                .into_iter()
                .map(|value| json!(value))
                .collect(),
        ),
        "sampleBezier" => Value::Array(
            easing_ops::sample_bezier(
                number(input, "x1", 0.0),
                number(input, "y1", 0.0),
                number(input, "x2", 1.0),
                number(input, "y2", 1.0),
                usize_field(input, "count", 8),
            )
            .into_iter()
            .map(|value| json!(value))
            .collect(),
        ),
        "staggerEasing" => json!(easing_ops::stagger_easing(
            usize_field(input, "index", 0),
            number(input, "step", 0.0)
        )),
        "remapEasing" => {
            let segments: Vec<easing_ops::EasingSegment> = input
                .get("segments")
                .and_then(Value::as_array)
                .map(|list| {
                    list.iter()
                        .map(|segment| easing_ops::EasingSegment {
                            start: number(segment, "start", 0.0),
                            end: number(segment, "end", 1.0),
                            from: number(segment, "from", 0.0),
                            to: number(segment, "to", 1.0),
                            easing: easing_field(segment, "easing"),
                        })
                        .collect()
                })
                .unwrap_or_default();
            json!(easing_ops::remap_easing(&segments, t))
        }
        other => return Err(format!("unknown easing helper: {other}")),
    })
}

// -- timeline dispatch ------------------------------------------------------

fn parse_timeline(value: &Value) -> Result<Timeline, String> {
    let mut timeline = Timeline::new();
    if let Some(clips) = value.get("clips").and_then(Value::as_array) {
        for clip in clips {
            let start = clip.get("start").and_then(Value::as_f64).unwrap_or(0.0);
            let channel = clip
                .get("channel")
                .ok_or_else(|| "a timeline clip needs a `channel`".to_string())?;
            timeline.add(parse_channel(channel)?, start);
        }
    }
    if let Some(children) = value.get("children").and_then(Value::as_array) {
        for child in children {
            timeline.add_child(parse_timeline(child)?);
        }
    }
    Ok(timeline)
}

fn timeline_json(timeline: &Timeline) -> Value {
    let clips: Vec<Value> = timeline
        .clips
        .iter()
        .map(|clip: &TimelineClip| {
            json!({ "start": clip.start, "channel": channel_json(&clip.channel) })
        })
        .collect();
    let children: Vec<Value> = timeline.children.iter().map(timeline_json).collect();
    json!({ "clips": clips, "children": children })
}

fn dispatch_timeline(name: &str, input: &Value) -> Result<Value, String> {
    let timeline = input
        .get("timeline")
        .map(parse_timeline)
        .transpose()?
        .unwrap_or_default();
    Ok(match name {
        "new" => timeline_json(&Timeline::new()),
        "duration" | "durationOf" => json!(timeline.duration()),
        "timeRange" | "timeRangeOf" => match timeline.time_range() {
            Some((start, end)) => json!({ "start": start, "end": end }),
            None => Value::Null,
        },
        "shift" | "shiftTimeline" => {
            let mut shifted = timeline;
            shifted.shift(number(input, "delta", 0.0));
            timeline_json(&shifted)
        }
        "scale" | "scaleTimeline" => {
            let mut scaled = timeline;
            scaled.scale(number(input, "factor", 1.0), number(input, "anchor", 0.0));
            timeline_json(&scaled)
        }
        "reverse" | "reverseTimeline" => {
            let end = number(input, "end", timeline.duration());
            let mut reversed = timeline;
            reversed.reverse(number(input, "start", 0.0), end);
            timeline_json(&reversed)
        }
        "repeat" | "repeatTimeline" => timeline_json(&timeline.repeat(
            usize_field(input, "count", 2),
            bool_field(input, "pingPong", false),
        )),
        "stagger" | "staggerTimeline" => {
            timeline_json(&timeline.stagger(number(input, "step", 0.0)))
        }
        "slice" | "sliceTimeline" => {
            let end = number(input, "end", timeline.duration());
            timeline_json(&timeline.slice(number(input, "start", 0.0), end))
        }
        "flush" | "flushTimeline" => timeline_json(&timeline.flush()),
        "merge" | "mergeTimelines" => {
            let other = input
                .get("other")
                .map(parse_timeline)
                .transpose()?
                .unwrap_or_default();
            timeline_json(&timeline.merge(&other))
        }
        "add" | "addToTimeline" => {
            let channel = input
                .get("channel")
                .ok_or_else(|| "add needs a `channel`".to_string())?;
            let mut added = timeline;
            added.add(parse_channel(channel)?, number(input, "start", 0.0));
            timeline_json(&added)
        }
        "group" | "addGroup" => {
            let mut grouped = timeline;
            let child = input
                .get("child")
                .map(parse_timeline)
                .transpose()?
                .unwrap_or_default();
            grouped.add_child(child);
            timeline_json(&grouped)
        }
        "allClips" | "flattenTimeline" => {
            let clips: Vec<Value> = timeline
                .all_clips()
                .iter()
                .map(|clip| json!({ "start": clip.start, "channel": channel_json(&clip.channel) }))
                .collect();
            Value::Array(clips)
        }
        "childCount" => json!(timeline.child_count()),
        "clipCount" => json!(timeline.clip_count()),
        other => return Err(format!("unknown timeline helper: {other}")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(name: &str, input: Value) -> Value {
        let request = json!({ "name": name, "input": input });
        serde_json::from_str(&helper(&request)).unwrap()
    }

    fn channel(keys: Value) -> Value {
        json!({ "keys": keys })
    }

    #[test]
    fn keyframe_helpers_work_for_2d_and_3d_channels() {
        // 2D channel (no Z): Z is optional.
        let value = run(
            "keyframe.scaleKeyframes",
            json!({
                "channel": channel(json!([
                    { "time": 0.0, "value": [1.0, 2.0] },
                    { "time": 100.0, "value": [3.0, 4.0] }
                ])),
                "factor": 2.0
            }),
        );
        assert_eq!(value["value"]["keys"][1]["value"][0], 6.0);
        assert_eq!(value["value"]["keys"][1]["value"][1], 8.0);

        // 3D channel.
        let value = run(
            "keyframe.offsetKeyframes",
            json!({
                "channel": channel(json!([{ "time": 0.0, "value": [1.0, 2.0, 3.0] }])),
                "delta": 1.0
            }),
        );
        assert_eq!(value["value"]["keys"][0]["value"][2], 4.0);
    }

    #[test]
    fn keyframe_helpers_bridge_2d_command_channels() {
        let commands = json!([
            { "kind": "fade", "startTime": 0.0, "endTime": 100.0, "easing": "None",
              "start": { "kind": "scalar", "value": 0.0 }, "end": { "kind": "scalar", "value": 1.0 } },
            { "kind": "fade", "startTime": 100.0, "endTime": 200.0, "easing": "None",
              "start": { "kind": "scalar", "value": 1.0 }, "end": { "kind": "scalar", "value": 0.0 } }
        ]);
        // Flatten the Fade command channel to a constant (mean = 0.5).
        let value = run(
            "keyframe.flatten",
            json!({ "commands": commands, "kind": "fade", "mode": "mean" }),
        );
        let result = value["value"].as_array().unwrap();
        // A single command spanning [0, 200] with value 0.5.
        assert_eq!(result.len(), 1);
        assert_eq!(result[0]["kind"], "fade");
        assert_eq!(result[0]["startTime"], 0.0);
        assert_eq!(result[0]["endTime"], 200.0);
    }

    #[test]
    fn scene3d_and_origin_helpers() {
        let orbit = run("scene3d.orbit", json!({ "radius": 10.0, "count": 5 }));
        assert_eq!(orbit["value"]["keys"].as_array().unwrap().len(), 5);

        let origin = run(
            "origin.getOriginVector",
            json!({ "origin": "Centre", "width": 100.0, "height": 100.0 }),
        );
        assert_eq!(origin["value"]["x"], 50.0);

        let resolved = run(
            "origin.resolveAnchor",
            json!({ "origin": "Centre", "anchor": { "x": 10.0, "y": -5.0 }, "width": 100.0, "height": 100.0 }),
        );
        assert_eq!(resolved["value"]["x"], 60.0);
        assert_eq!(resolved["value"]["y"], 45.0);
    }

    #[test]
    fn command_helpers_and_legacy_names() {
        let commands = json!([
            { "kind": "fade", "startTime": 0.0, "endTime": 100.0, "easing": "None",
              "start": { "kind": "scalar", "value": 0.0 }, "end": { "kind": "scalar", "value": 1.0 } }
        ]);
        // Legacy unprefixed name resolves to command.*.
        let value = run(
            "shiftCommands",
            json!({ "commands": commands, "delta": 50.0 }),
        );
        assert_eq!(value["value"][0]["startTime"], 50.0);
    }

    #[test]
    fn rejects_unknown_helpers() {
        assert!(run("keyframe.nope", json!({})).get("error").is_some());
        assert!(run("nope.thing", json!({})).get("error").is_some());
    }

    #[test]
    fn color_helpers_cover_spaces_and_gradients() {
        // A CSS-style lighten in HSL.
        let lightened = run(
            "color.apply",
            json!({ "color": { "r": 0.5, "g": 0.5, "b": 0.5, "a": 1.0 }, "op": "lighten", "amount": 0.25 }),
        );
        assert!((lightened["value"]["r"].as_f64().unwrap() - 0.75).abs() < 1e-6);

        // Offset in HSB (brightness +0.2).
        let offset = run(
            "color.offset",
            json!({ "color": { "r": 0.2, "g": 0.2, "b": 0.2, "a": 1.0 }, "space": "hsb", "delta": [0.0, 0.0, 0.2, 0.0] }),
        );
        assert!(offset["value"]["r"].as_f64().unwrap() > 0.3);

        // A color command channel flattens through the shared keyframe helper.
        let commands = json!([
            { "kind": "color", "startTime": 0.0, "endTime": 100.0, "easing": "None",
              "start": { "kind": "color", "value": { "r": 0.0, "g": 0.0, "b": 0.0, "a": 1.0 } },
              "end": { "kind": "color", "value": { "r": 1.0, "g": 1.0, "b": 1.0, "a": 1.0 } } }
        ]);
        let flattened = run(
            "keyframe.flatten",
            json!({ "commands": commands, "kind": "color", "mode": "mean" }),
        );
        let mean = flattened["value"][0]["start"]["value"]["r"]
            .as_f64()
            .unwrap();
        assert!((mean - 0.5).abs() < 1e-6, "mean = {mean}");

        // A gradient channel has 4-component keys.
        let gradient = run(
            "color.gradientChannel",
            json!({ "stops": [
                { "offset": 0.0, "color": { "r": 0.0, "g": 0.0, "b": 0.0, "a": 1.0 } },
                { "offset": 1.0, "color": { "r": 1.0, "g": 1.0, "b": 1.0, "a": 1.0 } }
            ], "count": 5, "start": 0.0, "end": 100.0 }),
        );
        assert_eq!(gradient["value"]["keys"].as_array().unwrap().len(), 5);
        assert_eq!(
            gradient["value"]["keys"][2]["value"]
                .as_array()
                .unwrap()
                .len(),
            4
        );
    }

    #[test]
    fn easing_and_timeline_helpers() {
        // InQuad at 0.5 is 0.25; amplifying about linear gives 0.5 + (0.25-0.5)*2.
        let value = run(
            "easing.amplifyEasing",
            json!({ "easing": "InQuad", "t": 0.5, "amount": 2.0 }),
        );
        assert!(value["value"].as_f64().unwrap().abs() < 1e-6);
        let samples = run(
            "easing.sampleEasing",
            json!({ "easing": "OutQuad", "count": 5 }),
        );
        assert_eq!(samples["value"].as_array().unwrap().len(), 5);

        let timeline = json!({ "clips": [
            { "start": 0.0, "channel": { "keys": [
                { "time": 0.0, "value": [0.0] }, { "time": 100.0, "value": [1.0] } ] } },
            { "start": 200.0, "channel": { "keys": [
                { "time": 0.0, "value": [0.0] }, { "time": 100.0, "value": [1.0] } ] } }
        ] });
        let duration = run("timeline.durationOf", json!({ "timeline": timeline }));
        assert_eq!(duration["value"], 300.0);
        let flushed = run("timeline.flushTimeline", json!({ "timeline": timeline }));
        assert_eq!(
            flushed["value"]["clips"][1]["channel"]["keys"][0]["time"],
            200.0
        );
        let merged = run(
            "timeline.mergeTimelines",
            json!({ "timeline": timeline, "other": timeline }),
        );
        assert_eq!(merged["value"]["clips"].as_array().unwrap().len(), 4);

        let anchor = run(
            "origin.resolveAnchor3d",
            json!({ "origin": "Centre", "anchor": { "x": 10.0, "y": -5.0, "z": 42.0 }, "width": 100.0, "height": 100.0 }),
        );
        assert_eq!(anchor["value"]["z"], 42.0);
    }

    #[test]
    fn timeline_hierarchy_cascades_and_flushes() {
        let child = json!({ "clips": [
            { "start": 100.0, "channel": { "keys": [
                { "time": 0.0, "value": [0.0] }, { "time": 100.0, "value": [1.0] } ] } }
        ] });
        let timeline = json!({
            "clips": [
                { "start": 0.0, "channel": { "keys": [
                    { "time": 0.0, "value": [0.0] }, { "time": 100.0, "value": [1.0] } ] } }
            ],
            "children": [ child ]
        });
        // A parent shift moves the nested child too.
        let shifted = run(
            "timeline.shiftTimeline",
            json!({ "timeline": timeline, "delta": 1000.0 }),
        );
        assert_eq!(shifted["value"]["children"][0]["clips"][0]["start"], 1100.0);
        assert_eq!(
            run("timeline.clipCount", json!({ "timeline": timeline }))["value"],
            2
        );
        // Flushing flattens the tree.
        let flushed = run("timeline.flushTimeline", json!({ "timeline": timeline }));
        assert!(flushed["value"]["children"].as_array().unwrap().is_empty());
        assert_eq!(flushed["value"]["clips"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn scene2d_resolves_and_controls_from_the_top() {
        let scene = json!({
            "root": {
                "name": "root",
                "children": [
                    {
                        "name": "group",
                        "transform": {
                            "position": { "keys": [
                                { "time": 0.0, "value": [0.0, 0.0] },
                                { "time": 100.0, "value": [0.0, 50.0] } ] }
                        },
                        "children": [
                            {
                                "name": "leaf",
                                "sprite": {
                                    "texturePath": "sb/dot.png",
                                    "origin": "Centre",
                                    "transform": { "position": { "keys": [
                                        { "time": 0.0, "value": [0.0, 0.0] },
                                        { "time": 100.0, "value": [100.0, 0.0] } ] } }
                                }
                            }
                        ]
                    }
                ]
            }
        });
        let resolved = run("scene2d.resolve", json!({ "scene": scene }));
        let sprites = resolved["value"].as_array().unwrap();
        assert_eq!(sprites.len(), 1);
        assert_eq!(sprites[0]["texturePath"], "sb/dot.png");
        // The resolved command set includes a Move, Scale, Rotate, Fade and Color.
        let kinds: Vec<&str> = sprites[0]["commands"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["kind"].as_str().unwrap())
            .collect();
        assert!(kinds.contains(&"move"));
        assert!(kinds.contains(&"scaleVec"));

        // A top-down translate cascades into the nested leaf.
        let moved = run(
            "scene2d.translate",
            json!({ "scene": scene, "delta": [10.0, 20.0] }),
        );
        let resolved = run(
            "scene2d.resolve",
            json!({ "scene": moved["value"].clone() }),
        );
        let move_command = resolved["value"][0]["commands"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["kind"] == "move")
            .unwrap();
        let start = &move_command["start"]["value"];
        assert_eq!(start["x"], 10.0);
        assert_eq!(start["y"], 20.0);
    }

    #[test]
    fn scene2d_node_timeline_composes_and_round_trips() {
        // The root's timeline adds +40x in local space on top of its static +10x.
        let scene = json!({
            "root": {
                "transform": { "position": { "keys": [
                    { "time": 0.0, "value": [10.0, 0.0] } ] } },
                "timeline": {
                    "position": { "clips": [{ "start": 0.0, "channel": { "keys": [
                        { "time": 0.0, "value": [0.0, 0.0] },
                        { "time": 100.0, "value": [40.0, 0.0] } ] } }] }
                },
                "children": [{ "sprite": { "texturePath": "sb/dot.png", "origin": "Centre" } }]
            }
        });
        let resolved = run("scene2d.resolve", json!({ "scene": scene.clone() }));
        let move_command = resolved["value"][0]["commands"]
            .as_array()
            .unwrap()
            .iter()
            .find(|command| command["kind"] == "move")
            .unwrap();
        assert_eq!(move_command["start"]["value"]["x"], 10.0);
        assert_eq!(move_command["end"]["value"]["x"], 50.0);

        // `scene2d.timeline` reads the root's timeline; `setTimeline` replaces it.
        let read = run("scene2d.timeline", json!({ "scene": scene.clone() }));
        assert!(read["value"]["position"]["clips"].is_array());
        let replaced = run(
            "scene2d.setTimeline",
            json!({ "scene": scene, "timeline": {
                "rotation": { "clips": [{ "start": 0.0, "channel": { "keys": [
                    { "time": 0.0, "value": [1.0] } ] } }] } } }),
        );
        assert!(replaced["value"]["root"]["timeline"]["rotation"]["clips"].is_array());
        assert!(replaced["value"]["root"]["timeline"]["position"]["clips"]
            .as_array()
            .unwrap()
            .is_empty());
    }

    #[test]
    fn scene2d_and_scene3d_bake_a_storyboard() {
        // A 2D scene bakes to a storyboard with one layer carrying the sprite.
        let scene = json!({
            "root": {
                "children": [{
                    "layer": "Foreground",
                    "sprite": { "texturePath": "sb/dot.png", "origin": "Centre" }
                }]
            }
        });
        let baked = run("scene2d.bake", json!({ "scene": scene }));
        let layers = baked["value"]["layers"].as_array().unwrap();
        let foreground = layers
            .iter()
            .find(|layer| layer["osbLayer"] == "Foreground")
            .unwrap();
        assert_eq!(foreground["sprites"].as_array().unwrap().len(), 1);

        // A 3D world bakes too.
        let world = json!({
            "nodes": [{ "sprite": "sb/3d.png" }],
            "camera": { "position": [{ "time": 0, "value": [0, 0, 5] }] },
            "options": { "endTime": 0 }
        });
        let baked = run("scene3d.bakeWorld", json!({ "world": world.clone() }));
        assert!(baked["value"]["layers"].is_array(), "{baked}");

        // A unified scene bakes 2D + 3D into one storyboard.
        let combined = run(
            "scene3d.bakeScene",
            json!({
                "worlds2d": [scene],
                "worlds3d": [{ "nodes": [{ "sprite": "sb/3d.png" }], "camera": { "position": [{ "time": 0, "value": [0, 0, 5] }] } }],
                "options": { "endTime": 0 },
                "options2d": { "startTime": 0, "endTime": 100 }
            }),
        );
        let total: usize = combined["value"]["layers"]
            .as_array()
            .unwrap()
            .iter()
            .map(|layer| layer["sprites"].as_array().map(Vec::len).unwrap_or(0))
            .sum();
        assert_eq!(total, 2);
    }

    #[test]
    fn command_easing_helpers_preserve_easing() {
        let commands = json!([
            { "kind": "fade", "startTime": 0.0, "endTime": 100.0, "easing": "None",
              "start": { "kind": "scalar", "value": 0.0 }, "end": { "kind": "scalar", "value": 1.0 } },
            { "kind": "fade", "startTime": 100.0, "endTime": 200.0, "easing": "None",
              "start": { "kind": "scalar", "value": 1.0 }, "end": { "kind": "scalar", "value": 0.0 } }
        ]);
        let set = run(
            "command.setEasing",
            json!({ "commands": commands, "easing": "InQuad" }),
        );
        assert_eq!(set["value"][0]["easing"], "InQuad");

        // A keyframe transform preserves the input easing on the resolved output.
        let flattened = run(
            "keyframe.flatten",
            json!({ "commands": set["value"].clone(), "kind": "fade", "mode": "last" }),
        );
        assert_eq!(flattened["value"][0]["easing"], "InQuad");

        // Re-easing resolves the channel into several commands.
        let applied = run(
            "command.applyEasing",
            json!({ "commands": commands, "kind": "fade", "easing": "InQuad", "samples": 4 }),
        );
        assert!(applied["value"].as_array().unwrap().len() > 1);
    }
}
