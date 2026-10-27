import type {
    EffectParameterType,
    OsbEasing,
    OsbLayer,
    OsbOrigin,
} from '@tochii/osb-toolkit-shared';

import { normalizeTypeName } from './compat.js';

export const KNOWN_ENUMS: Record<string, readonly string[]> = {
    OsbOrigin: [
        'TopLeft',
        'TopCentre',
        'TopRight',
        'CentreLeft',
        'Centre',
        'CentreRight',
        'BottomLeft',
        'BottomCentre',
        'BottomRight',
    ],
    OsbEasing: [
        'None',
        'Out',
        'In',
        'InQuad',
        'OutQuad',
        'InOutQuad',
        'InCubic',
        'OutCubic',
        'InOutCubic',
        'InQuart',
        'OutQuart',
        'InOutQuart',
        'InQuint',
        'OutQuint',
        'InOutQuint',
        'InSine',
        'OutSine',
        'InOutSine',
        'InExpo',
        'OutExpo',
        'InOutExpo',
        'InCirc',
        'OutCirc',
        'InOutCirc',
        'InElastic',
        'OutElastic',
        'OutElasticHalf',
        'OutElasticQuarter',
        'InOutElastic',
        'InBack',
        'OutBack',
        'InOutBack',
        'InBounce',
        'OutBounce',
        'InOutBounce',
    ],
    OsbLayer: ['Background', 'Fail', 'Pass', 'Foreground', 'Overlay'],
    ParameterType: ['None', 'FlipHorizontal', 'FlipVertical', 'AdditiveBlending'],
};

export function mapCSharpType(csType: string): EffectParameterType {
    const normalized = normalizeTypeName(csType);
    if (normalized in KNOWN_ENUMS) {
        if (normalized === 'OsbOrigin') return 'origin';
        if (normalized === 'OsbEasing') return 'easing';
        return 'enum';
    }
    switch (normalized) {
        case 'int':
        case 'short':
        case 'sbyte':
        case 'uint':
        case 'ushort':
        case 'byte':
            return 'int';
        case 'long':
        case 'ulong':
            return 'long';
        case 'double':
            return 'double';
        case 'float':
        case 'decimal':
            return 'float';
        case 'bool':
            return 'bool';
        case 'string':
            return 'string';
        case 'Vector2':
            return 'vec2';
        case 'Vector3':
            return 'vec3';
        case 'CommandColor':
        case 'Color4':
            return 'color';
        default:
            return 'unknown';
    }
}

export function enumMembers(csType: string): readonly string[] | undefined {
    return KNOWN_ENUMS[normalizeTypeName(csType)];
}

export const TYPE_NAMES = new Set([
    ...Object.keys(KNOWN_ENUMS),
    'int',
    'short',
    'sbyte',
    'uint',
    'ushort',
    'byte',
    'long',
    'ulong',
    'double',
    'float',
    'decimal',
    'bool',
    'string',
    'char',
    'object',
    'void',
    'Vector2',
    'Vector3',
    'Vector4',
    'Color4',
    'CommandColor',
    'CommandPosition',
    'CommandScale',
    'CommandDecimal',
    'CommandParameter',
    'Bitmap',
    'Stream',
    'Vector4',
]);

export type KnownOrigin = OsbOrigin;
export type KnownEasing = OsbEasing;
export type KnownLayer = OsbLayer;
