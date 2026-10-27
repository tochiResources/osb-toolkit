// `@tochii/osb-sdk-js` - the entry point (§4.2 of `osbTK-ARCHITECTURE.md`).
//
// The runtime SDK is exported as `osb` from the `client` wrapper; the
// non-runtime `Core` subpath is re-exported from `core` (and is also available
// on its own at `@tochii/osb-sdk-js/core`). The generated client lives in
// `lib/` and the one hand-written transport seam in `interceptor.ts`.
export * from './client';
export * from './core';
export type { BridgeExec } from './interceptor';
export {
    bridgeFetch,
    nativeConfiguration,
    nativeFetch,
    runtimeConfiguration,
    runtimeFetch,
} from './interceptor';
