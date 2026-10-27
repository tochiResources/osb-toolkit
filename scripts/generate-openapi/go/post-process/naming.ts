import { readdirSync, readFileSync, renameSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

/**
 * Removes the generated `Api` prefix from the Go request wrappers and `api_`
 * filename prefix, and unexports the raw `<op>Execute` service helpers.
 */
export function cleanGoNames(dir: string): void {
    for (const file of readdirSync(dir)) {
        if (!file.startsWith('api_') || !file.endsWith('.go')) continue;
        const path = join(dir, file);
        let text = readFileSync(path, 'utf8');
        const cls = text.match(/^type (\w+)Service service$/m)?.[1];
        if (cls) {
            text = text.replace(/\bApi(\w+?)Request\b/g, `${cls}$1Request`);
            text = text.replace(
                new RegExp(`func \\(a \\*${cls}Service\\) (\\w+)Execute\\(`, 'g'),
                (_match, name: string) =>
                    `func (a *${cls}Service) ${name.charAt(0).toLowerCase()}${name.slice(1)}Execute(`
            );
            text = text.replace(/ApiService\.(\w+)Execute\(/g, (_match, name: string) => {
                const lowered = `${name.charAt(0).toLowerCase()}${name.slice(1)}`;
                return `ApiService.${lowered}Execute(`;
            });
        }
        writeFileSync(path, text, 'utf8');
        renameSync(path, join(dir, file.slice('api_'.length)));
    }
}
