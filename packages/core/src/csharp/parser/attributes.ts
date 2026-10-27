export interface RawAttribute {
    name: string;
    args: string[];
}

export function parseAttributes(text: string): RawAttribute[] {
    const result: RawAttribute[] = [];
    const segments = splitTopLevel(text, ',');
    for (const segment of segments) {
        const trimmed = segment.trim();
        if (!trimmed) continue;
        const parenIndex = trimmed.indexOf('(');
        if (parenIndex === -1) {
            result.push({ name: trimmed, args: [] });
            continue;
        }
        const name = trimmed.slice(0, parenIndex).trim();
        const closeIndex = trimmed.lastIndexOf(')');
        const inner = trimmed.slice(parenIndex + 1, closeIndex === -1 ? undefined : closeIndex);
        result.push({ name, args: splitTopLevel(inner, ',').map(arg => arg.trim()) });
    }
    return result;
}

export function splitTopLevel(text: string, delimiter: string): string[] {
    const parts: string[] = [];
    let depth = 0;
    let current = '';
    let inString: string | null = null;
    for (let i = 0; i < text.length; i += 1) {
        const char = text[i] as string;
        if (inString) {
            current += char;
            if (char === '\\') {
                current += text[i + 1] ?? '';
                i += 1;
            } else if (char === inString) {
                inString = null;
            }
            continue;
        }
        if (char === '"' || char === "'") {
            inString = char;
            current += char;
            continue;
        }
        if (char === '(' || char === '[' || char === '<') depth += 1;
        else if (char === ')' || char === ']' || char === '>') depth -= 1;
        if (char === delimiter && depth === 0) {
            parts.push(current);
            current = '';
            continue;
        }
        current += char;
    }
    parts.push(current);
    return parts;
}

export function unquote(value: string): string {
    const trimmed = value.trim();
    if (
        (trimmed.startsWith('"') && trimmed.endsWith('"')) ||
        (trimmed.startsWith("'") && trimmed.endsWith("'"))
    ) {
        return trimmed
            .slice(1, -1)
            .replace(/\\"/g, '"')
            .replace(/\\n/g, '\n')
            .replace(/\\t/g, '\t');
    }
    return trimmed;
}
