import { z } from 'zod';

import type { EffectScript } from './types/ir.js';

export const EFFECT_PARAMETER_TYPES = [
    'int',
    'long',
    'double',
    'float',
    'bool',
    'string',
    'enum',
    'vec2',
    'vec3',
    'color',
    'origin',
    'easing',
    'unknown',
] as const;

export const effectParameterTypeSchema = z.enum(EFFECT_PARAMETER_TYPES);

export const effectEnumValueSchema = z.object({ name: z.string(), value: z.number().optional() });

export const PARAMETER_CONTROLS = [
    'slider',
    'checkbox',
    'color',
    'angle',
    'popup',
    'point',
    'text',
    'picker',
] as const;
export const parameterControlSchema = z.enum(PARAMETER_CONTROLS);

export const ASSET_KINDS = [
    'sprite',
    'background',
    'audio',
    'font',
    'subtitle',
    'composition',
    'layer',
    'media',
    'file',
] as const;
export const assetKindSchema = z.enum(ASSET_KINDS);

export const effectParameterSchema = z.object({
    name: z.string(),
    csType: z.string(),
    type: effectParameterTypeSchema,
    defaultValue: z.unknown().optional(),
    defaultValueLiteral: z.string().optional(),
    group: z.string().optional(),
    description: z.string().optional(),
    enumValues: z.array(effectEnumValueSchema).optional(),
    order: z.number().int().nonnegative(),
    control: parameterControlSchema.optional(),
    assetKind: assetKindSchema.optional(),
});

export const effectFieldSchema = z.object({
    name: z.string(),
    csType: z.string(),
    modifiers: z.array(z.string()),
    initializer: z.string().optional(),
});

export const effectMethodSchema = z.object({
    name: z.string(),
    signature: z.string(),
    body: z.string(),
    isOverride: z.boolean(),
});

export const diagnosticSeveritySchema = z.enum(['info', 'warning', 'error']);

export const diagnosticSchema = z.object({
    severity: diagnosticSeveritySchema,
    message: z.string(),
    code: z.string().optional(),
    line: z.number().int().positive().optional(),
    column: z.number().int().positive().optional(),
});

export const effectScriptSchema = z.object({
    className: z.string(),
    namespace: z.string().optional(),
    baseClass: z.string(),
    sourceFile: z.string().optional(),
    usingDirectives: z.array(z.string()),
    fields: z.array(effectFieldSchema),
    parameters: z.array(effectParameterSchema),
    methods: z.array(effectMethodSchema),
    generateBody: z.string(),
    generateLine: z.number().int().positive().optional(),
    transpiledBody: z.string().optional(),
    diagnostics: z.array(diagnosticSchema),
});

export const OSB_ORIGINS = [
    'TopLeft',
    'TopCentre',
    'TopRight',
    'CentreLeft',
    'Centre',
    'CentreRight',
    'BottomLeft',
    'BottomCentre',
    'BottomRight',
] as const;
export const osbOriginSchema = z.enum(OSB_ORIGINS);

export const OSB_EASINGS = [
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
] as const;
export const osbEasingSchema = z.enum(OSB_EASINGS);

export const OSB_LAYERS = ['Background', 'Fail', 'Pass', 'Foreground', 'Overlay'] as const;
export const osbLayerSchema = z.enum(OSB_LAYERS);

export const generatedFileSchema = z.object({
    path: z.string(),
    contents: z.string(),
    kind: z.enum(['source', 'project', 'build', 'manifest', 'runtime', 'readme', 'asset']),
    executable: z.boolean().optional(),
});

export const convertOptionsSchema = z.object({
    target: z.string(),
    pluginName: z.string().optional(),
    pluginId: z.string().optional(),
    category: z.string().optional(),
    outputDir: z.string().optional(),
    resolution: z
        .object({ width: z.number().int().positive(), height: z.number().int().positive() })
        .optional(),
});

export function parseEffectScript(input: unknown): EffectScript {
    return effectScriptSchema.parse(input) as EffectScript;
}

export function safeParseEffectScript(input: unknown) {
    return effectScriptSchema.safeParse(input);
}
