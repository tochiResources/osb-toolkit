import * as native from '@tochii/osb-toolkit-native';

import { bridgeFetch } from '../interceptor';
import {
    AudioAPI,
    type Beatmap,
    Configuration,
    type ControlPoint,
    type FftResult,
    GeneratorAPI,
    LayerAPI,
    type OsbOrigin,
    SpriteAPI,
    type Storyboard,
} from '../lib';
import {
    Audio,
    type CreateAnimationOptions,
    Generator as GeneratorClient,
    Layer,
    Sprite,
} from '../lib/client';

/**
 * A live storyboard runtime bound to a WASM handle. Its clients are driven
 * through the bridge transport; `free()` releases the native runtime (called
 * automatically by the `FinalizationRegistry` if you forget).
 */
export class Runtime {
    private readonly handle: native.Runtime;

    /** The clean generator client bound to this runtime. */
    readonly generator: GeneratorClient;

    /** The clean audio client bound to this runtime. */
    readonly audio: Audio;

    private readonly layerApi: LayerAPI;
    private readonly spriteApi: SpriteAPI;

    constructor(width = 640, height = 480) {
        this.handle = new native.Runtime(width, height);
        const configuration = new Configuration({
            fetchApi: bridgeFetch(request => this.handle.exec(request)),
        });
        this.generator = new GeneratorClient(new GeneratorAPI(configuration));
        this.audio = new Audio(new AudioAPI(configuration));
        this.layerApi = new LayerAPI(configuration);
        this.spriteApi = new SpriteAPI(configuration);
        runtimeFinalizer.register(this, this.handle, this);
    }

    /** The loaded audio duration in milliseconds. */
    audioDuration(): Promise<number> {
        return this.audio.audioDuration();
    }

    /** Resolves (or creates) a layer by name and binds its handle. */
    async layer(name: string): Promise<Layer> {
        const handle = await this.generator.layerHandle(name);
        return new Layer(this.layerApi, this.spriteApi, handle);
    }

    /** Binds an existing layer/sprite pair. */
    sprite(layer: number, sprite: number): Sprite {
        return new Sprite(this.spriteApi, layer, sprite);
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
    async createSprite(layer: string, path: string, origin: OsbOrigin = 'Centre'): Promise<Sprite> {
        return (await this.getLayer(layer)).createSprite(path, origin);
    }

    /** Returns a layer that creates sprites/animations/samples. */
    getLayer(name: string): Promise<Layer> {
        return this.runtime.layer(name);
    }

    /** Creates an animation sprite on a layer. */
    async createAnimation(layer: string, options: CreateAnimationOptions): Promise<Sprite> {
        return (await this.getLayer(layer)).createAnimation(options);
    }

    /** Adds a sound sample to a layer. */
    async createSample(layer: string, path: string, time: number, volume: number): Promise<void> {
        await (await this.getLayer(layer)).createSample(path, time, volume);
    }

    /** The loaded beatmap. */
    getBeatmap(): Promise<Beatmap> {
        return this.runtime.generator.beatmap();
    }

    /** The FFT magnitudes of the loaded audio. */
    getFft(time: number, magnitudes: number): Promise<FftResult> {
        return this.runtime.audio.getFft(time, magnitudes);
    }

    /** The active timing point at a time. */
    getTimingPointAt(time: number): Promise<ControlPoint> {
        return this.runtime.generator.timingPointAt(time);
    }

    /** The number of sprites on the storyboard. */
    spriteCount(): Promise<number> {
        return this.runtime.generator.spriteCount();
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
