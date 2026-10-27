#!/usr/bin/env -S tsx
/**
 * Installer generation step for the build pipeline (M17b): packages every
 * `installers/*.json` config into a distributable installer using the
 * `@tochii/installer` programmatic API. Safe to run with no configs (no-op).
 *
 * Usage: `yarn installers [dir]` (default: `installers`).
 */
import { readdir, readFile } from 'node:fs/promises';
import { join } from 'node:path';

import { packageInstaller, type InstallerConfig } from '@tochii/installer';

const dir = process.argv[2] ?? 'installers';

let names: string[];
try {
    names = (await readdir(dir)).filter(name => name.endsWith('.json'));
} catch (error) {
    if ((error as NodeJS.ErrnoException).code === 'ENOENT') {
        console.log(`no ${dir}/ directory; nothing to package`);
        process.exit(0);
    }
    throw error;
}

if (names.length === 0) {
    console.log(`no installer configs in ${dir}/; nothing to package`);
    process.exit(0);
}

for (const name of names) {
    const config = JSON.parse(await readFile(join(dir, name), 'utf8')) as InstallerConfig;
    const installer = await packageInstaller(config);
    console.log(
        `${name}: ${installer.product} -> ${installer.outputDir} (${installer.files.length} file(s))`
    );
}
