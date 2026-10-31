import { execSync } from 'node:child_process';
import { cpSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { emitRustClean } from '../clean';
import { root, runGenerator, targets } from '../shared';

/** Generates the Rust SDK models (the client/runtime are hand-written). */
export function generateRust(): void {
    const out = mkdtempSync(join(tmpdir(), 'osbtk-ogen-rust-'));
    try {
        runGenerator({
            out,
            generator: 'rust',
            globalProperty: 'models,supportingFiles',
            additionalProperties:
                'packageName=osb_sdk,packageVersion=0.1.0,hideGenerationTimestamp=true',
        });

        // Only the generated models are used; `lib.rs`/`client.rs`/`runtime.rs`
        // are hand-written and live beside them.
        rmSync(targets.rust, { recursive: true, force: true });
        cpSync(join(out, 'src', 'models'), targets.rust, { recursive: true });
        emitRustClean();
        execSync('cargo fmt -p osb-sdk', { cwd: root, stdio: 'inherit', shell: true });
        console.log(`openapi: generated Rust SDK models into ${targets.rust}`);
    } finally {
        rmSync(out, { recursive: true, force: true });
    }
}
