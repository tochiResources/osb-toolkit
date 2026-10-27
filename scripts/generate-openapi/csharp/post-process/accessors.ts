import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

/**
 * Drops the `*WithHttpInfo` declarations from the generated API interfaces and
 * makes the class implementations protected and `_`-prefixed, so only the clean
 * operations are publicly reachable.
 */
export function protectTransport(apiDir: string): void {
    for (const file of readdirSync(apiDir)) {
        if (!file.endsWith('.cs')) continue;
        const path = join(apiDir, file);
        let text = readFileSync(path, 'utf8');

        // Remove the raw declarations from the generated interfaces.
        text = text.replace(
            /^\s*(?:System\.Threading\.Tasks\.Task<ApiResponse<[^>]+>>|ApiResponse<[^>]+>) \w+WithHttpInfo(?:Async)?\([^;]*\);\s*$\n/gm,
            ''
        );

        for (const match of text.matchAll(
            /public ([\w.]+ApiResponse<[^>]+>) (\w+)WithHttpInfo\(/g
        )) {
            const [, type, name] = match;
            text = text
                .replace(
                    `public ${type} ${name}WithHttpInfo(`,
                    `protected ${type} _${name}WithHttpInfo(`
                )
                .replace(
                    new RegExp(`(?<!_)\\b${name}WithHttpInfo\\(`, 'g'),
                    `_${name}WithHttpInfo(`
                );
        }

        for (const match of text.matchAll(/public async (.*?) (\w+)WithHttpInfoAsync\(/g)) {
            const [, type, name] = match;
            text = text
                .replace(
                    `public async ${type} ${name}WithHttpInfoAsync(`,
                    `protected async ${type} _${name}WithHttpInfoAsync(`
                )
                .replace(
                    new RegExp(`(?<!_)\\b${name}WithHttpInfoAsync\\(`, 'g'),
                    `_${name}WithHttpInfoAsync(`
                );
        }

        writeFileSync(path, text, 'utf8');
    }
}
