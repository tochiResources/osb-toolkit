import type { TochiResErrorOptions } from '@tochii/errors';

export type { SerializedError, TochiResErrorOptions } from '@tochii/errors';

/** Options accepted by every osbTK error. */
export interface osbTKErrorOptions extends TochiResErrorOptions {
    /** The osbTK-specific code. */
    code: import('../errors/codes.js').osbTKErrorCode;
    /** Human-readable message. */
    message: string;
    /** Extra, human-readable detail (e.g. the offending token). */
    detail?: string;
    /** 1-based source line, when the error is tied to a location. */
    line?: number;
    /** 1-based source column, when the error is tied to a location. */
    column?: number;
    /** Source file the error refers to. */
    file?: string;
}

/** Options accepted by the domain errors (the code and message are supplied by the class). */
export type osbTKDomainErrorOptions = Omit<osbTKErrorOptions, 'code' | 'message'>;
