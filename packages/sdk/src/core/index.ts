import * as native from '@tochii/osb-toolkit-native';

import { nativeConfiguration } from '../interceptor';
import {
    AnimationsAPI,
    CommonAPI,
    type Configuration,
    CurveAPI,
    OriginAPI,
    ParsingAPI,
} from '../lib';

// The `Core` subpath: the non-runtime surface -
// math, curves, easing and the stateless parsers/converters - exposed as a
// sibling of the runtime `client` entry. It mirrors the `client` folder's shape
// but owns only the core OpenAPI groups, and is disjoint from the runtime by
// construction (no handles, no storyboard state). When the contract folds the
// raw math helpers into a single `Core` tag, it lands here.

// Re-export the core generated groups and every shared model/enum.
export { AnimationsAPI, CommonAPI, CurveAPI, OriginAPI, ParsingAPI } from '../lib';
export * from '../lib/models';
export type { FetchAPI } from '../lib/runtime';

/** The `Core` subpath: the non-runtime surface, each a configured generated client. */
export interface Core {
    /** The native runtime version string. */
    version(): string;
    /** The common surface (`Common.version`). */
    common: CommonAPI;
    /** The curve library: Bezier/Catmull-Rom curves and simplification. */
    curve: CurveAPI;
    /** The animations library: easing evaluation. */
    animations: AnimationsAPI;
    /** The stateless parsers: `.osu`/`.osb`. */
    parsing: ParsingAPI;
    /** The stateless origin/screen-bound queries. */
    origin: OriginAPI;
}

/** Builds the `Core` surface over a transport (defaults to the native bridge). */
export function createCore(configuration: Configuration = nativeConfiguration()): Core {
    return {
        version: (): string => native.version(),
        common: new CommonAPI(configuration),
        curve: new CurveAPI(configuration),
        animations: new AnimationsAPI(configuration),
        parsing: new ParsingAPI(configuration),
        origin: new OriginAPI(configuration),
    };
}

/** The default `Core` surface, bound to the stateless native bridge. */
export const Core: Core = createCore();
