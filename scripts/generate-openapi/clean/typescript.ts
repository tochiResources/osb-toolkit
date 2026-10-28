import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { targets } from '../shared';
import {
    cleanResultInfo,
    clear,
    jsdoc,
    operationSummary,
    optionFor,
    optionsDescription,
    orderedProperties,
    parameterDescription,
    pascal,
    readContract,
    sectionOf,
} from './shared';
import type { Contract, Operation, OptionModel, OptionProperty } from './shared';

export function tsPropertyType(property: OptionProperty): string {
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

export function tsResultType(tag: string, op: Operation): string {
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

export function emitTypeScriptClient(): void {
    emitTypeScript(readContract());
}
