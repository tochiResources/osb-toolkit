import type {
    Diagnostic,
    EffectScript,
    GeneratedFile,
    PluginTarget,
} from '@tochii/osb-toolkit-shared';

import type { GenerateForTargetOptions } from './generate.js';

export interface ConvertOptions extends GenerateForTargetOptions {
    target: PluginTarget;
    sourceFile?: string;
    /**
     * External library sources (`scriptslibrary/*.cs`) compiled
     * alongside the effect; their methods are flattened into the plug-in.
     */
    libraries?: EffectSource[];
    /**
     * Run the C# syntax linter (`tools/csharp-lint`) over the script before
     * transpiling, so a broken consumer script fails early. The CLI/service set
     * this; it is skipped automatically when the .NET SDK is unavailable.
     */
    lintCSharp?: boolean;
}

export interface ConvertResult {
    target: PluginTarget;
    effectName: string;
    pluginId: string;
    script: EffectScript;
    files: GeneratedFile[];
    diagnostics: Diagnostic[];
}

/** A C# source (an effect script or a library class) to convert. */
export interface EffectSource {
    source: string;
    /** Display name / default file name (defaults to the class name). */
    name?: string;
    /** Path recorded on diagnostics. */
    file?: string;
}

export interface ConvertBundleOptions extends ConvertOptions {
    /** The effect scripts to convert. */
    effects: EffectSource[];
    /** `true` = one plug-in for all effects; `false` (default) = one per effect. */
    bundle?: boolean;
    /**
     * How bundled parameters are grouped: `effect` (default) groups each script's
     * parameters under the effect; `combined` merges them by the original
     * `[Group]`.
     */
    parameterGrouping?: 'effect' | 'combined';
}

export interface ConvertBundleResult {
    target: PluginTarget;
    bundle: boolean;
    plugins: ConvertResult[];
    diagnostics: Diagnostic[];
}
