import {
    cpSync,
    mkdirSync,
    mkdtempSync,
    readdirSync,
    readFileSync,
    renameSync,
    rmSync,
    writeFileSync,
} from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { runGenerator, targets } from '../shared';
import { emitTypeScriptClient, protectTransport } from './post-process';

/** Generates the TypeScript SDK and normalizes its folder/barrel names. */
export function generateTypeScript(): void {
    const out = mkdtempSync(join(tmpdir(), 'osbtk-ogen-ts-'));
    try {
        runGenerator({
            out,
            generator: 'typescript-fetch',
            globalProperty: 'models,apis,supportingFiles',
            additionalProperties:
                'npmName=@tochii/osb-sdk-js,npmVersion=0.1.0,supportsES6=true,typescriptThreePlus=true,useSingleRequestParameter=false,apiNameSuffix=API,removeOperationIdPrefix=true,removeOperationIdPrefixDelimiter=.',
        });

        rmSync(targets.typescript, { recursive: true, force: true });
        mkdirSync(targets.typescript, { recursive: true });
        cpSync(join(out, 'src'), targets.typescript, { recursive: true });
        // The generator names each operation's parameter interface after the
        // method alone (`InfoRequest`), which collides when two classes share a
        // method name (e.g. `Image.info` and `Media.info`). The generator has
        // already kept these names clear of models, so qualifying them with the
        // class (as `Api<Class><Name>`) is both unique and model-safe.
        const apisDir = join(targets.typescript, 'apis');
        for (const file of readdirSync(apisDir)) {
            if (!file.endsWith('.ts') || file === 'index.ts') {
                continue;
            }
            const path = join(apisDir, file);
            const className = file.slice(0, -'.ts'.length);
            let text = readFileSync(path, 'utf8');
            for (const match of text.matchAll(/^export interface (\w+)/gm)) {
                text = text.replace(
                    new RegExp(`\\b${match[1]}\\b`, 'g'),
                    `Api${className}${match[1]}`
                );
            }
            writeFileSync(path, text, 'utf8');
        }
        // `typescript-fetch` hardcodes the plural `apis/` folder (its
        // `--api-package` option is ignored). Rename it to the singular `api/`
        // to match the other SDKs, and point the barrel export at it.
        renameSync(apisDir, join(targets.typescript, 'api'));
        const barrelPath = join(targets.typescript, 'index.ts');
        writeFileSync(
            barrelPath,
            readFileSync(barrelPath, 'utf8')
                .replaceAll("'./apis/index'", "'./api/index'")
                .replaceAll("'./apis'", "'./api'"),
            'utf8'
        );
        protectTransport(join(targets.typescript, 'api'));
        emitTypeScriptClient();
        console.log(`openapi: generated TypeScript SDK into ${targets.typescript}`);
    } finally {
        rmSync(out, { recursive: true, force: true });
    }
}
