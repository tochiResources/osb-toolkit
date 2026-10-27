import type {
    EffectScript,
    GeneratedFile,
    PluginSpec,
    PluginTarget,
} from '@tochii/osb-toolkit-shared';

export interface GenerateOptions {
    pluginName?: string;
    pluginId?: string;
    category?: string;
    description?: string;
    resolution?: { width: number; height: number };
    supportUrl?: string;
    /** Path from the generated Rust crate to the `osb-runtime` crate. */
    runtimeCratePath?: string;
    /** Path from the generated Rust crate to the host adapter crate. */
    adapterCratePath?: string;
}

export interface TargetGenerateContext {
    script: EffectScript;
    spec: PluginSpec;
    options: GenerateOptions;
    resolution: { width: number; height: number };
}

export interface OfxGeneratorOptions extends GenerateOptions {
    /** Path to the OFX SDK root (containing `include/`). */
    ofxSdkPath?: string;
}

/** Options for the shared PF_ Effect generator (After Effects and Premiere Pro). */
export interface PfGeneratorOptions extends GenerateOptions {
    /** `aex` (After Effects-style effect, loads in AE + Premiere) or `prm` (Premiere-native). */
    artifact?: 'aex' | 'prm';
}

export type AeGeneratorOptions = PfGeneratorOptions;
export type PproGeneratorOptions = PfGeneratorOptions;

export interface GenerateForTargetOptions extends GenerateOptions {
    ofxSdkPath?: string;
    artifact?: 'aex' | 'prm';
    /**
     * External library classes (parsed) compiled alongside the effect; their
     * methods are flattened into the generated crate.
     */
    parsedLibraries?: EffectScript[];
}

export type { EffectScript, GeneratedFile, PluginSpec, PluginTarget };
