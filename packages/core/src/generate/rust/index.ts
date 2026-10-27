import type { EffectScript, GeneratedFile, PluginTarget } from '@tochii/osb-toolkit-shared';

import { transpileCSharpToRust } from '../../csharp/rust/index.js';
import {
    buildManifest,
    buildPluginSpec,
    readmeFor,
    resolveEffectName,
    resolveResolution,
} from '../common.js';
import { buildCargoToml } from './cargo.js';
import { aeEntryC, buildRs, piplR } from './entry-ae.js';
import { buildRsOfx, ofxEntryC } from './entry-ofx.js';
import {
    buildLibRs,
    buildLibraryMethods,
    buildRtLibraryMethods,
    computeRtHelpers,
    rustCrateName,
    withConcurrencyParameters,
} from './lib.js';
import { type RustCrateOptions, TARGETS } from './targets.js';

/**
 * Emits a self-contained Rust plug-in crate for a converted effect.
 *
 * The crate implements `osb_runtime::storyboarding::Effect` (the transpiled
 * `Generate()` body) and `parameters()` (one `ParameterDescriptor` per
 * `[Configurable]`), and links the host adapter crate which supplies the host
 * entry points and packaging.
 */
export function generateRustEffectCrate(
    script: EffectScript,
    target: PluginTarget,
    options: RustCrateOptions = {}
): GeneratedFile[] {
    const config = TARGETS[target];
    if (!config) throw new Error(`Unsupported target: ${target}`);

    const name = resolveEffectName(script, options);
    const spec = withConcurrencyParameters(buildPluginSpec(script, target, options));
    const resolution = resolveResolution(options);
    const crateName = rustCrateName(name);
    const folder = name;
    const runtimePath = options.runtimeCratePath ?? '../../crates/osb-runtime';
    const adapterPath = options.adapterCratePath ?? `../../crates/${config.adapterCrate}`;
    const libraryMethods = buildLibraryMethods(options.parsedLibraries ?? []);

    const body =
        script.transpiledBody ??
        transpileCSharpToRust(script.generateBody, {
            file: script.sourceFile,
            configurableNames: script.parameters.map(parameter => parameter.name),
            helperNames: script.methods
                .filter(method => method.name !== 'Generate')
                .map(method => method.name),
            libraryMethods,
            rtHelpers: [...computeRtHelpers(script)],
            rtLibraryMethods: buildRtLibraryMethods(options.parsedLibraries ?? []),
        }).code;

    return [
        {
            path: `${folder}/Cargo.toml`,
            contents: buildCargoToml({ crateName, spec, runtimePath, adapterPath, config }),
            kind: 'project',
        },
        {
            path: `${folder}/src/lib.rs`,
            contents: buildLibRs({
                script,
                spec,
                config,
                body,
                libraries: options.parsedLibraries,
                libraryMethods,
            }),
            kind: 'source',
        },
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
            contents: readmeFor({ script, spec, options, resolution }, [
                'Generated Rust plug-in crate; depends on the `osb-runtime` and adapter crates by path.',
                'Build with `cargo build --release`; the adapter crate produces the host entry points and PiPL/bundle.',
                target === 'davinci-resolve'
                    ? 'Copy the built `.ofx.bundle` into `%COMMONPROGRAMFILES%\\OFX\\Plugins`.'
                    : 'Copy the built `.aex` into the shared MediaCore folder.',
            ]),
            kind: 'readme',
        },
    ];
}
