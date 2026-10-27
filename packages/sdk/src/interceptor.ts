import * as native from '@tochii/osb-toolkit-native';

import { Configuration, type FetchAPI } from './lib';

/**
 * The SDK's single hand-written seam: it
 * intercepts the generated OpenAPI clients' transport and routes it to the
 * native JSON bridge instead of the network. The generated client stays a
 * *normal* OpenAPI client - it can still run over real HTTP (against
 * `osb-server`) when the interception is not installed.
 */

/** A bridge executor: takes a JSON request object and returns the JSON response. */
export type BridgeExec = (request: unknown) => unknown;

/**
 * A `FetchAPI` that routes the generated clients' HTTP calls to the native JSON
 * bridge instead of the network. The operation is the last path segment (which is
 * exactly the native `op`); the request body is merged with it. Bridge errors come
 * back as a non-2xx response, so the generated client raises them.
 */
export function bridgeFetch(exec: BridgeExec): FetchAPI {
    return async (url, init) => {
        const href = typeof url === 'string' ? url : url instanceof URL ? url.href : url.url;
        const path = new URL(href, 'http://osbtk.invalid').pathname;
        const op = path.split('/').filter(Boolean).pop() ?? '';
        const body =
            typeof init?.body === 'string' && init.body.length > 0
                ? (JSON.parse(init.body) as Record<string, unknown>)
                : {};
        try {
            const response = exec({ ...body, op });
            return jsonResponse(
                isError(response) ? response : (response ?? null),
                isError(response) ? 400 : 200
            );
        } catch (error) {
            const message = error instanceof Error ? error.message : String(error);
            return jsonResponse({ error: message }, 500);
        }
    };
}

/**
 * A `FetchAPI` over the stateless native bridge (`osbtk_exec`): the non-runtime
 * surface - parsing, curves, easing, image, media, diagnostics, optimization,
 * helpers and the 3D bake.
 */
export function nativeFetch(): FetchAPI {
    return bridgeFetch(request => native.exec(request));
}

/**
 * A `FetchAPI` over a live runtime handle's bridge (`osbtk_runtime_exec`): the
 * storyboard runtime surface (`Generator`/`Layer`/`Sprite` and the loaded audio).
 */
export function runtimeFetch(handle: { exec(request: unknown): unknown }): FetchAPI {
    return bridgeFetch(request => handle.exec(request));
}

/** A `Configuration` whose transport is the stateless native bridge. */
export function nativeConfiguration(): Configuration {
    return new Configuration({ fetchApi: nativeFetch() });
}

/** A `Configuration` whose transport is a live runtime handle's bridge. */
export function runtimeConfiguration(handle: { exec(request: unknown): unknown }): Configuration {
    return new Configuration({ fetchApi: runtimeFetch(handle) });
}

function isError(value: unknown): value is { error: unknown } {
    return typeof value === 'object' && value !== null && 'error' in value;
}

function jsonResponse(value: unknown, status: number): Response {
    return new Response(JSON.stringify(value), {
        status,
        headers: { 'Content-Type': 'application/json' },
    });
}
