import type { EffectParameter } from '@tochii/osb-toolkit-shared';

import { enumMembers, mapCSharpType } from '../known.js';
import { type RawAttribute, splitTopLevel, unquote } from './attributes.js';

export function buildParameter(input: {
    name: string;
    csType: string;
    attributes: RawAttribute[];
    initializer?: string;
    order: number;
}): EffectParameter {
    const group = input.attributes.find(attr => attr.name === 'Group')?.args[0];
    const description = input.attributes.find(attr => attr.name === 'Description')?.args[0];
    const type = mapCSharpType(input.csType);
    const parsed = input.initializer
        ? parseDefaultValue(input.initializer, input.csType)
        : undefined;
    const members = enumMembers(input.csType);

    return {
        name: input.name,
        csType: input.csType,
        type,
        defaultValue: parsed?.value,
        defaultValueLiteral: input.initializer?.trim(),
        group: group ? unquote(group) : undefined,
        description: description ? unquote(description) : undefined,
        enumValues: members ? members.map(member => ({ name: member })) : undefined,
        order: input.order,
    };
}

export function parseDefaultValue(
    initializer: string,
    csType: string
): { value: unknown } | undefined {
    const trimmed = initializer.trim();
    if (!trimmed) return undefined;

    if (trimmed === 'true') return { value: true };
    if (trimmed === 'false') return { value: false };
    if (trimmed === 'null') return { value: null };

    const numberMatch = trimmed.match(/^(-?\d+(?:\.\d+)?)(?:[fFdDmMlLuU]+)?$/);
    if (numberMatch?.[1] !== undefined) {
        return { value: Number(numberMatch[1]) };
    }

    if (
        (trimmed.startsWith('"') && trimmed.endsWith('"')) ||
        (trimmed.startsWith("'") && trimmed.endsWith("'"))
    ) {
        return { value: unquote(trimmed) };
    }

    const vectorMatch = trimmed.match(/^new\s*Vector[23]\s*\(([^)]*)\)$/);
    if (vectorMatch?.[1] !== undefined) {
        const [x, y] = splitTopLevel(vectorMatch[1], ',').map(part => Number(part.trim()));
        return { value: { x: x ?? 0, y: y ?? 0 } };
    }

    const colorMatch = trimmed.match(/^CommandColor\.(FromHsb|FromRgb)\s*\((.*)\)$/);
    if (colorMatch?.[2] !== undefined) {
        const parts = splitTopLevel(colorMatch[2], ',').map(part => Number(part.trim()));
        return { value: { mode: colorMatch[1], values: parts } };
    }

    const enumMatch = trimmed.match(/^([A-Za-z_][A-Za-z0-9_]*)\.([A-Za-z_][A-Za-z0-9_]*)$/);
    if (enumMatch?.[2] !== undefined && enumMembers(enumMatch[1] ?? '')) {
        return { value: enumMatch[2] };
    }

    return undefined;
}
