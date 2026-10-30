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

/** Whether an operation belongs on the stateless Rust clean surface. */
function isRustCleanOp(tag: string, op: Operation): boolean {
    return !(tag === 'Audio' && (op.name === 'audioDuration' || op.name === 'getFft'));
}

/** Writes the generated clean Rust clients and `<X>Options` types. */
export function emitRustClean(): void {
    const contract = readContract();
    const options = new Map<string, OptionModel>();
    for (const tag of rustCleanTags) {
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

    const classes = rustCleanTags
        .map(tag => {
            const methods = (contract.operations.get(tag) ?? [])
                .filter(op => isRustCleanOp(tag, op))
                .map(op => {
                    const option = optionFor(contract, op);
                    const hasOptions = (option?.properties.length ?? 0) > 0;
                    const args = hasOptions ? `, options: ${option!.name}` : '';
                    const doc = op.description
                        ? `    /// ${op.description.replace(/\s+/g, ' ').trim()}\n`
                        : '';
                    return `${doc}    pub fn ${snake(op.name)}(&self${args}) -> Result<${rustResultType(
                        tag,
                        op
                    )}, Error> {\n        ${rustBody(tag, op)}\n    }`;
                })
                .join('\n\n');
            return `/// The clean \`${tag}\` client.\npub struct ${tag};\n\nimpl ${tag} {\n${methods}\n}`;
        })
        .join('\n\n');

    const composition = rustCleanTags
        .map(tag => {
            const field = tag === 'Imagegen' ? 'image_gen' : tag === 'Storyboarding3d' ? 'storyboarding3d' : tag.toLowerCase();
            return `    pub ${field}: ${tag},`;
        })
        .join('\n');
    const initializers = rustCleanTags
        .map(tag => {
            const field = tag === 'Imagegen' ? 'image_gen' : tag === 'Storyboarding3d' ? 'storyboarding3d' : tag.toLowerCase();
            return `            ${field}: ${tag},`;
        })
        .join('\n');

    const text = `//! The clean high-level Rust clients: one class per contract tag with
//! \`<X>Options\` inputs, mirroring the generated clean clients in the other SDKs.
//!
//! The runtime classes stay the hand-written [\`crate::Runtime\`]/[\`crate::Segment\`]
//! surface; these clients cover the stateless core and library operations over
//! \`crate::Client\`.

use serde_json::{json, Value};

use crate::error::Error;
use crate::models;

${structs}

${classes}

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
