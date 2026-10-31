//! The stateful `Runtime` handle and its JSON operations (`osbtk_runtime_exec`):
//! layers/sprites, the command surface, beatmap loading/queries and sprite
//! sampling - the storyboard API from other languages. Responses are serialized
//! from the typed `osb-api` models; requests accept the string enums from the
//! OpenAPI contract and, for compatibility, their legacy numeric ids.

use osb_api::{
    AnimationMeta, AudioDurationResult, Beatmap, BoolResult, ControlPoint, FftResult, Handle,
    ImportOsbResult, LayerHandle, LoadBeatmapResult, OkResult, RandomResult, SegmentCleanResult,
    SegmentDiscardResult, SegmentIndexesResult, SpriteCountResult, SpriteInfo, SpriteState,
    Storyboard as ApiStoryboard,
};
use osb_api::{
    OsbLayer as ApiLayer, OsbLoopType as ApiLoopType, OsbOrigin as ApiOrigin,
    ParameterType as ApiParameter,
};
use osb_runtime::context::Context;
use osb_runtime::mapset::parse_osu;
use osb_runtime::math::{Color4, Vec2};
use osb_runtime::storyboarding::{
    LayerHandle as RuntimeLayerHandle, OsbLayer, OsbLoopType, OsbOrigin, ParameterType,
    SpriteHandle, Storyboard,
};
use serde_json::Value;

use crate::json::{apply_easing, easing, enum_field};
use crate::{error_json, to_json};

/// An opaque handle to a live runtime: the effect context (beatmap, audio, RNG)
/// plus the storyboard being built.
pub struct OsbtkRuntime {
    context: Context,
    storyboard: Storyboard,
}

impl OsbtkRuntime {
    pub fn new(width: f32, height: f32) -> Self {
        OsbtkRuntime {
            context: Context::new(width, height),
            storyboard: Storyboard::new(),
        }
    }
}

/// Runs a JSON request against a runtime handle.
pub fn runtime_exec(runtime: &mut OsbtkRuntime, request: &str) -> String {
    let value: Value = match serde_json::from_str(request) {
        Ok(value) => value,
        Err(error) => return error_json(&error.to_string()),
    };
    match value.get("op").and_then(Value::as_str).unwrap_or("") {
        "Generator.importOsb" => import_osb(runtime, &value),
        "bakeScene" => bake_scene(runtime, &value),
        "Generator.loadBeatmap" => load_beatmap(runtime, &value),
        "Generator.beatmap" => beatmap(runtime),
        "Generator.timingPointAt" => timing_point_at(runtime, &value),
        "Generator.controlPointAt" => control_point_at(runtime, &value),
        "Generator.layerHandle" => layer_handle(runtime, &value),
        "Layer.createSprite" => create_sprite(runtime, &value),
        "Layer.createAnimation" => create_animation(runtime, &value),
        "Sprite.command" => command(runtime, &value),
        "Sprite.startLoopGroup" => start_loop_group(runtime, &value),
        "Sprite.endGroup" => end_group(runtime, &value),
        "Sprite.startTriggerGroup" => start_trigger_group(runtime, &value),
        "Layer.createSample" => create_sample(runtime, &value),
        "Layer.createSegment" => create_segment(runtime, &value),
        "Layer.reverseDepth" => segment_reverse_depth(runtime, &value),
        "Layer.addIndex" => segment_add_index(runtime, &value),
        "Layer.indexes" => segment_indexes(runtime, &value),
        "Layer.changePosition" => segment_change_position(runtime, &value),
        "Layer.moveToTop" => segment_move_to_top(runtime, &value),
        "Layer.moveToBottom" => segment_move_to_bottom(runtime, &value),
        "Layer.clean" => segment_clean(runtime, &value),
        "Layer.discard" => segment_discard(runtime, &value),
        "Layer.discardSprite" => segment_discard_sprite(runtime, &value),
        "Audio.getFft" => get_fft(runtime, &value),
        "Sprite.stateAt" => sprite_state_at(runtime, &value),
        "Sprite.info" => sprite_info(runtime, &value),
        "Generator.random" => random(runtime, &value),
        "Core.applyEasing" => apply_easing(&value),
        "Generator.spriteCount" => to_json(&SpriteCountResult {
            sprite_count: runtime.storyboard.sprite_count(),
        }),
        "Generator.storyboard" => to_json(&ApiStoryboard::from(&runtime.storyboard)),
        "Audio.audioDuration" => to_json(&AudioDurationResult {
            audio_duration: runtime.context.audio_duration(),
        }),
        other => error_json(&format!("unknown runtime op: {other}")),
    }
}

fn handle(value: &Value) -> Option<SpriteHandle> {
    Some(SpriteHandle {
        layer: value.get("layer")?.as_u64()? as usize,
        sprite: value.get("sprite")?.as_u64()? as usize,
    })
}

fn layer_of(value: &Value) -> Option<RuntimeLayerHandle> {
    Some(RuntimeLayerHandle(value.get("layer")?.as_u64()? as usize))
}

/// A sprite/sample creation target: a segment handle or a layer name.
enum LayerTarget {
    Handle(usize),
    Name(String),
}

fn layer_target(value: &Value) -> Option<LayerTarget> {
    if let Some(id) = value.get("layerHandle").and_then(Value::as_u64) {
        return Some(LayerTarget::Handle(id as usize));
    }
    value
        .get("layer")
        .and_then(Value::as_str)
        .map(|name| LayerTarget::Name(name.to_string()))
}

fn osb_layer(value: &Value) -> OsbLayer {
    if let Some(layer) = enum_field::<ApiLayer>(value, "osbLayer") {
        return layer;
    }
    match value.get("osbLayer").and_then(Value::as_u64).unwrap_or(0) {
        1 => OsbLayer::Fail,
        2 => OsbLayer::Pass,
        3 => OsbLayer::Foreground,
        4 => OsbLayer::Overlay,
        _ => OsbLayer::Background,
    }
}

fn scalar(value: &Value, key: &str) -> f64 {
    value.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn vec2(value: &Value, key: &str) -> Vec2 {
    let point = value.get(key);
    Vec2::new(
        point
            .and_then(|p| p.get("x"))
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        point
            .and_then(|p| p.get("y"))
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
    )
}

fn color4(value: &Value, key: &str) -> Color4 {
    let color = value.get(key);
    Color4::new(
        color
            .and_then(|c| c.get("r"))
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        color
            .and_then(|c| c.get("g"))
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        color
            .and_then(|c| c.get("b"))
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        color
            .and_then(|c| c.get("a"))
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
    )
}

fn origin(value: &Value) -> OsbOrigin {
    if let Some(origin) = enum_field::<ApiOrigin>(value, "origin") {
        return origin;
    }
    value
        .get("origin")
        .and_then(Value::as_u64)
        .map(|id| id as u8)
        .and_then(OsbOrigin::from_osb_id)
        .unwrap_or(OsbOrigin::Centre)
}

fn loop_type(value: &Value) -> OsbLoopType {
    if let Some(loop_type) = enum_field::<ApiLoopType>(value, "loopType") {
        return loop_type;
    }
    match value.get("loopType").and_then(Value::as_u64).unwrap_or(0) {
        1 => OsbLoopType::LoopForever,
        _ => OsbLoopType::LoopOnce,
    }
}

fn parameter_type(value: &Value) -> ParameterType {
    if let Some(parameter) = enum_field::<ApiParameter>(value, "parameter") {
        return parameter;
    }
    match value.get("parameter").and_then(Value::as_u64).unwrap_or(0) {
        1 => ParameterType::FlipVertical,
        2 => ParameterType::Additive,
        _ => ParameterType::FlipHorizontal,
    }
}

fn import_osb(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(text) = value.get("text").and_then(Value::as_str) else {
        return error_json("Generator.importOsb requires `text`");
    };
    osb_runtime::storyboarding::import_osb(&mut runtime.storyboard, text);
    to_json(&ImportOsbResult {
        sprite_count: runtime.storyboard.sprite_count(),
    })
}

/// Bakes a `{ worlds2d, worlds3d, options, options2d }` scene into the runtime's
/// storyboard (so a generator can compose baked worlds with authored sprites).
fn bake_scene(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    match crate::helpers::build_storyboard_scene(value) {
        Ok((scene, options2d, options3d)) => {
            scene.bake_into(&mut runtime.storyboard, &options2d, &options3d);
            to_json(&SpriteCountResult {
                sprite_count: runtime.storyboard.sprite_count(),
            })
        }
        Err(message) => error_json(&message),
    }
}

fn load_beatmap(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(text) = value.get("text").and_then(Value::as_str) else {
        return error_json("Generator.loadBeatmap requires `text`");
    };
    match parse_osu(text) {
        Ok(beatmap) => {
            let summary = LoadBeatmapResult {
                hit_objects: beatmap.hit_objects.len(),
                timing_points: beatmap.timing_points.len(),
            };
            runtime.context.set_beatmap(beatmap);
            to_json(&summary)
        }
        Err(error) => error_json(&error.to_string()),
    }
}

fn beatmap(runtime: &OsbtkRuntime) -> String {
    match runtime.context.beatmap() {
        Some(beatmap) => to_json(&Beatmap::from(beatmap)),
        None => "null".to_string(),
    }
}

fn timing_point_at(runtime: &OsbtkRuntime, value: &Value) -> String {
    let Some(beatmap) = runtime.context.beatmap() else {
        return "null".to_string();
    };
    to_json(&ControlPoint::from(
        &beatmap.timing_point_at_value(scalar(value, "time")),
    ))
}

fn control_point_at(runtime: &OsbtkRuntime, value: &Value) -> String {
    let Some(beatmap) = runtime.context.beatmap() else {
        return "null".to_string();
    };
    match beatmap.control_point_at(scalar(value, "time")) {
        Some(point) => to_json(&ControlPoint::from(point)),
        None => "null".to_string(),
    }
}

fn layer_handle(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(name) = value.get("name").and_then(Value::as_str) else {
        return error_json("Generator.layerHandle requires `name`");
    };
    let handle = runtime.storyboard.layer_handle(name);
    to_json(&LayerHandle { layer: handle.0 })
}

fn create_sprite(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(target) = layer_target(value) else {
        return error_json("Layer.createSprite requires `layer` or `layerHandle`");
    };
    let Some(path) = value.get("path").and_then(Value::as_str) else {
        return error_json("Layer.createSprite requires `path`");
    };
    let origin = origin(value);
    let with_position = value.get("position").is_some();
    let storyboard = &mut runtime.storyboard;
    let handle = match target {
        LayerTarget::Handle(id) => {
            let segment = RuntimeLayerHandle(id);
            if with_position {
                storyboard.create_sprite_at(
                    segment,
                    path.to_string(),
                    origin,
                    vec2(value, "position"),
                )
            } else {
                storyboard.create_sprite(segment, path.to_string(), origin)
            }
        }
        LayerTarget::Name(name) => {
            if with_position {
                storyboard.create_sprite_at(name, path.to_string(), origin, vec2(value, "position"))
            } else {
                storyboard.create_sprite(name, path.to_string(), origin)
            }
        }
    };
    to_json(&Handle {
        layer: handle.layer,
        sprite: handle.sprite,
    })
}

fn create_animation(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(target) = layer_target(value) else {
        return error_json("Layer.createAnimation requires `layer` or `layerHandle`");
    };
    let Some(path) = value.get("path").and_then(Value::as_str) else {
        return error_json("Layer.createAnimation requires `path`");
    };
    let frames = value.get("frames").and_then(Value::as_i64).unwrap_or(1) as i32;
    let storyboard = &mut runtime.storyboard;
    let handle = match target {
        LayerTarget::Handle(id) => storyboard.create_animation(
            RuntimeLayerHandle(id),
            path.to_string(),
            frames,
            scalar(value, "frameDelay"),
            loop_type(value),
            origin(value),
            vec2(value, "position"),
        ),
        LayerTarget::Name(name) => storyboard.create_animation(
            name,
            path.to_string(),
            frames,
            scalar(value, "frameDelay"),
            loop_type(value),
            origin(value),
            vec2(value, "position"),
        ),
    };
    to_json(&Handle {
        layer: handle.layer,
        sprite: handle.sprite,
    })
}

fn command(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = handle(value) else {
        return error_json("Sprite.command requires `layer`/`sprite`");
    };
    let start = scalar(value, "startTime");
    let end = value
        .get("endTime")
        .and_then(Value::as_f64)
        .unwrap_or(start);
    let easing = easing(value);
    let commands = &mut runtime.storyboard;
    match value.get("kind").and_then(Value::as_str).unwrap_or("") {
        "move" => commands.move_(
            handle,
            start,
            end,
            vec2(value, "start"),
            vec2(value, "end"),
            easing,
        ),
        "moveX" => commands.move_x(
            handle,
            start,
            end,
            scalar(value, "start"),
            scalar(value, "end"),
            easing,
        ),
        "moveY" => commands.move_y(
            handle,
            start,
            end,
            scalar(value, "start"),
            scalar(value, "end"),
            easing,
        ),
        "scale" => commands.scale(
            handle,
            start,
            end,
            scalar(value, "start"),
            scalar(value, "end"),
            easing,
        ),
        "scaleVec" => commands.scale_vec(
            handle,
            start,
            end,
            vec2(value, "start"),
            vec2(value, "end"),
            easing,
        ),
        "rotate" => commands.rotate(
            handle,
            start,
            end,
            scalar(value, "start"),
            scalar(value, "end"),
            easing,
        ),
        "fade" => commands.fade(
            handle,
            start,
            end,
            scalar(value, "start"),
            scalar(value, "end"),
            easing,
        ),
        "color" => commands.color(
            handle,
            start,
            end,
            color4(value, "start"),
            color4(value, "end"),
            easing,
        ),
        "colorHsb" => commands.color_hsb(
            handle,
            start,
            end,
            color4(value, "start"),
            color4(value, "end"),
            easing,
        ),
        "anchor" => commands.anchor(
            handle,
            start,
            end,
            vec2(value, "start"),
            vec2(value, "end"),
            easing,
        ),
        "flipH" | "flipV" | "additive" | "parameter" => commands.parameter(
            handle,
            start,
            end,
            parameter_type(value),
            scalar(value, "start"),
            scalar(value, "end"),
            easing,
        ),
        other => return error_json(&format!("unknown command: {other}")),
    }
    to_json(&OkResult { ok: true })
}

fn start_loop_group(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = handle(value) else {
        return error_json("Sprite.startLoopGroup requires `layer`/`sprite`");
    };
    runtime
        .storyboard
        .start_loop_group(handle, scalar(value, "startTime"), scalar(value, "count"));
    to_json(&OkResult { ok: true })
}

fn end_group(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = handle(value) else {
        return error_json("Sprite.endGroup requires `layer`/`sprite`");
    };
    runtime.storyboard.end_group(handle);
    to_json(&OkResult { ok: true })
}

fn start_trigger_group(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = handle(value) else {
        return error_json("Sprite.startTriggerGroup requires `layer`/`sprite`");
    };
    let name = value.get("name").and_then(Value::as_str).unwrap_or("");
    runtime.storyboard.start_trigger_group(
        handle,
        name.to_string(),
        scalar(value, "startTime"),
        scalar(value, "endTime"),
        value.get("group").and_then(Value::as_i64).unwrap_or(0) as i32,
    );
    to_json(&OkResult { ok: true })
}

fn create_sample(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(target) = layer_target(value) else {
        return error_json("Layer.createSample requires `layer` or `layerHandle`");
    };
    let Some(path) = value.get("path").and_then(Value::as_str) else {
        return error_json("Layer.createSample requires `path`");
    };
    let volume = value.get("volume").and_then(Value::as_f64).unwrap_or(100.0);
    let time = scalar(value, "time");
    match target {
        LayerTarget::Handle(id) => runtime
            .storyboard
            .segment_mut(RuntimeLayerHandle(id))
            .create_sample(path.to_string(), time, volume),
        LayerTarget::Name(name) => {
            runtime
                .storyboard
                .layer(name)
                .create_sample(path.to_string(), time, volume)
        }
    };
    to_json(&OkResult { ok: true })
}

fn create_segment(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(parent) = layer_of(value) else {
        return error_json("Layer.createSegment requires `layer`");
    };
    let handle = runtime.storyboard.create_segment(parent);
    to_json(&LayerHandle { layer: handle.0 })
}

fn segment_reverse_depth(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = layer_of(value) else {
        return error_json("Layer.reverseDepth requires `layer`");
    };
    let flag = value.get("value").and_then(Value::as_bool).unwrap_or(false);
    runtime
        .storyboard
        .segment_mut(handle)
        .set_reverse_depth(flag);
    to_json(&OkResult { ok: true })
}

fn segment_add_index(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = layer_of(value) else {
        return error_json("Layer.addIndex requires `layer`");
    };
    runtime
        .storyboard
        .segment_mut(handle)
        .add_index(scalar(value, "index"));
    to_json(&OkResult { ok: true })
}

fn segment_indexes(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = layer_of(value) else {
        return error_json("Layer.indexes requires `layer`");
    };
    let segment = runtime.storyboard.segment(handle);
    to_json(&SegmentIndexesResult {
        indexes: segment.indexes().to_vec(),
        lowest: segment.lowest_index(),
        highest: segment.highest_index(),
    })
}

fn segment_change_position(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = layer_of(value) else {
        return error_json("Layer.changePosition requires `layer`");
    };
    runtime.storyboard.change_position(handle, osb_layer(value));
    to_json(&OkResult { ok: true })
}

fn segment_move_to_top(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = layer_of(value) else {
        return error_json("Layer.moveToTop requires `layer`");
    };
    runtime.storyboard.move_to_top(handle);
    to_json(&OkResult { ok: true })
}

fn segment_move_to_bottom(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = layer_of(value) else {
        return error_json("Layer.moveToBottom requires `layer`");
    };
    runtime.storyboard.move_to_bottom(handle);
    to_json(&OkResult { ok: true })
}

fn segment_clean(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = layer_of(value) else {
        return error_json("Layer.clean requires `layer`");
    };
    let removed = runtime.storyboard.clean(handle);
    to_json(&SegmentCleanResult { removed })
}

fn segment_discard(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = layer_of(value) else {
        return error_json("Layer.discard requires `layer`");
    };
    let layer_count = runtime.storyboard.discard(handle);
    to_json(&SegmentDiscardResult { layer_count })
}

fn segment_discard_sprite(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = handle(value) else {
        return error_json("Layer.discardSprite requires `layer`/`sprite`");
    };
    let value = runtime.storyboard.discard_sprite(handle);
    to_json(&BoolResult { value })
}

fn get_fft(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let magnitudes = value
        .get("magnitudes")
        .and_then(Value::as_u64)
        .unwrap_or(64);
    let cutoff = value.get("cutoff").and_then(Value::as_f64).unwrap_or(0.0);
    to_json(&FftResult {
        magnitudes: runtime.context.get_fft(
            scalar(value, "time"),
            magnitudes as f64,
            None,
            easing(value),
            cutoff,
        ),
    })
}

fn sprite_state_at(runtime: &OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = handle(value) else {
        return error_json("Sprite.stateAt requires `layer`/`sprite`");
    };
    let time = scalar(value, "time");
    let sprite = runtime.storyboard.sprite(handle);
    to_json(&SpriteState {
        position: sprite.position_at(time),
        scale: sprite.scale_at(time),
        rotation: sprite.rotation_at(time),
        opacity: sprite.opacity_at(time),
        additive: sprite.additive_at(time),
        color: sprite.color_at(time),
        anchor: sprite.anchor_at(time),
    })
}

fn sprite_info(runtime: &OsbtkRuntime, value: &Value) -> String {
    let Some(handle) = handle(value) else {
        return error_json("Sprite.info requires `layer`/`sprite`");
    };
    let sprite = runtime.storyboard.sprite(handle);
    to_json(&SpriteInfo {
        texture_path: sprite.texture_path.clone(),
        origin: sprite.origin,
        position: sprite.initial_position,
        scale: sprite.initial_scale,
        rotation: sprite.initial_rotation,
        opacity: sprite.initial_opacity,
        color: sprite.initial_color,
        additive: sprite.additive,
        flip_h: sprite.flip_h,
        flip_v: sprite.flip_v,
        animation: sprite.animation.map(AnimationMeta::from),
        loop_count: sprite.loops().len(),
        trigger_count: sprite.triggers().len(),
    })
}

fn random(runtime: &mut OsbtkRuntime, value: &Value) -> String {
    let min = scalar(value, "min");
    let max = value.get("max").and_then(Value::as_f64).unwrap_or(1.0);
    to_json(&RandomResult {
        value: runtime.context.random(min, max),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(runtime: &mut OsbtkRuntime, request: Value) -> Value {
        serde_json::from_str(&runtime_exec(runtime, &request.to_string())).unwrap()
    }

    #[test]
    fn builds_and_samples_a_sprite() {
        let mut runtime = OsbtkRuntime::new(640.0, 480.0);
        let handle = run(
            &mut runtime,
            serde_json::json!({ "op": "Layer.createSprite", "layer": "Foreground", "path": "sb/dot.png" }),
        );
        let (layer, sprite) = (
            handle["layer"].as_u64().unwrap(),
            handle["sprite"].as_u64().unwrap(),
        );

        run(
            &mut runtime,
            serde_json::json!({
                "op": "Sprite.command", "kind": "move", "layer": layer, "sprite": sprite,
                "startTime": 0, "endTime": 1000,
                "start": { "x": 0, "y": 0 }, "end": { "x": 100, "y": 0 }
            }),
        );
        let state = run(
            &mut runtime,
            serde_json::json!({ "op": "Sprite.stateAt", "layer": layer, "sprite": sprite, "time": 500 }),
        );
        assert_eq!(state["position"]["x"], 50.0);
    }

    #[test]
    fn animates_and_samples_a_sprite_anchor() {
        let mut runtime = OsbtkRuntime::new(640.0, 480.0);
        let handle = run(
            &mut runtime,
            serde_json::json!({ "op": "Layer.createSprite", "layer": "Foreground", "path": "sb/dot.png" }),
        );
        let (layer, sprite) = (
            handle["layer"].as_u64().unwrap(),
            handle["sprite"].as_u64().unwrap(),
        );
        run(
            &mut runtime,
            serde_json::json!({
                "op": "Sprite.command", "kind": "anchor", "layer": layer, "sprite": sprite,
                "startTime": 0, "endTime": 1000,
                "start": { "x": 0, "y": 0 }, "end": { "x": 100, "y": 50 }
            }),
        );
        let state = run(
            &mut runtime,
            serde_json::json!({ "op": "Sprite.stateAt", "layer": layer, "sprite": sprite, "time": 500 }),
        );
        assert_eq!(state["anchor"]["x"], 50.0);
        assert_eq!(state["anchor"]["y"], 25.0);
    }

    #[test]
    fn reads_back_the_built_storyboard() {
        let mut runtime = OsbtkRuntime::new(640.0, 480.0);
        let handle = run(
            &mut runtime,
            serde_json::json!({ "op": "Layer.createSprite", "layer": "Foreground", "path": "sb/dot.png", "origin": "Centre" }),
        );
        let (layer, sprite) = (
            handle["layer"].as_u64().unwrap(),
            handle["sprite"].as_u64().unwrap(),
        );
        run(
            &mut runtime,
            serde_json::json!({
                "op": "Sprite.command", "kind": "move", "layer": layer, "sprite": sprite,
                "startTime": 0, "endTime": 1000,
                "start": { "x": 0, "y": 0 }, "end": { "x": 100, "y": 0 }
            }),
        );
        run(
            &mut runtime,
            serde_json::json!({
                "op": "Sprite.command", "kind": "fade", "layer": layer, "sprite": sprite,
                "startTime": 0, "endTime": 500, "start": 0, "end": 1
            }),
        );
        let storyboard = run(
            &mut runtime,
            serde_json::json!({ "op": "Generator.storyboard" }),
        );
        let foreground = storyboard["layers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["identifier"] == "Foreground")
            .expect("foreground layer");
        assert_eq!(foreground["sprites"][0]["texturePath"], "sb/dot.png");
        let commands = foreground["sprites"][0]["commands"].as_array().unwrap();
        assert_eq!(commands.len(), 2);
        let move_command = commands
            .iter()
            .find(|command| command["kind"] == "move")
            .expect("move command");
        assert_eq!(move_command["start"]["kind"], "vector2");
        assert!(commands.iter().any(|command| command["kind"] == "fade"));
    }

    #[test]
    fn accepts_string_and_legacy_numeric_origins() {
        let mut runtime = OsbtkRuntime::new(640.0, 480.0);
        let named = run(
            &mut runtime,
            serde_json::json!({ "op": "Layer.createSprite", "layer": "Foreground", "path": "a.png", "origin": "BottomLeft" }),
        );
        let info = run(
            &mut runtime,
            serde_json::json!({ "op": "Sprite.info", "layer": named["layer"], "sprite": named["sprite"] }),
        );
        assert_eq!(info["origin"], "BottomLeft");

        let numeric = run(
            &mut runtime,
            serde_json::json!({ "op": "Layer.createSprite", "layer": "Foreground", "path": "b.png", "origin": 6 }),
        );
        let info = run(
            &mut runtime,
            serde_json::json!({ "op": "Sprite.info", "layer": numeric["layer"], "sprite": numeric["sprite"] }),
        );
        assert_eq!(info["origin"], "BottomLeft");
    }

    #[test]
    fn imports_an_osb() {
        let mut runtime = OsbtkRuntime::new(640.0, 480.0);
        let value = run(
            &mut runtime,
            serde_json::json!({
                "op": "Generator.importOsb",
                "text": "[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n"
            }),
        );
        assert_eq!(value["spriteCount"], 1);
    }

    #[test]
    fn creates_an_animation_and_loops() {
        let mut runtime = OsbtkRuntime::new(640.0, 480.0);
        let handle = run(
            &mut runtime,
            serde_json::json!({
                "op": "Layer.createAnimation", "layer": "Foreground", "path": "sb/frame.png",
                "frames": 4, "frameDelay": 50.0, "loopType": "LoopForever", "origin": "Centre"
            }),
        );
        let (layer, sprite) = (
            handle["layer"].as_u64().unwrap(),
            handle["sprite"].as_u64().unwrap(),
        );
        run(
            &mut runtime,
            serde_json::json!({ "op": "Sprite.startLoopGroup", "layer": layer, "sprite": sprite, "startTime": 0, "count": 3 }),
        );
        run(
            &mut runtime,
            serde_json::json!({
                "op": "Sprite.command", "kind": "fade", "layer": layer, "sprite": sprite,
                "startTime": 0, "endTime": 100, "start": 0, "end": 1
            }),
        );
        run(
            &mut runtime,
            serde_json::json!({ "op": "Sprite.endGroup", "layer": layer, "sprite": sprite }),
        );
        let info = run(
            &mut runtime,
            serde_json::json!({ "op": "Sprite.info", "layer": layer, "sprite": sprite }),
        );
        assert_eq!(info["animation"]["frames"], 4);
        assert_eq!(info["animation"]["loopType"], "LoopForever");
        assert_eq!(info["loopCount"], 1);
    }

    #[test]
    fn creates_segments_and_discards_sprites() {
        let mut runtime = OsbtkRuntime::new(640.0, 480.0);
        let group = run(
            &mut runtime,
            serde_json::json!({ "op": "Generator.layerHandle", "name": "Group" }),
        );
        let group_id = group["layer"].as_u64().unwrap();
        let segment = run(
            &mut runtime,
            serde_json::json!({ "op": "Layer.createSegment", "layer": group_id }),
        );
        let segment_id = segment["layer"].as_u64().unwrap();

        let handle = run(
            &mut runtime,
            serde_json::json!({ "op": "Layer.createSprite", "layerHandle": segment_id, "path": "sb/dot.png" }),
        );
        run(
            &mut runtime,
            serde_json::json!({
                "op": "Sprite.command", "kind": "fade", "layer": handle["layer"], "sprite": handle["sprite"],
                "startTime": 0, "endTime": 100, "start": 0, "end": 1
            }),
        );
        let count = run(
            &mut runtime,
            serde_json::json!({ "op": "Generator.spriteCount" }),
        );
        assert_eq!(count["spriteCount"], 1);

        run(
            &mut runtime,
            serde_json::json!({ "op": "Layer.reverseDepth", "layer": segment_id, "value": true }),
        );
        run(
            &mut runtime,
            serde_json::json!({ "op": "Layer.addIndex", "layer": segment_id, "index": 3 }),
        );
        let indexes = run(
            &mut runtime,
            serde_json::json!({ "op": "Layer.indexes", "layer": segment_id }),
        );
        assert_eq!(indexes["indexes"][0], 3.0);
        assert_eq!(indexes["highest"], 3.0);

        let discarded = run(
            &mut runtime,
            serde_json::json!({
                "op": "Layer.discardSprite", "layer": handle["layer"], "sprite": handle["sprite"]
            }),
        );
        assert_eq!(discarded["value"], true);
        let count = run(
            &mut runtime,
            serde_json::json!({ "op": "Generator.spriteCount" }),
        );
        assert_eq!(count["spriteCount"], 0);
    }

    #[test]
    fn loads_and_queries_a_beatmap() {
        let mut runtime = OsbtkRuntime::new(640.0, 480.0);
        run(
            &mut runtime,
            serde_json::json!({
                "op": "Generator.loadBeatmap",
                "text": "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n\
                         [TimingPoints]\n0,500,4,2,1,100,1,0\n\
                         [HitObjects]\n256,192,1000,1,0\n"
            }),
        );
        let point = run(
            &mut runtime,
            serde_json::json!({ "op": "Generator.timingPointAt", "time": 1500 }),
        );
        assert_eq!(point["bpm"], 120.0);
        let beatmap = run(
            &mut runtime,
            serde_json::json!({ "op": "Generator.beatmap" }),
        );
        assert_eq!(beatmap["audioFilename"], "song.mp3");
        assert_eq!(beatmap["hitObjects"][0]["kind"], "Circle");
    }

    #[test]
    fn applies_easing_by_name_and_id() {
        let mut runtime = OsbtkRuntime::new(1.0, 1.0);
        let named = run(
            &mut runtime,
            serde_json::json!({ "op": "Core.applyEasing", "easing": "InQuad", "t": 0.5 }),
        );
        assert!((named["value"].as_f64().unwrap() - 0.25).abs() < 1e-6);

        let numeric = run(
            &mut runtime,
            serde_json::json!({ "op": "Core.applyEasing", "id": 3, "t": 0.5 }),
        );
        assert!((numeric["value"].as_f64().unwrap() - 0.25).abs() < 1e-6);
    }
}
