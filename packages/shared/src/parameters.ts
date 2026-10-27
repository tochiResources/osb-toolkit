import type { AssetKind, EffectParameter, ParameterControl } from './types/ir.js';

export interface ParameterClassification {
    control: ParameterControl;
    assetKind?: AssetKind;
}

const ASSET_HINTS: Array<{ kind: AssetKind; pattern: RegExp }> = [
    { kind: 'audio', pattern: /audio|sound|song|music|fft|track/ },
    { kind: 'subtitle', pattern: /subtitle|caption/ },
    { kind: 'font', pattern: /font/ },
    { kind: 'composition', pattern: /comp\b|composition|precomp/ },
    { kind: 'layer', pattern: /layer/ },
    { kind: 'media', pattern: /video|movie|clip|media|footage/ },
    { kind: 'background', pattern: /background|\bbg\b/ },
    { kind: 'sprite', pattern: /sprite|texture|image|bitmap|icon|png|path|file/ },
];

/**
 * Classifies a `[Configurable]` field into the native control it maps to and,
 * for asset inputs, the kind of host asset the picker should resolve.
 */
export function classifyParameter(parameter: EffectParameter): ParameterClassification {
    switch (parameter.type) {
        case 'bool':
            return { control: 'checkbox' };
        case 'color':
            return { control: 'color' };
        case 'origin':
        case 'easing':
        case 'enum':
            return { control: 'popup' };
        case 'vec2':
        case 'vec3':
            return { control: 'point' };
        case 'int':
        case 'long':
        case 'double':
        case 'float':
            return { control: angleLike(parameter.name) ? 'angle' : 'slider' };
        case 'string':
            return classifyString(parameter);
        default:
            return classifyUnknown(parameter);
    }
}

function classifyString(parameter: EffectParameter): ParameterClassification {
    const assetKind = assetKindFor(parameter);
    if (assetKind) return { control: 'picker', assetKind };
    const literal = parameter.defaultValueLiteral ?? '';
    if (/[/\\]|\.[a-z0-9]{2,4}$/i.test(literal)) {
        return { control: 'picker', assetKind: 'file' };
    }
    return { control: 'text' };
}

function classifyUnknown(parameter: EffectParameter): ParameterClassification {
    if (/bitmap|stream|image|texture/i.test(parameter.csType)) {
        return { control: 'picker', assetKind: 'sprite' };
    }
    return { control: 'text' };
}

function assetKindFor(parameter: EffectParameter): AssetKind | undefined {
    const haystack = `${parameter.name} ${parameter.csType}`.toLowerCase();
    for (const hint of ASSET_HINTS) {
        if (hint.pattern.test(haystack)) return hint.kind;
    }
    return undefined;
}

function angleLike(name: string): boolean {
    return /angle|rotation|rotate|radian|degree/i.test(name);
}
