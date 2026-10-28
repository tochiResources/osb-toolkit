import { mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { specPath, root, targets } from './shared';

interface Operation {
    tag: string;
    name: string;
    body?: string;
    result?: string;
    resultArray?: boolean;
    description?: string;
    responseDescription?: string;
}

interface OptionProperty {
    name: string;
    description?: string;
    required: boolean;
    type: string;
    nullable: boolean;
    refs: string[];
    schema: any;
}

interface OptionModel {
    name: string;
    source: string;
    description?: string;
    properties: OptionProperty[];
}

interface Contract {
    operations: Map<string, Operation[]>;
    options: Map<string, OptionModel>;
    schemas: Record<string, any>;
}

const core = new Set(['Animations', 'Common', 'Curve', 'Origin', 'Parsing']);
const runtime = new Set(['Audio', 'Generator', 'Helpers', 'Layer', 'Sprite', 'Storyboarding3d']);

function sectionOf(tag: string): 'runtime' | 'core' | 'libraries' {
    if (core.has(tag)) return 'core';
    if (runtime.has(tag)) return 'runtime';
    return 'libraries';
}

function refName(schema: any): string | undefined {
    return schema?.$ref?.split('/').pop();
}

function readOperations(spec: any): Map<string, Operation[]> {
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
function isBindingProperty(name: string, schema: any): boolean {
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

function readContract(): Contract {
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

function snake(name: string): string {
    return name
        .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
        .replace(/([A-Z])([A-Z][a-z])/g, '$1_$2')
        .toLowerCase();
}

function pascal(name: string): string {
    return name.charAt(0).toUpperCase() + name.slice(1);
}

function clear(dir: string, keep?: (file: string) => boolean): void {
    if (!dir) return;
    mkdirSync(dir, { recursive: true });
    for (const file of readdirSync(dir)) {
        if (keep && keep(file)) continue;
        rmSync(join(dir, file), { recursive: true, force: true });
    }
}

type ResultKind =
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

interface ResultInfo {
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

function cleanResultInfo(tag: string, op: Operation): ResultInfo {
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

function jsdoc(text: string | undefined, indent = ''): string {
    if (!text) return '';
    const lines = text.split('\n').map(line => `${indent} * ${line}`.trimEnd());
    return `${indent}/**\n${lines.join('\n')}\n${indent} */\n`;
}

/** The clean option type for a request schema. */
function optionFor(contract: Contract, op: Operation): OptionModel | undefined {
    return op.body ? contract.options.get(op.body) : undefined;
}

/** The short summary shown above a clean accessor. */
function operationSummary(tag: string, op: Operation, option?: OptionModel): string {
    return op.description || option?.description || `Calls the ${tag}.${op.name} bridge operation.`;
}

/** The description of one positional parameter. */
function parameterDescription(property: OptionProperty): string {
    return property.description || `The \`${property.name}\` value.`;
}

/** The description of the `<X>Options` parameter. */
function optionsDescription(option: OptionModel): string {
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
function orderedProperties(option: OptionModel): OptionProperty[] {
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

function tsPropertyType(property: OptionProperty): string {
    return `${property.type}${property.nullable ? ' | null' : ''}`;
}

function tsGuardName(option: OptionModel): string {
    return `is${option.name}`;
}

function tsGuard(option: OptionModel): string {
    const keys = option.properties.filter(property => property.required);
    const names = (keys.length ? keys : option.properties).map(
        property => `'${property.name}' in value`
    );
    const check = names.join(keys.length ? ' && ' : ' || ');
    return `export function ${tsGuardName(option)}(value: unknown): value is ${option.name} {\n    return typeof value === 'object' && value !== null && ${check};\n}`;
}

function emitTypeScriptOptions(dir: string, contract: Contract): void {
    const models = new Set<string>();
    for (const option of contract.options.values()) {
        for (const property of option.properties) {
            for (const ref of property.refs) models.add(ref);
        }
    }
    const imports = [...models]
        .sort()
        .map(type => `import type { ${type} } from '../models/${type}';`);
    const interfaces = [...contract.options.values()]
        .map(option => {
            const properties = option.properties
                .map(property => {
                    const optional = property.required ? '' : '?';
                    const nullable = property.nullable ? ' | null' : '';
                    return `${jsdoc(property.description, '    ')}    ${property.name}${optional}: ${property.type}${nullable};`;
                })
                .join('\n');
            const guard = option.properties.length > 0 ? `\n\n${tsGuard(option)}` : '';
            return `${jsdoc(option.description)}export interface ${option.name} {\n${properties}\n}${guard}`;
        })
        .join('\n\n');
    const text = `${imports.length ? `${imports.join('\n')}\n\n` : ''}${interfaces}\n`;
    writeFileSync(join(dir, 'options.ts'), text, 'utf8');
}

function tsResultType(tag: string, op: Operation): string {
    const info = cleanResultInfo(tag, op);
    switch (info.kind) {
        case 'sprite':
            return 'Sprite';
        case 'layerObject':
            return 'Layer';
        case 'void':
            return 'void';
        case 'bool':
            return 'boolean';
        case 'int':
        case 'float':
        case 'layer':
            return 'number';
        case 'string':
            return 'string';
        case 'point':
            return 'Vector2 | null';
        case 'unknown':
            return 'unknown';
        default:
            return info.array ? `${info.raw}[]` : (info.raw ?? 'void');
    }
}

/** One JSDoc block with a summary, parameter docs and an optional return doc. */
function tsDoc(
    tag: string,
    op: Operation,
    option: OptionModel | undefined,
    params: { name: string; description: string }[],
    indent = '    '
): string {
    const oneLine = (text: string) => text.replace(/\s+/g, ' ').trim();
    const lines = [`${indent}/**`, `${indent} * ${oneLine(operationSummary(tag, op, option))}`];
    if (params.length > 0) lines.push(`${indent} *`);
    for (const param of params) {
        lines.push(`${indent} * @param ${param.name} ${oneLine(param.description)}`);
    }
    if (op.responseDescription) {
        lines.push(`${indent} *`, `${indent} * @returns ${oneLine(op.responseDescription)}`);
    }
    lines.push(`${indent} */`);
    return `${lines.join('\n')}\n`;
}

function tsMethod(tag: string, op: Operation, option: OptionModel | undefined): string {
    const result = tsResultType(tag, op);
    const hasOptions = (option?.properties.length ?? 0) > 0;
    const isLayer = tag === 'Layer';
    const isSprite = tag === 'Sprite';
    let args = '';
    let call = '';
    let statement: string;

    if (isLayer && op.body === 'HandleRequest') {
        args = 'sprite: number';
        call = `{ layer: this.handle, sprite }`;
    } else if (isLayer && option) {
        args = hasOptions ? `options: ${option.name}` : '';
        const body = hasOptions
            ? `{ layerHandle: this.handle, ...options }`
            : `{ layerHandle: this.handle }`;
        const segment = hasOptions
            ? `{ layer: this.handle, ...options }`
            : `{ layer: this.handle }`;
        call = op.body?.startsWith('Create') ? body : segment;
    } else if (isSprite && option) {
        args = hasOptions ? `options: ${option.name}` : '';
        call = hasOptions
            ? `{ layer: this.layer, sprite: this.sprite, ...options }`
            : `{ layer: this.layer, sprite: this.sprite }`;
    } else if (option) {
        args = hasOptions ? `options: ${option.name}` : '';
        call = hasOptions ? 'options' : '{}';
    }
    if (isSprite && op.name === 'command') {
        call = `${call} as Parameters<SpriteAPI['command']>[0]`;
    }

    const info = cleanResultInfo(tag, op);
    if (info.kind === 'sprite') {
        statement = `        const handle = await this.api.${op.name}(${call});\n        return new Sprite(this.sprites, handle.layer ?? this.handle, handle.sprite ?? 0);`;
    } else if (info.kind === 'layerObject') {
        statement = `        const handle = await this.api.createSegment(${call});\n        return new Layer(this.api, this.sprites, handle.layer ?? this.handle);`;
    } else if (info.kind === 'void') {
        statement = `        await this.api.${op.name}(${call});`;
    } else if (info.property) {
        const value = `(await this.api.${op.name}(${call})).${info.property}`;
        statement = `        return ${info.kind === 'point' ? `${value} ?? null` : value};`;
    } else {
        statement = `        return this.api.${op.name}(${call});`;
    }

    const overload = option && option.properties.length > 0 ? orderedProperties(option) : [];
    if (overload.length === 0) {
        const params =
            isLayer && op.body === 'HandleRequest'
                ? [{ name: 'sprite', description: 'The sprite handle.' }]
                : [];
        return `${tsDoc(tag, op, option, params)}    async ${op.name}(${args}): Promise<${result}> {\n${statement}\n    }`;
    }

    const first = overload[0];
    const firstVar = `optionsOr${pascal(first.name)}`;
    const implementation = [
        `${firstVar}?: ${option.name} | ${tsPropertyType(first)}`,
        ...overload.slice(1).map(property => `${property.name}?: ${tsPropertyType(property)}`),
    ].join(', ');
    const fields = overload
        .map(property => `${property.name}: ${property === first ? firstVar : property.name}`)
        .join(', ');
    const prelude = `        const options: ${option.name} = ${tsGuardName(option)}(${firstVar})\n            ? ${firstVar}\n            : ({ ${fields} } as ${option.name});`;
    const requiredCount = overload.filter(property => property.required).length;
    const start = requiredCount === 0 ? 0 : requiredCount;
    const signatures: string[] = [];
    for (let arity = start; arity <= overload.length; arity++) {
        const subset = overload.slice(0, arity);
        const parameters = subset
            .map(property => `${property.name}: ${tsPropertyType(property)}`)
            .join(', ');
        const doc = tsDoc(
            tag,
            op,
            option,
            subset.map(property => ({
                name: property.name,
                description: parameterDescription(property),
            }))
        );
        signatures.push(`${doc}    async ${op.name}(${parameters}): Promise<${result}>;`);
    }
    signatures.push(
        `${tsDoc(tag, op, option, [{ name: 'options', description: optionsDescription(option) }])}    async ${op.name}(options: ${option.name}): Promise<${result}>;`
    );
    return `${signatures.join('\n')}\n    async ${op.name}(${implementation}): Promise<${result}> {\n${prelude}\n${statement}\n    }`;
}

function emitTypeScript(contract: Contract): void {
    const dir = join(targets.typescript, 'client');
    clear(dir);
    emitTypeScriptOptions(dir, contract);
    const sections: Record<string, string[]> = { runtime: [], core: [], libraries: [] };
    for (const [tag, operations] of [...contract.operations].sort()) {
        const section = sectionOf(tag);
        const outDir = join(dir, section);
        mkdirSync(outDir, { recursive: true });
        const isLayer = tag === 'Layer';
        const bound = isLayer || tag === 'Sprite';
        const results = new Set<string>();
        for (const op of operations) {
            const info = cleanResultInfo(tag, op);
            if (info.kind === 'raw' && info.raw) results.add(info.raw);
            if (info.kind === 'point') results.add('Vector2');
            const option = optionFor(contract, op);
            if (option) {
                for (const property of option.properties) {
                    for (const ref of property.refs) results.add(ref);
                }
            }
        }
        const imports = [
            `import { ${tag}API } from '../../api/${tag}API';`,
            ...[...results]
                .sort()
                .map(type => `import type { ${type} } from '../../models/${type}';`),
            ...(isLayer
                ? [
                      `import type { SpriteAPI } from '../../api/SpriteAPI';`,
                      `import { Sprite } from './Sprite';`,
                  ]
                : []),
        ];
        const options = new Set<string>();
        const guards = new Set<string>();
        for (const op of operations) {
            const option = optionFor(contract, op);
            if (option && option.properties.length > 0) {
                options.add(option.name);
                if (option.properties.length < 8) guards.add(tsGuardName(option));
            }
        }
        if (options.size) {
            imports.push(`import type { ${[...options].sort().join(', ')} } from '../options';`);
        }
        if (guards.size) {
            imports.push(`import { ${[...guards].sort().join(', ')} } from '../options';`);
        }
        const methods = operations
            .map(op => tsMethod(tag, op, optionFor(contract, op)))
            .join('\n\n');
        const constructor = isLayer
            ? `    constructor(\n        private readonly api: LayerAPI,\n        private readonly sprites: SpriteAPI,\n        private readonly handle: number\n    ) {}`
            : bound
              ? `    constructor(\n        private readonly api: ${tag}API,\n        private readonly layer: number,\n        private readonly sprite: number\n    ) {}`
              : `    constructor(private readonly api: ${tag}API) {}`;
        const text = `${imports.join('\n')}\n\n${jsdoc(`The clean \`${tag}\` client.`)}export class ${tag} {\n${constructor}\n\n    static instanceOf(value: unknown): value is ${tag} {\n        return value instanceof ${tag};\n    }\n\n${methods}\n}\n`;
        writeFileSync(join(outDir, `${tag}.ts`), text, 'utf8');
        sections[section].push(tag);
    }
    for (const [section, tags] of Object.entries(sections)) {
        writeFileSync(
            join(dir, section, 'index.ts'),
            `${tags.map(tag => `export { ${tag} } from './${tag}';`).join('\n')}\n`,
            'utf8'
        );
    }
    writeFileSync(
        join(dir, 'index.ts'),
        `export * from './options';\n${Object.entries(sections)
            .map(([section]) => `export * from './${section}';`)
            .join('\n')}\n`,
        'utf8'
    );
}

function pythonType(property: { name: string; property: any; required: boolean }): string {
    const inner =
        property.property.oneOf?.find((part: any) => part.type !== 'null') ?? property.property;
    const types = Array.isArray(inner.type)
        ? inner.type.filter((type: string) => type !== 'null')
        : [inner.type];
    let base: string;
    if (inner.$ref) {
        base = refName(inner)!;
    } else if (types.includes('array')) {
        base = `List[${pythonType({
            name: property.name,
            property: inner.items ?? {},
            required: true,
        })}]`;
    } else {
        switch (types[0]) {
            case 'integer':
                base = 'int';
                break;
            case 'number':
                base = 'float';
                break;
            case 'boolean':
                base = 'bool';
                break;
            case 'string':
                base = 'str';
                break;
            case 'object':
                base = 'Dict[str, Any]';
                break;
            default:
                base = 'Any';
        }
    }
    const optional = !property.required || !!property.nullable;
    return optional && !base.startsWith('Optional[') ? `Optional[${base}]` : base;
}

function pythonDoc(text: string | undefined, indent = ''): string {
    if (!text) return '';
    return text
        .split('\n')
        .map(line => `${indent}# ${line}`.trimEnd())
        .join('\n');
}

/** A method docstring with the summary, parameter docs and an optional return. */
function pythonMethodDoc(
    tag: string,
    op: Operation,
    option: OptionModel | undefined,
    params: { name: string; description: string }[],
    indent = '        '
): string {
    const oneLine = (text: string) => text.replace(/\s+/g, ' ').trim();
    const lines = [`${indent}"""${oneLine(operationSummary(tag, op, option))}`];
    if (params.length > 0) {
        lines.push('', `${indent}Args:`);
        for (const param of params) {
            lines.push(`${indent}    ${param.name}: ${oneLine(param.description)}`);
        }
    }
    if (op.responseDescription) {
        lines.push('', `${indent}Returns:`, `${indent}    ${oneLine(op.responseDescription)}`);
    }
    lines.push(`${indent}"""`);
    return `${lines.join('\n')}\n`;
}

function emitPythonOptions(dir: string, contract: Contract): void {
    const refs = new Set<string>();
    for (const option of contract.options.values()) {
        for (const property of option.properties) {
            for (const ref of property.refs) refs.add(ref);
        }
    }
    const classes = [...contract.options.values()]
        .map(option => {
            const properties = orderedProperties(option)
                .map(property => {
                    const type = pythonType({
                        name: property.name,
                        property: property.schema,
                        required: property.required,
                    });
                    const doc = pythonDoc(property.description, '    ');
                    const declaration = `    ${property.name}: ${type}${property.required ? '' : ' = None'}`;
                    return doc ? `${doc}\n${declaration}` : declaration;
                })
                .join('\n');
            const title = option.description ? `    """${option.description}"""\n` : '';
            return `@dataclass\nclass ${option.name}:\n${title}${properties}`;
        })
        .join('\n\n');
    const text = `from __future__ import annotations\n\nfrom dataclasses import asdict, dataclass\nfrom typing import Any, Dict, List, Optional\n${[
        ...refs,
    ]
        .sort()
        .map(type => `\nfrom ..models.${snake(type)} import ${type}`)
        .join(
            ''
        )}\n\n${classes}\n\n\ndef to_kwargs(options: object) -> Dict[str, Any]:\n    """Convert a clean options dataclass to request model keyword arguments."""\n    return {key: value for key, value in asdict(options).items() if value is not None}\n`;
    writeFileSync(join(dir, 'options.py'), text, 'utf8');
}

function pythonResult(tag: string, name: string, op: Operation): string {
    const info = cleanResultInfo(tag, op);
    switch (info.kind) {
        case 'sprite':
            return 'Sprite';
        case 'layerObject':
            return 'Layer';
        case 'void':
            return 'None';
        case 'bool':
            return 'bool';
        case 'int':
        case 'layer':
            return 'int';
        case 'float':
            return 'float';
        case 'string':
            return 'str';
        case 'point':
            return 'Optional[Vector2]';
        case 'unknown':
            return 'Any';
        default:
            return info.array ? `List[${info.raw}]` : (info.raw ?? 'None');
    }
}

function pythonRequest(tag: string, source: string, options: boolean): string {
    if (tag === 'Layer') {
        if (source.startsWith('Create')) {
            return `${source}(layer_handle=self._handle${options ? ', **to_kwargs(options)' : ''})`;
        }
        return `${source}(layer=self._handle${options ? ', **to_kwargs(options)' : ''})`;
    }
    if (tag === 'Sprite') {
        return `${source}(layer=self._layer, sprite=self._sprite${options ? ', **to_kwargs(options)' : ''})`;
    }
    return options ? `${source}(**to_kwargs(options))` : `${source}()`;
}

function emitPython(contract: Contract): void {
    const dir = join(targets.python, 'client');
    clear(dir);
    emitPythonOptions(dir, contract);
    const sections: Record<string, string[]> = { runtime: [], core: [], libraries: [] };
    for (const [tag, operations] of [...contract.operations].sort()) {
        const isLayer = tag === 'Layer';
        const isSprite = tag === 'Sprite';
        const bound = isLayer || isSprite;
        const section = sectionOf(tag);
        const outDir = join(dir, section);
        mkdirSync(outDir, { recursive: true });
        const models = new Set<string>();
        const optionNames = new Set<string>();
        let hasOverload = false;
        for (const op of operations) {
            if (op.body) models.add(op.body);
            const info = cleanResultInfo(tag, op);
            if (info.kind === 'raw' && info.raw) models.add(info.raw);
            if (info.kind === 'point') models.add('Vector2');
            const option = optionFor(contract, op);
            if (option) {
                if (option.properties.length > 0) optionNames.add(option.name);
                if (option.properties.length > 0) {
                    hasOverload = true;
                }
                for (const property of option.properties) {
                    for (const ref of property.refs) models.add(ref);
                }
            }
        }
        const imports = [
            'from __future__ import annotations',
            '',
            `from ...api.${snake(tag)}_api import ${tag}API`,
            ...[...models].sort().map(type => `from ...models.${snake(type)} import ${type}`),
            ...(isLayer
                ? ['from ...api.sprite_api import SpriteAPI', 'from .sprite import Sprite']
                : []),
            ...(optionNames.size
                ? [`from ..options import ${[...optionNames].sort().join(', ')}, to_kwargs`]
                : []),
            'from typing import Any, Dict, List, Optional',
            ...(hasOverload ? ['from typing import overload'] : []),
        ];
        const methods = operations
            .map(op => {
                const option = optionFor(contract, op);
                const hasOptions = (option?.properties.length ?? 0) > 0;
                let args = '';
                let call = '';
                if (isLayer && op.body === 'HandleRequest') {
                    args = ', sprite: int';
                    call = 'HandleRequest(layer=self._handle, sprite=sprite)';
                } else if (option) {
                    args = hasOptions ? `, options: ${option.name}` : '';
                    call = pythonRequest(tag, option.source, hasOptions);
                }
                const result = pythonResult(tag, op.name, op);
                const name = snake(op.name);
                let body: string;
                if (isLayer && (op.name === 'createSprite' || op.name === 'createAnimation')) {
                    body = `        handle = self._api.${name}(${call})\n        return Sprite(self._sprites, handle.layer, handle.sprite)`;
                } else if (isLayer && op.name === 'createSegment') {
                    body = `        handle = self._api.${name}(${call})\n        return Layer(self._api, self._sprites, handle.layer)`;
                } else {
                    const info = cleanResultInfo(tag, op);
                    if (info.kind === 'void') {
                        body = `        self._api.${name}(${call})`;
                    } else if (info.property) {
                        body = `        return self._api.${name}(${call}).${snake(info.property)}`;
                    } else {
                        body = `        return self._api.${name}(${call})`;
                    }
                }
                const overload =
                    option && option.properties.length > 0 ? orderedProperties(option) : [];
                const docParams =
                    isLayer && op.body === 'HandleRequest'
                        ? [{ name: 'sprite', description: 'The sprite handle.' }]
                        : [];
                if (overload.length === 0) {
                    return `    def ${name}(self${args}) -> ${result}:\n${pythonMethodDoc(tag, op, option, docParams)}${body}`;
                }
                const requiredCount = overload.filter(property => property.required).length;
                const start = requiredCount === 0 ? 0 : requiredCount;
                const signatures: string[] = [];
                for (let arity = start; arity <= overload.length; arity++) {
                    const subset = overload.slice(0, arity);
                    const parameters = subset
                        .map(property => {
                            const type = pythonType({
                                name: property.name,
                                property: property.schema,
                                required: property.required,
                            });
                            return `${property.name}: ${type}`;
                        })
                        .join(', ');
                    signatures.push(
                        `    @overload\n    def ${name}(self${parameters ? `, ${parameters}` : ''}) -> ${result}:\n${pythonMethodDoc(
                            tag,
                            op,
                            option,
                            subset.map(property => ({
                                name: property.name,
                                description: parameterDescription(property),
                            }))
                        )}        ...`
                    );
                }
                signatures.push(
                    `    @overload\n    def ${name}(self, options: ${option.name}) -> ${result}:\n${pythonMethodDoc(
                        tag,
                        op,
                        option,
                        [{ name: 'options', description: optionsDescription(option) }]
                    )}        ...`
                );
                return `${signatures.join('\n\n')}\n\n    def ${name}(self, *args: object, **kwargs: object) -> ${result}:\n${pythonMethodDoc(tag, op, option, docParams)}        options = args[0] if args and isinstance(args[0], ${option.name}) else ${option.name}(*args, **kwargs)\n${body}`;
            })
            .join('\n\n');
        const constructor = isLayer
            ? '    def __init__(self, api: LayerAPI, sprites: SpriteAPI, handle: int) -> None:\n        self._api = api\n        self._sprites = sprites\n        self._handle = handle'
            : bound
              ? `    def __init__(self, api: ${tag}API, layer: int, sprite: int) -> None:\n        self._api = api\n        self._layer = layer\n        self._sprite = sprite`
              : `    def __init__(self, api: ${tag}API) -> None:\n        self._api = api`;
        const text = `${imports.join('\n')}\n\n\nclass ${tag}:\n    """The clean \`${tag}\` client."""\n\n${constructor}\n\n    @staticmethod\n    def instance_of(value: object) -> bool:\n        return isinstance(value, ${tag})\n\n${methods}\n`;
        writeFileSync(join(outDir, `${snake(tag)}.py`), text, 'utf8');
        sections[section].push(tag);
    }
    for (const [section, tags] of Object.entries(sections)) {
        writeFileSync(
            join(dir, section, '__init__.py'),
            `${tags.map(tag => `from .${snake(tag)} import ${tag}`).join('\n')}\n\n__all__ = [${tags.map(tag => `"${tag}"`).join(', ')}]\n`,
            'utf8'
        );
    }
    writeFileSync(
        join(dir, '__init__.py'),
        `${Object.entries(sections)
            .map(([section, tags]) => `from .${section} import ${tags.join(', ')}`)
            .join('\n')}\n\n__all__ = [${Object.values(sections)
            .flat()
            .map(tag => `"${tag}"`)
            .join(', ')}]\n`,
        'utf8'
    );
}

function goType(property: { name: string; property: any; required: boolean }): {
    type: string;
    pointer: boolean;
    container: boolean;
} {
    const inner =
        property.property.oneOf?.find((part: any) => part.type !== 'null') ?? property.property;
    const types = Array.isArray(inner.type)
        ? inner.type.filter((type: string) => type !== 'null')
        : [inner.type];
    let base: string;
    let container = false;
    if (inner.$ref) {
        base = `osbtk.${refName(inner)!}`;
    } else if (types.includes('array')) {
        base = `[]${goType({ name: property.name, property: inner.items ?? {}, required: true }).type}`;
        container = true;
    } else {
        switch (types[0]) {
            case 'integer':
                base = 'int32';
                break;
            case 'number':
                base = 'float64';
                break;
            case 'boolean':
                base = 'bool';
                break;
            case 'string':
                base = 'string';
                break;
            case 'object':
                base = 'map[string]interface{}';
                container = true;
                break;
            default:
                base = 'interface{}';
                container = true;
        }
    }
    if (property.required || container) return { type: base, pointer: false, container };
    return { type: `*${base}`, pointer: true, container: false };
}

function goDoc(text: string | undefined, indent = ''): string {
    if (!text) return '';
    return `${text
        .split('\n')
        .map(line => `${indent}// ${line}`.trimEnd())
        .join('\n')}\n`;
}

/** A method comment with the summary, parameter docs and an optional return. */
function goMethodDoc(
    tag: string,
    op: Operation,
    option: OptionModel | undefined,
    params: { name: string; description: string }[]
): string {
    const oneLine = (text: string) => text.replace(/\s+/g, ' ').trim();
    const lines = [`// ${pascal(op.name)}: ${oneLine(operationSummary(tag, op, option))}`];
    if (params.length > 0) lines.push('//');
    for (const param of params) {
        lines.push(`// ${param.name}: ${oneLine(param.description)}`);
    }
    if (op.responseDescription) {
        lines.push('//', `// returns: ${oneLine(op.responseDescription)}`);
    }
    return `${lines.join('\n')}\n`;
}

function emitGoOptions(dir: string, contract: Contract): void {
    const structs = [...contract.options.values()]
        .map(option => {
            const fields = option.properties
                .map(property => {
                    const type = goType({
                        name: property.name,
                        property: property.schema,
                        required: property.required,
                    });
                    return `${goDoc(property.description, '\t')}\t${pascal(property.name)} ${type.type}`;
                })
                .join('\n');
            return `${goDoc(option.description)}type ${option.name} struct {\n${fields}\n}`;
        })
        .join('\n\n');
    const text = `package options\n\nimport (\n\tosbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"\n)\n\n${structs}\n`;
    const outDir = join(dir, 'options');
    mkdirSync(outDir, { recursive: true });
    writeFileSync(join(outDir, 'options.go'), text, 'utf8');
}

function goBinding(tag: string, name: string): string {
    if (name === 'layerHandle') return 'c.handle';
    if (name === 'sprite') return tag === 'Sprite' ? 'c.sprite' : 'sprite';
    return tag === 'Sprite' ? 'c.layer' : 'c.handle';
}

function goRequest(tag: string, source: string, contract: Contract): string[] {
    const properties = allRequestProperties(source, contract);
    const required = properties.filter(property => property.required);
    const optional = properties.filter(property => !property.required);
    const args = required.map(property =>
        isBindingProperty(property.name, property.property)
            ? goBinding(tag, property.name)
            : `options.${pascal(property.name)}`
    );
    const lines = [`body := osbtk.New${source}(${args.join(', ')})`];
    for (const property of optional) {
        if (isBindingProperty(property.name, property.property)) {
            lines.push(`body.Set${pascal(property.name)}(${goBinding(tag, property.name)})`);
            continue;
        }
        const type = goType({
            name: property.name,
            property: property.property,
            required: property.required,
        });
        const value = type.pointer
            ? `*options.${pascal(property.name)}`
            : `options.${pascal(property.name)}`;
        lines.push(
            `if options.${pascal(property.name)} != nil {\n\t\tbody.Set${pascal(property.name)}(${value})\n\t}`
        );
    }
    return lines;
}

function goResult(tag: string, name: string, op: Operation): string {
    const info = cleanResultInfo(tag, op);
    switch (info.kind) {
        case 'sprite':
            return '*Sprite';
        case 'layerObject':
            return '*Layer';
        case 'void':
            return '';
        case 'bool':
            return 'bool';
        case 'int':
        case 'layer':
            return 'int32';
        case 'float':
            return 'float64';
        case 'string':
            return 'string';
        case 'point':
            return '*osbtk.Vector2';
        case 'unknown':
            return 'any';
        default:
            return info.array ? `[]osbtk.${info.raw}` : `*osbtk.${info.raw}`;
    }
}

function goZero(kind: ResultKind): string {
    switch (kind) {
        case 'bool':
            return 'false';
        case 'int':
        case 'layer':
            return '0';
        case 'float':
            return '0';
        case 'string':
            return '""';
        default:
            return 'nil';
    }
}

function emitGo(contract: Contract): void {
    const dir = join(targets.go, 'client');
    clear(dir);
    emitGoOptions(dir, contract);
    for (const [tag, operations] of [...contract.operations].sort()) {
        const isLayer = tag === 'Layer';
        const isSprite = tag === 'Sprite';
        const section = sectionOf(tag);
        const outDir = join(dir, section);
        mkdirSync(outDir, { recursive: true });
        const usesOptions = operations.some(
            op => (optionFor(contract, op)?.properties.length ?? 0) > 0
        );
        const methods = operations
            .map(op => {
                const name = pascal(op.name);
                const option = optionFor(contract, op);
                const hasOptions = (option?.properties.length ?? 0) > 0;
                let args = 'ctx context.Context';
                let body: string[] = [];
                if (isLayer && op.body === 'HandleRequest') {
                    args += ', sprite int32';
                    body = goRequest(tag, op.body, contract);
                } else if (option && op.body) {
                    if (hasOptions) args += `, options clean.${option.name}`;
                    body = goRequest(tag, option.source, contract);
                }
                const result = goResult(tag, op.name, op);
                const info = cleanResultInfo(tag, op);
                const call = op.body
                    ? `c.api.${name}(ctx).${op.body}(*body).Execute()`
                    : `c.api.${name}(ctx).Execute()`;
                const statements = [...body];
                if (info.kind === 'sprite') {
                    statements.push(
                        `handle, resp, err := ${call}\n\tif err != nil {\n\t\treturn nil, resp, err\n\t}\n\treturn NewSprite(c.sprites, handle.Layer, handle.Sprite), resp, nil`
                    );
                } else if (info.kind === 'layerObject') {
                    statements.push(
                        `handle, resp, err := ${call}\n\tif err != nil {\n\t\treturn nil, resp, err\n\t}\n\treturn NewLayer(c.api, c.sprites, handle.Layer), resp, nil`
                    );
                } else if (info.kind === 'void') {
                    statements.push(`_, resp, err := ${call}\n\treturn resp, err`);
                } else if (info.kind === 'raw') {
                    statements.push(`return ${call}`);
                } else {
                    const access = info.kind === 'point' ? 'Point.Get()' : pascal(info.property!);
                    statements.push(
                        `value, resp, err := ${call}\n\tif err != nil {\n\t\treturn ${goZero(info.kind)}, resp, err\n\t}\n\treturn value.${access}, resp, nil`
                    );
                }
                const signature = result
                    ? `(${result}, *http.Response, error)`
                    : '(*http.Response, error)';
                const docParams = [
                    { name: 'ctx', description: 'The request context.' },
                    ...(isLayer && op.body === 'HandleRequest'
                        ? [{ name: 'sprite', description: 'The sprite handle.' }]
                        : hasOptions
                          ? [{ name: 'options', description: optionsDescription(option!) }]
                          : []),
                ];
                const doc = goMethodDoc(tag, op, option, docParams);
                return `${doc}func (c *${tag}) ${name}(${args}) ${signature} {\n\t${statements.join('\n\t')}\n}`;
            })
            .join('\n\n');
        const member = isLayer
            ? `// Layer is the handle-bound clean layer client.\ntype Layer struct {\n\tapi     *osbtk.LayerAPIService\n\tsprites *osbtk.SpriteAPIService\n\thandle  int32\n}\n\nfunc NewLayer(api *osbtk.LayerAPIService, sprites *osbtk.SpriteAPIService, handle int32) *Layer {\n\treturn &Layer{api: api, sprites: sprites, handle: handle}\n}`
            : isSprite
              ? `// Sprite is the handle-bound clean sprite client.\ntype Sprite struct {\n\tapi    *osbtk.SpriteAPIService\n\tlayer  int32\n\tsprite int32\n}\n\nfunc NewSprite(api *osbtk.SpriteAPIService, layer int32, sprite int32) *Sprite {\n\treturn &Sprite{api: api, layer: layer, sprite: sprite}\n}`
              : `// ${tag} is the clean high-level client.\ntype ${tag} struct {\n\tapi *osbtk.${tag}APIService\n}\n\nfunc New${tag}(api *osbtk.${tag}APIService) *${tag} {\n\treturn &${tag}{api: api}\n}`;
        const optionsImport = usesOptions
            ? '\tclean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"\n'
            : '';
        const text = `package ${section}\n\nimport (\n\t"context"\n\t"net/http"\n\n${optionsImport}\tosbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"\n)\n\n${member}\n\nfunc Is${tag}(value any) bool {\n\t_, ok := value.(*${tag})\n\treturn ok\n}\n\n${methods}\n`;
        writeFileSync(join(outDir, `client_${snake(tag)}.go`), text, 'utf8');
    }
}

function javaType(property: { name: string; property: any; required: boolean }): string {
    const inner =
        property.property.oneOf?.find((part: any) => part.type !== 'null') ?? property.property;
    const types = Array.isArray(inner.type)
        ? inner.type.filter((type: string) => type !== 'null')
        : [inner.type];
    if (inner.$ref) return refName(inner)!;
    if (types.includes('array')) {
        return `List<${javaType({ name: property.name, property: inner.items ?? {}, required: true })}>`;
    }
    switch (types[0]) {
        case 'integer':
            return 'Integer';
        case 'number':
            return 'Double';
        case 'boolean':
            return 'Boolean';
        case 'string':
            return 'String';
        case 'object':
            return 'Object';
        default:
            return 'Object';
    }
}

function javaDoc(text: string | undefined, indent = ''): string {
    if (!text) return '';
    const body = text
        .split('\n')
        .map(line => `${indent} * ${line}`.trimEnd())
        .join('\n');
    return `${indent}/**\n${body}\n${indent} */\n`;
}

/** A method Javadoc block with the summary, parameter docs and return doc. */
function javaMethodDoc(
    tag: string,
    op: Operation,
    option: OptionModel | undefined,
    params: { name: string; description: string }[],
    hasReturn: boolean
): string {
    const oneLine = (text: string) => text.replace(/\s+/g, ' ').trim();
    const lines = ['    /**', `     * ${oneLine(operationSummary(tag, op, option))}`];
    if (params.length > 0) lines.push('     *');
    for (const param of params) {
        lines.push(`     * @param ${param.name} ${oneLine(param.description)}`);
    }
    if (hasReturn && op.responseDescription) {
        lines.push('     *', `     * @return ${oneLine(op.responseDescription)}`);
    }
    lines.push('     */');
    return `${lines.join('\n')}\n`;
}

function emitJavaOptions(dir: string, contract: Contract): void {
    for (const option of contract.options.values()) {
        const models = new Set<string>();
        for (const property of option.properties) {
            for (const ref of property.refs) models.add(ref);
        }
        const imports = [
            ...[...models].sort().map(type => `import moe.tochi.osbtk.model.${type};`),
            'import java.util.List;',
        ];
        const fields = option.properties
            .map(property => {
                const type = javaType({
                    name: property.name,
                    property: property.schema,
                    required: property.required,
                });
                return `${javaDoc(property.description, '    ')}    public final ${type} ${property.name};`;
            })
            .join('\n');
        const assignments = option.properties
            .map(property => `        this.${property.name} = builder.${property.name};`)
            .join('\n');
        const builderFields = option.properties
            .map(property => {
                const type = javaType({
                    name: property.name,
                    property: property.schema,
                    required: property.required,
                });
                return `        private ${type} ${property.name};`;
            })
            .join('\n');
        const builderSetters = option.properties
            .map(property => {
                const type = javaType({
                    name: property.name,
                    property: property.schema,
                    required: property.required,
                });
                return `        public Builder ${property.name}(${type} value) {\n            this.${property.name} = value;\n            return this;\n        }`;
            })
            .join('\n\n');
        const required = option.properties
            .filter(property => property.required)
            .map(
                property =>
                    `            java.util.Objects.requireNonNull(${property.name}, "${property.name}");`
            )
            .join('\n');
        const text = `package moe.tochi.osbtk.client;\n\n${imports.join('\n')}\n\n${javaDoc(option.description)}public final class ${option.name} {\n${fields}\n\n    private ${option.name}(Builder builder) {\n${assignments}\n    }\n\n    public static Builder builder() {\n        return new Builder();\n    }\n\n    public static final class Builder {\n${builderFields}\n\n${builderSetters}\n\n        public ${option.name} build() {\n${required ? `${required}\n` : ''}            return new ${option.name}(this);\n        }\n    }\n}\n`;
        writeFileSync(join(dir, `${option.name}.java`), text, 'utf8');
    }
}

function javaResult(tag: string, name: string, op: Operation): string {
    const info = cleanResultInfo(tag, op);
    switch (info.kind) {
        case 'sprite':
            return 'Sprite';
        case 'layerObject':
            return 'Layer';
        case 'void':
            return 'void';
        case 'bool':
            return 'boolean';
        case 'int':
        case 'layer':
            return 'int';
        case 'float':
            return 'double';
        case 'string':
            return 'String';
        case 'point':
            return 'Vector2';
        case 'unknown':
            return 'Object';
        default:
            return info.array ? `List<${info.raw}>` : (info.raw ?? 'void');
    }
}

function javaRequest(tag: string, source: string, contract: Contract): string {
    const properties = allRequestProperties(source, contract)
        .map(property => {
            let value: string;
            if (isBindingProperty(property.name, property.property)) {
                if (property.name === 'layerHandle') value = 'this.handle';
                else if (property.name === 'sprite')
                    value = tag === 'Sprite' ? 'this.sprite' : 'sprite';
                else value = tag === 'Sprite' ? 'this.layer' : 'this.handle';
            } else {
                value = `options.${property.name}`;
            }
            return `            .${property.name}(${value})`;
        })
        .join('\n');
    return `new ${source}()\n${properties}`;
}

function emitJava(contract: Contract): void {
    const dir = join(targets.java, 'moe', 'tochi', 'osbtk', 'client');
    clear(dir);
    emitJavaOptions(dir, contract);
    for (const [tag, operations] of [...contract.operations].sort()) {
        const isLayer = tag === 'Layer';
        const isSprite = tag === 'Sprite';
        const bound = isLayer || isSprite;
        const section = sectionOf(tag);
        const outDir = join(dir, section);
        mkdirSync(outDir, { recursive: true });
        const models = new Set<string>();
        const optionNames = new Set<string>();
        for (const op of operations) {
            if (op.body) models.add(op.body);
            const info = cleanResultInfo(tag, op);
            if (info.kind === 'raw' && info.raw) models.add(info.raw);
            if (info.kind === 'point') models.add('Vector2');
            if (isLayer && (op.name === 'createSprite' || op.name === 'createAnimation')) {
                models.add('Handle');
            }
            if (isLayer && op.name === 'createSegment') models.add('LayerHandle');
            const option = optionFor(contract, op);
            if (option) {
                if (option.properties.length > 0) optionNames.add(option.name);
                for (const property of option.properties) {
                    for (const ref of property.refs) models.add(ref);
                }
            }
        }
        const imports = [
            'import moe.tochi.osbtk.ApiException;',
            `import moe.tochi.osbtk.api.${tag}API;`,
            ...[...models].sort().map(type => `import moe.tochi.osbtk.model.${type};`),
            ...[...optionNames].sort().map(type => `import moe.tochi.osbtk.client.${type};`),
            ...(isLayer ? ['import moe.tochi.osbtk.api.SpriteAPI;'] : []),
        ];
        const methods = operations
            .map(op => {
                const option = optionFor(contract, op);
                const hasOptions = (option?.properties.length ?? 0) > 0;
                let args = '';
                let call = '';
                if (isLayer && op.body === 'HandleRequest') {
                    args = 'int sprite';
                    call = javaRequest(tag, op.body, contract);
                } else if (option) {
                    args = hasOptions ? `${option.name} options` : '';
                    call = javaRequest(tag, option.source, contract);
                }
                const result = javaResult(tag, op.name, op);
                let text: string;
                if (isLayer && (op.name === 'createSprite' || op.name === 'createAnimation')) {
                    text = `    public ${result} ${op.name}(${args}) throws ApiException {\n        Handle handle = api.${op.name}(${call});\n        return new Sprite(this.sprites, handle.getLayer(), handle.getSprite());\n    }`;
                } else if (isLayer && op.name === 'createSegment') {
                    text = `    public ${result} ${op.name}(${args}) throws ApiException {\n        LayerHandle handle = api.${op.name}(${call});\n        return new Layer(this.api, this.sprites, handle.getLayer());\n    }`;
                } else {
                    const info = cleanResultInfo(tag, op);
                    if (info.kind === 'void') {
                        text = `    public void ${op.name}(${args}) throws ApiException {\n        api.${op.name}(${call});\n    }`;
                    } else {
                        const access = info.property ? `.get${pascal(info.property)}()` : '';
                        text = `    public ${result} ${op.name}(${args}) throws ApiException {\n        return api.${op.name}(${call})${access};\n    }`;
                    }
                }
                const overloads: string[] = [];
                if (option && option.properties.length > 0) {
                    const order = orderedProperties(option);
                    const requiredCount = order.filter(property => property.required).length;
                    const start = requiredCount === 0 ? 0 : requiredCount;
                    for (let arity = start; arity <= order.length; arity++) {
                        const subset = order.slice(0, arity);
                        const parameters = subset
                            .map(property => {
                                const type = javaType({
                                    name: property.name,
                                    property: property.schema,
                                    required: property.required,
                                });
                                return `${type} ${property.name}`;
                            })
                            .join(', ');
                        const setters = subset
                            .map(property => `.${property.name}(${property.name})`)
                            .join('');
                        const invocation = `${op.name}(${option.name}.builder()${setters}.build());`;
                        const body =
                            result === 'void'
                                ? `        ${invocation}`
                                : `        return ${invocation}`;
                        const doc = javaMethodDoc(
                            tag,
                            op,
                            option,
                            subset.map(property => ({
                                name: property.name,
                                description: parameterDescription(property),
                            })),
                            result !== 'void'
                        );
                        overloads.push(
                            `${doc}    public ${result} ${op.name}(${parameters}) throws ApiException {\n${body}\n    }`
                        );
                    }
                }
                const docParams =
                    isLayer && op.body === 'HandleRequest'
                        ? [{ name: 'sprite', description: 'The sprite handle.' }]
                        : hasOptions
                          ? [{ name: 'options', description: optionsDescription(option!) }]
                          : [];
                const doc = javaMethodDoc(tag, op, option, docParams, result !== 'void');
                return [...overloads, `${doc}${text}`].join('\n\n');
            })
            .join('\n\n');
        const member = isLayer
            ? `    private final LayerAPI api;\n    private final SpriteAPI sprites;\n    private final int handle;\n\n    public Layer(LayerAPI api, SpriteAPI sprites, int handle) {\n        this.api = api;\n        this.sprites = sprites;\n        this.handle = handle;\n    }`
            : bound
              ? `    private final SpriteAPI api;\n    private final int layer;\n    private final int sprite;\n\n    public Sprite(SpriteAPI api, int layer, int sprite) {\n        this.api = api;\n        this.layer = layer;\n        this.sprite = sprite;\n    }`
              : `    private final ${tag}API api;\n\n    public ${tag}(${tag}API api) {\n        this.api = api;\n    }`;
        const text = `package moe.tochi.osbtk.client.${section};\n\n${imports.join('\n')}\nimport java.util.List;\n\n${javaDoc(`The clean \`${tag}\` client.`)}public final class ${tag} {\n${member}\n\n    public static boolean instanceOf(Object value) {\n        return value instanceof ${tag};\n    }\n\n${methods}\n}\n`;
        writeFileSync(join(outDir, `${tag}.java`), text, 'utf8');
    }
}

function allRequestProperties(
    source: string,
    contract: Contract
): { name: string; property: any; required: boolean }[] {
    const schema = contract.schemas[source];
    const required = new Set<string>(schema?.required ?? []);
    return Object.entries<any>(schema?.properties ?? {})
        .map(([name, property]) => ({ name, property, required: required.has(name) }))
        .sort((a, b) => a.name.localeCompare(b.name));
}

function csharpType(
    property: { name: string; property: any; required: boolean },
    contract: Contract
): string {
    const inner =
        property.property.oneOf?.find((part: any) => part.type !== 'null') ?? property.property;
    const types = Array.isArray(inner.type)
        ? inner.type.filter((type: string) => type !== 'null')
        : [inner.type];
    if (inner.$ref) {
        const name = refName(inner)!;
        const isEnum = Array.isArray(contract.schemas[name]?.enum);
        return isEnum && !property.required ? `${name}?` : name;
    }
    if (types.includes('array')) {
        const item = csharpType(
            { name: property.name, property: inner.items ?? {}, required: true },
            contract
        );
        return `List<${item}>`;
    }
    switch (types[0]) {
        case 'integer':
            return property.required ? 'int' : 'int?';
        case 'number':
            return property.required ? 'double' : 'double?';
        case 'boolean':
            return property.required ? 'bool' : 'bool?';
        case 'string':
            return 'string';
        case 'object':
            return 'object';
        default:
            return 'object';
    }
}

function xml(text: string | undefined, indent = '    '): string {
    if (!text) return '';
    const body = text
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;')
        .split('\n')
        .map(line => `${indent}/// ${line}`.trimEnd())
        .join('\n');
    return `${indent}/// <summary>\n${body}\n${indent}/// </summary>\n`;
}

/** A method XML-doc block with the summary, parameter docs and return doc. */
function xmlMethodDoc(
    tag: string,
    op: Operation,
    option: OptionModel | undefined,
    params: { name: string; description: string }[],
    hasReturn: boolean
): string {
    const escape = (text: string) =>
        text.replace(/\s+/g, ' ').trim().replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
    const lines = [
        '    /// <summary>',
        `    /// ${escape(operationSummary(tag, op, option))}`,
        '    /// </summary>',
    ];
    for (const param of params) {
        lines.push(`    /// <param name="${param.name}">${escape(param.description)}</param>`);
    }
    if (hasReturn && op.responseDescription) {
        lines.push(`    /// <returns>${escape(op.responseDescription)}</returns>`);
    }
    return `${lines.join('\n')}\n`;
}

function emitCSharpOptions(dir: string, contract: Contract): void {
    const classes = [...contract.options.values()]
        .map(option => {
            const properties = option.properties
                .map(property => {
                    const type = csharpType(
                        {
                            name: property.name,
                            property: property.schema,
                            required: property.required,
                        },
                        contract
                    );
                    return `${xml(property.description)}    public ${type} ${pascal(property.name)} { get; set; }`;
                })
                .join('\n\n');
            return `${xml(option.description, '')}public sealed class ${option.name}\n{\n${properties}\n}`;
        })
        .join('\n\n');
    const text = `#nullable disable\nusing System.Collections.Generic;\nusing tochiRes.osbTK.Model;\n\nnamespace tochiRes.osbTK.Client;\n\n${classes}\n`;
    writeFileSync(join(dir, 'Options.cs'), text, 'utf8');
}

function csharpRequest(tag: string, source: string, contract: Contract): string {
    const properties = allRequestProperties(source, contract).map(property => {
        if (isBindingProperty(property.name, property.property)) {
            if (property.name === 'layerHandle') return 'this.handle';
            if (property.name === 'sprite') return tag === 'Sprite' ? 'this.sprite' : 'sprite';
            return tag === 'Sprite' ? 'this.layer' : 'this.handle';
        }
        return `options.${pascal(property.name)}`;
    });
    return `new ${source}(${properties.join(', ')})`;
}

function emitCSharp(contract: Contract): void {
    const dir = join(targets.csharp, '..', 'Client');
    clear(dir);
    emitCSharpOptions(dir, contract);
    for (const [tag, operations] of [...contract.operations].sort()) {
        const isLayer = tag === 'Layer';
        const isSprite = tag === 'Sprite';
        const bound = isLayer || isSprite;
        const section = sectionOf(tag);
        const outDir = join(dir, pascal(section));
        mkdirSync(outDir, { recursive: true });
        const methods = operations
            .map(op => {
                const option = optionFor(contract, op);
                const hasOptions = (option?.properties.length ?? 0) > 0;
                let args = '';
                let call = '';
                if (isLayer && op.body === 'HandleRequest') {
                    args = 'int sprite';
                    call = csharpRequest(tag, op.body, contract);
                } else if (option) {
                    args = hasOptions ? `${option.name} options` : '';
                    call = csharpRequest(tag, option.source, contract);
                }
                const result = csharpResult(tag, op.name, op);
                const method = pascal(op.name);
                let text: string;
                if (isLayer && (op.name === 'createSprite' || op.name === 'createAnimation')) {
                    text = `    public ${result} ${method}(${args})\n    {\n        var handle = api.${method}(${call});\n        return new Sprite(this.sprites, handle.Layer, handle.Sprite);\n    }`;
                } else if (isLayer && op.name === 'createSegment') {
                    text = `    public ${result} ${method}(${args})\n    {\n        var handle = api.${method}(${call});\n        return new Layer(this.api, this.sprites, handle.Layer);\n    }`;
                } else {
                    const info = cleanResultInfo(tag, op);
                    if (info.kind === 'void') {
                        text = `    public void ${method}(${args})\n    {\n        api.${method}(${call});\n    }`;
                    } else {
                        const access = info.property
                            ? `.${info.property === 'version' ? 'VarVersion' : pascal(info.property)}`
                            : '';
                        text = `    public ${result} ${method}(${args}) => api.${method}(${call})${access};`;
                    }
                }
                const overload =
                    option && option.properties.length > 0 ? orderedProperties(option) : [];
                const overloads: string[] = [];
                if (overload.length) {
                    const requiredCount = overload.filter(property => property.required).length;
                    const start = requiredCount === 0 ? 0 : requiredCount;
                    for (let arity = start; arity <= overload.length; arity++) {
                        const subset = overload.slice(0, arity);
                        const parameters = subset
                            .map(property => {
                                const type = csharpType(
                                    {
                                        name: property.name,
                                        property: property.schema,
                                        required: property.required,
                                    },
                                    contract
                                );
                                return `${type} ${pascal(property.name)}`;
                            })
                            .join(', ');
                        const initializer = subset
                            .map(property => `${pascal(property.name)} = ${pascal(property.name)}`)
                            .join(', ');
                        const doc = xmlMethodDoc(
                            tag,
                            op,
                            option,
                            subset.map(property => ({
                                name: pascal(property.name),
                                description: parameterDescription(property),
                            })),
                            result !== 'void'
                        );
                        overloads.push(
                            `${doc}    public ${result} ${method}(${parameters}) =>\n        ${method}(new ${option.name} { ${initializer} });`
                        );
                    }
                }
                const docParams =
                    isLayer && op.body === 'HandleRequest'
                        ? [{ name: 'sprite', description: 'The sprite handle.' }]
                        : hasOptions
                          ? [{ name: 'options', description: optionsDescription(option!) }]
                          : [];
                const doc = xmlMethodDoc(tag, op, option, docParams, result !== 'void');
                return [...overloads, `${doc}${text}`].join('\n\n');
            })
            .join('\n\n');
        const member = isLayer
            ? `    private readonly LayerAPI api;\n    private readonly SpriteAPI sprites;\n    private readonly int handle;\n\n    public Layer(LayerAPI api, SpriteAPI sprites, int handle)\n    {\n        this.api = api;\n        this.sprites = sprites;\n        this.handle = handle;\n    }`
            : bound
              ? `    private readonly ${tag}API api;\n    private readonly int layer;\n    private readonly int sprite;\n\n    public Sprite(SpriteAPI api, int layer, int sprite)\n    {\n        this.api = api;\n        this.layer = layer;\n        this.sprite = sprite;\n    }`
              : `    private readonly ${tag}API api;\n\n    public ${tag}(${tag}API api)\n    {\n        this.api = api;\n    }`;
        const text = `#nullable disable\nusing tochiRes.osbTK.Api;\nusing tochiRes.osbTK.Client;\nusing tochiRes.osbTK.Model;\n\nnamespace tochiRes.osbTK.Client.${pascal(section)};\n\n${xml(`The clean \`${tag}\` client.`, '')}public sealed class ${tag}\n{\n${member}\n\n    public static bool InstanceOf(object value) => value is ${tag};\n\n${methods}\n}\n`;
        writeFileSync(join(outDir, `${tag}.cs`), text, 'utf8');
    }
}

function csharpResult(tag: string, name: string, op: Operation): string {
    const info = cleanResultInfo(tag, op);
    switch (info.kind) {
        case 'sprite':
            return 'Sprite';
        case 'layerObject':
            return 'Layer';
        case 'void':
            return 'void';
        case 'bool':
            return 'bool';
        case 'int':
            return 'int';
        case 'float':
            return 'double';
        case 'string':
            return 'string';
        case 'point':
            return 'Vector2';
        case 'unknown':
            return 'object';
        case 'layer':
            return 'int';
        default:
            return info.array ? `List<${info.raw}>` : (info.raw ?? 'void');
    }
}

/** The stateless tags that get a clean Rust client. */
const rustCleanTags = [
    'Animations',
    'Audio',
    'Common',
    'Curve',
    'Diagnostics',
    'Helpers',
    'Image',
    'Imagegen',
    'Media',
    'Optimize',
    'Origin',
    'Parsing',
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
        case 'Common.version':
            return 'crate::Client::version()';
        case 'Curve.length':
            return 'crate::Client::curve_length(options.curve)';
        case 'Curve.boundingBox':
            return 'crate::Client::curve_bounding_box(options.curve)';
        case 'Curve.point':
            return 'crate::Client::curve_point(\n            options.curve,\n            options.t,\n            options.by_arc_length.unwrap_or(false),\n        )';
        case 'Curve.tangent':
            return 'crate::Client::curve_tangent(options.curve, options.t)';
        case 'Curve.sample':
            return 'crate::Client::curve_sample(\n            options.curve,\n            options.count as usize,\n            options.by_arc_length.unwrap_or(false),\n        )';
        case 'Curve.simplify':
            return 'crate::Client::simplify(options.points, options.tolerance)';
        case 'Curve.normal':
            return 'crate::Client::curve_normal(options.curve, options.t)';
        case 'Curve.curvature':
            return 'crate::Client::curve_curvature(options.curve, options.t)';
        case 'Curve.distanceTo':
            return 'crate::Client::curve_distance_to(options.curve, options.point)';
        case 'Curve.offset':
            return 'crate::Client::curve_offset(options.curve, options.distance)';
        case 'Curve.arc':
            return 'crate::Client::curve_arc(\n            options.center,\n            options.radius,\n            options.start_angle,\n            options.sweep_angle,\n            options.count.unwrap_or(64).max(0) as usize,\n        )';
        case 'Curve.ellipse':
            return 'crate::Client::curve_ellipse(\n            options.center,\n            options.radius_x,\n            options.radius_y,\n            options.count.unwrap_or(64).max(0) as usize,\n        )';
        case 'Curve.lineIntersect':
            return 'crate::Client::line_intersect(options.a1, options.a2, options.b1, options.b2)';
        case 'Animations.applyEasing':
            return 'crate::Client::apply_easing(options.easing, options.t)';
        case 'Animations.easingValue':
            return 'crate::Client::easing_value(options.t, options.easing, options.cubic_bezier)';
        case 'Parsing.parseOsu':
            return 'crate::Client::parse_beatmap(&options.text)';
        case 'Parsing.parseOsb':
            return 'crate::Client::parse_osb(&options.text)';
        case 'Origin.getOriginVector':
            return 'crate::Client::get_origin_vector(options.origin, options.width, options.height)';
        case 'Origin.inScreenBounds':
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

/**
 * Writes the class-shaped SDK reference from the contract: the clean clients,
 * their `<X>Options` types, the positional overloads and the unwrapped result
 * shapes. The raw model pages are generated separately from OpenAPI markdown.
 */
export function emitCleanDocs(): void {
    const contract = readContract();
    const output = join(root, 'docs', 'reference', 'api');
    const sections: Record<'runtime' | 'core' | 'libraries', string[]> = {
        runtime: [],
        core: [],
        libraries: [],
    };

    for (const section of ['runtime', 'core', 'libraries'] as const) {
        rmSync(join(output, section), { recursive: true, force: true });
    }

    for (const [tag, operations] of [...contract.operations].sort()) {
        const section = sectionOf(tag);
        sections[section].push(tag);
        mkdirSync(join(output, section), { recursive: true });
        const lines = [
            `# ${tag}`,
            '',
            `The clean \`${tag}\` client. Its overloads take positional values from the`,
            'smallest required arity upward, then the `<X>Options` form; results are',
            'unwrapped into ordinary values.',
            '',
            '## Methods',
            '',
        ];

        for (const op of operations) {
            const option = optionFor(contract, op);
            const result = tsResultType(tag, op);
            const signatures: string[] = [];
            if (tag === 'Layer' && op.body === 'HandleRequest') {
                signatures.push(`${op.name}(sprite: number): ${result}`);
            } else if (option && option.properties.length > 0) {
                const order = orderedProperties(option);
                const requiredCount = order.filter(property => property.required).length;
                for (let arity = requiredCount; arity <= order.length; arity++) {
                    const parameters = order
                        .slice(0, arity)
                        .map(property => `${property.name}: ${tsPropertyType(property)}`)
                        .join(', ');
                    signatures.push(`${op.name}(${parameters}): ${result}`);
                }
                signatures.push(`${op.name}(options: ${option.name}): ${result}`);
            } else {
                signatures.push(`${op.name}(): ${result}`);
            }

            lines.push(`### \`${op.name}\``, '');
            if (op.description) lines.push(op.description.trim(), '');
            lines.push('```ts', ...signatures, '```', '');

            if (option && option.properties.length > 0) {
                lines.push(
                    `**Options — \`${option.name}\`**`,
                    '',
                    option.description ? `${option.description.trim()}\n` : '',
                    '| Property | Type | Required | Description |',
                    '| --- | --- | --- | --- |',
                    ...orderedProperties(option).map(property => {
                        const description = (property.description ?? '').replaceAll('|', '\\|');
                        return `| \`${property.name}\` | \`${tsPropertyType(property)}\` | ${property.required ? 'yes' : 'no'} | ${description} |`;
                    }),
                    ''
                );
            }
        }

        writeFileSync(join(output, section, `${tag}.md`), `${lines.join('\n').trimEnd()}\n`, 'utf8');
    }

    const list = (names: string[]) =>
        names
            .sort()
            .map(name => `- [${name}](./${sectionOf(name)}/${name}.md)`)
            .join('\n');

    writeFileSync(
        join(output, 'index.md'),
        [
            '# SDK reference',
            '',
            'The typed SDK surface, generated from the native bridge contract. The',
            'non-runtime `Core` classes have no handles or storyboard state; the',
            'runtime classes are bound to a live runtime; the libraries are the',
            'stateless media/image/etc. helpers.',
            '',
            'Every options-based method takes the individual values positionally',
            '(ordered by ascending arity), then the `<X>Options` form. TypeScript, C#,',
            'Java and Python emit real overloads; Go has no overloading and uses the',
            'options form only. Single-field results are unwrapped: `OkResult` is',
            '`void`, `BoolResult` a boolean, scalar wrappers a number, and so on.',
            '',
            '## Runtime',
            '',
            list(sections.runtime),
            '',
            '## Core',
            '',
            list(sections.core),
            '',
            '## Libraries',
            '',
            list(sections.libraries),
            '',
            '## Models',
            '',
            'The shared request/response models are documented in the',
            '[models index](./models/index.md).',
            '',
        ].join('\n'),
        'utf8'
    );

    console.log(`openapi: generated SDK docs into ${output}`);
}

/**
 * Generates the clean, `API`-suffix-free high-level client classes for every
 * OpenAPI SDK from the contract. Each class holds its generated client
 * privately and forwards the operations 1:1. Handle-bound classes (`Layer`,
 * `Sprite`) own their handles and expose clean `<X>Options` types without the
 * binding-only props, so a consumer never passes one.
 */
export function emitTypeScriptClient(): void {
    emitTypeScript(readContract());
}

export function emitPythonClient(): void {
    emitPython(readContract());
}

export function emitGoClient(): void {
    emitGo(readContract());
}

export function emitJavaClient(): void {
    emitJava(readContract());
}

export function emitCSharpClient(): void {
    emitCSharp(readContract());
}
