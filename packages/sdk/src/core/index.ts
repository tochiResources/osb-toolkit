import { nativeConfiguration } from '../interceptor';
import { Configuration, CoreAPI } from '../lib';
import { Core as CoreClient } from '../lib/client';

// The `Core` subpath: the non-runtime surface - math, curves, easing and the
// stateless parsers/converters. It mirrors the runtime `client` entry and owns
// only the generated `Core` clean client, so it stays disjoint from the runtime
// by construction (no handles, no storyboard state).
export { Core as CoreClient } from '../lib/client';
export * from '../lib/client/options';
export * from '../lib/models';
export type { FetchAPI } from '../lib/runtime';

/** Builds the `Core` surface over a transport (defaults to the native bridge). */
export function createCore(configuration: Configuration = nativeConfiguration()): CoreClient {
    return new CoreClient(new CoreAPI(configuration));
}

/** The default `Core` surface, bound to the stateless native bridge. */
export const Core: CoreClient = createCore();
