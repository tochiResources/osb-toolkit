import type { Diagnostic } from '@tochii/osb-toolkit-shared';

import type { Token } from '../tokenizer/index.js';

export function convertString(
    token: Token,
    diagnostics: Diagnostic[],
    transpile?: (text: string) => string
): string {
    const raw = token.value;
    const inner = raw.slice(1, -1);

    if (token.interpolated) {
        return convertInterpolation(inner, diagnostics, transpile);
    }
    if (token.verbatim) {
        const unescaped = inner.replace(/""/g, '"');
        return `"${unescaped.replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/\r/g, '\\r').replace(/\n/g, '\\n')}"`;
    }
    return raw;
}

function convertInterpolation(
    inner: string,
    diagnostics: Diagnostic[],
    transpile?: (text: string) => string
): string {
    let format = '';
    const args: string[] = [];
    let index = 0;
    while (index < inner.length) {
        const char = inner[index] as string;
        if (char === '\\') {
            format += char;
            format += inner[index + 1] ?? '';
            index += 2;
            continue;
        }
        if (char === '{' && inner[index + 1] === '{') {
            format += '{';
            index += 2;
            continue;
        }
        if (char === '}' && inner[index + 1] === '}') {
            format += '}';
            index += 2;
            continue;
        }
        if (char === '{') {
            let depth = 1;
            let cursor = index + 1;
            let expression = '';
            while (cursor < inner.length && depth > 0) {
                const c = inner[cursor] as string;
                if (c === '{') depth += 1;
                else if (c === '}') {
                    depth -= 1;
                    if (depth === 0) break;
                }
                expression += c;
                cursor += 1;
            }
            const [value, formatSpec] = splitFormatSpec(expression);
            if (formatSpec) {
                diagnostics.push({
                    severity: 'warning',
                    message: `Format specifier ':${formatSpec}' in interpolation was dropped`,
                    code: 'OSB_UNSUPPORTED_CONSTRUCT',
                });
            }
            const trimmed = (value ?? '').trim();
            args.push(transpile ? transpile(trimmed) : trimmed);
            format += '{}';
            index = cursor + 1;
            continue;
        }
        if (char === '"') {
            format += '\\"';
            index += 1;
            continue;
        }
        format += char;
        index += 1;
    }
    return args.length > 0 ? `format!("${format}", ${args.join(', ')})` : `"${format}".to_string()`;
}

/**
 * Splits an interpolation expression from an optional `:format` specifier. The
 * split colon must be at the top level and outside a ternary `?:`, so
 * `{cond ? a : b}` stays a single expression.
 */
function splitFormatSpec(expression: string): [string, string | undefined] {
    let depth = 0;
    let ternary = 0;
    for (let i = 0; i < expression.length; i += 1) {
        const char = expression[i] as string;
        if (char === '(' || char === '[' || char === '{') depth += 1;
        else if (char === ')' || char === ']' || char === '}') depth -= 1;
        else if (depth === 0 && char === '?') {
            if (expression[i + 1] === '.' || expression[i + 1] === '?') i += 1;
            else ternary += 1;
        } else if (depth === 0 && char === ':') {
            if (ternary === 0) return [expression.slice(0, i), expression.slice(i + 1)];
            ternary -= 1;
        }
    }
    return [expression, undefined];
}
