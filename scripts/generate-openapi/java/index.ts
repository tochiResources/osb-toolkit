import { cpSync, mkdtempSync, readdirSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { runGenerator, targets } from '../shared';

/** Generates the Java SDK (models + one API class per tag + supporting files). */
export function generateJava(): void {
    const out = mkdtempSync(join(tmpdir(), 'osbtk-ogen-java-'));
    try {
        runGenerator({
            out,
            generator: 'java',
            globalProperty: 'models,apis,supportingFiles',
            additionalProperties:
                'groupId=moe.tochi,artifactId=osbtk-sdk,invokerPackage=moe.tochi.osbtk,apiPackage=moe.tochi.osbtk.api,modelPackage=moe.tochi.osbtk.model,library=okhttp-gson,hideGenerationTimestamp=true,apiNameSuffix=API,removeOperationIdPrefix=true,removeOperationIdPrefixDelimiter=.',
        });

        // The hand-written `osbTK.java`/`Runtime.java`/`bridge/**` stay; the
        // generated `api`/`model` trees are replaced (stale files removed) and
        // the generated supporting files are copied over.
        const generated = join(out, 'src', 'main', 'java', 'moe', 'tochi', 'osbtk');
        const target = join(targets.java, 'moe', 'tochi', 'osbtk');
        for (const entry of readdirSync(generated)) {
            const from = join(generated, entry);
            const to = join(target, entry);
            // Replace the generated `api`/`model` trees so renamed classes never
            // leave stale files behind; copy the rest (supporting files, `auth/`)
            // over the existing tree.
            if (entry === 'api' || entry === 'model') {
                rmSync(to, { recursive: true, force: true });
                cpSync(from, to, { recursive: true });
            } else if (statSync(from).isDirectory()) {
                cpSync(from, to, { recursive: true, force: true });
            } else if (entry.endsWith('.java')) {
                cpSync(from, to, { force: true });
            }
        }
        console.log(`openapi: generated Java SDK into ${targets.java}`);
    } finally {
        rmSync(out, { recursive: true, force: true });
    }
}
