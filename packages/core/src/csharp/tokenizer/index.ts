import { osbTKError } from '@tochii/osb-toolkit-shared';

import type { Token, TokenType } from '../../types/csharp.js';
import {
    CSHARP_KEYWORDS,
    MULTI_CHAR_OPERATORS,
    PUNCTUATION,
    SINGLE_CHAR_OPERATORS,
} from './keywords.js';
import { readString } from './strings.js';

export type { Token, TokenType } from '../../types/csharp.js';
export { CSHARP_KEYWORDS } from './keywords.js';

export function tokenizeCSharp(source: string, file?: string): Token[] {
    // C# sources are commonly saved with a UTF-8 BOM; strip it so it is not
    // mistaken for an unexpected character on the first line.
    if (source.charCodeAt(0) === 0xfeff) source = source.slice(1);

    const tokens: Token[] = [];
    let index = 0;
    let line = 1;
    let column = 1;
    const length = source.length;

    const push = (
        type: TokenType,
        value: string,
        startLine: number,
        startColumn: number,
        extra: Partial<Token> = {}
    ) => {
        tokens.push({ type, value, line: startLine, column: startColumn, ...extra });
    };

    while (index < length) {
        const startLine = line;
        const startColumn = column;
        const char = source[index] as string;

        if (char === '\r') {
            if (source[index + 1] === '\n') {
                push('newline', '\r\n', startLine, startColumn);
                index += 2;
            } else {
                push('newline', '\r', startLine, startColumn);
                index += 1;
            }
            line += 1;
            column = 1;
            continue;
        }

        if (char === '\n') {
            push('newline', '\n', startLine, startColumn);
            index += 1;
            line += 1;
            column = 1;
            continue;
        }

        if (char === ' ' || char === '\t') {
            let value = '';
            while (index < length && (source[index] === ' ' || source[index] === '\t')) {
                value += source[index];
                index += 1;
                column += 1;
            }
            push('whitespace', value, startLine, startColumn);
            continue;
        }

        if (char === '/' && source[index + 1] === '/') {
            let value = '';
            while (index < length && source[index] !== '\n' && source[index] !== '\r') {
                value += source[index];
                index += 1;
                column += 1;
            }
            push('comment', value, startLine, startColumn);
            continue;
        }

        if (char === '/' && source[index + 1] === '*') {
            let value = '/*';
            index += 2;
            column += 2;
            while (index < length && !(source[index] === '*' && source[index + 1] === '/')) {
                if (source[index] === '\n') {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
                value += source[index];
                index += 1;
            }
            value += '*/';
            index += 2;
            column += 2;
            push('comment', value, startLine, startColumn);
            continue;
        }

        let verbatim = false;
        let interpolated = false;
        if (char === '@' && source[index + 1] === '"') {
            verbatim = true;
            index += 1;
            column += 1;
        } else if (char === '$' && source[index + 1] === '"') {
            interpolated = true;
            index += 1;
            column += 1;
        } else if (char === '$' && source[index + 1] === '@' && source[index + 2] === '"') {
            interpolated = true;
            verbatim = true;
            index += 2;
            column += 2;
        } else if (char === '@' && source[index + 1] === '$' && source[index + 2] === '"') {
            interpolated = true;
            verbatim = true;
            index += 2;
            column += 2;
        }

        if (verbatim || interpolated) {
            const { value, nextIndex, nextLine, nextColumn } = readString(
                source,
                index,
                verbatim,
                interpolated,
                line,
                column,
                file
            );
            index = nextIndex;
            line = nextLine;
            column = nextColumn;
            push('string', value, startLine, startColumn, { verbatim, interpolated });
            continue;
        }

        if (char === '"') {
            const { value, nextIndex, nextLine, nextColumn } = readString(
                source,
                index,
                false,
                false,
                line,
                column,
                file
            );
            index = nextIndex;
            line = nextLine;
            column = nextColumn;
            push('string', value, startLine, startColumn);
            continue;
        }

        if (char === "'") {
            let value = "'";
            index += 1;
            column += 1;
            while (index < length && source[index] !== "'") {
                if (source[index] === '\\') {
                    value += source[index] as string;
                    index += 1;
                    column += 1;
                }
                value += source[index] as string;
                index += 1;
                column += 1;
            }
            value += "'";
            index += 1;
            column += 1;
            push('char', value, startLine, startColumn);
            continue;
        }

        if (isDigit(char) || (char === '.' && isDigit(source[index + 1] ?? ''))) {
            let value = '';
            while (index < length && /[0-9a-fA-FxX._]/.test(source[index] as string)) {
                value += source[index];
                index += 1;
                column += 1;
            }
            while (index < length && /[fFdDmMlLuU]/.test(source[index] as string)) {
                value += source[index];
                index += 1;
                column += 1;
            }
            push('number', value, startLine, startColumn);
            continue;
        }

        if (isIdentifierStart(char)) {
            let value = '';
            if (char === '@') {
                value += '@';
                index += 1;
                column += 1;
            }
            while (index < length && isIdentifierPart(source[index] as string)) {
                value += source[index];
                index += 1;
                column += 1;
            }
            const bare = value.startsWith('@') ? value.slice(1) : value;
            push(
                CSHARP_KEYWORDS.has(bare) ? 'keyword' : 'identifier',
                value,
                startLine,
                startColumn
            );
            continue;
        }

        const three = source.slice(index, index + 3);
        const two = source.slice(index, index + 2);
        const multi = MULTI_CHAR_OPERATORS.find(op =>
            op.length === 3 ? op === three : op === two
        );
        if (multi) {
            push('operator', multi, startLine, startColumn);
            index += multi.length;
            column += multi.length;
            continue;
        }

        if (SINGLE_CHAR_OPERATORS.has(char)) {
            push('operator', char, startLine, startColumn);
            index += 1;
            column += 1;
            continue;
        }

        if (PUNCTUATION.has(char)) {
            push('punctuation', char, startLine, startColumn);
            index += 1;
            column += 1;
            continue;
        }

        // Preprocessor directives (`#region`, `#if`, ...) carry no runtime meaning
        // for the generated effect, so drop the whole line.
        if (char === '#') {
            let end = index;
            while (end < length && source[end] !== '\n' && source[end] !== '\r') end += 1;
            column += end - index;
            index = end;
            continue;
        }

        throw new osbTKError({
            code: 'OSB_PARSE_ERROR',
            message: `Unexpected character ${JSON.stringify(char)}`,
            line: startLine,
            column: startColumn,
            file,
        });
    }

    push('eof', '', line, column);
    return tokens;
}

function isDigit(char: string): boolean {
    return char >= '0' && char <= '9';
}

function isIdentifierStart(char: string): boolean {
    return /[A-Za-z_@]/.test(char);
}

function isIdentifierPart(char: string): boolean {
    return /[A-Za-z0-9_]/.test(char);
}
