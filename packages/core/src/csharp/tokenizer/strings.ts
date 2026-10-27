import { osbTKError } from '@tochii/osb-toolkit-shared';

/**
 * Reads a string literal starting at `startIndex` (the opening `"`), honouring
 * verbatim (`@"..."`) and interpolated (`$"..."`) rules.
 */
export function readString(
    source: string,
    startIndex: number,
    verbatim: boolean,
    interpolated: boolean,
    startLine: number,
    startColumn: number,
    file?: string
): { value: string; nextIndex: number; nextLine: number; nextColumn: number } {
    let index = startIndex + 1;
    let line = startLine;
    let column = startColumn + 1;
    let value = '"';
    const length = source.length;

    while (index < length) {
        const char = source[index] as string;
        if (char === '"') {
            if (verbatim && source[index + 1] === '"') {
                value += '""';
                index += 2;
                column += 2;
                continue;
            }
            value += '"';
            index += 1;
            column += 1;
            return { value, nextIndex: index, nextLine: line, nextColumn: column };
        }
        if (char === '\\' && !verbatim) {
            value += char;
            value += source[index + 1] ?? '';
            index += 2;
            column += 2;
            continue;
        }
        if (char === '\n') {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
        value += char;
        index += 1;
    }

    throw new osbTKError({
        code: 'OSB_PARSE_ERROR',
        message: 'Unterminated string literal',
        line: startLine,
        column: startColumn,
        file,
    });
}
