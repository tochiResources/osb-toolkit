import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { targets } from '../shared';
import {
    allRequestProperties,
    cleanResultInfo,
    clear,
    isBindingProperty,
    operationSummary,
    optionFor,
    optionsDescription,
    orderedProperties,
    parameterDescription,
    pascal,
    readContract,
    refName,
    sectionOf,
} from './shared';
import type { Contract, Operation, OptionModel } from './shared';

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

export function emitCSharpClient(): void {
    emitCSharp(readContract());
}
