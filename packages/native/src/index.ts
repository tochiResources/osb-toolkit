/**
 * `@tochii/osb-toolkit-native` - the `wasm-bindgen` binding of the osbTK runtime
 * (). It drives the same engine as the Rust plug-ins and the
 * C#/C++/Python bindings, so **Rust is the only source of truth**: the toolkit's
 * 3D/keyframe/math logic lives in Rust and TypeScript consumes it through this
 * binding (there is no TypeScript reimplementation).
 *
 * The generated `wasm/osb_wasm.js` is a CommonJS module (wasm-bindgen
 * `--target nodejs`), loaded here with `createRequire`.
 */
import { createRequire } from 'node:module';

import type * as Wasm from '../wasm/osb_wasm.js';

const require = createRequire(import.meta.url);
const wasm = require('../wasm/osb_wasm.js') as typeof Wasm;

/**
 * The Node filesystem host, installed into `globalThis.__osbtk_io` so the WASM
 * file API (`osb-core`) can reach real files under Node/Bun. Installed only
 * when a Node `process` is present, so browser bundles never reference `fs`.
 */
function installNodeIOHost(): void {
    if (typeof process === 'undefined' || !process.versions?.node) {
        return;
    }
    // eslint-disable-next-line @typescript-eslint/no-require-imports
    const fs = require('node:fs') as typeof import('node:fs');
    (globalThis as Record<string, unknown>).__osbtk_io = {
        read: (path: string) => new Uint8Array(fs.readFileSync(path)),
        write: (path: string, bytes: Uint8Array) => fs.writeFileSync(path, bytes),
        exists: (path: string) => fs.existsSync(path),
        isFile: (path: string) => {
            try {
                return fs.statSync(path).isFile();
            } catch {
                return false;
            }
        },
        isDir: (path: string) => {
            try {
                return fs.statSync(path).isDirectory();
            } catch {
                return false;
            }
        },
        fileSize: (path: string) => fs.statSync(path).size,
        list: (path: string) => fs.readdirSync(path).sort(),
        createDirAll: (path: string) => {
            fs.mkdirSync(path, { recursive: true });
        },
        removeFile: (path: string) => {
            fs.rmSync(path);
        },
        removeDirAll: (path: string) => {
            fs.rmSync(path, { recursive: true, force: true });
        },
        copy: (from: string, to: string) => {
            fs.copyFileSync(from, to);
        },
        rename: (from: string, to: string) => fs.renameSync(from, to),
    };
    wasm.ioInstallGlobalHost();
}

installNodeIOHost();

export type Json = unknown;

/** The runtime version string. */
export function version(): string {
    return wasm.version();
}

/** Bakes a plain 3D input (`Bake3dInput`) into baked command-span JSON. */
export function bake3d(input: Json): Json {
    return JSON.parse(wasm.bake3d(JSON.stringify(input))) as Json;
}

/** Runs a JSON request (`{"op": "..."}`) against the whole runtime. */
export function exec(request: Json): Json {
    return JSON.parse(wasm.exec(JSON.stringify(request))) as Json;
}

/** Parses a `.osu` beatmap into a summary. */
export function parseOsu(text: string): Json {
    return exec({ op: 'Core.parseOsu', text });
}

/** Parses a `.osb` storyboard into a summary. */
export function parseOsb(text: string): Json {
    return exec({ op: 'Core.parseOsb', text });
}

/** FFT magnitudes for a WAV file at `time` (ms). */
export function fft(path: string, time = 0, magnitudes = 64): number[] {
    return (exec({ op: 'Audio.fft', path, time, magnitudes }) as { magnitudes: number[] })
        .magnitudes;
}

// -- File operations (osb-core / osb-core) --------------------------------

/** The detected host environment: `native`/`node`/`bun`/`deno`/`browser`/`unknown`. */
export function ioEnvironment(): string {
    return wasm.ioEnvironment();
}

/**
 * Installs a filesystem host from `globalThis.__osbtk_io` when the embedder
 * provides one (e.g. Node's `fs`), returning the detected environment.
 */
export function ioInstallGlobalHost(): string {
    return wasm.ioInstallGlobalHost();
}

/** Reads a file into a byte array. */
export function ioRead(path: string): Uint8Array {
    return wasm.ioRead(path);
}

/** Reads a file into a UTF-8 string. */
export function ioReadString(path: string): string {
    return wasm.ioReadString(path);
}

/** Writes bytes to a file. */
export function ioWrite(path: string, bytes: Uint8Array): void {
    wasm.ioWrite(path, bytes);
}

/** Writes a UTF-8 string to a file. */
export function ioWriteString(path: string, contents: string): void {
    wasm.ioWriteString(path, contents);
}

/** Whether a path exists. */
export function ioExists(path: string): boolean {
    return wasm.ioExists(path);
}

/** Whether a path is a regular file. */
export function ioIsFile(path: string): boolean {
    return wasm.ioIsFile(path);
}

/** Whether a path is a directory. */
export function ioIsDir(path: string): boolean {
    return wasm.ioIsDir(path);
}

/** The size of a file in bytes. */
export function ioFileSize(path: string): number {
    return wasm.ioFileSize(path);
}

/** Lists a directory's entry names, sorted. */
export function ioList(path: string): string[] {
    return wasm.ioList(path);
}

/** Creates a directory and any missing parents. */
export function ioCreateDirAll(path: string): void {
    wasm.ioCreateDirAll(path);
}

/** Copies a file. */
export function ioCopy(from: string, to: string): void {
    wasm.ioCopy(from, to);
}

/** Renames (or moves) a file. */
export function ioRename(from: string, to: string): void {
    wasm.ioRename(from, to);
}

/** Removes a file. */
export function ioRemoveFile(path: string): void {
    wasm.ioRemoveFile(path);
}

/** Removes a directory and everything inside it. */
export function ioRemoveDirAll(path: string): void {
    wasm.ioRemoveDirAll(path);
}

export interface SpriteHandle {
    layer: number;
    sprite: number;
}

/** A live runtime (storyboard + context) - the storyboard API. */
export class Runtime {
    private readonly inner: Wasm.Runtime;

    constructor(width = 640, height = 480) {
        this.inner = new wasm.Runtime(width, height);
    }

    /** Runs a JSON request against this runtime. */
    exec(request: Json): Json {
        return JSON.parse(this.inner.exec(JSON.stringify(request))) as Json;
    }

    /** Creates a sprite and returns its `{ layer, sprite }` handle. */
    createSprite(layer: string, path: string, origin = 4): SpriteHandle {
        return this.exec({ op: 'Layer.createSprite', layer, path, origin }) as SpriteHandle;
    }

    /** Frees the underlying native runtime. */
    free(): void {
        this.inner.free();
    }
}
