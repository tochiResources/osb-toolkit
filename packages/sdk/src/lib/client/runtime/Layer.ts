import { LayerAPI } from '../../api/LayerAPI';
import type { OsbLayer } from '../../models/OsbLayer';
import type { OsbLoopType } from '../../models/OsbLoopType';
import type { OsbOrigin } from '../../models/OsbOrigin';
import type { SegmentIndexesResult } from '../../models/SegmentIndexesResult';
import type { Vector2 } from '../../models/Vector2';
import type { SpriteAPI } from '../../api/SpriteAPI';
import { Sprite } from './Sprite';
import type { CreateAnimationOptions, CreateSampleOptions, CreateSpriteOptions, SegmentAddIndexOptions, SegmentChangePositionOptions, SegmentReverseDepthOptions } from '../options';
import { isCreateAnimationOptions, isCreateSampleOptions, isCreateSpriteOptions, isSegmentAddIndexOptions, isSegmentChangePositionOptions, isSegmentReverseDepthOptions } from '../options';

export class Layer {
    constructor(
        private readonly api: LayerAPI,
        private readonly sprites: SpriteAPI,
        private readonly handle: number
    ) {}

    static instanceOf(value: unknown): value is Layer {
        return value instanceof Layer;
    }

    /**
     * Record a depth index on a segment.
     *
     * @param index The index to record.
     *
     * @returns Acknowledged
     */
    async addIndex(index: number): Promise<void>;
    /**
     * Record a depth index on a segment.
     *
     * @param options Record a depth index on a segment.
     *
     * @returns Acknowledged
     */
    async addIndex(options: SegmentAddIndexOptions): Promise<void>;
    async addIndex(optionsOrIndex?: SegmentAddIndexOptions | number): Promise<void> {
        const options: SegmentAddIndexOptions = isSegmentAddIndexOptions(optionsOrIndex)
            ? optionsOrIndex
            : ({ index: optionsOrIndex } as SegmentAddIndexOptions);
        await this.api.addIndex({ layer: this.handle, ...options });
    }

    /**
     * Move a segment to another osu! layer (`ChangePosition`).
     *
     * @param osbLayer The destination osu! layer.
     *
     * @returns Acknowledged
     */
    async changePosition(osbLayer: OsbLayer): Promise<void>;
    /**
     * Move a segment to another osu! layer (`ChangePosition`).
     *
     * @param options Move a segment to another osu! layer (`ChangePosition`).
     *
     * @returns Acknowledged
     */
    async changePosition(options: SegmentChangePositionOptions): Promise<void>;
    async changePosition(optionsOrOsbLayer?: SegmentChangePositionOptions | OsbLayer): Promise<void> {
        const options: SegmentChangePositionOptions = isSegmentChangePositionOptions(optionsOrOsbLayer)
            ? optionsOrOsbLayer
            : ({ osbLayer: optionsOrOsbLayer } as SegmentChangePositionOptions);
        await this.api.changePosition({ layer: this.handle, ...options });
    }

    /**
     * A segment handle request.
     *
     * @returns The number of sprites removed
     */
    async clean(): Promise<number> {
        return (await this.api.clean({ layer: this.handle })).removed;
    }

    /**
     * Create an animation sprite.
     *
     * @param path The texture path (relative to the mapset).
     * @param frames The number of frames.
     * @param frameDelay Delay between frames, in milliseconds.
     * @param loopType How the animation loops.
     * @param origin The sprite anchor origin.
     * @param position The initial position.
     *
     * @returns The new animation handle
     */
    async createAnimation(path: string, frames: number, frameDelay: number, loopType: OsbLoopType, origin: OsbOrigin, position: Vector2): Promise<Sprite>;
    /**
     * Create an animation sprite.
     *
     * @param path The texture path (relative to the mapset).
     * @param frames The number of frames.
     * @param frameDelay Delay between frames, in milliseconds.
     * @param loopType How the animation loops.
     * @param origin The sprite anchor origin.
     * @param position The initial position.
     * @param layer The target layer name (omitted when `layerHandle` is set).
     *
     * @returns The new animation handle
     */
    async createAnimation(path: string, frames: number, frameDelay: number, loopType: OsbLoopType, origin: OsbOrigin, position: Vector2, layer: string | null): Promise<Sprite>;
    /**
     * Create an animation sprite.
     *
     * @param options Create an animation sprite.
     *
     * @returns The new animation handle
     */
    async createAnimation(options: CreateAnimationOptions): Promise<Sprite>;
    async createAnimation(optionsOrPath?: CreateAnimationOptions | string, frames?: number, frameDelay?: number, loopType?: OsbLoopType, origin?: OsbOrigin, position?: Vector2, layer?: string | null): Promise<Sprite> {
        const options: CreateAnimationOptions = isCreateAnimationOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath, frames: frames, frameDelay: frameDelay, loopType: loopType, origin: origin, position: position, layer: layer } as CreateAnimationOptions);
        const handle = await this.api.createAnimation({ layerHandle: this.handle, ...options });
        return new Sprite(this.sprites, handle.layer ?? this.handle, handle.sprite ?? 0);
    }

    /**
     * Create a sound sample on a layer.
     *
     * @param path The sample path (relative to the mapset).
     * @param time The sample time in milliseconds.
     *
     * @returns Acknowledged
     */
    async createSample(path: string, time: number): Promise<void>;
    /**
     * Create a sound sample on a layer.
     *
     * @param path The sample path (relative to the mapset).
     * @param time The sample time in milliseconds.
     * @param volume The sample volume (`0..=100`).
     *
     * @returns Acknowledged
     */
    async createSample(path: string, time: number, volume: number | null): Promise<void>;
    /**
     * Create a sound sample on a layer.
     *
     * @param path The sample path (relative to the mapset).
     * @param time The sample time in milliseconds.
     * @param volume The sample volume (`0..=100`).
     * @param layer The target layer name (omitted when `layerHandle` is set).
     *
     * @returns Acknowledged
     */
    async createSample(path: string, time: number, volume: number | null, layer: string | null): Promise<void>;
    /**
     * Create a sound sample on a layer.
     *
     * @param options Create a sound sample on a layer.
     *
     * @returns Acknowledged
     */
    async createSample(options: CreateSampleOptions): Promise<void>;
    async createSample(optionsOrPath?: CreateSampleOptions | string, time?: number, volume?: number | null, layer?: string | null): Promise<void> {
        const options: CreateSampleOptions = isCreateSampleOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath, time: time, volume: volume, layer: layer } as CreateSampleOptions);
        await this.api.createSample({ layerHandle: this.handle, ...options });
    }

    /**
     * A segment handle request.
     *
     * @returns The new segment handle
     */
    async createSegment(): Promise<Layer> {
        const handle = await this.api.createSegment({ layer: this.handle });
        return new Layer(this.api, this.sprites, handle.layer ?? this.handle);
    }

    /**
     * Create a sprite.
     *
     * @param path The texture path (relative to the mapset).
     * @param origin The sprite anchor origin.
     *
     * @returns The new sprite handle
     */
    async createSprite(path: string, origin: OsbOrigin): Promise<Sprite>;
    /**
     * Create a sprite.
     *
     * @param path The texture path (relative to the mapset).
     * @param origin The sprite anchor origin.
     * @param position The initial position; omitted means the sprite's default.
     *
     * @returns The new sprite handle
     */
    async createSprite(path: string, origin: OsbOrigin, position: Vector2 | null): Promise<Sprite>;
    /**
     * Create a sprite.
     *
     * @param path The texture path (relative to the mapset).
     * @param origin The sprite anchor origin.
     * @param position The initial position; omitted means the sprite's default.
     * @param layer The target layer name (omitted when `layerHandle` is set).
     *
     * @returns The new sprite handle
     */
    async createSprite(path: string, origin: OsbOrigin, position: Vector2 | null, layer: string | null): Promise<Sprite>;
    /**
     * Create a sprite.
     *
     * @param options Create a sprite.
     *
     * @returns The new sprite handle
     */
    async createSprite(options: CreateSpriteOptions): Promise<Sprite>;
    async createSprite(optionsOrPath?: CreateSpriteOptions | string, origin?: OsbOrigin, position?: Vector2 | null, layer?: string | null): Promise<Sprite> {
        const options: CreateSpriteOptions = isCreateSpriteOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath, origin: origin, position: position, layer: layer } as CreateSpriteOptions);
        const handle = await this.api.createSprite({ layerHandle: this.handle, ...options });
        return new Sprite(this.sprites, handle.layer ?? this.handle, handle.sprite ?? 0);
    }

    /**
     * A segment handle request.
     *
     * @returns The remaining top-level layer count
     */
    async discard(): Promise<number> {
        return (await this.api.discard({ layer: this.handle })).layerCount;
    }

    /**
     * A sprite handle pair.
     *
     * @param sprite The sprite handle.
     *
     * @returns Whether the sprite was discarded
     */
    async discardSprite(sprite: number): Promise<boolean> {
        return (await this.api.discardSprite({ layer: this.handle, sprite })).value;
    }

    /**
     * A segment handle request.
     *
     * @returns The segment's indexes and bounds
     */
    async indexes(): Promise<SegmentIndexesResult> {
        return this.api.indexes({ layer: this.handle });
    }

    /**
     * A segment handle request.
     *
     * @returns Acknowledged
     */
    async moveToBottom(): Promise<void> {
        await this.api.moveToBottom({ layer: this.handle });
    }

    /**
     * A segment handle request.
     *
     * @returns Acknowledged
     */
    async moveToTop(): Promise<void> {
        await this.api.moveToTop({ layer: this.handle });
    }

    /**
     * Set a segment's reverse-depth flag.
     *
     * @param value Whether children are emitted in reverse order.
     *
     * @returns Acknowledged
     */
    async reverseDepth(value: boolean): Promise<void>;
    /**
     * Set a segment's reverse-depth flag.
     *
     * @param options Set a segment's reverse-depth flag.
     *
     * @returns Acknowledged
     */
    async reverseDepth(options: SegmentReverseDepthOptions): Promise<void>;
    async reverseDepth(optionsOrValue?: SegmentReverseDepthOptions | boolean): Promise<void> {
        const options: SegmentReverseDepthOptions = isSegmentReverseDepthOptions(optionsOrValue)
            ? optionsOrValue
            : ({ value: optionsOrValue } as SegmentReverseDepthOptions);
        await this.api.reverseDepth({ layer: this.handle, ...options });
    }
}
