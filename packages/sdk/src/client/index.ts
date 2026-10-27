import * as native from '@tochii/osb-toolkit-native';

import { nativeConfiguration } from '../interceptor';
import {
    AudioAPI,
    type Configuration,
    DiagnosticsAPI,
    HelpersAPI,
    ImageAPI,
    ImagegenAPI,
    MediaAPI,
    OptimizeAPI,
    Storyboarding3dAPI,
} from '../lib';
import { createHelpers, type Helpers } from './helpers';
import { createIO, type IO } from './io';

// The runtime client wrapper (§4.2 of `osbTK-ARCHITECTURE.md`): a single
// entrypoint that exposes every **non-core** generated OpenAPI group, sourced
// from the generated output (`../lib`), plus the conveniences OpenAPI cannot
// express (the fluent `Layer`/`Sprite` builders, the `Runtime`/`Generator`
// lifetime scope, the typed helper namespaces and the file-operations bridge).
// The non-runtime `Core` surface lives in the sibling `../core` entry.

// Re-export the generated groups this entry owns, plus every shared model/enum
// (the models are dimension- and transport-agnostic, so both entries re-export
// them). The generated classes carry the `API` suffix (`GeneratorAPI`,
// `AudioAPI`, …) so the plain names stay free for the conveniences below.
export {
    AudioAPI,
    DiagnosticsAPI,
    HelpersAPI,
    ImageAPI,
    ImagegenAPI,
    MediaAPI,
    OptimizeAPI,
    Storyboarding3dAPI,
} from '../lib';
export * from '../lib/models';
export type { FetchAPI } from '../lib/runtime';

// The conveniences OpenAPI cannot express.
export { Generator, Runtime, run } from './generator';
export { Layer, Sprite, spriteFrom } from './sprite';
export * from './helpers';
export * from './io';

/**
 * The runtime SDK: the storyboarding runtime and the non-core libraries, each a
 * configured generated client (or a convenience over one).
 */
export interface Client {
    /** The native runtime version string. */
    version(): string;
    /** The shared motion-graphics helper namespaces (M38/M40). */
    helpers: Helpers;
    /** The audio library: FFT and the loaded audio. */
    audio: AudioAPI;
    /** The diagnostics library: storyboard and mapset analysis (M29). */
    diagnostics: DiagnosticsAPI;
    /** The optimization library: storyboard optimization and `.osb` round-trip (M30). */
    optimize: OptimizeAPI;
    /** The image library: decode/encode and processing (M23). */
    image: ImageAPI;
    /** The image-generation library: draw primitive shapes (M25). */
    imageGen: ImagegenAPI;
    /** The media library: probing, frame extraction/encoding and audio (M26). */
    media: MediaAPI;
    /** The 3D storyboard library: the 3D -> 2D bake (M28). */
    storyboarding3d: Storyboarding3dAPI;
    /** The file-operations library (`osb-core`): the toolkit's filesystem API. */
    io: IO;
}

/**
 * Builds the runtime client over a transport. Defaults to the stateless native
 * bridge; pass a `Configuration` (e.g. an HTTP one) to talk to `osb-server`
 * instead.
 */
export function createClient(configuration: Configuration = nativeConfiguration()): Client {
    return {
        version: (): string => native.version(),
        helpers: createHelpers(new HelpersAPI(configuration)),
        audio: new AudioAPI(configuration),
        diagnostics: new DiagnosticsAPI(configuration),
        optimize: new OptimizeAPI(configuration),
        image: new ImageAPI(configuration),
        imageGen: new ImagegenAPI(configuration),
        media: new MediaAPI(configuration),
        storyboarding3d: new Storyboarding3dAPI(configuration),
        io: createIO(),
    };
}

/** The default runtime SDK, bound to the stateless native bridge. */
export const osb: Client = createClient();
