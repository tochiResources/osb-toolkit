import type { EffectScript, PluginTarget } from '@tochii/osb-toolkit-shared';

import type { GenerateOptions } from '../../types/generate.js';

export interface RustCrateOptions extends GenerateOptions {
    /** Path from the generated crate to the `osb-runtime` crate. */
    runtimeCratePath?: string;
    /** Path from the generated crate to the adapter crate. */
    adapterCratePath?: string;
    /** External library classes (parsed) flattened into the generated crate. */
    parsedLibraries?: EffectScript[];
}

export interface RustTarget {
    target: PluginTarget;
    adapterCrate: string;
    /** Extra Cargo features the generated crate declares. */
    features: Array<{ name: string; enables: string }>;
    /** Extra `use` lines for the generated `src/lib.rs`. */
    extraUse: string[];
    /** When set, the crate emits the C ABI macro + the host entry + build.rs. */
    hostGlue?: string;
    /** Which host entry the build.rs compiles. */
    hostEntry?: 'ae' | 'ofx';
}

export const TARGETS: Record<string, RustTarget> = {
    'after-effects': {
        target: 'after-effects',
        adapterCrate: 'osb-plugin-ae',
        features: [],
        extraUse: [],
        hostGlue: 'osb_plugin_ae::export_ae_effect',
        hostEntry: 'ae',
    },
    'premiere-pro': {
        target: 'premiere-pro',
        adapterCrate: 'osb-plugin-ae',
        features: [{ name: 'premiere', enables: 'osb-plugin-ae/premiere' }],
        extraUse: [],
        hostGlue: 'osb_plugin_ae::export_ae_effect',
        hostEntry: 'ae',
    },
    'davinci-resolve': {
        target: 'davinci-resolve',
        adapterCrate: 'osb-plugin-ofx',
        features: [],
        extraUse: [],
        hostGlue: 'osb_plugin_ofx::export_ofx_effect',
        hostEntry: 'ofx',
    },
};
