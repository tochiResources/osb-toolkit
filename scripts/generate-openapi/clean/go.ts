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
    pascal,
    readContract,
    refName,
    sectionOf,
    snake,
} from './shared';
import type { Contract, Operation, ResultKind } from './shared';

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
        .filter(option => option.properties.length > 0)
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

const clientkitRuntime = new Set(['Audio', 'Generator', 'Helpers', 'Storyboarding3d']);
const runtimeLive = new Set(['Audio', 'Generator']);

/** Writes the composed clean `clientkit` facade over the generated clients. */
function emitGoClientkit(contract: Contract): void {
    const tags = [...contract.operations.keys()].sort();
    const runtime = tags.filter(tag => clientkitRuntime.has(tag));
    const live = tags.filter(tag => runtimeLive.has(tag));
    const libraries = tags.filter(
        tag => tag !== 'Core' && !clientkitRuntime.has(tag) && tag !== 'Layer' && tag !== 'Sprite'
    );
    const hasCore = tags.includes('Core');

    const fields: string[] = ['\tAPI *osbtk.ClientAPI', ''];
    if (hasCore) fields.push('\tCore *core.Core', '');
    for (const tag of runtime) fields.push(`\t${tag} *cleanruntime.${tag}`);
    if (runtime.length > 0 && libraries.length > 0) fields.push('');
    for (const tag of libraries) fields.push(`\t${tag} *libraries.${tag}`);

    const inits: string[] = ['\t\tAPI: api,'];
    if (hasCore) inits.push('\t\tCore: core.NewCore(api.CoreAPI),');
    for (const tag of runtime) inits.push(`\t\t${tag}: cleanruntime.New${tag}(api.${tag}API),`);
    for (const tag of libraries) inits.push(`\t\t${tag}: libraries.New${tag}(api.${tag}API),`);

    const importLines = [
        '\t"context"',
        '\t"errors"',
        '',
        '\t"github.com/tochiResources/osb-toolkit/bindings/go/osbtk"',
        ...(hasCore
            ? ['\tcore "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/core"']
            : []),
        ...(libraries.length > 0
            ? [
                  '\tlibraries "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/libraries"',
              ]
            : []),
        '\tclean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"',
        '\tcleanruntime "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/runtime"',
    ];

    const text = `// Code generated by scripts/generate-openapi/clean/go.ts; DO NOT EDIT.

// Package clientkit composes the generated clean clients over the native
// bridge. The root osbtk package cannot import its own client subpackages (they
// import the generated models from the root), so this subpackage is the
// composed clean surface: one Client for the stateless libraries and one
// Runtime that resolves handle-bound Layer/Sprite values.
package clientkit

import (
${importLines.join('\n')}
)

// Client is the stateless SDK surface: the core and library clean clients.
type Client struct {
${fields.join('\n')}
}

// New builds the stateless client over the native bridge.
func New() (*Client, error) {
	api, err := osbtk.Client()
	if err != nil {
		return nil, err
	}
	return &Client{
${inits.join('\n')}
	}, nil
}

// Runtime is a live storyboard runtime with the clean generator/audio clients
// and helpers that bind Layer/Sprite handles.
type Runtime struct {
	*osbtk.Runtime

${live.map(tag => `\t${tag} *cleanruntime.${tag}`).join('\n')}

	layers  *osbtk.LayerAPIService
	sprites *osbtk.SpriteAPIService
}

// NewRuntime creates a runtime of the given storyboard resolution.
func NewRuntime(width, height float64) (*Runtime, error) {
	instance, err := osbtk.NewRuntime(width, height)
	if err != nil {
		return nil, err
	}
	return &Runtime{
		Runtime: instance,
${live.map(tag => `\t\t${tag}: cleanruntime.New${tag}(instance.${tag}()),`).join('\n')}
		layers:  instance.Layer(),
		sprites: instance.Sprite(),
	}, nil
}

// Layer resolves (creating on first use) a layer by name and binds its handle.
func (r *Runtime) Layer(name string) (*cleanruntime.Layer, error) {
	if name == "" {
		return nil, errors.New("osbtk: layer name is required")
	}
	handle, _, err := r.Generator.LayerHandle(
		context.Background(),
		clean.LayerHandleOptions{Name: name},
	)
	if err != nil {
		return nil, err
	}
	return cleanruntime.NewLayer(r.layers, r.sprites, handle), nil
}

// Sprite binds an existing layer/sprite handle pair.
func (r *Runtime) Sprite(layer, sprite int32) *cleanruntime.Sprite {
	return cleanruntime.NewSprite(r.sprites, layer, sprite)
}
`;
    writeFileSync(join(targets.go, 'clientkit', 'clientkit.go'), text, 'utf8');
}

export function emitGoClient(): void {
    const contract = readContract();
    emitGo(contract);
    emitGoClientkit(contract);
}
