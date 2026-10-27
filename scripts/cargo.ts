#!/usr/bin/env -S tsx
import { spawn } from 'node:child_process';
import { existsSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { createInterface } from 'node:readline';
import { fileURLToPath } from 'node:url';

// A `cargo` wrapper that keeps the Rust artifact cache bounded.
//
// Cargo stores one artifact per *fingerprint* (per command/profile/target/feature
// combination) under `target/*/deps`, and never removes the ones a later build no
// longer references - so repeated `build`/`test`/`clippy`/`run` invocations grow the
// cache without bound. This wrapper runs cargo with
// `--message-format=json-render-diagnostics`, records each build's artifact hashes in
// `target/.osbtk-live.json`, and then deletes every `deps`/`.fingerprint`/`build`
// entry whose hash is not among the last N builds. The artifacts of the commands you
// actually re-run stay cached; only the stale ones (changed flags/features/toolchain)
// are reclaimed. Worst case a deleted-but-needed artifact is simply rebuilt.
//
//   OSBTK_RUST_PRUNE=0            disable the prune (plain cargo passthrough)
//   OSBTK_RUST_LIVE_BUILDS=<n>    how many recent builds to keep (default 8)
//
// Usage: tsx scripts/cargo.ts <cargo args...>

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const args = process.argv.slice(2);
const keepBuilds = Math.max(1, Number(process.env.OSBTK_RUST_LIVE_BUILDS ?? 8));
const pruneEnabled = process.env.OSBTK_RUST_PRUNE !== '0';
const cargo = process.platform === 'win32' ? 'cargo.exe' : 'cargo';
const HASH = /-([0-9a-f]{16})(?=[./\\]|$)/;

const hashes = new Set<string>();
// `--message-format` must go to cargo, before any `--` that forwards the rest to
// the wrapped compiler (e.g. `clippy -- -D warnings`).
const separator = args.indexOf('--');
const cargoArgs =
    separator === -1
        ? [...args, '--message-format=json-render-diagnostics']
        : [
              ...args.slice(0, separator),
              '--message-format=json-render-diagnostics',
              ...args.slice(separator),
          ];
const child = spawn(cargo, cargoArgs, {
    cwd: root,
    stdio: ['inherit', 'pipe', 'inherit'],
});

createInterface({ input: child.stdout }).on('line', line => {
    let message: { reason?: string; filenames?: string[]; out_dir?: string } | undefined;
    if (line.startsWith('{')) {
        try {
            message = JSON.parse(line) as typeof message;
        } catch {
            message = undefined;
        }
    }
    if (message) {
        // A cargo JSON message: collect artifact hashes, never print it.
        if (message.reason === 'compiler-artifact' && Array.isArray(message.filenames)) {
            for (const file of message.filenames) {
                const match = HASH.exec(file);
                if (match) {
                    hashes.add(match[1]);
                }
            }
        } else if (
            message.reason === 'build-script-executed' &&
            typeof message.out_dir === 'string'
        ) {
            // A build script owns two hashed dirs - the compiled script and its
            // `out/` - with different hashes; keep the output dir too.
            const match = HASH.exec(message.out_dir);
            if (match) {
                hashes.add(match[1]);
            }
        }
        return;
    }
    // Not a cargo message (build progress goes to stderr; test-harness output comes
    // here): pass it through unchanged.
    process.stdout.write(`${line}\n`);
});

child.on('close', code => {
    if (pruneEnabled && code === 0) {
        try {
            prune();
        } catch (error) {
            console.error(
                `rust: cache prune skipped (${error instanceof Error ? error.message : error})`
            );
        }
    }
    process.exit(code ?? 1);
});

function prune(): void {
    const manifestPath = join(root, 'target', '.osbtk-live.json');
    const key = args.join(' ');

    let records: Array<{ key: string; hashes: string[] }> = [];
    if (existsSync(manifestPath)) {
        try {
            records = JSON.parse(readFileSync(manifestPath, 'utf8')) as typeof records;
        } catch {
            records = [];
        }
    }
    records = records.filter(record => record.key !== key);
    records.push({ key, hashes: [...hashes] });
    records = records.slice(-keepBuilds);
    const live = new Set(records.flatMap(record => record.hashes));
    writeFileSync(manifestPath, JSON.stringify(records), 'utf8');

    let removed = 0;
    let bytes = 0;

    const remove = (path: string): void => {
        try {
            if (process.env.OSBTK_RUST_DEBUG) {
                console.error(`  remove ${path}`);
            }
            bytes += dirSize(path);
            rmSync(path, { recursive: true, force: true });
            removed += 1;
        } catch {
            // A file may vanish mid-walk; ignore.
        }
    };

    const pruned = new Set<string>();

    const pruneHashed = (dir: string): void => {
        for (const entry of readdirSync(dir, { withFileTypes: true })) {
            const match = HASH.exec(entry.name);
            if (match && !live.has(match[1])) {
                pruned.add(match[1]);
                remove(join(dir, entry.name));
            }
        }
    };

    // Pass 1: drop the stale `deps`/`build` artifacts (the big files).
    const walk = (dir: string): void => {
        let entries;
        try {
            entries = readdirSync(dir, { withFileTypes: true });
        } catch {
            return;
        }
        for (const entry of entries) {
            if (!entry.isDirectory()) {
                continue;
            }
            const path = join(dir, entry.name);
            if (entry.name === 'deps' || entry.name === 'build') {
                pruneHashed(path);
                continue;
            }
            if (entry.name === '.fingerprint' || entry.name === 'incremental') {
                continue;
            }
            walk(path);
        }
    };
    walk(join(root, 'target'));

    // Pass 2: drop the fingerprints of the units we just pruned, so cargo does not
    // treat them as fresh while their artifact is gone (which would fail the next
    // build). Fingerprints of units that were NOT pruned are left alone - notably a
    // top-level crate, whose fingerprint hash is not derivable from its unhashed
    // output name.
    const dropFingerprints = (dir: string): void => {
        let entries;
        try {
            entries = readdirSync(dir, { withFileTypes: true });
        } catch {
            return;
        }
        for (const entry of entries) {
            if (!entry.isDirectory()) {
                continue;
            }
            const path = join(dir, entry.name);
            if (entry.name === '.fingerprint') {
                for (const fp of readdirSync(path, { withFileTypes: true })) {
                    const match = HASH.exec(fp.name);
                    if (match && pruned.has(match[1])) {
                        remove(join(path, fp.name));
                    }
                }
                continue;
            }
            if (entry.name === 'deps' || entry.name === 'build' || entry.name === 'incremental') {
                continue;
            }
            dropFingerprints(path);
        }
    };
    dropFingerprints(join(root, 'target'));
    console.error(
        `rust: cache pruned (${removed} stale entries, ${(bytes / 1024 / 1024).toFixed(0)} MB) after "${key}"`
    );
}

function dirSize(path: string): number {
    let total = 0;
    for (const entry of readdirSync(path, { withFileTypes: true })) {
        const child = join(path, entry.name);
        try {
            if (entry.isDirectory()) {
                total += dirSize(child);
            } else if (entry.isFile()) {
                total += statSync(child).size;
            }
        } catch {
            // A file may vanish mid-walk; ignore.
        }
    }
    return total;
}
