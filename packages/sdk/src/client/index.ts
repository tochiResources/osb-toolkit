import * as native from '@tochii/osb-toolkit-native';

import { nativeConfiguration } from '../interceptor';
import {
    AudioAPI,
    Configuration,
    DiagnosticsAPI,
    HelpersAPI,
    ImageAPI,
    ImagegenAPI,
    MediaAPI,
    OptimizeAPI,
    Storyboarding3dAPI,
} from '../lib';
import {
    Audio,
    Diagnostics,
    Image,
    Imagegen,
    Media,
    Optimize,
    Storyboarding3d,
} from '../lib/client';
import { createHelpers, type Helpers } from './helpers';
import { createIO, type IO } from './io';

// The runtime SDK: the clean generated clients plus the conveniences OpenAPI
// cannot express (the effect `Generator`/`Runtime` lifetime scope, the typed
// helper namespaces and the file-operations bridge). The clean clients are the
// primary surface; the `interceptor` remains the only transport seam.
export {
    Audio,
    Diagnostics,
    Image,
    Imagegen,
    Media,
    Optimize,
    Storyboarding3d,
    Layer,
    Sprite,
} from '../lib/client';
export { Generator as GeneratorClient, Helpers as HelpersClient } from '../lib/client';
export * from '../lib/client/options';
export * from '../lib/models';
export type { FetchAPI } from '../lib/runtime';

export { Generator, Runtime, run } from './generator';
export * from './helpers';
export * from './io';

/**
 * The runtime SDK: the storyboarding runtime and the non-core libraries, each a
 * clean generated client (or a convenience over one).
 */
export interface Client {
    /** The native runtime version string. */
    version(): string;
    /** The shared motion-graphics helper namespaces. */
    helpers: Helpers;
    /** The audio library: FFT and the loaded audio. */
    audio: Audio;
    /** The diagnostics library: storyboard and mapset analysis. */
    diagnostics: Diagnostics;
    /** The optimization library: storyboard optimization and `.osb` round-trip. */
    optimize: Optimize;
    /** The image library: decode/encode and processing. */
    image: Image;
    /** The image-generation library: draw primitive shapes. */
    imageGen: Imagegen;
    /** The media library: probing, frame extraction/encoding and audio. */
    media: Media;
    /** The 3D storyboard library: the 3D -> 2D bake. */
    storyboarding3d: Storyboarding3d;
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
        audio: new Audio(new AudioAPI(configuration)),
        diagnostics: new Diagnostics(new DiagnosticsAPI(configuration)),
        optimize: new Optimize(new OptimizeAPI(configuration)),
        image: new Image(new ImageAPI(configuration)),
        imageGen: new Imagegen(new ImagegenAPI(configuration)),
        media: new Media(new MediaAPI(configuration)),
        storyboarding3d: new Storyboarding3d(new Storyboarding3dAPI(configuration)),
        io: createIO(),
    };
}

/** The default runtime SDK, bound to the stateless native bridge. */
export const osb: Client = createClient();
