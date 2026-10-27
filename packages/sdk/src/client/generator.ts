import * as native from '@tochii/osb-toolkit-native';

import { bridgeFetch } from '../interceptor';
import {
    AudioAPI,
    type Beatmap,
    Configuration,
    type ControlPoint,
    type CreateAnimationRequest,
    type FftResult,
    type Handle,
    type OsbOrigin,
    type Storyboard,
    GeneratorAPI,
    LayerAPI,
    SpriteAPI,
} from '../lib';
import { Layer } from './sprite';

/**
 * A live storyboard runtime bound to a WASM handle. Its clients are driven
 * through the bridge transport; `free()` releases the native runtime (called
 * automatically by the `FinalizationRegistry` if you forget).
 */
export class Runtime {
    private readonly handle: native.Runtime;

    /** The generated generator client. */
    readonly generator: GeneratorAPI;

    /** The generated layer client. */
    readonly layer: LayerAPI;

    /** The generated sprite client. */
    readonly sprite: SpriteAPI;

    /** The generated audio client bound to this runtime. */
    readonly audio: AudioAPI;

    constructor(width = 640, height = 480) {
        this.handle = new native.Runtime(width, height);
        const configuration = new Configuration({
            fetchApi: bridgeFetch(request => this.handle.exec(request)),
        });
        this.generator = new GeneratorAPI(configuration);
        this.layer = new LayerAPI(configuration);
        this.sprite = new SpriteAPI(configuration);
        this.audio = new AudioAPI(configuration);
        runtimeFinalizer.register(this, this.handle, this);
    }

    /** The loaded audio duration in milliseconds. */
    async audioDuration(): Promise<number> {
        return (await this.audio.audioDuration()).audioDuration;
    }

    /** Frees the underlying native runtime. */
    free(): void {
        runtimeFinalizer.unregister(this);
        this.handle.free();
    }
}

/** Frees a runtime's native handle if `free()` was never called. */
const runtimeFinalizer = new FinalizationRegistry<native.Runtime>(handle => handle.free());

/**
 * A storyboard generator (the `StoryboardObjectGenerator` equivalent).
 * Subclass it, override `generate()` and drive the storyboard through the
 * effect-script API.
 */
export abstract class Generator {
    /** The live storyboard runtime. */
    readonly runtime: Runtime;

    /** Creates a generator for a storyboard of the given resolution. */
    constructor(width = 640, height = 480) {
        this.runtime = new Runtime(width, height);
    }

    /** Creates a sprite on a layer. */
    createSprite(layer: string, path: string, origin: OsbOrigin = 'Centre'): Promise<Handle> {
        return this.runtime.layer.createSprite({ layer, path, origin });
    }

    /** Returns a layer that creates sprites/animations/samples. */
    async getLayer(name: string): Promise<Layer> {
        const handle = await this.runtime.generator.layerHandle({ name });
        return new Layer(this.runtime.layer, this.runtime.sprite, name, handle.layer);
    }

    /** Creates an animation sprite. */
    createAnimation(request: CreateAnimationRequest): Promise<Handle> {
        return this.runtime.layer.createAnimation(request);
    }

    /** Adds a sound sample to a layer. */
    async createSample(layer: string, path: string, time: number, volume: number): Promise<void> {
        await this.runtime.layer.createSample({ layer, path, time, volume });
    }

    /** The loaded beatmap. */
    getBeatmap(): Promise<Beatmap> {
        return this.runtime.generator.beatmap();
    }

    /** The FFT magnitudes of the loaded audio. */
    getFft(time: number, magnitudes: number): Promise<FftResult> {
        return this.runtime.audio.getFft({ time, magnitudes });
    }

    /** The active timing point at a time. */
    getTimingPointAt(time: number): Promise<ControlPoint> {
        return this.runtime.generator.timingPointAt({ time });
    }

    /** The number of sprites on the storyboard. */
    async spriteCount(): Promise<number> {
        return (await this.runtime.generator.spriteCount()).spriteCount;
    }

    /**
     * The built storyboard model, so `generate()`'s result can be read back like
     * the runtime `Effect::generate` return value.
     */
    storyboard(): Promise<Storyboard> {
        return this.runtime.generator.storyboard();
    }

    /** Frees the underlying runtime. */
    free(): void {
        this.runtime.free();
    }

    /** Builds the storyboard (the effect entry point). */
    abstract generate(): Promise<void>;
}

/** Runs an effect's `generate()` against a fresh generator. */
export async function run(effect: Generator): Promise<Generator> {
    await effect.generate();
    return effect;
}
