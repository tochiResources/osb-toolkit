import {
    createPluginSpec,
    type EffectScript,
    type GeneratedFile,
    type PluginParameter,
    type PluginTarget,
} from '@tochii/osb-toolkit-shared';

import { rustSnakeCase, transpileCSharpToRust } from '../../csharp/rust/index.js';
import { buildManifest, readmeFor, resolveResolution, toPluginParameter } from '../common.js';
import { buildCargoToml } from './cargo.js';
import { aeEntryC, buildRs, piplR } from './entry-ae.js';
import { buildRsOfx, ofxEntryC } from './entry-ofx.js';
import {
    buildEffectLibRs,
    buildHelperMethods,
    buildLibraryMethods,
    buildRtLibraryMethods,
    computeRtHelpers,
    rustCrateName,
    rustFieldType,
    rustParameterDescriptor,
    rustSetArm,
    rustStructName,
    withConcurrencyParameters,
} from './lib.js';
import { type RustCrateOptions, TARGETS } from './targets.js';

export interface RustBundleOptions extends RustCrateOptions {
    /**
     * How bundled parameters are grouped: `effect` (default) puts each script's
     * parameters in their own group; `combined` keeps the original `[Group]`s and
     * merges parameters across scripts.
     */
    parameterGrouping?: 'effect' | 'combined';
}

/**
 * Emits a single Rust plug-in crate that bundles several effect scripts: one
 * struct whose `generate()` runs every script's `Generate()` into the same
 * runtime, with each script's configurables and helper methods namespaced by its
 * class name to avoid collisions.
 */
export function generateRustBundleCrate(
    scripts: EffectScript[],
    target: PluginTarget,
    options: RustBundleOptions = {}
): GeneratedFile[] {
    const config = TARGETS[target];
    if (!config) throw new Error(`Unsupported target: ${target}`);

    const name = options.pluginName?.trim() || scripts.map(script => script.className).join('+');
    const structName = rustStructName(name);
    const resolution = resolveResolution(options);
    const runtimePath = options.runtimeCratePath ?? '../../crates/osb-runtime';
    const adapterPath = options.adapterCratePath ?? `../../crates/${config.adapterCrate}`;

    const libraries = options.parsedLibraries ?? [];
    const libraryMethods = buildLibraryMethods(libraries);
    const grouping = options.parameterGrouping ?? 'effect';

    const mergedParameters: PluginParameter[] = [];
    const bodies: string[] = [];
    const helperBlocks: string[] = [];

    for (const script of scripts) {
        const prefix = rustSnakeCase(script.className);
        const names: Record<string, string> = {};
        for (const parameter of script.parameters) {
            const mergedName = `${script.className}_${parameter.name}`;
            const base = toPluginParameter(parameter);
            // By default each effect is its own parameter group (with its own
            // `[Group]` as a sub-label); `combined` keeps the original groups and
            // merges parameters across effects.
            const group =
                grouping === 'combined'
                    ? base.group
                    : parameter.group
                      ? `${script.className} - ${parameter.group}`
                      : script.className;
            mergedParameters.push({
                ...base,
                name: mergedName,
                label: `${script.className}: ${parameter.name}`,
                group,
            });
            names[parameter.name] = rustSnakeCase(mergedName);
        }
        const helperNames = script.methods
            .filter(method => method.name !== 'Generate')
            .map(method => method.name);
        for (const helperName of helperNames) {
            names[helperName] = `${prefix}_${rustSnakeCase(helperName)}`;
        }

        if (script.importOsb) {
            const pathField =
                script.parameters.find(parameter => parameter.type === 'string')?.name ?? 'Path';
            bodies.push(`rt.import_osb(self.${names[pathField]}.clone());`);
        } else {
            bodies.push(
                transpileCSharpToRust(script.generateBody, {
                    file: script.sourceFile,
                    configurableNames: script.parameters.map(parameter => parameter.name),
                    helperNames,
                    names,
                    libraryMethods,
                    rtHelpers: [...computeRtHelpers(script)],
                    rtLibraryMethods: buildRtLibraryMethods(libraries),
                }).code
            );
        }
        helperBlocks.push(
            buildHelperMethods(script, {
                structName,
                names,
                prefix,
                rtHelpers: computeRtHelpers(script),
                rtLibraryMethods: buildRtLibraryMethods(libraries),
            })
        );
    }

    // Library methods are flattened onto the struct under their class prefix.
    for (const library of libraries) {
        helperBlocks.push(
            buildHelperMethods(library, {
                structName,
                prefix: rustSnakeCase(library.className),
                libraryMethods,
                rtHelpers: computeRtHelpers(library),
                rtLibraryMethods: buildRtLibraryMethods(libraries),
            })
        );
    }

    const spec = withConcurrencyParameters(
        createPluginSpec({
            target,
            name,
            id: options.pluginId,
            category: options.category,
            description:
                options.description ??
                `Bundle of ${scripts.length} effects: ${scripts.map(script => script.className).join(', ')}`,
            supportUrl: options.supportUrl,
            parameters: mergedParameters,
        })
    );
    const crateName = rustCrateName(name);
    const folder = name;

    const libRs = buildEffectLibRs({
        spec,
        config,
        structName,
        fields: mergedParameters
            .map(
                parameter =>
                    `    pub ${rustSnakeCase(parameter.name)}: ${rustFieldType(parameter)},`
            )
            .join('\n'),
        parameterLines: spec.parameters
            .map(parameter => `            ${rustParameterDescriptor(parameter)},`)
            .join('\n'),
        setArms: mergedParameters
            .map(parameter => `            "${parameter.name}" => { ${rustSetArm(parameter)} }`)
            .join('\n'),
        generateBody: bodies.join('\n'),
        helpers: helperBlocks.join(''),
        header: `${name} - bundle of ${scripts.length} effects.`,
    });

    return [
        {
            path: `${folder}/Cargo.toml`,
            contents: buildCargoToml({ crateName, spec, runtimePath, adapterPath, config }),
            kind: 'project',
        },
        { path: `${folder}/src/lib.rs`, contents: libRs, kind: 'source' },
        buildManifest(spec),
        ...(config.hostEntry === 'ae'
            ? [
                  { path: `${folder}/build.rs`, contents: buildRs(), kind: 'build' as const },
                  {
                      path: `${folder}/PiPL.r`,
                      contents: piplR(name, spec.matchName, spec.category, config),
                      kind: 'source' as const,
                  },
                  {
                      path: `${folder}/src/ae_entry.cpp`,
                      contents: aeEntryC(name, spec.matchName, spec.category),
                      kind: 'source' as const,
                  },
              ]
            : config.hostEntry === 'ofx'
              ? [
                    { path: `${folder}/build.rs`, contents: buildRsOfx(), kind: 'build' as const },
                    {
                        path: `${folder}/src/ofx_entry.cpp`,
                        contents: ofxEntryC(name, spec.id, spec.category),
                        kind: 'source' as const,
                    },
                ]
              : []),
        {
            path: `${folder}/README.md`,
            contents: readmeFor({ script: scripts[0] as EffectScript, spec, options, resolution }, [
                `Bundle of ${scripts.length} effects: ${scripts.map(script => script.className).join(', ')}.`,
                'Generated Rust plug-in crate; depends on the `osb-runtime` and adapter crates by path.',
                'Build with `cargo build --release`; the adapter crate produces the host entry points and PiPL/bundle.',
            ]),
            kind: 'readme',
        },
    ];
}
