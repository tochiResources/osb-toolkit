import { execSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { root, specPath } from './shared';

/** Where the generated SDK reference is written (VitePress source). */
const output = join(root, 'docs', 'reference', 'api');

/** Non-runtime classes (the `Core` surface). */
const core = new Set(['Animations', 'Common', 'Curve', 'Origin', 'Parsing']);

/** Runtime-handle classes. */
const runtime = new Set(['Audio', 'Generator', 'Helpers', 'Layer', 'Sprite', 'Storyboarding3d']);

function cleanClass(file: string): string {
    return file.replace(/\.md$/, '').replace(/Api$/, '');
}

function sectionOf(name: string): 'runtime' | 'core' | 'libraries' {
    if (core.has(name)) return 'core';
    if (runtime.has(name)) return 'runtime';
    return 'libraries';
}

function postProcess(text: string, cls: string): string {
    const camel = cls.charAt(0).toLowerCase() + cls.slice(1);
    return text
        .replaceAll(`${cls}Api`, cls)
        .replaceAll(`${camel}.`, `${cls}.`)
        .replaceAll('../Models/', '../models/')
        .replaceAll('](../README.md', '](../index.md')
        .replaceAll(
            '[[Back to Model list]](../index.md#documentation-for-models) [[Back to API list]](../index.md#documentation-for-api-endpoints) [[Back to README]](../index.md)',
            ''
        )
        .replace('All URIs are relative to *http://localhost*\n\n', '')
        .replace('| Method | HTTP request | Description |', '| Method | Description |')
        .replace(
            '|------------- | ------------- | -------------|',
            '|------------- | -------------|'
        )
        .replace(/ \| \*\*(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)\*\* [^|]+ \|/g, ' |')
        .replace(/\n### Authorization\n\nNo authorization required\n/g, '')
        .replace(
            /\n### HTTP request headers\n\n- \*\*Content-Type\*\*: application\/json\n- \*\*Accept\*\*: application\/json\n/g,
            ''
        )
        .replace(/^# .+$/m, `# ${cls}`);
}

/**
 * Generates the SDK reference from the checked-in contract, shaped like the
 * clean high-level clients: one page per class (no `API` suffix), split into
 * the runtime, core and library sections, with the generated models kept
 * verbatim.
 */
export function generateDocs(): void {
    const out = mkdtempSync(join(tmpdir(), 'osbtk-docs-'));
    const sections: Record<string, string[]> = { runtime: [], core: [], libraries: [] };
    try {
        execSync(
            [
                'yarn openapi-generator-cli generate',
                `-i "${specPath}"`,
                '-g markdown',
                `-o "${out}"`,
            ].join(' '),
            { cwd: root, stdio: 'inherit', shell: true }
        );

        rmSync(output, { recursive: true, force: true });
        mkdirSync(join(output, 'models'), { recursive: true });

        const modelFiles = readdirSync(join(out, 'Models')).filter(file => file.endsWith('.md'));
        for (const file of modelFiles) {
            const text = readFileSync(join(out, 'Models', file), 'utf8')
                .replaceAll('../Models/', './')
                .replace(/\[\[Back to Model list\].*$/m, '');
            writeFileSync(join(output, 'models', file), text, 'utf8');
        }
        writeFileSync(
            join(output, 'models', 'index.md'),
            [
                '# Models',
                '',
                ...modelFiles.map(file => `- [${file.replace(/\.md$/, '')}](./${file})`).sort(),
                '',
            ].join('\n'),
            'utf8'
        );

        for (const file of readdirSync(join(out, 'Apis'))) {
            if (!file.endsWith('.md')) continue;
            const cls = cleanClass(file);
            const section = sectionOf(cls);
            mkdirSync(join(output, section), { recursive: true });
            const text = postProcess(readFileSync(join(out, 'Apis', file), 'utf8'), cls);
            writeFileSync(join(output, section, `${cls}.md`), text, 'utf8');
            sections[section].push(cls);
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
    } finally {
        rmSync(out, { recursive: true, force: true });
    }
}
