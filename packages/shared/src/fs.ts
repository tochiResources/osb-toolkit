import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';

import type { WriteFilesOptions, WrittenFile } from './types/fs.js';
import type { GeneratedFile } from './types/ir.js';

export async function pathExists(path: string): Promise<boolean> {
    return existsSync(path);
}

export async function ensureDir(path: string): Promise<void> {
    await mkdir(path, { recursive: true });
}

export async function readTextFile(path: string): Promise<string> {
    return readFile(path, 'utf8');
}

export async function writeTextFile(path: string, contents: string): Promise<void> {
    await ensureDir(dirname(path));
    await writeFile(path, contents, 'utf8');
}

export async function writeGeneratedFiles(
    files: GeneratedFile[],
    options: WriteFilesOptions
): Promise<WrittenFile[]> {
    const root = resolve(options.root);
    const written: WrittenFile[] = [];
    for (const file of files) {
        const absolutePath = join(root, file.path);
        await writeTextFile(absolutePath, file.contents);
        written.push({ path: file.path, absolutePath });
    }
    return written;
}
