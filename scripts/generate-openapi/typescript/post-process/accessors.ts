import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

/**
 * Makes the generated transport plumbing non-public: the per-class
 * `<op>Raw` / `<op>RequestOpts` helpers and the base `request` / `isJsonMime`
 * become protected and `_`-prefixed, leaving only the clean `<op>` method
 * public.
 */
export function protectTransport(apiDir: string): void {
    for (const file of readdirSync(apiDir)) {
        if (!file.endsWith('.ts')) continue;
        const path = join(apiDir, file);
        let text = readFileSync(path, 'utf8');
        for (const match of text.matchAll(/async (\w+)Raw\(/g)) {
            const name = match[1];
            text = text
                .replaceAll(`async ${name}Raw(`, `protected async _${name}Raw(`)
                .replaceAll(`async ${name}RequestOpts(`, `protected async _${name}RequestOpts(`)
                .replaceAll(`this.${name}Raw(`, `this._${name}Raw(`)
                .replaceAll(`this.${name}RequestOpts(`, `this._${name}RequestOpts(`);
        }
        text = text.replaceAll('this.request(', 'this._request(');
        writeFileSync(path, text, 'utf8');
    }

    const runtimePath = join(apiDir, '..', 'runtime.ts');
    let runtime = readFileSync(runtimePath, 'utf8');
    runtime = runtime
        .replaceAll('protected async request(', 'protected async _request(')
        .replaceAll('this.request(', 'this._request(')
        .replaceAll('protected isJsonMime(', 'protected _isJsonMime(')
        .replaceAll('this.isJsonMime(', 'this._isJsonMime(');
    writeFileSync(runtimePath, runtime, 'utf8');
}
