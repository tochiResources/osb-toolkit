import {
    existsSync,
    mkdirSync,
    readdirSync,
    readFileSync,
    rmSync,
    writeFileSync,
} from 'node:fs';
import { join } from 'node:path';

import { root } from '../shared';
import { optionFor, orderedProperties, readContract, sectionOf } from './shared';
import { tsPropertyType, tsResultType } from './typescript';

/** Result wrappers the clean clients unwrap into ordinary values. */
const unwrappedResults: Record<string, string> = {
    OkResult: '`void`',
    BoolResult: '`boolean`',
    ScalarResult: '`number`',
    EasingResult: '`number`',
    RandomResult: '`number`',
    AudioDurationResult: '`number`',
    SpriteCountResult: '`number`',
    ImportOsbResult: '`number`',
    SegmentCleanResult: '`number`',
    SegmentDiscardResult: '`number`',
    VersionResult: '`string`',
    OptionalPointResult: '`Vector2 | null`',
    HelperResult: '`unknown`',
    LayerHandle: '`number` (the layer index)',
    Handle: '`Sprite`',
};

interface RequestUsage {
    option: string;
    properties: number;
    tags: string[];
}

/** Adds the clean mapping notes to the raw model pages and rebuilds their index. */
function annotateModels(output: string, requests: Map<string, RequestUsage>): void {
    const models = join(output, 'models');
    const files = readdirSync(models).filter(file => file.endsWith('.md') && file !== 'index.md');
    for (const file of files) {
        const name = file.replace(/\.md$/, '');
        const path = join(models, file);
        const notes: string[] = [];
        const usage = requests.get(name);
        if (usage) {
            const clients = usage.tags
                .map(tag => `[\`${tag}\`](../${sectionOf(tag)}/${tag}.md)`)
                .join(', ');
            if (usage.properties === 0) {
                notes.push(
                    `Handle-bound only${clients ? ` (${clients})` : ''}: the clean client takes the bound handle directly, so no \`<X>Options\` type is generated.`
                );
            } else {
                notes.push(
                    `Clean SDK type: \`${usage.option}\`${clients ? ` (${clients})` : ''}. ` +
                        'Handle properties (`layerHandle`, `sprite` and numeric `layer`) are injected by the handle-bound client.'
                );
            }
        }
        if (unwrappedResults[name]) {
            notes.push(`Clean SDKs unwrap this result to ${unwrappedResults[name]}.`);
        }
        if (notes.length > 0) {
            writeFileSync(path, `${notes.map(note => `> ${note}`).join('\n')}\n\n${readFileSync(path, 'utf8')}`, 'utf8');
        }
    }

    const modelList = files
        .map(file => `- [${file.replace(/\.md$/, '')}](./${file})`)
        .sort();
    const requestRows = [...requests.entries()]
        .sort(([a], [b]) => a.localeCompare(b))
        .map(([request, usage]) => {
            const clients = usage.tags
                .map(tag => `[\`${tag}\`](../${sectionOf(tag)}/${tag}.md)`)
                .join(', ');
            const cleanType = usage.properties > 0 ? `\`${usage.option}\`` : '—';
            return `| [\`${request}\`](./${request}.md) | ${cleanType} | ${clients} |`;
        });
    const resultRows = Object.entries(unwrappedResults).map(
        ([name, value]) => `| [\`${name}\`](./${name}.md) | ${value} |`
    );

    writeFileSync(
        join(models, 'index.md'),
        [
            '# Models',
            '',
            ...modelList,
            '',
            '## Clean options',
            '',
            'Every `*Request` model is exposed to the clean SDKs as the matching `*Options` type.',
            'Handle properties (`layerHandle`, `sprite` and numeric `layer`) are bound by the',
            '`Layer`/`Sprite` clients and omitted from `<X>Options`. Requests that contain only',
            'handle fields have no options type; the clean client takes the bound handle',
            'directly (shown as `—`).',
            '',
            '| Request model | Clean type | Client |',
            '| --- | --- | --- |',
            ...requestRows,
            '',
            '## Unwrapped results',
            '',
            'The clean clients return ordinary values instead of these wrapper models;',
            'the raw models stay available on the generated API clients.',
            '',
            '| Result model | Clean value |',
            '| --- | --- |',
            ...resultRows,
            '',
        ].join('\n'),
        'utf8'
    );
}

const rustDoc = `# Rust clean module

The \`osb-sdk\` crate generates \`crates/osb-sdk/src/clean.rs\`: one \`<X>Options\`
struct per request and one client per tag.

- Stateless tags (\`Core\`, \`Audio\`, \`Diagnostics\`, \`Helpers\`, \`Image\`,
  \`Imagegen\`, \`Media\`, \`Optimize\`, \`Storyboarding3d\`) are fields on
  \`clean::Client\`.
- The handle-bound \`Generator\`, \`Layer\` and \`Sprite\` clients live behind
  \`clean::Runtime\`, which owns the hand-written \`Runtime\`.
- \`Audio::fft\` works without a runtime; \`Runtime::audio()\` binds the same
  client to a runtime for \`get_fft\` and \`audio_duration\`.

\`\`\`rust
use osb_sdk::clean::*;
use osb_sdk::models;

let mut runtime = Runtime::new(640.0, 480.0);
let mut layer = runtime.layer("Foreground")?;
let mut sprite = layer.create_sprite(CreateSpriteOptions {
    path: "sb/dot.png".to_string(),
    origin: models::OsbOrigin::Centre,
    position: None,
    layer: None,
})?;
sprite.command(CommandOptions {
    kind: "move".to_string(),
    start_time: 0.0,
    end_time: 1000.0,
    easing: models::Easing::None,
    start: Some(serde_json::json!({ "x": 0.0, "y": 0.0 })),
    end: Some(serde_json::json!({ "x": 100.0, "y": 0.0 })),
    parameter: None,
})?;
\`\`\`
`;

const cppDoc = `# C++ clean header

\`bindings/cpp/osbtk_clean.hpp\` is generated from the contract alongside
\`osbtk_sdk.hpp\`. It contains the \`<X>Options\` structs and one class per tag.

- Stateless clients (\`Core\`, \`Audio\`, \`Diagnostics\`, \`Optimize\`, \`Image\`,
  \`Imagegen\`, \`Media\`, \`Storyboarding3d\`, \`Helpers\`) are members of
  \`osbtk::clean::Client\`.
- \`osbtk::clean::Runtime\` owns an \`sdk::Runtime\` and exposes \`generator()\`,
  \`audio()\`, \`layer(name)\` and \`sprite(layer, sprite)\`.
- Complex option fields use JSON strings (\`start_json\`, \`end_json\`,
  \`input_json\`, \`request_json\`) while typed fields such as \`position\` are
  \`std::optional\` where the contract makes them optional.

\`\`\`cpp
osbtk::Library library(path);
osbtk::clean::Runtime runtime(library);
osbtk::clean::Layer layer = runtime.layer("Foreground");
osbtk::clean::Sprite sprite =
    layer.create_sprite({"sb/dot.png", osbtk::sdk::OsbOrigin::Centre});
sprite.command(
    {"move", 0.0, 1000.0, "None", R"({"x":0,"y":0})", R"({"x":100,"y":0})", ""});
\`\`\`
`;

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
    const requests = new Map<string, RequestUsage>();
    for (const [tag, operations] of contract.operations) {
        for (const op of operations) {
            if (!op.body?.endsWith('Request')) continue;
            const option = optionFor(contract, op);
            if (!option) continue;
            const entry = requests.get(op.body) ?? {
                option: option.name,
                properties: option.properties.length,
                tags: [],
            };
            if (!entry.tags.includes(tag)) entry.tags.push(tag);
            requests.set(op.body, entry);
        }
    }

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

    if (existsSync(join(output, 'models'))) annotateModels(output, requests);
    writeFileSync(join(output, 'rust.md'), rustDoc, 'utf8');
    writeFileSync(join(output, 'cpp.md'), cppDoc, 'utf8');

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
            '## Clean mapping',
            '',
            'Every `*Request` model maps to a `<X>Options` type in the clean SDKs,',
            'and wrapper results are unwrapped into ordinary values. The',
            '[models index](./models/index.md#clean-options) lists both mappings.',
            '',
            '## Other clean surfaces',
            '',
            '- [Rust clean module](./rust.md)',
            '- [C++ clean header](./cpp.md)',
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
