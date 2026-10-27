import {
    classifyParameter,
    createPluginSpec,
    type EffectParameter,
    type EffectScript,
    type GeneratedFile,
    type PluginParameter,
    type PluginSpec,
    type PluginTarget,
} from '@tochii/osb-toolkit-shared';

import type { GenerateOptions, TargetGenerateContext } from '../types/generate.js';

export function resolveEffectName(script: EffectScript, options: GenerateOptions): string {
    return options.pluginName?.trim() || script.className;
}

export function resolveResolution(options: GenerateOptions): { width: number; height: number } {
    return options.resolution ?? { width: 1920, height: 1080 };
}

export function buildPluginSpec(
    script: EffectScript,
    target: PluginTarget,
    options: GenerateOptions
): PluginSpec {
    return createPluginSpec({
        target,
        name: resolveEffectName(script, options),
        id: options.pluginId,
        category: options.category,
        description: options.description,
        supportUrl: options.supportUrl,
        parameters: script.parameters.map(toPluginParameter),
    });
}

export function toPluginParameter(parameter: EffectParameter): PluginParameter {
    const scriptName = parameter.name.charAt(0).toLowerCase() + parameter.name.slice(1);
    const range = parameterRange(parameter);
    const { control, assetKind } = classifyParameter(parameter);
    return {
        ...parameter,
        label: humanize(parameter.name),
        scriptName,
        control,
        assetKind,
        min: range?.min,
        max: range?.max,
    };
}

function parameterRange(parameter: EffectParameter): { min: number; max: number } | undefined {
    switch (parameter.type) {
        case 'int':
        case 'long':
            return { min: 0, max: 100000 };
        case 'double':
        case 'float':
            return { min: 0, max: 10000 };
        default:
            return undefined;
    }
}

export function humanize(name: string): string {
    return name
        .replace(/([a-z0-9])([A-Z])/g, '$1 $2')
        .replace(/_/g, ' ')
        .replace(/\s+/g, ' ')
        .trim();
}

/** Documents each configurable, its native control and any asset-override picker. */
export function buildManifest(spec: PluginSpec): GeneratedFile {
    const parameters = spec.parameters.map(parameter => ({
        name: parameter.name,
        label: parameter.label,
        scriptName: parameter.scriptName,
        csType: parameter.csType,
        type: parameter.type,
        control: parameter.control,
        assetKind: parameter.assetKind,
        group: parameter.group,
        description: parameter.description,
        min: parameter.min,
        max: parameter.max,
        defaultValue: parameter.defaultValue,
    }));
    const pickers = spec.parameters
        .filter(parameter => parameter.control === 'picker')
        .map(parameter => ({
            name: parameter.name,
            label: parameter.label,
            assetKind: parameter.assetKind,
        }));
    return {
        path: `${spec.name}/manifest.json`,
        kind: 'manifest',
        contents: `${JSON.stringify(
            {
                name: spec.name,
                id: spec.id,
                matchName: spec.matchName,
                target: spec.target,
                version: spec.version,
                parameters,
                pickers,
            },
            null,
            4
        )}\n`,
    };
}

export function readmeFor(context: TargetGenerateContext, instructions: string[]): string {
    const { spec, script } = context;
    const lines = [
        `# ${spec.name}`,
        '',
        spec.description,
        '',
        `- Target: ${spec.target}`,
        `- Plug-in id: \`${spec.id}\``,
        `- Match name: \`${spec.matchName}\``,
        `- Source script: \`${script.sourceFile ?? `${script.className}.cs`}\``,
        `- Parameters: ${script.parameters.length}`,
        '',
        '## Build & install',
        '',
        ...instructions.map(line => (line ? `- ${line}` : '')),
        '',
    ];
    if (script.diagnostics.length > 0) {
        lines.push('## Conversion diagnostics', '');
        for (const diagnostic of script.diagnostics) {
            lines.push(`- [${diagnostic.severity}] ${diagnostic.message}`);
        }
        lines.push('');
    }
    return lines.join('\n');
}
