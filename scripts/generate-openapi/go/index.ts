import { cpSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { runGenerator, targets } from '../shared';

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
        // Go names each operation's request wrapper `Api<Op>Request`, which
        // collides when two classes share a method name (e.g. `Image.info` and
        // `Media.info`). Qualify it with the class so the names stay unique,
        // keeping the `Api` prefix so it never collides with a model.
        for (const file of readdirSync(targets.go)) {
            if (!file.startsWith('api_') || !file.endsWith('.go')) {
                continue;
            }
            const path = join(targets.go, file);
            const text = readFileSync(path, 'utf8');
            const cls = text.match(/^type (\w+)Service service$/m)?.[1];
            if (cls) {
                writeFileSync(
                    path,
                    text.replace(/\bApi(\w+?)Request\b/g, `Api${cls}$1Request`),
                    'utf8'
                );
            }
        }
        console.log(`openapi: generated Go SDK into ${targets.go}`);
    } finally {
        rmSync(out, { recursive: true, force: true });
    }
}
