/**
 * `@tochii/osb-toolkit-shared/errors`
 *
 * osbTK's error layer: it re-exports the shared `@tochii/errors`
 * base, guards and serialization, and adds converter-specific codes and domain
 * classes on top. Import this (not `@tochii/errors` directly) so the code set
 * stays in one place.
 *
 * Type declarations live in `src/types/errors.ts`.
 */

import { isTochiResError, type SerializedError, type TochiResError } from '@tochii/errors';

import { osbTKError } from './errors/base.js';
import { isosbTKErrorCode } from './errors/codes.js';
import type { osbTKErrorOptions } from './types/errors.js';

export {
    assertTochiRes,
    deserializeError,
    ErrorCode,
    hasErrorCode,
    isTochiResError,
    serializeError,
    TochiResError,
    toTochiResError,
} from '@tochii/errors';

export { osbTKError } from './errors/base.js';
export {
    isosbTKErrorCode,
    OSB_TK_ERROR_CODES,
    osbTKCategoryOf,
    osbTKErrorCode,
} from './errors/codes.js';
export {
    BuildError,
    GenerationError,
    InvalidScriptError,
    IOError,
    OptionError,
    ParseError,
    SdkError,
    TranspileError,
    UnsupportedConstructError,
    UnsupportedTargetError,
} from './errors/domains.js';

/**
 * Whether a value is an osbTK error â€” either our own class or any
 * {@link TochiResError} carrying an osbTK code (e.g. after a round-trip).
 */
export function isosbTKError(value: unknown): value is osbTKError {
    if (value instanceof osbTKError) return true;
    return isTochiResError(value) && isosbTKErrorCode((value as TochiResError).code);
}

/** Build a parse error. */
export function parseError(
    message: string,
    options: Partial<Omit<osbTKErrorOptions, 'code' | 'message'>> = {}
): osbTKError {
    return new osbTKError({ code: 'OSB_PARSE_ERROR', message, ...options });
}

/** Build an unsupported-construct error. */
export function unsupportedConstruct(
    message: string,
    options: Partial<Omit<osbTKErrorOptions, 'code' | 'message'>> = {}
): osbTKError {
    return new osbTKError({ code: 'OSB_UNSUPPORTED_CONSTRUCT', message, ...options });
}

/** Serialize an osbTK (or any) error to the shared wire shape. */
export function serializeosbTKError(value: unknown): SerializedError {
    if (value instanceof osbTKError) return value.toJSON();
    return isTochiResError(value) ? (value as TochiResError).toJSON() : ({} as SerializedError);
}
