import {
    DEFAULT_PLUGIN_ID_PREFIX,
    type Diagnostic,
    type EffectScript,
    isPluginTarget,
} from '@tochii/osb-toolkit-shared';

import { lintCSharpSource } from './csharp/lint.js';
import { parseEffectScript } from './csharp/parser/index.js';
import { rustSnakeCase, transpileCSharpToRust } from './csharp/rust/index.js';
import { resolveEffectName } from './generate/common.js';
import { generateForTarget, generateRustBundleCrate } from './generate/index.js';
import {
    buildLibraryMethods,
    buildRtLibraryMethods,
    computeRtHelpers,
} from './generate/rust/lib.js';
import type {
    ConvertBundleOptions,
    ConvertBundleResult,
    ConvertOptions,
    ConvertResult,
    EffectSource,
} from './types/convert.js';

/** Parses external library sources as classes (no `Generate()` required). */
function parseLibraries(sources?: EffectSource[]): EffectScript[] {
    return (sources ?? []).map(library =>
        parseEffectScript(library.source, {
            file: library.file ?? library.name,
            requireGenerate: false,
        })
    );
}

/**
 * Whether `script` is an `.osb` importer. Its hand-written parser is
 * replaced wholesale by the runtime's `.osb` helper (`Runtime::import_osb`),
 * the pragmatic approach, rather than transpiling switch
 * statements, `using` declarations and `StreamReader` line by line.
 */
function isOsbImportScript(script: EffectScript): boolean {
    if (script.className === 'ImportOsb') return true;
    return (
        script.generateBody.includes('ParseSections') &&
        script.generateBody.includes('OpenProjectFile')
    );
}

/**
 * Runs the C# syntax linter over a consumer script and maps its diagnostics into
 * the shared shape, so a broken script fails early with the original locations.
 */
function lintCSharpDiagnostics(source: string, file?: string): Diagnostic[] {
    return lintCSharpSource(source, file).map(diagnostic => ({
        severity: diagnostic.severity,
        message: diagnostic.message,
        code: diagnostic.id,
        line: diagnostic.line,
        column: diagnostic.column,
    }));
}

export function convertEffectScript(
    source: string,
    options: ConvertOptions,
    parsedLibraries?: EffectScript[]
): ConvertResult {
    if (!isPluginTarget(options.target)) {
        throw new Error(`Unknown target: ${String(options.target)}`);
    }

    const lintDiagnostics = options.lintCSharp
        ? lintCSharpDiagnostics(source, options.sourceFile)
        : [];

    const libraries = parsedLibraries ?? parseLibraries(options.libraries);
    const libraryMethods = buildLibraryMethods(libraries);
    const script = parseEffectScript(source, { file: options.sourceFile });
    if (isOsbImportScript(script)) {
        // The `.osb` importer is a runtime facility, not transpilable C#:
        // map its `Generate()` onto the runtime's `.osb` parser instead of
        // converting the hand-written parser line by line. The parser's warnings
        // describe the C# constructs we deliberately bypass, so they are dropped.
        const pathField = script.parameters.find(parameter => parameter.type === 'string');
        const field = rustSnakeCase(pathField?.name ?? 'Path');
        script.importOsb = true;
        script.transpiledBody = `rt.import_osb(self.${field}.clone());`;
        script.diagnostics = script.diagnostics.filter(
            diagnostic => diagnostic.code !== 'OSB_UNSUPPORTED_CONSTRUCT'
        );
    } else {
        const transpiled = transpileCSharpToRust(script.generateBody, {
            file: options.sourceFile,
            configurableNames: script.parameters.map(parameter => parameter.name),
            helperNames: script.methods
                .filter(method => method.name !== 'Generate')
                .map(method => method.name),
            libraryMethods,
            rtHelpers: [...computeRtHelpers(script)],
            rtLibraryMethods: buildRtLibraryMethods(libraries),
        });
        script.transpiledBody = transpiled.code;
        script.diagnostics.push(...transpiled.diagnostics);
    }
    for (const library of libraries) script.diagnostics.push(...library.diagnostics);

    const files = generateForTarget(script, options.target, {
        ...options,
        parsedLibraries: libraries,
    });
    const configFile = files.find(file => file.path.endsWith('.config.json'));
    let pluginId = `${DEFAULT_PLUGIN_ID_PREFIX}.${script.className.toLowerCase()}`;
    if (configFile) {
        try {
            pluginId = (JSON.parse(configFile.contents) as { id?: string }).id ?? pluginId;
        } catch {
            // keep the fallback id
        }
    }

    return {
        target: options.target,
        effectName: resolveEffectName(script, options),
        pluginId,
        script,
        files,
        diagnostics: [...lintDiagnostics, ...script.diagnostics],
    };
}

/**
 * Converts several effect scripts, either as **one bundled plug-in** (all
 * effects share a single plug-in and run into the same runtime) or as
 * **individual plug-ins** (one per script). External library sources are
 * compiled alongside the effects and their methods are flattened in.
 */
export function convertEffectBundle(options: ConvertBundleOptions): ConvertBundleResult {
    if (!isPluginTarget(options.target)) {
        throw new Error(`Unknown target: ${String(options.target)}`);
    }

    const diagnostics: ConvertBundleResult['diagnostics'] = [];
    const libraries = parseLibraries(options.libraries);
    for (const library of libraries) diagnostics.push(...library.diagnostics);

    if (!options.bundle) {
        const plugins = options.effects.map(effect =>
            convertEffectScript(
                effect.source,
                {
                    ...options,
                    sourceFile: effect.file ?? effect.name,
                    pluginName: undefined,
                    pluginId: undefined,
                },
                libraries
            )
        );
        for (const plugin of plugins) diagnostics.push(...plugin.diagnostics);
        return { target: options.target, bundle: false, plugins, diagnostics };
    }

    const scripts = options.effects.map(effect =>
        parseEffectScript(effect.source, { file: effect.file ?? effect.name })
    );
    scripts.forEach((script, index) => {
        if (options.lintCSharp) {
            const effect = options.effects[index] as EffectSource;
            diagnostics.push(...lintCSharpDiagnostics(effect.source, effect.file ?? effect.name));
        }
        if (isOsbImportScript(script)) {
            script.importOsb = true;
            script.diagnostics = script.diagnostics.filter(
                diagnostic => diagnostic.code !== 'OSB_UNSUPPORTED_CONSTRUCT'
            );
        }
        diagnostics.push(...script.diagnostics);
    });

    const files = generateRustBundleCrate(scripts, options.target, {
        ...options,
        parsedLibraries: libraries,
    });
    const effectName =
        options.pluginName?.trim() || scripts.map(script => script.className).join('+');
    const manifest = files.find(file => file.path.endsWith('manifest.json'));
    let pluginId = `${DEFAULT_PLUGIN_ID_PREFIX}.bundle`;
    if (manifest) {
        try {
            pluginId = (JSON.parse(manifest.contents) as { id?: string }).id ?? pluginId;
        } catch {
            // keep the fallback id
        }
    }

    return {
        target: options.target,
        bundle: true,
        plugins: [
            {
                target: options.target,
                effectName,
                pluginId,
                script: scripts[0] as (typeof scripts)[number],
                files,
                diagnostics,
            },
        ],
        diagnostics,
    };
}
