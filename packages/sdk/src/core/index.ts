import * as native from '@tochii/osb-toolkit-native';

import { nativeConfiguration } from '../interceptor';
import { AnimationsAPI, CommonAPI, Configuration, CurveAPI, OriginAPI, ParsingAPI } from '../lib';
import { Animations, Common, Curve, Origin, Parsing } from '../lib/client';

// The `Core` subpath: the non-runtime surface - math, curves, easing and the
// stateless parsers/converters. It mirrors the runtime `client` entry and owns
// only the core clean clients, so it stays disjoint from the runtime by
// construction (no handles, no storyboard state). When the contract folds the
// raw math helpers into a single `Core` tag, it lands here.
export { Animations, Common, Curve, Origin, Parsing } from '../lib/client';
export * from '../lib/client/options';
export * from '../lib/models';
export type { FetchAPI } from '../lib/runtime';

/** The `Core` subpath: the non-runtime surface, each a clean generated client. */
export interface Core {
    /** The native runtime version string. */
    version(): string;
    /** The common surface (`Common.version`). */
    common: Common;
    /** The curve library: Bezier/Catmull-Rom curves and simplification. */
    curve: Curve;
    /** The animations library: easing evaluation. */
    animations: Animations;
    /** The stateless parsers: `.osu`/`.osb`. */
    parsing: Parsing;
    /** The stateless origin/screen-bound queries. */
    origin: Origin;
}

/** Builds the `Core` surface over a transport (defaults to the native bridge). */
export function createCore(configuration: Configuration = nativeConfiguration()): Core {
    return {
        version: (): string => native.version(),
        common: new Common(new CommonAPI(configuration)),
        curve: new Curve(new CurveAPI(configuration)),
        animations: new Animations(new AnimationsAPI(configuration)),
        parsing: new Parsing(new ParsingAPI(configuration)),
        origin: new Origin(new OriginAPI(configuration)),
    };
}

/** The default `Core` surface, bound to the stateless native bridge. */
export const Core: Core = createCore();
