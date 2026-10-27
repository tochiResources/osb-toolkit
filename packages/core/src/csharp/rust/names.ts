import { FLOAT_32, INTEGER_32, INTEGER_64 } from './maps.js';

/** PascalCase/camelCase -> snake_case, matching the generated struct fields. */
export function rustSnakeCase(name: string): string {
    return name
        .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
        .replace(/([A-Z]+)([A-Z][a-z])/g, '$1_$2')
        .toLowerCase();
}

/** PascalCase -> SCREAMING_SNAKE_CASE, matching the runtime `Color4` constants. */
export function screamingSnake(name: string): string {
    return rustSnakeCase(name).toUpperCase();
}

const ASSIGNMENT_OPERATORS = new Set(['=', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '??=']);

/** Whether the token after a configurable reference makes it an assignment target. */
export function isAssignmentTarget(next: string | undefined): boolean {
    return next !== undefined && ASSIGNMENT_OPERATORS.has(next);
}

/**
 * A C# cast target expressed in the generated Rust numeric model: all scalar
 * numerics are `f64`, so an integer cast truncates and `float`/`double` widen
 * to f64 (matching the runtime's `Vec2`/`Color4` components).
 */
export function rustCastType(castType: string): { type: string; truncate?: boolean } | undefined {
    if (INTEGER_32.has(castType) || INTEGER_64.has(castType))
        return { type: 'f64', truncate: true };
    if (castType === 'double') return { type: 'f64' };
    if (FLOAT_32.has(castType)) return { type: 'f64' };
    return undefined;
}
