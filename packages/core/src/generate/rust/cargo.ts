import type { PluginSpec } from '@tochii/osb-toolkit-shared';

import type { RustTarget } from './targets.js';

export function buildCargoToml(input: {
    crateName: string;
    spec: PluginSpec;
    runtimePath: string;
    adapterPath: string;
    config: RustTarget;
}): string {
    const { crateName, spec, runtimePath, adapterPath, config } = input;
    const featureLines = config.features
        .map(feature => `${feature.name} = ["${feature.enables}"]`)
        .join('\n');
    const hostFeature = config.hostGlue ? 'host = []\n' : '';
    return `[package]
name = "${crateName}"
version = "${spec.version.major}.${spec.version.minor}.${spec.version.patch}"
edition = "2021"
description = "${spec.description.replace(/"/g, '\\"')}"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
osb-runtime = { path = "${runtimePath}" }
${config.adapterCrate} = { path = "${adapterPath}" }

[features]
default = []
${featureLines}${featureLines ? '\n' : ''}${hostFeature}
# Detach the generated crate from any enclosing workspace so it builds standalone
# against the path dependencies above.
[workspace]
`;
}
