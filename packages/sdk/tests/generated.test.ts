import { readdirSync, readFileSync } from 'node:fs';
import { extname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, it } from 'vitest';

const root = fileURLToPath(new URL('../../../', import.meta.url));
const bindings = join(root, 'bindings');
const skipped = new Set(['node_modules', 'target', 'build', 'dist', '.git']);
const text = new Set(['.ts', '.cs', '.java', '.py', '.go', '.rs', '.hpp', '.h', '.cpp', '.json']);

function filesUnder(dir: string): string[] {
    return readdirSync(dir, { withFileTypes: true }).flatMap(entry => {
        if (skipped.has(entry.name)) return [];
        const path = join(dir, entry.name);
        if (entry.isDirectory()) return filesUnder(path);
        return entry.isFile() && text.has(extname(entry.name)) ? [path] : [];
    });
}

describe('generated clean options', () => {
    const files = filesUnder(bindings).map(file => ({
        path: relative(root, file),
        source: readFileSync(file, 'utf8'),
    }));

    it('does not emit models for handle-only requests', () => {
        for (const name of ['HandleOptions', 'SegmentOptions']) {
            const offenders = files
                .filter(file => new RegExp(`\\b${name}\\b`).test(file.source))
                .map(file => file.path);
            expect(offenders, `${name} is generated for a handle-only request`).toEqual([]);
        }
    });

    it('does not emit empty TypeScript option interfaces', () => {
        const options = readFileSync(join(root, 'packages/sdk/src/lib/client/options.ts'), 'utf8');
        const empty = options.match(/export interface \w+Options \{\s*\}/g) ?? [];
        expect(empty).toEqual([]);
    });
});
