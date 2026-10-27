import { type ErrorCode, type SerializedError, TochiResError } from '@tochii/errors';

import type { osbTKErrorOptions } from '../types/errors.js';
import { type osbTKCategory, osbTKCategoryOf, type osbTKErrorCode } from './codes.js';

/**
 * Base class for every osbTK-specific error.
 *
 * Extends the shared {@link TochiResError} so guards, serialization and the C#
 * mirror keep working, while exposing the osbTK code/category
 * directly (the shared `categoryOf` does not know these codes).
 */
export class osbTKError extends TochiResError {
    /** The osbTK-specific code. */
    readonly osbCode: osbTKErrorCode;
    /** The osbTK-specific category. */
    readonly osbCategory: osbTKCategory;
    readonly detail?: string;
    readonly line?: number;
    readonly column?: number;
    readonly file?: string;

    constructor(options: osbTKErrorOptions) {
        // The code is outside the shared union by design; the base stores it verbatim.
        super(options.code as unknown as ErrorCode, options.message, options);
        this.name = 'osbTKError';
        this.osbCode = options.code;
        this.osbCategory = osbTKCategoryOf(options.code);
        this.detail = options.detail;
        this.line = options.line;
        this.column = options.column;
        this.file = options.file;
    }

    override toJSON(): SerializedError & {
        osbCode: osbTKErrorCode;
        osbCategory: osbTKCategory;
        detail?: string;
        line?: number;
        column?: number;
        file?: string;
    } {
        return {
            ...super.toJSON(),
            osbCode: this.osbCode,
            osbCategory: this.osbCategory,
            detail: this.detail,
            line: this.line,
            column: this.column,
            file: this.file,
        };
    }
}
