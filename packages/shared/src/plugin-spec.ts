import { z } from 'zod';

import {
    DEFAULT_CATEGORY,
    DEFAULT_MATCH_NAME_PREFIX,
    DEFAULT_PLUGIN_ID_PREFIX,
} from './constants.js';
import { effectParameterSchema } from './ir.js';
import { PLUGIN_TARGETS } from './targets.js';
import type { CreatePluginSpecOptions, PluginSpec, PluginVersion } from './types/plugin-spec.js';

export const pluginTargetSchema = z.enum(PLUGIN_TARGETS);

export const pluginVersionSchema = z.object({
    major: z.number().int().nonnegative(),
    minor: z.number().int().nonnegative(),
    patch: z.number().int().nonnegative(),
});

export const pluginParameterSchema = effectParameterSchema.extend({
    label: z.string(),
    scriptName: z.string(),
    min: z.number().optional(),
    max: z.number().optional(),
    precision: z.number().int().optional(),
    keyframeable: z.boolean().optional(),
});

export const pluginSpecSchema = z.object({
    id: z.string().min(1),
    name: z.string().min(1),
    matchName: z.string().min(1),
    category: z.string().min(1),
    description: z.string(),
    target: pluginTargetSchema,
    version: pluginVersionSchema,
    entryPoint: z.string().min(1),
    supportUrl: z.string().optional(),
    parameters: z.array(pluginParameterSchema),
});

export function createPluginSpec(options: CreatePluginSpecOptions): PluginSpec {
    const version: PluginVersion = {
        major: options.version?.major ?? 1,
        minor: options.version?.minor ?? 0,
        patch: options.version?.patch ?? 0,
    };
    return {
        id: options.id ?? `${DEFAULT_PLUGIN_ID_PREFIX}.${slugId(options.name)}`,
        name: options.name,
        matchName: options.matchName ?? `${DEFAULT_MATCH_NAME_PREFIX} ${options.name}`,
        category: options.category ?? DEFAULT_CATEGORY,
        description: options.description ?? `Generated from an effect script: ${options.name}`,
        target: options.target,
        version,
        entryPoint: options.entryPoint ?? 'EffectMain',
        supportUrl: options.supportUrl ?? 'https://osb-toolkit.tochi.moe',
        parameters: options.parameters ?? [],
    };
}

export function slugId(name: string): string {
    return name
        .replace(/([a-z0-9])([A-Z])/g, '$1-$2')
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, '-')
        .replace(/^-+|-+$/g, '');
}
