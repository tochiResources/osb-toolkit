import { readFileSync, writeFileSync } from 'node:fs';

/** The first generated runtime error class. */
const errorsStart = 'export class ResponseError extends Error {';
/** The declaration that follows the generated runtime error classes. */
const errorsEnd = 'export const COLLECTION_FORMATS = {';

/** Makes the generated runtime errors typed `TochiResError`s. */
export function extendTypeScriptErrors(runtimePath: string): void {
    let text = readFileSync(runtimePath, 'utf8');
    const start = text.indexOf(errorsStart);
    const end = text.indexOf(errorsEnd);
    if (start === -1 || end === -1) {
        throw new Error(`openapi: could not find the runtime error classes in ${runtimePath}`);
    }

    text = `${text.slice(0, start)}${typedErrors}\n${text.slice(end)}`;
    if (!text.includes("from '@tochii/errors'")) {
        text = text.replace(
            '/* eslint-disable */\n',
            "/* eslint-disable */\nimport { ErrorCode, TochiResError } from '@tochii/errors';\n"
        );
    }
    writeFileSync(runtimePath, text, 'utf8');
}

/** The typed replacements for the three generated runtime error classes. */
const typedErrors = `/** Map an HTTP status code to the matching shared error code. */
function errorCodeForStatus(status: number): ErrorCode {
    switch (status) {
        case 400:
            return ErrorCode.ApiBadRequest;
        case 401:
            return ErrorCode.ApiUnauthorized;
        case 403:
            return ErrorCode.ApiForbidden;
        case 404:
            return ErrorCode.ApiNotFound;
        case 408:
        case 504:
            return ErrorCode.ApiTimeout;
        case 409:
            return ErrorCode.ApiConflict;
        case 422:
            return ErrorCode.ApiUnprocessable;
        case 429:
            return ErrorCode.ApiTooManyRequests;
        case 503:
            return ErrorCode.ApiUnavailable;
        default:
            return status >= 500 ? ErrorCode.ApiUnavailable : ErrorCode.ApiBadRequest;
    }
}

export class ResponseError extends TochiResError {
    override name = 'ResponseError';

    constructor(public response: Response, msg?: string) {
        super(errorCodeForStatus(response.status), msg ?? 'Response returned an error code', {
            context: {
                status: response.status,
                statusText: response.statusText,
                url: response.url,
            },
        });
    }
}

export class FetchError extends TochiResError {
    override name = 'FetchError';

    constructor(cause: Error, msg?: string) {
        super(ErrorCode.ApiUnavailable, msg ?? 'The request failed', { cause });
    }
}

export class RequiredError extends TochiResError {
    override name = 'RequiredError';

    constructor(public field: string, msg?: string) {
        super(ErrorCode.ValidationFailed, msg ?? \`Missing required parameter '\${field}'\`, {
            context: { field },
        });
    }
}`;
