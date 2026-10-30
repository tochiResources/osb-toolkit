import { writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { root } from '../shared';
import {
    cleanResultInfo,
    operationSummary,
    optionFor,
    readContract,
    refName,
    snake,
} from './shared';
import type { Contract, Operation, OptionModel, OptionProperty } from './shared';

/** The stateless tags that get a clean C++ client. */
const cppCleanTags = [
    'Core',
    'Audio',
    'Diagnostics',
    'Optimize',
    'Image',
    'Imagegen',
    'Media',
    'Storyboarding3d',
    'Helpers',
] as const;

/** The C++ member order inside each `<X>Options` struct. */
const fieldOrder: Record<string, string[]> = {
    ApplyEasingRequest: ['easing', 't'],
    EasingValueRequest: ['t', 'easing', 'cubicBezier'],
    TextRequest: ['text'],
    CurveRequest: ['curve'],
    CurvePointRequest: ['curve', 't', 'byArcLength'],
    CurveSampleRequest: ['count', 'curve', 'byArcLength'],
    SimplifyRequest: ['points', 'tolerance'],
    CurveDistanceRequest: ['curve', 'point'],
    CurveOffsetRequest: ['curve', 'distance'],
    ArcRequest: ['center', 'radius', 'startAngle', 'sweepAngle', 'count'],
    EllipseRequest: ['center', 'radiusX', 'radiusY', 'count'],
    LineIntersectRequest: ['a1', 'a2', 'b1', 'b2'],
    GetOriginVectorRequest: ['origin', 'width', 'height'],
    InScreenBoundsRequest: ['position', 'size', 'rotation', 'origin'],
    FftRequest: ['path', 'time', 'magnitudes'],
    PathRequest: ['path'],
    ExtractAudioRequest: ['input', 'output'],
};

interface CppField {
    name: string;
    type: string;
    default?: string;
    doc?: string;
}

/**
 * Complex requests stay as a JSON string field because the hand-written C++ SDK
 * already takes their request body as JSON.
 */
const fieldOverrides: Record<string, CppField[]> = {
    OptimizeRequest: [
        { name: 'text', type: 'std::string', default: '""', doc: 'The storyboard text.' },
        {
            name: 'deduplicate',
            type: 'bool',
            default: 'false',
            doc: 'Whether to deduplicate commands.',
        },
    ],
    HelperRequest: [
        { name: 'name', type: 'std::string', default: '""', doc: 'The helper name.' },
        {
            name: 'input_json',
            type: 'std::string',
            default: '"{}"',
            doc: 'The helper input as a JSON object.',
        },
    ],
    ImageProcessRequest: [
        { name: 'path', type: 'std::string', default: '""', doc: 'The input image path.' },
        { name: 'output', type: 'std::string', default: '""', doc: 'The output image path.' },
        {
            name: 'options_json',
            type: 'std::string',
            default: '"{}"',
            doc: 'The `ImageOptions` object as JSON.',
        },
    ],
    RenderShapesRequest: [
        {
            name: 'request_json',
            type: 'std::string',
            default: '"{}"',
            doc: 'The render request as JSON.',
        },
    ],
    ExtractFramesRequest: [
        {
            name: 'request_json',
            type: 'std::string',
            default: '"{}"',
            doc: 'The frame-extraction request as JSON.',
        },
    ],
    EncodeVideoRequest: [
        {
            name: 'request_json',
            type: 'std::string',
            default: '"{}"',
            doc: 'The video-encoding request as JSON.',
        },
    ],
    Bake3dRequest: [
        {
            name: 'input_json',
            type: 'std::string',
            default: '"{}"',
            doc: 'The 3D scene as JSON.',
        },
    ],
};

function oneLine(text: string): string {
    return text.replace(/\s+/g, ' ').trim();
}

function indent(text: string, spaces: number): string {
    const pad = ' '.repeat(spaces);
    return text
        .split('\n')
        .map(line => (line ? `${pad}${line}` : line))
        .join('\n');
}

function cppClassName(tag: string): string {
    return tag === 'Imagegen' ? 'ImageGen' : tag;
}

function cppFieldName(tag: string): string {
    if (tag === 'Imagegen') return 'image_gen';
    if (tag === 'Storyboarding3d') return 'storyboarding3d';
    return tag.toLowerCase();
}

function cppType(property: OptionProperty): string {
    const schema = property.schema;
    const inner = schema?.oneOf?.find((part: any) => part.type !== 'null') ?? schema;
    if (inner?.$ref) {
        const name = refName(inner)!;
        return name === 'Easing' ? 'std::string' : `sdk::${name}`;
    }
    const types = Array.isArray(inner?.type)
        ? inner.type.filter((type: string) => type !== 'null')
        : [inner?.type];
    if (types.includes('array')) {
        const item: OptionProperty = {
            name: property.name,
            description: property.description,
            required: true,
            type: 'unknown',
            nullable: false,
            refs: [],
            schema: inner?.items ?? {},
        };
        return `std::vector<${cppType(item)}>`;
    }
    switch (types[0]) {
        case 'integer':
            return 'int';
        case 'number':
            return 'double';
        case 'boolean':
            return 'bool';
        case 'string':
            return 'std::string';
        default:
            return 'std::string';
    }
}

function cppDefault(property: OptionProperty, type: string): string | undefined {
    const inner = property.schema?.oneOf?.find((part: any) => part.type !== 'null') ?? property.schema;
    if (type === 'std::string') {
        if (refName(inner) === 'Easing') return '"None"';
        if (property.name.endsWith('Json') || property.name.endsWith('_json')) return '"{}"';
        return '""';
    }
    if (type === 'double') return '0.0';
    if (type === 'int') {
        if (property.required) return '0';
        if (property.name === 'count' || property.name === 'magnitudes') return '64';
        return '0';
    }
    if (type === 'bool') return 'false';
    if (type === 'sdk::OsbOrigin') return 'sdk::OsbOrigin::Centre';
    if (type.startsWith('sdk::') || type.startsWith('std::vector')) return '{}';
    return undefined;
}

function cppFields(source: string, contract: Contract): CppField[] {
    const override = fieldOverrides[source];
    if (override) return override;
    const schema = contract.schemas[source];
    const required = new Set<string>(schema?.required ?? []);
    const properties = schema?.properties ?? {};
    const order = fieldOrder[source] ?? Object.keys(properties);
    return order.flatMap(name => {
        const property = properties[name];
        if (!property) return [];
        const optionProperty: OptionProperty = {
            name,
            description: property.description,
            required: required.has(name),
            type: '',
            nullable: false,
            refs: [],
            schema: property,
        };
        const type = cppType(optionProperty);
        return [
            {
                name: snake(name),
                type,
                default: cppDefault(optionProperty, type),
                doc: property.description,
            },
        ];
    });
}

function emitOptions(option: OptionModel, contract: Contract): string {
    const fields = cppFields(option.source, contract)
        .map(field => {
            const doc = field.doc ? `    /// ${oneLine(field.doc)}\n` : '';
            const value = field.default ? ` = ${field.default}` : '';
            return `${doc}    ${field.type} ${field.name}${value};`;
        })
        .join('\n');
    const doc = option.description
        ? `/// ${oneLine(option.description)}\n`
        : `/// A \`${option.name}\` request.\n`;
    return `${doc}struct ${option.name} {\n${fields}\n};`;
}

function cppResultType(tag: string, op: Operation): string {
    if (tag === 'Core' && op.name === 'lineIntersect') return 'Json';
    const info = cleanResultInfo(tag, op);
    switch (info.kind) {
        case 'void':
            return 'void';
        case 'bool':
            return 'bool';
        case 'int':
        case 'layer':
            return 'int';
        case 'float':
            return 'double';
        case 'string':
            return 'std::string';
        case 'point':
        case 'unknown':
            return 'Json';
        default:
            if (info.raw === 'Beatmap') return 'sdk::Beatmap';
            if (info.raw === 'Box2') return 'sdk::Box2';
            if (info.raw === 'Vector2') return 'sdk::Vector2';
            if (info.raw === 'PointListResult') return 'sdk::PointList';
            return 'Json';
    }
}

function cppBody(tag: string, op: Operation): string {
    const key = `${tag}.${op.name}`;
    switch (key) {
        case 'Core.version':
            return 'return impl_.version();';
        case 'Core.applyEasing':
            return 'return impl_.apply_easing(options.easing, options.t);';
        case 'Core.easingValue':
            return [
                'std::string body = "{\\"t\\":" + sdk::detail::number(options.t);',
                'if (!options.cubic_bezier.empty()) {',
                '    body += ",\\"cubicBezier\\":[";',
                '    for (std::size_t index = 0; index < options.cubic_bezier.size(); ++index) {',
                '        if (index != 0) body += ",";',
                '        body += sdk::detail::number(options.cubic_bezier[index]);',
                '    }',
                '    body += "]";',
                '} else {',
                '    body += ",\\"easing\\":" + sdk::detail::quote(options.easing);',
                '}',
                'body += "}";',
                'return detail::call(*library_, "Animations.easingValue", body).at("value").number();',
            ].join('\n');
        case 'Core.parseOsu':
            return 'return impl_.parse_beatmap(options.text);';
        case 'Core.parseOsb':
            return 'return impl_.parse_osb(options.text);';
        case 'Core.getOriginVector':
            return 'return impl_.get_origin_vector(options.origin, options.width, options.height);';
        case 'Core.inScreenBounds':
            return 'return impl_.in_screen_bounds(options.position, options.size, options.rotation, options.origin);';
        case 'Core.length':
            return 'return curve_.length(options.curve);';
        case 'Core.boundingBox':
            return 'return curve_.bounding_box(options.curve);';
        case 'Core.point':
            return 'return curve_.point(options.curve, options.t, options.by_arc_length);';
        case 'Core.tangent':
            return 'return curve_.tangent(options.curve, options.t);';
        case 'Core.sample':
            return 'return curve_.sample(options.curve, options.count, options.by_arc_length);';
        case 'Core.simplify':
            return 'return curve_.simplify(options.points, options.tolerance);';
        case 'Core.normal':
            return 'return curve_.normal(options.curve, options.t);';
        case 'Core.curvature':
            return 'return curve_.curvature(options.curve, options.t);';
        case 'Core.distanceTo':
            return 'return curve_.distance_to(options.curve, options.point);';
        case 'Core.offset':
            return 'return curve_.offset(options.curve, options.distance);';
        case 'Core.arc':
            return 'return curve_.arc(options.center, options.radius, options.start_angle, options.sweep_angle, options.count);';
        case 'Core.ellipse':
            return 'return curve_.ellipse(options.center, options.radius_x, options.radius_y, options.count);';
        case 'Core.lineIntersect':
            return 'return curve_.line_intersect(options.a1, options.a2, options.b1, options.b2);';
        case 'Audio.fft':
            return [
                'return detail::call(*library_, "Audio.fft",',
                '    "{\\"path\\":" + sdk::detail::quote(options.path) +',
                '    ",\\"time\\":" + sdk::detail::number(options.time) +',
                '    ",\\"magnitudes\\":" + std::to_string(options.magnitudes) + "}");',
            ].join('\n');
        case 'Diagnostics.diagnoseOsb':
            return 'return impl_.diagnose_osb(options.text);';
        case 'Diagnostics.diagnoseOsu':
            return 'return impl_.diagnose_osu(options.text);';
        case 'Helpers.call':
            return 'return impl_.call(options.name, options.input_json);';
        case 'Helpers.commands':
            return [
                'return detail::call(*library_, "Helpers.commands",',
                '    "{\\"name\\":" + sdk::detail::quote(options.name) +',
                '    ",\\"input\\":" + options.input_json + "}");',
            ].join('\n');
        case 'Helpers.keyframes':
            return [
                'return detail::call(*library_, "Helpers.keyframes",',
                '    "{\\"name\\":" + sdk::detail::quote(options.name) +',
                '    ",\\"input\\":" + options.input_json + "}");',
            ].join('\n');
        case 'Image.info':
            return 'return impl_.info(options.path);';
        case 'Image.process':
            return 'return impl_.process(options.path, options.output, options.options_json);';
        case 'Imagegen.renderShapes':
            return 'return impl_.render_shapes(options.request_json);';
        case 'Media.info':
            return 'return impl_.info(options.path);';
        case 'Media.extractFrames':
            return 'return impl_.extract_frames(options.request_json);';
        case 'Media.encodeVideo':
            return 'return impl_.encode_video(options.request_json);';
        case 'Media.extractAudio':
            return 'impl_.extract_audio(options.input, options.output);';
        case 'Optimize.optimizeOsb':
            return 'return impl_.optimize_osb(options.text, options.deduplicate);';
        case 'Storyboarding3d.bake3d':
            return 'return impl_.bake3d(options.input_json);';
        default:
            throw new Error(`clean (cpp): no body for ${key}`);
    }
}

function cppConstructor(tag: string): string {
    switch (tag) {
        case 'Core':
            return '    explicit Core(const Library& library) : library_(&library), impl_(library), curve_(library) {}';
        case 'Audio':
            return '    explicit Audio(const Library& library) : library_(&library) {}';
        case 'Helpers':
            return '    explicit Helpers(const Library& library) : library_(&library), impl_(library) {}';
        default:
            return `    explicit ${cppClassName(tag)}(const Library& library) : impl_(library) {}`;
    }
}

function cppPrivateMembers(tag: string): string {
    switch (tag) {
        case 'Core':
            return '    const Library* library_;\n    sdk::Client impl_;\n    sdk::Curve curve_;';
        case 'Audio':
            return '    const Library* library_;';
        case 'Helpers':
            return '    const Library* library_;\n    sdk::Storyboarding3d::Helpers impl_;';
        case 'Imagegen':
            return '    sdk::ImageGen impl_;';
        default:
            return `    sdk::${tag} impl_;`;
    }
}

function cppMethod(tag: string, op: Operation, option: OptionModel | undefined): string {
    const result = cppResultType(tag, op);
    const name = snake(op.name);
    const body = cppBody(tag, op);
    const doc = op.description
        ? `${op.description
              .split('\n')
              .map(line => `    /// ${line.trimEnd()}`)
              .join('\n')}\n`
        : `    /// ${oneLine(operationSummary(tag, op, option))}\n`;
    if ((option?.properties.length ?? 0) === 0) {
        return `${doc}    ${result} ${name}() const {\n${indent(body, 8)}\n    }`;
    }
    return `${doc}    ${result} ${name}(const ${option!.name}& options) const {\n${indent(body, 8)}\n    }`;
}

function emitClass(tag: string, operations: Operation[], contract: Contract): string {
    const className = cppClassName(tag);
    const methods = operations
        .filter(op => isCppCleanOp(tag, op))
        .map(op => cppMethod(tag, op, optionFor(contract, op)))
        .join('\n\n');
    const doc =
        tag === 'Core'
            ? '/// The clean non-runtime `Core` client: version, parsers, easing and math.\n'
            : `/// The clean \`${tag}\` client.\n`;
    return `${doc}class ${className} {\npublic:\n${cppConstructor(tag)}\n\n${methods}\n\nprivate:\n${cppPrivateMembers(tag)}\n};`;
}

function isCppCleanOp(tag: string, op: Operation): boolean {
    return !(tag === 'Audio' && (op.name === 'audioDuration' || op.name === 'getFft'));
}

/** Writes the generated clean C++ clients and `<X>Options` types. */
export function emitCppClean(): void {
    const contract = readContract();
    const options = new Map<string, OptionModel>();
    for (const tag of cppCleanTags) {
        for (const op of contract.operations.get(tag) ?? []) {
            if (!isCppCleanOp(tag, op)) continue;
            const option = optionFor(contract, op);
            if (option && option.properties.length > 0) options.set(option.name, option);
        }
    }

    const optionText = [...options.values()]
        .sort((a, b) => a.name.localeCompare(b.name))
        .map(option => emitOptions(option, contract))
        .join('\n\n');

    const classText = cppCleanTags
        .map(tag => emitClass(tag, contract.operations.get(tag) ?? [], contract))
        .join('\n\n');

    const composition = cppCleanTags
        .map((tag, index) => `${index === 0 ? '' : '          '}${cppFieldName(tag)}(library)`)
        .join(',\n');
    const members = cppCleanTags
        .map(tag => `    ${cppClassName(tag)} ${cppFieldName(tag)};`)
        .join('\n');

    const text = `// <auto-generated/>
// Generated from the OpenAPI contract by scripts/generate-openapi/clean/cpp.ts; do not edit.
#pragma once

// Clean high-level C++ clients: one class per contract tag with \`<X>Options\`
// inputs, mirroring the generated clean clients in the other SDKs. The classes
// are thin wrappers over \`osbtk_sdk.hpp\`; the runtime classes are re-exported
// because they are already handle-bound.

#include <string>
#include <utility>
#include <vector>

#include "osbtk_sdk.hpp"

namespace osbtk::clean {

using Json = sdk::Json;
using Library = osbtk::Library;

namespace detail {

inline Json call(const Library& library, const std::string& op, const std::string& body) {
    Json json = sdk::detail::parse_json(
        library.exec("{\\"op\\":" + sdk::detail::quote(op) + "," + body.substr(1)));
    if (const Json* error = json.find("error"); error != nullptr) {
        throw std::runtime_error("osbtk: " + error->string());
    }
    return json;
}

} // namespace detail

${optionText}

${classText}

/// The runtime classes are already high-level and handle-bound.
using Runtime = sdk::Runtime;
using Generator = sdk::Generator;
using Layer = sdk::Layer;
using Sprite = sdk::Sprite;

/// The composed stateless clean client.
class Client {
public:
    explicit Client(const Library& library)
        : ${composition} {}

${members}
};

} // namespace osbtk::clean
`;
    writeFileSync(join(root, 'bindings', 'cpp', 'osbtk_clean.hpp'), text, 'utf8');
    console.log('openapi: generated C++ clean clients');
}
