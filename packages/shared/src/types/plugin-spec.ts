import type { EffectParameter } from './ir.js';
import type { PluginTarget } from './targets.js';

export interface PluginVersion {
    major: number;
    minor: number;
    patch: number;
}

export interface PluginParameter extends EffectParameter {
    label: string;
    scriptName: string;
    min?: number;
    max?: number;
    precision?: number;
    keyframeable?: boolean;
}

export interface PluginSpec {
    id: string;
    name: string;
    matchName: string;
    category: string;
    description: string;
    target: PluginTarget;
    version: PluginVersion;
    entryPoint: string;
    supportUrl?: string;
    parameters: PluginParameter[];
}

export interface CreatePluginSpecOptions {
    target: PluginTarget;
    name: string;
    id?: string;
    matchName?: string;
    category?: string;
    description?: string;
    version?: Partial<PluginVersion>;
    entryPoint?: string;
    supportUrl?: string;
    parameters?: PluginParameter[];
}
