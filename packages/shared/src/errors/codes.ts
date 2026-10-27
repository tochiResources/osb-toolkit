/**
 * osbTK error codes.
 *
 * These extend the shared `@tochii/errors` contract *locally* (the same
 * integration pattern as `@tochii/aetoosb`): they are converter-specific, use the
 * `OSB_` prefix, and are not registered in the shared `ErrorCode` map, so they are
 * classified by {@link osbTKCategoryOf}. The shared base class, guards and
 * serialization are reused unchanged â€” these codes ride along inside them.
 *
 * A code is part of the contract: never rename or reuse one.
 */
export const osbTKErrorCode = {
    /** The effect script could not be parsed. */
    ParseFailed: 'OSB_PARSE_ERROR',
    /** The script is syntactically valid but not a usable effect script. */
    InvalidScript: 'OSB_INVALID_SCRIPT',
    /** A C# construct has no supported transpilation. */
    UnsupportedConstruct: 'OSB_UNSUPPORTED_CONSTRUCT',
    /** The C# and JS transpilation failed. */
    TranspileFailed: 'OSB_TRANSPILE_ERROR',

    /** A target plug-in could not be generated. */
    GenerationFailed: 'OSB_GENERATION_ERROR',
    /** The requested target is unknown or unsupported. */
    UnsupportedTarget: 'OSB_UNSUPPORTED_TARGET',

    /** An option passed to the converter is invalid. */
    InvalidOption: 'OSB_INVALID_OPTION',

    /** A native plug-in build failed. */
    BuildFailed: 'OSB_BUILD_ERROR',
    /** A required host SDK could not be located. */
    SdkNotFound: 'OSB_SDK_NOT_FOUND',

    /** A filesystem operation failed. */
    IOFailed: 'OSB_IO_ERROR',
} as const;

/** Union of every osbTK-specific code. */
export type osbTKErrorCode = (typeof osbTKErrorCode)[keyof typeof osbTKErrorCode];

/** Coarse grouping used for reporting and error classes. */
export type osbTKCategory = 'parse' | 'transpile' | 'generate' | 'option' | 'build' | 'sdk' | 'io';

const CATEGORY_BY_CODE: Record<osbTKErrorCode, osbTKCategory> = {
    [osbTKErrorCode.ParseFailed]: 'parse',
    [osbTKErrorCode.InvalidScript]: 'parse',
    [osbTKErrorCode.UnsupportedConstruct]: 'transpile',
    [osbTKErrorCode.TranspileFailed]: 'transpile',

    [osbTKErrorCode.GenerationFailed]: 'generate',
    [osbTKErrorCode.UnsupportedTarget]: 'generate',

    [osbTKErrorCode.InvalidOption]: 'option',

    [osbTKErrorCode.BuildFailed]: 'build',
    [osbTKErrorCode.SdkNotFound]: 'sdk',

    [osbTKErrorCode.IOFailed]: 'io',
};

/** All osbTK codes, as an array â€” handy for exhaustiveness checks and docs. */
export const OSB_TK_ERROR_CODES = Object.values(osbTKErrorCode) as osbTKErrorCode[];

/** Whether a value is one of the known osbTK codes. */
export function isosbTKErrorCode(value: unknown): value is osbTKErrorCode {
    return typeof value === 'string' && Object.hasOwn(CATEGORY_BY_CODE, value);
}

/** The category a code belongs to. */
export function osbTKCategoryOf(code: osbTKErrorCode): osbTKCategory {
    return CATEGORY_BY_CODE[code];
}
