import type { osbTKDomainErrorOptions } from '../types/errors.js';
import { osbTKError } from './base.js';
import { osbTKErrorCode } from './codes.js';

/** The effect script could not be parsed. */
export class ParseError extends osbTKError {
    constructor(message: string, options: osbTKDomainErrorOptions = {}) {
        super({ code: osbTKErrorCode.ParseFailed, message, ...options });
        this.name = 'ParseError';
    }
}

/** The script is syntactically valid but not a usable effect script. */
export class InvalidScriptError extends osbTKError {
    constructor(message: string, options: osbTKDomainErrorOptions = {}) {
        super({ code: osbTKErrorCode.InvalidScript, message, ...options });
        this.name = 'InvalidScriptError';
    }
}

/** A C# construct has no supported transpilation. */
export class UnsupportedConstructError extends osbTKError {
    constructor(message: string, options: osbTKDomainErrorOptions = {}) {
        super({ code: osbTKErrorCode.UnsupportedConstruct, message, ...options });
        this.name = 'UnsupportedConstructError';
    }
}

/** The C# and JS transpilation failed. */
export class TranspileError extends osbTKError {
    constructor(message: string, options: osbTKDomainErrorOptions = {}) {
        super({ code: osbTKErrorCode.TranspileFailed, message, ...options });
        this.name = 'TranspileError';
    }
}

/** A target plug-in could not be generated. */
export class GenerationError extends osbTKError {
    constructor(message: string, options: osbTKDomainErrorOptions = {}) {
        super({ code: osbTKErrorCode.GenerationFailed, message, ...options });
        this.name = 'GenerationError';
    }
}

/** The requested target is unknown or unsupported. */
export class UnsupportedTargetError extends osbTKError {
    constructor(message: string, options: osbTKDomainErrorOptions = {}) {
        super({ code: osbTKErrorCode.UnsupportedTarget, message, ...options });
        this.name = 'UnsupportedTargetError';
    }
}

/** An option passed to the converter is invalid. */
export class OptionError extends osbTKError {
    constructor(message: string, options: osbTKDomainErrorOptions = {}) {
        super({ code: osbTKErrorCode.InvalidOption, message, ...options });
        this.name = 'OptionError';
    }
}

/** A native plug-in build failed. */
export class BuildError extends osbTKError {
    constructor(message: string, options: osbTKDomainErrorOptions = {}) {
        super({ code: osbTKErrorCode.BuildFailed, message, ...options });
        this.name = 'BuildError';
    }
}

/** A required host SDK could not be located. */
export class SdkError extends osbTKError {
    constructor(message: string, options: osbTKDomainErrorOptions = {}) {
        super({ code: osbTKErrorCode.SdkNotFound, message, ...options });
        this.name = 'SdkError';
    }
}

/** A filesystem operation failed. */
export class IOError extends osbTKError {
    constructor(message: string, options: osbTKDomainErrorOptions = {}) {
        super({ code: osbTKErrorCode.IOFailed, message, ...options });
        this.name = 'IOError';
    }
}
