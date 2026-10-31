import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

/**
 * Rebases the generated `ApiException` on the shared `TochiRes.Errors`
 * exception, so every C# SDK error participates in the same code/category
 * contract as the TypeScript side.
 */
export function extendCSharpErrors(clientDir: string): void {
    const path = join(clientDir, 'ApiException.cs');
    let text = readFileSync(path, 'utf8');
    if (!text.includes('public class ApiException : Exception')) {
        throw new Error(`openapi: could not find the generated ApiException in ${path}`);
    }

    text = text
        .replace('using System;', 'using System;\n\nusing TochiRes.Errors;')
        .replace(
            'public class ApiException : Exception',
            'public class ApiException : TochiResException'
        )
        .replace(
            'public ApiException() { }',
            'public ApiException() : base(ErrorCodes.ApiBadRequest, "API request failed") { }'
        )
        .replace(
            'public ApiException(int errorCode, string message) : base(message)',
            'public ApiException(int errorCode, string message) : base(ErrorCodes.ApiBadRequest, message)'
        )
        .replace(
            'public ApiException(int errorCode, string message, object errorContent = null, Multimap<string, string> headers = null) : base(message)',
            'public ApiException(int errorCode, string message, object errorContent = null, Multimap<string, string> headers = null) : base(ErrorCodes.ApiBadRequest, message)'
        );

    writeFileSync(path, text, 'utf8');
}
