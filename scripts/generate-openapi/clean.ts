import { mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { specPath, targets } from './shared';

interface Operation {
    tag: string;
    name: string;
    body?: string;
    result?: string;
    resultArray?: boolean;
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

function readOperations(): Map<string, Operation[]> {
    const spec = JSON.parse(readFileSync(specPath, 'utf8'));
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
            });
            byTag.set(tag, list);
        }
    }
    return byTag;
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

function resultType(op: Operation, array: (t: string) => string, fallback: string): string {
    if (!op.result) return fallback;
    const base = op.result;
    return op.resultArray ? array(base) : base;
}

function emitTypeScript(ops: Map<string, Operation[]>): void {
    const dir = join(targets.typescript, 'client');
    clear(dir);
    const sections: Record<string, string[]> = { runtime: [], core: [], libraries: [] };
    for (const [tag, operations] of [...ops].sort()) {
        const section = sectionOf(tag);
        const outDir = join(dir, section);
        mkdirSync(outDir, { recursive: true });
        const models = new Set<string>();
        for (const op of operations) {
            if (op.body) models.add(op.body);
            if (op.result) models.add(op.result);
        }
        const imports = [
            `import { ${tag}API } from '../../api/${tag}API';`,
            ...[...models].sort().map(type => `import type { ${type} } from '../../models/${type}';`),
        ];
        const methods = operations
            .map(op => {
                const args = op.body ? `body: ${op.body}` : '';
                const call = op.body ? 'body' : '';
                const result = resultType(op, type => `${type}[]`, 'void');
                return `    async ${op.name}(${args}): Promise<${result}> {\n        return this.api.${op.name}(${call});\n    }`;
            })
            .join('\n\n');
        const text = `${imports.join('\n')}\n\nexport class ${tag} {\n    constructor(private readonly api: ${tag}API) {}\n\n    static instanceOf(value: unknown): value is ${tag} {\n        return value instanceof ${tag};\n    }\n\n${methods}\n}\n`;
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
        `${Object.entries(sections)
            .map(([section]) => `export * from './${section}';`)
            .join('\n')}\n`,
        'utf8'
    );
}

function emitPython(ops: Map<string, Operation[]>): void {
    const dir = join(targets.python, 'client');
    clear(dir);
    const exports: string[] = [];
    for (const [tag, operations] of [...ops].sort()) {
        const models = new Set<string>();
        for (const op of operations) {
            if (op.body) models.add(op.body);
            if (op.result) models.add(op.result);
        }
        const imports = [
            'from __future__ import annotations',
            '',
            `from ..api.${snake(tag)}_api import ${tag}API`,
            ...[...models].sort().map(type => `from ..models.${snake(type)} import ${type}`),
        ];
        const methods = operations
            .map(op => {
                const args = op.body ? `, body: ${op.body}` : '';
                const call = op.body ? 'body' : '';
                const result = resultType(op, type => `List[${type}]`, 'None');
                return `    def ${snake(op.name)}(self${args}) -> ${result}:\n        return self._api.${snake(op.name)}(${call})`;
            })
            .join('\n\n');
        const text = `${imports.join('\n')}\n\n\nclass ${tag}:\n    def __init__(self, api: ${tag}API) -> None:\n        self._api = api\n\n    @staticmethod\n    def instance_of(value: object) -> bool:\n        return isinstance(value, ${tag})\n\n${methods}\n`;
        writeFileSync(join(dir, `${snake(tag)}.py`), text, 'utf8');
        exports.push(tag);
    }
    writeFileSync(
        join(dir, '__init__.py'),
        `${exports.map(tag => `from .${snake(tag)} import ${tag}`).join('\n')}\n\n__all__ = [${exports.map(tag => `"${tag}"`).join(', ')}]\n`,
        'utf8'
    );
}

function emitGo(ops: Map<string, Operation[]>): void {
    const dir = join(targets.go, 'client');
    clear(dir);
    const resultOf = (op: Operation): string => {
        if (!op.result) return 'any';
        return op.resultArray ? `[]osbtk.${op.result}` : `*osbtk.${op.result}`;
    };
    for (const [tag, operations] of [...ops].sort()) {
        const methods = operations
            .map(op => {
                const name = pascal(op.name);
                if (op.body) {
                    return `func (c *${tag}) ${name}(ctx context.Context, body osbtk.${op.body}) (${resultOf(op)}, *http.Response, error) {\n\treturn c.api.${name}(ctx).${op.body}(body).Execute()\n}`;
                }
                return `func (c *${tag}) ${name}(ctx context.Context) (${resultOf(op)}, *http.Response, error) {\n\treturn c.api.${name}(ctx).Execute()\n}`;
            })
            .join('\n\n');
        const text = `package client\n\nimport (\n\t"context"\n\t"net/http"\n\n\tosbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"\n)\n\n// ${tag} is the clean high-level client.\ntype ${tag} struct {\n\tapi *osbtk.${tag}APIService\n}\n\nfunc New${tag}(api *osbtk.${tag}APIService) *${tag} {\n\treturn &${tag}{api: api}\n}\n\nfunc Is${tag}(value any) bool {\n\t_, ok := value.(*${tag})\n\treturn ok\n}\n\n${methods}\n`;
        writeFileSync(join(dir, `client_${snake(tag)}.go`), text, 'utf8');
    }
}

function emitJava(ops: Map<string, Operation[]>): void {
    const dir = join(targets.java, 'moe', 'tochi', 'osbtk', 'client');
    clear(dir);
    for (const [tag, operations] of [...ops].sort()) {
        const models = new Set<string>();
        for (const op of operations) {
            if (op.body) models.add(op.body);
            if (op.result) models.add(op.result);
        }
        const imports = [
            'import moe.tochi.osbtk.ApiException;',
            `import moe.tochi.osbtk.api.${tag}API;`,
            ...[...models].sort().map(type => `import moe.tochi.osbtk.model.${type};`),
        ];
        const methods = operations
            .map(op => {
                const args = op.body ? `${op.body} body` : '';
                const call = op.body ? 'body' : '';
                const result = resultType(op, type => `List<${type}>`, 'void');
                return `    public ${result} ${op.name}(${args}) throws ApiException {\n        return api.${op.name}(${call});\n    }`;
            })
            .join('\n\n');
        const text = `package moe.tochi.osbtk.client;\n\n${imports.join('\n')}\nimport java.util.List;\n\npublic final class ${tag} {\n    private final ${tag}API api;\n\n    public ${tag}(${tag}API api) {\n        this.api = api;\n    }\n\n    public static boolean instanceOf(Object value) {\n        return value instanceof ${tag};\n    }\n\n${methods}\n}\n`;
        writeFileSync(join(dir, `${tag}.java`), text, 'utf8');
    }
}

function emitCSharp(ops: Map<string, Operation[]>): void {
    const dir = join(targets.csharp, '..', 'Client');
    clear(dir);
    for (const [tag, operations] of [...ops].sort()) {
        const models = new Set<string>();
        for (const op of operations) {
            if (op.body) models.add(op.body);
            if (op.result) models.add(op.result);
        }
        const methods = operations
            .map(op => {
                const args = op.body ? `${op.body} body` : '';
                const call = op.body ? 'body' : '';
                const result = resultType(op, type => `List<${type}>`, 'void');
                return `    public ${result} ${pascal(op.name)}(${args}) => api.${pascal(op.name)}(${call});`;
            })
            .join('\n\n');
        const text = `using tochiRes.osbTK.Api;\nusing tochiRes.osbTK.Model;\n\nnamespace tochiRes.osbTK.Client;\n\npublic sealed class ${tag}\n{\n    private readonly ${tag}API api;\n\n    public ${tag}(${tag}API api)\n    {\n        this.api = api;\n    }\n\n    public static bool InstanceOf(object value) => value is ${tag};\n\n${methods}\n}\n`;
        writeFileSync(join(dir, `${tag}.cs`), text, 'utf8');
    }
}

/**
 * Generates the clean, `API`-suffix-free high-level client classes for every
 * OpenAPI SDK from the contract. Each class holds its generated client
 * privately and forwards the operations 1:1.
 */
export function emitTypeScriptClient(): void {
    emitTypeScript(readOperations());
}

export function emitPythonClient(): void {
    emitPython(readOperations());
}

export function emitGoClient(): void {
    emitGo(readOperations());
}

export function emitJavaClient(): void {
    emitJava(readOperations());
}

export function emitCSharpClient(): void {
    emitCSharp(readOperations());
}
