import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { targets } from '../shared';
import {
    cleanResultInfo,
    clear,
    operationSummary,
    optionFor,
    optionsDescription,
    orderedProperties,
    parameterDescription,
    readContract,
    refName,
    sectionOf,
    snake,
} from './shared';
import type { Contract, Operation, OptionModel } from './shared';

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

export function emitPythonClient(): void {
    emitPython(readContract());
}
