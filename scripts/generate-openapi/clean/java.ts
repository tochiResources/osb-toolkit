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
        if (option.properties.length === 0) continue;
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

export function emitJavaClient(): void {
    emitJava(readContract());
}
