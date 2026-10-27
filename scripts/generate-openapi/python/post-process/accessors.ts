import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

/** Prefixes the generated `_with_http_info`/`_without_preload_content` helpers. */
export function protectTransport(apiDir: string): void {
    for (const file of readdirSync(apiDir)) {
        if (!file.endsWith('.py') || file === '__init__.py') continue;
        const path = join(apiDir, file);
        let text = readFileSync(path, 'utf8');
        for (const suffix of ['_with_http_info', '_without_preload_content']) {
            for (const match of text.matchAll(new RegExp(`def (\\w+)${suffix}\\(`, 'g'))) {
                const name = match[1];
                text = text
                    .replaceAll(`def ${name}${suffix}(`, `def _${name}${suffix}(`)
                    .replaceAll(`self.${name}${suffix}(`, `self._${name}${suffix}(`);
            }
        }
        writeFileSync(path, text, 'utf8');
    }
}
