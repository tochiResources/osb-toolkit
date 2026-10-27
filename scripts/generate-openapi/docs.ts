import { execSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { emitCleanDocs } from './clean';
import { root, specPath } from './shared';

/** Where the generated SDK reference is written (VitePress source). */
const output = join(root, 'docs', 'reference', 'api');

/**
 * Generates the SDK reference from the checked-in contract: the clean
 * class/options pages come from the contract metadata, while the raw
 * request/response models stay verbatim from the OpenAPI markdown generator.
 */
export function generateDocs(): void {
    const out = mkdtempSync(join(tmpdir(), 'osbtk-docs-'));
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

        emitCleanDocs();
    } finally {
        rmSync(out, { recursive: true, force: true });
    }
}
