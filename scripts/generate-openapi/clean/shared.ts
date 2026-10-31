import { mkdirSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import { join } from 'node:path';

import { specPath } from '../shared';

export interface Operation {
    tag: string;
    name: string;
    body?: string;
    result?: string;
    resultArray?: boolean;
    description?: string;
    responseDescription?: string;
}

export interface OptionProperty {
    name: string;
    description?: string;
    required: boolean;
    type: string;
    nullable: boolean;
    refs: string[];
    schema: any;
}

export interface OptionModel {
    name: string;
    source: string;
    description?: string;
    properties: OptionProperty[];
}

export interface Contract {
    operations: Map<string, Operation[]>;
    options: Map<string, OptionModel>;
    schemas: Record<string, any>;
}

const core = new Set(['Core']);
const runtime = new Set(['Audio', 'Generator', 'Helpers', 'Layer', 'Sprite', 'Storyboarding3d']);

export function sectionOf(tag: string): 'runtime' | 'core' | 'libraries' {
    if (core.has(tag)) return 'core';
    if (runtime.has(tag)) return 'runtime';
    return 'libraries';
}

export function refName(schema: any): string | undefined {
    return schema?.$ref?.split('/').pop();
}

export function readOperations(spec: any): Map<string, Operation[]> {
    const byTag = new Map<string, Operation[]>();
    for (const item of Object.values<any>(spec.paths)) {
        for (const op of Object.values<any>(item)) {
            if (!op?.operationId || !op.tags?.length) continue;
            const tag = op.tags[0] as string;
            const name = op.operationId.split('.').slice(1).join('.');
            const bodySchema = op.requestBody?.content?.['application/json']?.schema;
            const resultSchema = op.responses?.['200']?.content?.['application/json']?.schema;
            const list = byTag.get(tag) ?? [];
            list.push({
                tag,
                name,
                body: refName(bodySchema),
                result: refName(resultSchema) ?? refName(resultSchema?.items),
                resultArray: resultSchema?.type === 'array',
                description: op.description || op.summary,
                responseDescription: op.responses?.['200']?.description,
            });
            byTag.set(tag, list);
        }
    }
    return byTag;
}

/**
 * Whether a request property is owned by a handle-bound clean class rather than
 * passed by the consumer: the segment/sprite handles themselves. A `layer`
 * *name* is a value prop and stays.
 */
export function isBindingProperty(name: string, schema: any): boolean {
    if (name === 'layerHandle' || name === 'sprite') return true;
    return name === 'layer' && (schema?.type === 'integer' || schema?.type === 'number');
}

function primitive(type: string | undefined): string {
    switch (type) {
        case 'integer':
        case 'number':
            return 'number';
        case 'boolean':
            return 'boolean';
        case 'string':
            return 'string';
        case 'object':
            return 'unknown';
        default:
            return 'unknown';
    }
}

function arrayType(schema: any): { type: string; refs: string[] } {
    const inner = schemaType(schema.items);
    return {
        type: inner.nullable ? `(${inner.type} | null)[]` : `${inner.type}[]`,
        refs: inner.refs,
    };
}

function schemaType(schema: any): { type: string; nullable: boolean; refs: string[] } {
    if (!schema) return { type: 'unknown', nullable: false, refs: [] };
    if (schema.$ref) {
        const ref = refName(schema)!;
        return { type: ref, nullable: false, refs: [ref] };
    }
    if (Array.isArray(schema.type)) {
        const types = schema.type.filter((type: string) => type !== 'null');
        const parts = types.map((type: string) =>
            type === 'array' ? arrayType(schema) : { type: primitive(type), refs: [] }
        );
        return {
            type: parts.map((part: { type: string }) => part.type).join(' | ') || 'unknown',
            nullable: types.length !== schema.type.length,
            refs: parts.flatMap((part: { refs: string[] }) => part.refs),
        };
    }
    if (schema.oneOf) {
        const parts = schema.oneOf.filter((part: any) => part.type !== 'null').map(schemaType);
        return {
            type: parts.map((part: { type: string }) => part.type).join(' | ') || 'unknown',
            nullable:
                parts.length !== schema.oneOf.length ||
                parts.some((part: { nullable: boolean }) => part.nullable),
            refs: parts.flatMap((part: { refs: string[] }) => part.refs),
        };
    }
    if (schema.type === 'array') {
        const array = arrayType(schema);
        return { type: array.type, nullable: false, refs: array.refs };
    }
    return { type: primitive(schema.type), nullable: false, refs: [] };
}

/** The description of a property, including one wrapped in a `oneOf`. */
function propertyDescription(property: any): string | undefined {
    return (
        property?.description ??
        property?.oneOf?.find((part: any) => part.type !== 'null')?.description
    );
}

export function readContract(): Contract {
    const spec = JSON.parse(readFileSync(specPath, 'utf8'));
    const options = new Map<string, OptionModel>();
    for (const [source, schema] of Object.entries<any>(spec.components.schemas)) {
        if (!source.endsWith('Request')) continue;
        const required = new Set<string>(schema.required ?? []);
        const properties = Object.entries<any>(schema.properties ?? {})
            .filter(([name, property]) => !isBindingProperty(name, property))
            .map(([name, property]) => {
                const type = schemaType(property);
                return {
                    name,
                    description: propertyDescription(property),
                    required: required.has(name),
                    type: type.type,
                    nullable: type.nullable,
                    refs: type.refs,
                    schema: property,
                };
            })
            .sort((a, b) => a.name.localeCompare(b.name));
        options.set(source, {
            name: source.replace(/Request$/, 'Options'),
            source,
            description: schema.description,
            properties,
        });
    }
    return { operations: readOperations(spec), options, schemas: spec.components.schemas };
}

export function snake(name: string): string {
    return name
        .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
        .replace(/([A-Z])([A-Z][a-z])/g, '$1_$2')
        .toLowerCase();
}

export function pascal(name: string): string {
    return name.charAt(0).toUpperCase() + name.slice(1);
}

export function clear(dir: string, keep?: (file: string) => boolean): void {
    if (!dir) return;
    mkdirSync(dir, { recursive: true });
    for (const file of readdirSync(dir)) {
        if (keep && keep(file)) continue;
        rmSync(join(dir, file), { recursive: true, force: true });
    }
}

export type ResultKind =
    | 'void'
    | 'bool'
    | 'int'
    | 'float'
    | 'string'
    | 'point'
    | 'unknown'
    | 'layer'
    | 'sprite'
    | 'layerObject'
    | 'raw';

export interface ResultInfo {
    kind: ResultKind;
    property?: string;
    raw?: string;
    array?: boolean;
}

/** Result wrappers the clean surface unwraps into ordinary values. */
const resultAdapters: Record<string, { kind: ResultKind; property: string }> = {
    OkResult: { kind: 'void', property: 'ok' },
    BoolResult: { kind: 'bool', property: 'value' },
    ScalarResult: { kind: 'float', property: 'value' },
    EasingResult: { kind: 'float', property: 'value' },
    AudioDurationResult: { kind: 'float', property: 'audioDuration' },
    SpriteCountResult: { kind: 'int', property: 'spriteCount' },
    VersionResult: { kind: 'string', property: 'version' },
    RandomResult: { kind: 'float', property: 'value' },
    OptionalPointResult: { kind: 'point', property: 'point' },
    HelperResult: { kind: 'unknown', property: 'value' },
    LayerHandle: { kind: 'layer', property: 'layer' },
    SegmentCleanResult: { kind: 'int', property: 'removed' },
    SegmentDiscardResult: { kind: 'int', property: 'layerCount' },
    ImportOsbResult: { kind: 'int', property: 'spriteCount' },
};

export function cleanResultInfo(tag: string, op: Operation): ResultInfo {
    if (tag === 'Layer' && (op.name === 'createSprite' || op.name === 'createAnimation')) {
        return { kind: 'sprite' };
    }
    if (tag === 'Layer' && op.name === 'createSegment') return { kind: 'layerObject' };
    if (!op.result) return { kind: 'void' };
    if (op.resultArray) return { kind: 'raw', raw: op.result, array: true };
    const adapter = resultAdapters[op.result];
    return adapter
        ? { kind: adapter.kind, property: adapter.property }
        : { kind: 'raw', raw: op.result };
}

export function jsdoc(text: string | undefined, indent = ''): string {
    if (!text) return '';
    const lines = text.split('\n').map(line => `${indent} * ${line}`.trimEnd());
    return `${indent}/**\n${lines.join('\n')}\n${indent} */\n`;
}

/** The clean option type for a request schema. */
export function allRequestProperties(
    source: string,
    contract: Contract
): { name: string; property: any; required: boolean }[] {
    const schema = contract.schemas[source];
    const required = new Set<string>(schema?.required ?? []);
    return Object.entries<any>(schema?.properties ?? {})
        .map(([name, property]) => ({ name, property, required: required.has(name) }))
        .sort((a, b) => a.name.localeCompare(b.name));
}

export function optionFor(contract: Contract, op: Operation): OptionModel | undefined {
    return op.body ? contract.options.get(op.body) : undefined;
}

/** The short summary shown above a clean accessor. */
export function operationSummary(tag: string, op: Operation, option?: OptionModel): string {
    return op.description || option?.description || `Calls the ${tag}.${op.name} bridge operation.`;
}

/** The description of one positional parameter. */
export function parameterDescription(property: OptionProperty): string {
    return property.description || `The \`${property.name}\` value.`;
}

/** The description of the `<X>Options` parameter. */
export function optionsDescription(option: OptionModel): string {
    return option.description || 'The request options.';
}

/** The friendly positional order for the most-used options. */
const preferredOrder: Record<string, string[]> = {
    CreateSpriteRequest: ['path', 'origin', 'position', 'layer'],
    CreateAnimationRequest: [
        'path',
        'frames',
        'frameDelay',
        'loopType',
        'origin',
        'position',
        'layer',
    ],
    CreateSampleRequest: ['path', 'time', 'volume', 'layer'],
    CommandRequest: ['kind', 'startTime', 'endTime', 'easing', 'start', 'end', 'parameter'],
    ArcRequest: ['center', 'radius', 'startAngle', 'sweepAngle', 'count'],
    EllipseRequest: ['center', 'radiusX', 'radiusY', 'count'],
    EasingValueRequest: ['easing', 't', 'cubicBezier'],
    GetFftRequest: ['time', 'magnitudes', 'cutoff', 'easing'],
    EncodeVideoRequest: ['inputPattern', 'output', 'fps', 'codec', 'crf'],
    GetOriginVectorRequest: ['origin', 'width', 'height'],
    InScreenBoundsRequest: ['position', 'size', 'rotation', 'origin'],
    RenderShapesRequest: ['width', 'height', 'shapes', 'output', 'background'],
    StartLoopGroupRequest: ['startTime', 'count'],
    StartTriggerGroupRequest: ['name', 'startTime', 'endTime', 'group'],
};

/** Properties in the order used by the positional convenience overload. */
export function orderedProperties(option: OptionModel): OptionProperty[] {
    const preferred = preferredOrder[option.source] ?? [];
    const byName = new Map(option.properties.map(property => [property.name, property]));
    const ordered = preferred.flatMap(name => {
        const property = byName.get(name);
        return property ? [property] : [];
    });
    const remaining = option.properties
        .filter(property => !preferred.includes(property.name))
        .sort((a, b) => Number(b.required) - Number(a.required) || a.name.localeCompare(b.name));
    return [...ordered, ...remaining].sort((a, b) => Number(b.required) - Number(a.required));
}
