#!/usr/bin/env -S tsx
import { existsSync, readdirSync, rmSync, statSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

// Flushes the toolkit's build outputs so stale artifacts do not accumulate on
// disk (M34 build-pipeline hygiene). By default it removes every generated build
// directory; pass `--rust` to flush only the Rust `target` caches (the largest
// offenders), which is what a normal "reclaim space" pass needs.

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const args = new Set(process.argv.slice(2));
const rustOnly = args.has('--rust');
const incrementalOnly = args.has('--incremental');
const ifLarge = args.has('--if-large');
const maxMb = Number(
    args.has('--max-mb')
        ? process.argv[process.argv.indexOf('--max-mb') + 1]
        : (process.env.OSBTK_RUST_CACHE_MAX_MB ?? 12288)
);

let removed = 0;
let bytes = 0;

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

function remove(relative: string): void {
    const path = join(root, relative);
    if (!existsSync(path)) {
        return;
    }
    let size = 0;
    try {
        size = dirSize(path);
    } catch {
        size = 0;
    }
    rmSync(path, { recursive: true, force: true });
    removed += 1;
    bytes += size;
    console.log(`clean: removed ${relative} (${(size / 1024 / 1024).toFixed(1)} MB)`);
}

/** Removes every direct child of `relative` whose name matches `predicate`. */
function removeChildren(relative: string, predicate: (name: string) => boolean): void {
    const path = join(root, relative);
    if (!existsSync(path)) {
        return;
    }
    for (const entry of readdirSync(path, { withFileTypes: true })) {
        if (entry.isDirectory() && predicate(entry.name)) {
            remove(join(relative, entry.name));
        }
    }
}

/** Removes `<parent>/<child>/target` for every child of `parent`. */
function removeNestedTargets(parent: string): void {
    const path = join(root, parent);
    if (!existsSync(path)) {
        return;
    }
    for (const entry of readdirSync(path, { withFileTypes: true })) {
        if (entry.isDirectory()) {
            remove(join(parent, entry.name, 'target'));
        }
    }
}

// A size guard: flush the whole Rust cache only when it grows past a threshold,
// so repeated build/test/clippy/run invocations (which accumulate per-fingerprint
// artifacts under `target/*/deps`) cannot fill the disk. Wired as a `pre` hook on
// the Rust-building scripts; tune with `--max-mb <n>` or `OSBTK_RUST_CACHE_MAX_MB`.
if (ifLarge) {
    const target = join(root, 'target');
    if (existsSync(target)) {
        const mb = dirSize(target) / 1024 / 1024;
        if (mb > maxMb) {
            remove('target');
            console.log(`clean: Rust target was ${mb.toFixed(0)} MB (> ${maxMb} MB); flushed`);
        } else {
            console.log(`clean: Rust target ${mb.toFixed(0)} MB (<= ${maxMb} MB); kept`);
        }
    }
    console.log(
        `clean: done - removed ${removed} directories (~${(bytes / 1024 / 1024).toFixed(1)} MB)`
    );
    process.exit(0);
}

// Rust: the root workspace target and every standalone example crate's target.
if (incrementalOnly) {
    // The per-crate incremental cache is the largest and fastest-growing
    // artifact; flushing just it reclaims most of the space without forcing a
    // full recompile of dependencies.
    for (const profile of existsSync(join(root, 'target'))
        ? readdirSync(join(root, 'target'), { withFileTypes: true })
        : []) {
        if (profile.isDirectory()) {
            remove(join('target', profile.name, 'incremental'));
        }
    }
} else {
    remove('target');
    for (const entry of existsSync(root) ? readdirSync(root, { withFileTypes: true }) : []) {
        if (entry.isDirectory() && entry.name.startsWith('dist-')) {
            removeNestedTargets(entry.name);
        }
    }
}

if (!rustOnly && !incrementalOnly) {
    // Native binding build outputs.
    remove('packages/native/wasm');
    removeChildren('bindings/csharp', name => name === 'bin' || name === 'obj');
    removeNestedTargets('bindings/csharp');
    remove('bindings/java/target');
    remove('bindings/cpp/test/build');

    // JS/TS build outputs and caches.
    removeChildren('packages', name => name === 'dist');
    removeChildren('apps', name => name === 'dist');
    remove('docs/.vitepress/dist');
    remove('.turbo');
}

console.log(
    `clean: done - removed ${removed} directories (~${(bytes / 1024 / 1024).toFixed(1)} MB)`
);
