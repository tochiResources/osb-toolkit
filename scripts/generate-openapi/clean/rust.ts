import { writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { targets } from '../shared';
import {
    cleanResultInfo,
    optionFor,
    orderedProperties,
    readContract,
    refName,
    snake,
} from './shared';
import type { Operation, OptionModel, OptionProperty } from './shared';

/** The stateless tags that get a clean Rust client. */
const rustCleanTags = [
    'Audio',
    'Core',
    'Diagnostics',
    'Helpers',
    'Image',
    'Imagegen',
    'Media',
    'Optimize',
    'Storyboarding3d',
] as const;

/** The handle-bound runtime tags; their clients borrow a `Runtime`. */
const rustRuntimeTags = ['Generator', 'Layer', 'Sprite'] as const;

function rustTypeName(property: OptionProperty): string {
    const inner =
        property.schema?.oneOf?.find((part: any) => part.type !== 'null') ?? property.schema;
    const types = Array.isArray(inner?.type)
        ? inner.type.filter((type: string) => type !== 'null')
        : [inner?.type];
    let base: string;
    if (inner?.$ref) {
        base = rustModelName(refName(inner)!);
    } else if (types.includes('array')) {
        base = `Vec<${rustTypeName({
            name: property.name,
            required: true,
            type: 'unknown',
            nullable: false,
            refs: [],
            schema: inner?.items ?? {},
        } as OptionProperty)}>`;
    } else {
        switch (types[0]) {
            case 'integer':
                base = 'i32';
                break;
            case 'number':
                base = 'f64';
                break;
            case 'boolean':
                base = 'bool';
                break;
            case 'string':
                base = 'String';
                break;
            default:
                base = 'serde_json::Value';
        }
    }
    return property.required ? base : `Option<${base}>`;
}

function rustModelName(type: string): string {
    return type === 'unknown' ? 'serde_json::Value' : `models::${type}`;
}

function rustResultType(tag: string, op: Operation): string {
    const info = cleanResultInfo(tag, op);
    switch (info.kind) {
        case 'void':
            return '()';
        case 'bool':
            return 'bool';
        case 'int':
        case 'layer':
            return 'i32';
        case 'float':
            return 'f64';
        case 'string':
            return 'String';
        case 'point':
            return 'models::OptionalPointResult';
        case 'unknown':
            return 'serde_json::Value';
        default:
            return info.array
                ? `Vec<models::${info.raw}>`
                : `models::${info.raw ?? 'serde_json::Value'}`;
    }
}

function rustBody(tag: string, op: Operation): string {
    const key = `${tag}.${op.name}`;
    switch (key) {
        case 'Core.version':
            return 'crate::Client::version()';
        case 'Core.length':
            return 'crate::Client::curve_length(options.curve)';
        case 'Core.boundingBox':
            return 'crate::Client::curve_bounding_box(options.curve)';
        case 'Core.point':
            return 'crate::Client::curve_point(\n            options.curve,\n            options.t,\n            options.by_arc_length.unwrap_or(false),\n        )';
        case 'Core.tangent':
            return 'crate::Client::curve_tangent(options.curve, options.t)';
        case 'Core.sample':
            return 'crate::Client::curve_sample(\n            options.curve,\n            options.count as usize,\n            options.by_arc_length.unwrap_or(false),\n        )';
        case 'Core.simplify':
            return 'crate::Client::simplify(options.points, options.tolerance)';
        case 'Core.normal':
            return 'crate::Client::curve_normal(options.curve, options.t)';
        case 'Core.curvature':
            return 'crate::Client::curve_curvature(options.curve, options.t)';
        case 'Core.distanceTo':
            return 'crate::Client::curve_distance_to(options.curve, options.point)';
        case 'Core.offset':
            return 'crate::Client::curve_offset(options.curve, options.distance)';
        case 'Core.arc':
            return 'crate::Client::curve_arc(\n            options.center,\n            options.radius,\n            options.start_angle,\n            options.sweep_angle,\n            options.count.unwrap_or(64).max(0) as usize,\n        )';
        case 'Core.ellipse':
            return 'crate::Client::curve_ellipse(\n            options.center,\n            options.radius_x,\n            options.radius_y,\n            options.count.unwrap_or(64).max(0) as usize,\n        )';
        case 'Core.lineIntersect':
            return 'crate::Client::line_intersect(options.a1, options.a2, options.b1, options.b2)';
        case 'Core.applyEasing':
            return 'crate::Client::apply_easing(options.easing, options.t)';
        case 'Core.easingValue':
            return 'crate::Client::easing_value(options.t, options.easing, options.cubic_bezier)';
        case 'Core.parseOsu':
            return 'crate::Client::parse_beatmap(&options.text)';
        case 'Core.parseOsb':
            return 'crate::Client::parse_osb(&options.text)';
        case 'Core.getOriginVector':
            return 'crate::Client::get_origin_vector(options.origin, options.width, options.height)';
        case 'Core.inScreenBounds':
            return 'crate::Client::in_screen_bounds(\n            options.position,\n            options.size,\n            options.rotation,\n            options.origin,\n        )';
        case 'Audio.fft':
            return 'crate::Client::fft(\n            &options.path,\n            options.time.unwrap_or(0.0),\n            options.magnitudes.unwrap_or(64).max(0) as usize,\n        )';
        case 'Audio.getFft':
            return [
                'let runtime = self.runtime.as_mut().ok_or_else(|| {',
                '            Error::Bridge("Audio.getFft needs a runtime".to_string())',
                '        })?;',
                '        let mut request = json!({',
                '            "op": "Audio.getFft",',
                '            "time": options.time,',
                '            "magnitudes": options.magnitudes,',
                '        });',
                '        if let Some(cutoff) = options.cutoff { request["cutoff"] = json!(cutoff); }',
                '        if let Some(easing) = options.easing { request["easing"] = json!(easing); }',
                '        runtime.exec(request)',
            ].join('\n');
        case 'Audio.audioDuration':
            return [
                'let runtime = self.runtime.as_mut().ok_or_else(|| {',
                '            Error::Bridge("Audio.audioDuration needs a runtime".to_string())',
                '        })?;',
                '        let result: models::AudioDurationResult = runtime.exec(json!({',
                '            "op": "Audio.audioDuration"',
                '        }))?;',
                '        Ok(result.audio_duration)',
            ].join('\n');
        case 'Diagnostics.diagnoseOsb':
            return 'crate::Client::diagnose_osb(&options.text)';
        case 'Diagnostics.diagnoseOsu':
            return 'crate::Client::diagnose_osu(&options.text)';
        case 'Optimize.optimizeOsb':
            return 'crate::Client::optimize_osb(&options.text, options.options)';
        case 'Image.info':
            return 'crate::Client::image_info(&options.path)';
        case 'Image.process':
            return 'crate::Client::image_process(&options.path, &options.output, options.options)';
        case 'Imagegen.renderShapes':
            return 'crate::Client::render_shapes(models::RenderShapesRequest {\n            background: options.background.map(|value| Some(Box::new(value))),\n            height: options.height,\n            output: options.output,\n            shapes: options.shapes,\n            width: options.width,\n        })';
        case 'Media.info':
            return 'crate::Client::media_info(&options.path)';
        case 'Media.extractFrames':
            return 'crate::Client::extract_frames(models::ExtractFramesRequest {\n            count: options.count.map(Some),\n            fps: options.fps.map(Some),\n            height: options.height.map(Some),\n            input: options.input,\n            output_dir: options.output_dir,\n            start: options.start.map(Some),\n            width: options.width.map(Some),\n        })';
        case 'Media.encodeVideo':
            return 'crate::Client::encode_video(models::EncodeVideoRequest {\n            codec: options.codec.map(Some),\n            crf: options.crf.map(Some),\n            fps: options.fps,\n            input_pattern: options.input_pattern,\n            output: options.output,\n        })';
        case 'Media.extractAudio':
            return 'crate::Client::extract_audio(&options.input, &options.output)';
        case 'Storyboarding3d.bake3d':
            return 'crate::Client::bake3d(options.input)';
        case 'Helpers.call':
            return 'helper("Helpers.call", options)';
        case 'Helpers.commands':
            return 'helper("Helpers.commands", options)';
        case 'Helpers.keyframes':
            return 'helper("Helpers.keyframes", options)';
        default:
            throw new Error(`clean (rust): no body for ${key}`);
    }
}

function rustRuntimeResultType(tag: string, op: Operation): string {
    if (tag === 'Generator' && op.name === 'beatmap') return 'Option<models::Beatmap>';
    if (tag === 'Generator' && op.name === 'controlPointAt') {
        return 'Option<models::ControlPoint>';
    }
    if (tag === 'Layer' && (op.name === 'createSprite' || op.name === 'createAnimation')) {
        return "Sprite<'_>";
    }
    if (tag === 'Layer' && op.name === 'createSegment') return "Layer<'_>";
    return rustResultType(tag, op);
}

function rustRuntimeBody(tag: string, op: Operation): string {
    const key = `${tag}.${op.name}`;
    switch (key) {
        case 'Generator.beatmap':
            return 'self.runtime.exec(json!({ "op": "Generator.beatmap" }))';
        case 'Generator.controlPointAt':
            return 'self.runtime.exec(json!({ "op": "Generator.controlPointAt", "time": options.time }))';
        case 'Generator.importOsb':
            return 'let result: models::ImportOsbResult = self.runtime.exec(json!({ "op": "Generator.importOsb", "text": options.text }))?;\n        Ok(result.sprite_count)';
        case 'Generator.layerHandle':
            return 'let result: models::LayerHandle = self.runtime.exec(json!({ "op": "Generator.layerHandle", "name": options.name }))?;\n        Ok(result.layer)';
        case 'Generator.loadBeatmap':
            return 'self.runtime.exec(json!({ "op": "Generator.loadBeatmap", "text": options.text }))';
        case 'Generator.random':
            return 'let result: models::RandomResult = self.runtime.exec(json!({ "op": "Generator.random", "max": options.max, "min": options.min }))?;\n        Ok(result.value)';
        case 'Generator.spriteCount':
            return 'let result: models::SpriteCountResult = self.runtime.exec(json!({ "op": "Generator.spriteCount" }))?;\n        Ok(result.sprite_count)';
        case 'Generator.storyboard':
            return 'self.runtime.exec(json!({ "op": "Generator.storyboard" }))';
        case 'Generator.timingPointAt':
            return 'self.runtime.exec(json!({ "op": "Generator.timingPointAt", "time": options.time }))';
        case 'Layer.addIndex':
            return 'let _: models::OkResult = self.runtime.exec(json!({ "op": "Layer.addIndex", "layer": self.handle.layer, "index": options.index }))?;\n        Ok(())';
        case 'Layer.changePosition':
            return 'let _: models::OkResult = self.runtime.exec(json!({ "op": "Layer.changePosition", "layer": self.handle.layer, "osbLayer": options.osb_layer }))?;\n        Ok(())';
        case 'Layer.clean':
            return 'let result: models::SegmentCleanResult = self.runtime.exec(json!({ "op": "Layer.clean", "layer": self.handle.layer }))?;\n        Ok(result.removed)';
        case 'Layer.createAnimation':
            return [
                'let handle: models::Handle = self.runtime.exec(json!({',
                '            "op": "Layer.createAnimation",',
                '            "layerHandle": self.handle.layer,',
                '            "path": options.path,',
                '            "frames": options.frames,',
                '            "frameDelay": options.frame_delay,',
                '            "loopType": options.loop_type,',
                '            "origin": options.origin,',
                '            "position": options.position,',
                '        }))?;',
                '        Ok(Sprite { runtime: &mut *self.runtime, handle })',
            ].join('\n');
        case 'Layer.createSample':
            return [
                'let mut request = json!({',
                '            "op": "Layer.createSample",',
                '            "layerHandle": self.handle.layer,',
                '            "path": options.path,',
                '            "time": options.time,',
                '            "volume": options.volume.unwrap_or(100.0),',
                '        });',
                '        if let Some(layer) = options.layer { request["layer"] = json!(layer); }',
                '        let _: models::OkResult = self.runtime.exec(request)?;',
                '        Ok(())',
            ].join('\n');
        case 'Layer.createSegment':
            return 'let handle: models::LayerHandle = self.runtime.exec(json!({ "op": "Layer.createSegment", "layer": self.handle.layer }))?;\n        Ok(Layer { runtime: &mut *self.runtime, handle })';
        case 'Layer.createSprite':
            return [
                'let mut request = json!({',
                '            "op": "Layer.createSprite",',
                '            "layerHandle": self.handle.layer,',
                '            "path": options.path,',
                '            "origin": options.origin,',
                '        });',
                '        if let Some(position) = options.position { request["position"] = json!(position); }',
                '        if let Some(layer) = options.layer { request["layer"] = json!(layer); }',
                '        let handle: models::Handle = self.runtime.exec(request)?;',
                '        Ok(Sprite { runtime: &mut *self.runtime, handle })',
            ].join('\n');
        case 'Layer.discard':
            return 'let result: models::SegmentDiscardResult = self.runtime.exec(json!({ "op": "Layer.discard", "layer": self.handle.layer }))?;\n        Ok(result.layer_count)';
        case 'Layer.discardSprite':
            return 'let result: models::BoolResult = self.runtime.exec(json!({ "op": "Layer.discardSprite", "layer": self.handle.layer, "sprite": sprite }))?;\n        Ok(result.value)';
        case 'Layer.indexes':
            return 'self.runtime.exec(json!({ "op": "Layer.indexes", "layer": self.handle.layer }))';
        case 'Layer.moveToBottom':
            return 'let _: models::OkResult = self.runtime.exec(json!({ "op": "Layer.moveToBottom", "layer": self.handle.layer }))?;\n        Ok(())';
        case 'Layer.moveToTop':
            return 'let _: models::OkResult = self.runtime.exec(json!({ "op": "Layer.moveToTop", "layer": self.handle.layer }))?;\n        Ok(())';
        case 'Layer.reverseDepth':
            return 'let _: models::OkResult = self.runtime.exec(json!({ "op": "Layer.reverseDepth", "layer": self.handle.layer, "value": options.value }))?;\n        Ok(())';
        case 'Sprite.command':
            return [
                'let mut request = json!({',
                '            "op": "Sprite.command",',
                '            "kind": options.kind,',
                '            "layer": self.handle.layer,',
                '            "sprite": self.handle.sprite,',
                '            "startTime": options.start_time,',
                '            "endTime": options.end_time,',
                '            "easing": options.easing,',
                '        });',
                '        if let Some(start) = options.start { request["start"] = start; }',
                '        if let Some(end) = options.end { request["end"] = end; }',
                '        if let Some(parameter) = options.parameter { request["parameter"] = json!(parameter); }',
                '        let _: models::OkResult = self.runtime.exec(request)?;',
                '        Ok(())',
            ].join('\n');
        case 'Sprite.endGroup':
            return 'let _: models::OkResult = self.runtime.exec(json!({ "op": "Sprite.endGroup", "layer": self.handle.layer, "sprite": self.handle.sprite }))?;\n        Ok(())';
        case 'Sprite.info':
            return 'self.runtime.exec(json!({ "op": "Sprite.info", "layer": self.handle.layer, "sprite": self.handle.sprite }))';
        case 'Sprite.startLoopGroup':
            return 'let _: models::OkResult = self.runtime.exec(json!({ "op": "Sprite.startLoopGroup", "layer": self.handle.layer, "sprite": self.handle.sprite, "startTime": options.start_time, "count": options.count }))?;\n        Ok(())';
        case 'Sprite.startTriggerGroup':
            return 'let _: models::OkResult = self.runtime.exec(json!({ "op": "Sprite.startTriggerGroup", "layer": self.handle.layer, "sprite": self.handle.sprite, "name": options.name, "startTime": options.start_time, "endTime": options.end_time, "group": options.group }))?;\n        Ok(())';
        case 'Sprite.stateAt':
            return 'self.runtime.exec(json!({ "op": "Sprite.stateAt", "layer": self.handle.layer, "sprite": self.handle.sprite, "time": options.time }))';
        default:
            throw new Error(`clean (rust): no runtime body for ${key}`);
    }
}

/** Whether an operation belongs on the stateless Rust clean surface. */
function isRustCleanOp(_tag: string, _op: Operation): boolean {
    return true;
}

function rustFieldName(tag: string): string {
    if (tag === 'Imagegen') return 'image_gen';
    if (tag === 'Storyboarding3d') return 'storyboarding3d';
    return tag.toLowerCase();
}

function rustMethod(tag: string, op: Operation, option: OptionModel | undefined): string {
    const hasOptions = (option?.properties.length ?? 0) > 0;
    const args = hasOptions ? `, options: ${option!.name}` : '';
    const doc = op.description ? `    /// ${op.description.replace(/\s+/g, ' ').trim()}\n` : '';
    return `${doc}    pub fn ${snake(op.name)}(&self${args}) -> Result<${rustResultType(
        tag,
        op
    )}, Error> {\n        ${rustBody(tag, op)}\n    }`;
}

function rustRuntimeMethod(tag: string, op: Operation, option: OptionModel | undefined): string {
    const hasOptions = (option?.properties.length ?? 0) > 0;
    const doc = op.description ? `    /// ${op.description.replace(/\s+/g, ' ').trim()}\n` : '';
    const args =
        tag === 'Layer' && op.body === 'HandleRequest'
            ? ', sprite: i32'
            : hasOptions
              ? `, options: ${option!.name}`
              : '';
    return `${doc}    pub fn ${snake(op.name)}(&mut self${args}) -> Result<${rustRuntimeResultType(
        tag,
        op
    )}, Error> {\n        ${rustRuntimeBody(tag, op)}\n    }`;
}

function emitAudioClass(operations: Operation[], contract: Contract): string {
    const methods = operations
        .map(op => {
            const option = optionFor(contract, op);
            const hasOptions = (option?.properties.length ?? 0) > 0;
            const args = hasOptions ? `, options: ${option!.name}` : '';
            const doc = op.description
                ? `    /// ${op.description.replace(/\s+/g, ' ').trim()}\n`
                : '';
            const mutable =
                op.name === 'getFft' || op.name === 'audioDuration' ? '&mut self' : '&self';
            return `${doc}    pub fn ${snake(op.name)}(${mutable}${args}) -> Result<${rustResultType(
                'Audio',
                op
            )}, Error> {\n        ${rustBody('Audio', op)}\n    }`;
        })
        .join('\n\n');
    return `/// The clean \`Audio\` client. \`fft\` is stateless; \`get_fft\` and
/// \`audio_duration\` need a live runtime.
pub struct Audio<'a> {
    runtime: Option<&'a mut crate::Runtime>,
}

impl<'a> Audio<'a> {
${methods}
}`;
}

function emitRuntimeClass(tag: string, operations: Operation[], contract: Contract): string {
    const methods = operations
        .map(op => rustRuntimeMethod(tag, op, optionFor(contract, op)))
        .join('\n\n');
    if (tag === 'Generator') {
        return `/// The clean \`Generator\` client bound to a live runtime.
pub struct Generator<'a> {
    runtime: &'a mut crate::Runtime,
}

impl<'a> Generator<'a> {
${methods}
}`;
    }
    if (tag === 'Sprite') {
        return `/// The clean \`Sprite\` client bound to a live runtime.
pub struct Sprite<'a> {
    runtime: &'a mut crate::Runtime,
    handle: models::Handle,
}

impl<'a> Sprite<'a> {
    /// The sprite's layer handle.
    pub fn layer(&self) -> i32 {
        self.handle.layer
    }

    /// The sprite's index within its layer.
    pub fn sprite(&self) -> i32 {
        self.handle.sprite
    }

${methods}
}`;
    }
    return `/// The clean \`Layer\` client bound to a live runtime.
pub struct Layer<'a> {
    runtime: &'a mut crate::Runtime,
    handle: models::LayerHandle,
}

impl<'a> Layer<'a> {
    /// The segment's handle.
    pub fn handle(&self) -> models::LayerHandle {
        self.handle.clone()
    }

${methods}
}`;
}

/** Writes the generated clean Rust clients and `<X>Options` types. */
export function emitRustClean(): void {
    const contract = readContract();
    const options = new Map<string, OptionModel>();
    for (const tag of [...rustCleanTags, ...rustRuntimeTags]) {
        for (const op of contract.operations.get(tag) ?? []) {
            if (!isRustCleanOp(tag, op)) continue;
            const option = optionFor(contract, op);
            if (option && option.properties.length > 0) options.set(option.name, option);
        }
    }

    const structs = [...options.values()]
        .sort((a, b) => a.name.localeCompare(b.name))
        .map(option => {
            const fields = orderedProperties(option)
                .map(property => {
                    const doc = property.description
                        ? `    /// ${property.description.replace(/\s+/g, ' ').trim()}\n`
                        : '';
                    return `${doc}    pub ${snake(property.name)}: ${rustTypeName(property)},`;
                })
                .join('\n');
            const doc = option.description
                ? `/// ${option.description.replace(/\s+/g, ' ').trim()}\n`
                : '';
            return `${doc}#[derive(Clone, Debug)]\npub struct ${option.name} {\n${fields}\n}`;
        })
        .join('\n\n');

    const statelessClasses = rustCleanTags
        .filter(tag => tag !== 'Audio')
        .map(tag => {
            const methods = (contract.operations.get(tag) ?? [])
                .filter(op => isRustCleanOp(tag, op))
                .map(op => rustMethod(tag, op, optionFor(contract, op)))
                .join('\n\n');
            return `/// The clean \`${tag}\` client.\npub struct ${tag};\n\nimpl ${tag} {\n${methods}\n}`;
        })
        .join('\n\n');

    const audioClass = emitAudioClass(contract.operations.get('Audio') ?? [], contract);
    const runtimeClasses = rustRuntimeTags
        .map(tag => emitRuntimeClass(tag, contract.operations.get(tag) ?? [], contract))
        .join('\n\n');

    const composition = rustCleanTags
        .map(tag => {
            const type = tag === 'Audio' ? "Audio<'static>" : tag;
            return `    pub ${rustFieldName(tag)}: ${type},`;
        })
        .join('\n');
    const initializers = rustCleanTags
        .map(tag => {
            const value = tag === 'Audio' ? 'Audio { runtime: None }' : tag;
            return `            ${rustFieldName(tag)}: ${value},`;
        })
        .join('\n');

    const text = `//! The clean high-level Rust clients: one class per contract tag with
//! \`<X>Options\` inputs, mirroring the generated clean clients in the other SDKs.
//!
//! The stateless classes wrap \`crate::Client\`; [\`Runtime\`] owns a live
//! [\`crate::Runtime\`] and exposes the handle-bound \`Generator\`, \`Layer\` and
//! \`Sprite\` clients.

use serde_json::{json, Value};

use crate::error::Error;
use crate::models;

${structs}

${statelessClasses}

${audioClass}

${runtimeClasses}

/// A live storyboard runtime: the clean entry point for the handle-bound clients.
pub struct Runtime {
    inner: crate::Runtime,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(640.0, 480.0)
    }
}

impl Runtime {
    /// Creates a runtime of the given storyboard resolution.
    pub fn new(width: f64, height: f64) -> Self {
        Runtime {
            inner: crate::Runtime::new(width as f32, height as f32),
        }
    }

    /// The clean generator client bound to this runtime.
    pub fn generator(&mut self) -> Generator<'_> {
        Generator {
            runtime: &mut self.inner,
        }
    }

    /// The clean audio client bound to this runtime.
    pub fn audio(&mut self) -> Audio<'_> {
        Audio {
            runtime: Some(&mut self.inner),
        }
    }

    /// Resolves (or creates) a layer by name and binds its handle.
    pub fn layer(&mut self, name: &str) -> Result<Layer<'_>, Error> {
        let handle = self.inner.layer_handle(name)?;
        Ok(Layer {
            runtime: &mut self.inner,
            handle,
        })
    }

    /// Binds an existing layer/sprite pair.
    pub fn sprite(&mut self, layer: i32, sprite: i32) -> Sprite<'_> {
        Sprite {
            runtime: &mut self.inner,
            handle: models::Handle { layer, sprite },
        }
    }
}

fn helper(op: &str, options: HelperOptions) -> Result<Value, Error> {
    let result: models::HelperResult = crate::client::call(json!({
        "op": op,
        "name": options.name,
        "input": options.input.unwrap_or(Value::Object(Default::default())),
    }))?;
    Ok(result.value.unwrap_or(Value::Null))
}

/// The composed stateless clean client, mirroring the other SDKs' entry points.
pub struct Client {
${composition}
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Client {
${initializers}
        }
    }
}
`;
    writeFileSync(join(targets.rust, '..', 'clean.rs'), text, 'utf8');
    console.log('openapi: generated Rust clean clients');
}
