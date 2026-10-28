import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import { root } from '../shared';
import { optionFor, orderedProperties, readContract, sectionOf } from './shared';
import { tsPropertyType, tsResultType } from './typescript';

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
