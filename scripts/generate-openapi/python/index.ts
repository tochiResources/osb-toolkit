import { cpSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { runGenerator, targets } from '../shared';

/** Generates the Python SDK (models + `*_api` modules + supporting files). */
export function generatePython(): void {
    const out = mkdtempSync(join(tmpdir(), 'osbtk-ogen-py-'));
    try {
        runGenerator({
            out,
            generator: 'python',
            globalProperty: 'models,apis,supportingFiles',
            additionalProperties:
                'packageName=osbtk,projectName=osbtk-sdk,library=urllib3,hideGenerationTimestamp=true,apiNameSuffix=API,removeOperationIdPrefix=true,removeOperationIdPrefixDelimiter=.',
        });

        // The hand-written `__init__.py` and `_native.py`/`bridge.py` stay; only
        // the generated modules are refreshed.
        const source = join(out, 'osbtk');
        for (const part of ['api', 'models']) {
            rmSync(join(targets.python, part), { recursive: true, force: true });
            cpSync(join(source, part), join(targets.python, part), { recursive: true });
        }
        for (const file of [
            'api_client.py',
            'api_response.py',
            'configuration.py',
            'exceptions.py',
            'rest.py',
            'py.typed',
        ]) {
            cpSync(join(source, file), join(targets.python, file));
        }
        console.log(`openapi: generated Python SDK into ${targets.python}`);
    } finally {
        rmSync(out, { recursive: true, force: true });
    }
}
