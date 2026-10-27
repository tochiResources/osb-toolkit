import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

/**
 * Makes the generated `<op>WithHttpInfo` raw helpers protected and `_`-prefixed,
 * leaving only the clean `<op>` method public.
 */
export function protectTransport(apiDir: string): void {
    for (const file of readdirSync(apiDir)) {
        if (!file.endsWith('.java')) continue;
        const path = join(apiDir, file);
        let text = readFileSync(path, 'utf8');
        for (const match of text.matchAll(
            /public ApiResponse<([^>]+)> (\w+)WithHttpInfo\(/g
        )) {
            const [, type, name] = match;
            text = text
                .replaceAll(
                    `public ApiResponse<${type}> ${name}WithHttpInfo(`,
                    `protected ApiResponse<${type}> _${name}WithHttpInfo(`
                )
                .replace(
                    new RegExp(`(?<!_)\\b${name}WithHttpInfo\\(`, 'g'),
                    `_${name}WithHttpInfo(`
                );
        }
        writeFileSync(path, text, 'utf8');
    }
}
