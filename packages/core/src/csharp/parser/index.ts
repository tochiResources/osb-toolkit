import {
    type Diagnostic,
    type EffectField,
    type EffectMethod,
    type EffectParameter,
    type EffectScript,
    osbTKError,
} from '@tochii/osb-toolkit-shared';

import type { ParseOptions, Token } from '../../types/csharp.js';
import { enumMembers, mapCSharpType } from '../known.js';
import { tokenizeCSharp } from '../tokenizer/index.js';
import { parseAttributes, type RawAttribute } from './attributes.js';
import { buildParameter } from './values.js';

export { parseDefaultValue } from './values.js';

interface SignificantToken {
    token: Token;
    index: number;
}

const MODIFIERS = new Set([
    'public',
    'private',
    'protected',
    'internal',
    'static',
    'readonly',
    'const',
    'virtual',
    'override',
    'abstract',
    'sealed',
    'new',
    'async',
    'extern',
    'unsafe',
    'partial',
]);

const UNSUPPORTED_METHOD_KEYWORDS = ['switch', 'try', 'lock', 'unsafe', 'fixed', 'goto', 'yield'];

export function parseEffectScript(source: string, options: ParseOptions = {}): EffectScript {
    const tokens = tokenizeCSharp(source, options.file);
    const sig: SignificantToken[] = tokens
        .map((token, index) => ({ token, index }))
        .filter(
            entry =>
                entry.token.type !== 'whitespace' &&
                entry.token.type !== 'newline' &&
                entry.token.type !== 'comment'
        );

    const diagnostics: Diagnostic[] = [];
    let pos = 0;

    const usingDirectives: string[] = [];
    while (sig[pos]?.token.value === 'using') {
        const collected: string[] = [];
        pos += 1;
        while (pos < sig.length && sig[pos]?.token.value !== ';') {
            collected.push(sig[pos]?.token.value as string);
            pos += 1;
        }
        usingDirectives.push(collected.join('').trim());
        pos += 1;
    }

    let namespace: string | undefined;
    if (sig[pos]?.token.value === 'namespace') {
        const collected: string[] = [];
        pos += 1;
        while (pos < sig.length && sig[pos]?.token.value !== '{') {
            collected.push(sig[pos]?.token.value as string);
            pos += 1;
        }
        namespace = collected.join('').trim();
        pos += 1;
    }

    while (pos < sig.length && sig[pos]?.token.value !== 'class') {
        pos += 1;
    }
    if (pos >= sig.length) {
        throw new osbTKError({
            code: 'OSB_PARSE_ERROR',
            message: 'No class declaration found in effect script',
            file: options.file,
        });
    }

    pos += 1;
    const className = sig[pos]?.token.value;
    if (!className) {
        throw new osbTKError({
            code: 'OSB_PARSE_ERROR',
            message: 'Class declaration is missing a name',
            file: options.file,
        });
    }
    pos += 1;

    let baseClass = 'StoryboardObjectGenerator';
    if (sig[pos]?.token.value === ':') {
        const collected: string[] = [];
        pos += 1;
        while (pos < sig.length && sig[pos]?.token.value !== '{') {
            collected.push(sig[pos]?.token.value as string);
            pos += 1;
        }
        baseClass = collected.join('').trim().split(',')[0]?.trim() || baseClass;
    }

    if (sig[pos]?.token.value !== '{') {
        throw new osbTKError({
            code: 'OSB_PARSE_ERROR',
            message: 'Expected class body',
            file: options.file,
        });
    }
    pos += 1;

    const fields: EffectField[] = [];
    const parameters: EffectParameter[] = [];
    const methods: EffectMethod[] = [];
    let generateBody = '';
    let generateLine: number | undefined;

    const readBalancedText = (openPos: number): { text: string; closePos: number } => {
        const open = sig[openPos] as SignificantToken;
        const openChar = open.token.value;
        const closeChar = openChar === '{' ? '}' : openChar === '(' ? ')' : ']';
        let depth = 0;
        let i = openPos;
        for (; i < sig.length; i += 1) {
            const value = sig[i]?.token.value;
            if (value === openChar) depth += 1;
            else if (value === closeChar) {
                depth -= 1;
                if (depth === 0) break;
            }
        }
        const close = sig[i] as SignificantToken;
        const text = tokens
            .slice(open.index + 1, close.index)
            .map(token => token.value)
            .join('');
        return { text, closePos: i };
    };

    const readHead = (
        startPos: number
    ): { headTokens: Token[]; terminator: string; terminatorPos: number } => {
        const headTokens: Token[] = [];
        let i = startPos;
        let depth = 0;
        while (i < sig.length) {
            const entry = sig[i] as SignificantToken;
            const value = entry.token.value;
            if (
                depth === 0 &&
                (value === '=' || value === ';' || value === '(' || value === '{' || value === '=>')
            ) {
                return { headTokens, terminator: value, terminatorPos: i };
            }
            if (value === '<' || value === '[' || value === '(') depth += 1;
            else if (value === '>' || value === ']' || value === ')')
                depth = Math.max(0, depth - 1);
            headTokens.push(entry.token);
            i += 1;
        }
        return { headTokens, terminator: '', terminatorPos: i };
    };

    while (pos < sig.length && sig[pos]?.token.value !== '}') {
        const attributes: RawAttribute[] = [];
        while (sig[pos]?.token.value === '[') {
            const { text, closePos } = readBalancedText(pos);
            attributes.push(...parseAttributes(text));
            pos = closePos + 1;
        }

        const modifiers: string[] = [];
        while (sig[pos] && MODIFIERS.has(sig[pos]?.token.value as string)) {
            modifiers.push(sig[pos]?.token.value as string);
            pos += 1;
        }

        if (sig[pos]?.token.value === '}') break;
        if (!sig[pos]) break;

        if (
            sig[pos]?.token.value === 'class' ||
            sig[pos]?.token.value === 'enum' ||
            sig[pos]?.token.value === 'struct' ||
            sig[pos]?.token.value === 'interface'
        ) {
            const bracePos = findNext(sig, pos, '{');
            if (bracePos < 0) break;
            pos = readBalancedText(bracePos).closePos + 1;
            continue;
        }

        const { headTokens, terminator, terminatorPos } = readHead(pos);
        if (headTokens.length === 0) {
            pos += 1;
            continue;
        }

        const nameToken = [...headTokens]
            .reverse()
            .find(token => token.type === 'identifier' || token.type === 'keyword');
        if (!nameToken) {
            pos = terminatorPos + 1;
            continue;
        }
        const nameIndex = headTokens.lastIndexOf(nameToken);
        const csType = headTokens
            .slice(0, nameIndex)
            .map(token => token.value)
            .join('')
            .trim();

        if (terminator === '(') {
            const { text: paramsText, closePos: paramsClose } = readBalancedText(terminatorPos);
            const afterParams = paramsClose + 1;
            let body = '';
            let nextPos = afterParams;
            const bodyToken = sig[afterParams]?.token.value;
            if (bodyToken === '{') {
                const balanced = readBalancedText(afterParams);
                body = balanced.text;
                nextPos = balanced.closePos + 1;
            } else if (bodyToken === '=>') {
                const collected: string[] = [];
                let i = afterParams + 1;
                while (i < sig.length && sig[i]?.token.value !== ';') {
                    collected.push(sig[i]?.token.value as string);
                    i += 1;
                }
                body = collected.join('');
                nextPos = i + 1;
            }
            const method: EffectMethod = {
                name: nameToken.value,
                signature: `${csType} ${nameToken.value}(${paramsText.trim()})`,
                body,
                isOverride: modifiers.includes('override'),
            };
            methods.push(method);
            if (nameToken.value === 'Generate') {
                generateBody = body;
                generateLine = nameToken.line;
            }
            for (const keyword of UNSUPPORTED_METHOD_KEYWORDS) {
                if (new RegExp(`\\b${keyword}\\b`).test(body)) {
                    diagnostics.push({
                        severity: 'warning',
                        message: `Method ${nameToken.value} uses unsupported C# construct '${keyword}'`,
                        line: nameToken.line,
                        code: 'OSB_UNSUPPORTED_CONSTRUCT',
                    });
                }
            }
            pos = nextPos;
            continue;
        }

        if (terminator === '=') {
            const collected: string[] = [];
            let i = terminatorPos + 1;
            let depth = 0;
            while (i < sig.length) {
                const value = sig[i]?.token.value;
                if (value === ';' && depth === 0) break;
                if (value === '(' || value === '{' || value === '[') depth += 1;
                else if (value === ')' || value === '}' || value === ']') depth -= 1;
                collected.push(value as string);
                i += 1;
            }
            const initializer = collected.join('');
            fields.push({ name: nameToken.value, csType, modifiers, initializer });

            const configurable = attributes.find(attr => attr.name === 'Configurable');
            if (configurable) {
                parameters.push(
                    buildParameter({
                        name: nameToken.value,
                        csType,
                        attributes,
                        initializer,
                        order: parameters.length,
                    })
                );
            }
            pos = i + 1;
            continue;
        }

        if (terminator === ';') {
            fields.push({ name: nameToken.value, csType, modifiers });
            const configurable = attributes.find(attr => attr.name === 'Configurable');
            if (configurable) {
                parameters.push(
                    buildParameter({
                        name: nameToken.value,
                        csType,
                        attributes,
                        initializer: undefined,
                        order: parameters.length,
                    })
                );
            }
            pos = terminatorPos + 1;
            continue;
        }

        if (terminator === '{') {
            const { closePos } = readBalancedText(terminatorPos);
            pos = closePos + 1;
            if (sig[pos]?.token.value === '=') {
                while (pos < sig.length && sig[pos]?.token.value !== ';') pos += 1;
                pos += 1;
            }
            continue;
        }

        if (terminator === '=>') {
            while (pos < sig.length && sig[pos]?.token.value !== ';') pos += 1;
            pos += 1;
            continue;
        }

        pos = terminatorPos + 1;
    }

    if (!generateBody && options.requireGenerate !== false) {
        throw new osbTKError({
            code: 'OSB_INVALID_SCRIPT',
            message: `Class ${className} does not define a Generate() method`,
            file: options.file,
        });
    }

    return {
        className,
        namespace,
        baseClass,
        sourceFile: options.file,
        usingDirectives,
        fields,
        parameters,
        methods,
        generateBody,
        generateLine,
        diagnostics,
    };
}

function findNext(sig: SignificantToken[], startPos: number, value: string): number {
    for (let i = startPos; i < sig.length; i += 1) {
        if (sig[i]?.token.value === value) return i;
    }
    return -1;
}
