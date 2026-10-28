import { execSync } from 'node:child_process';
import { cpSync, mkdtempSync, readdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { runGenerator, targets } from '../shared';
import { cleanGoNames, emitGoClient, renameGoTransport } from './post-process';

/** Generates the Go SDK (models + one service per tag + the client). */
export function generateGo(): void {
    const out = mkdtempSync(join(tmpdir(), 'osbtk-ogen-go-'));
    try {
        runGenerator({
            out,
            generator: 'go',
            globalProperty: 'models,apis,supportingFiles',
            additionalProperties:
                'packageName=osbtk,moduleName=github.com/tochiResources/osb-toolkit/bindings/go,enumClassPrefix=true,apiNameSuffix=API,removeOperationIdPrefix=true,removeOperationIdPrefixDelimiter=.',
        });

        // The hand-written `native*.go`/`bridge.go`/`sdk.go`/`library_*.go`/
        // `sdk_test.go` stay; stale generated files are removed before the fresh
        // ones are copied.
        const goKeep = new Set([
            'native.go',
            'native_windows.go',
            'native_unix.go',
            'bridge.go',
            'sdk.go',
            'sdk_test.go',
        ]);
        const isHandWritten = (file: string) => goKeep.has(file) || file.startsWith('library_');
        for (const file of readdirSync(targets.go)) {
            if (file.endsWith('.go') && !isHandWritten(file)) {
                rmSync(join(targets.go, file));
            }
        }
        for (const file of readdirSync(out)) {
            if (file.endsWith('.go') && !file.endsWith('_test.go')) {
                cpSync(join(out, file), join(targets.go, file), { force: true });
            }
        }
        renameGoTransport(targets.go);
        cleanGoNames(targets.go);
        emitGoClient();
        execSync('gofmt -w .', {
            cwd: join(targets.go, 'client'),
            stdio: 'inherit',
            shell: true,
        });
        console.log(`openapi: generated Go SDK into ${targets.go}`);
    } finally {
        rmSync(out, { recursive: true, force: true });
    }
}
