import { LayerAPI } from '../../api/LayerAPI';
import type { BoolResult } from '../../models/BoolResult';
import type { OkResult } from '../../models/OkResult';
import type { OsbLayer } from '../../models/OsbLayer';
import type { OsbLoopType } from '../../models/OsbLoopType';
import type { OsbOrigin } from '../../models/OsbOrigin';
import type { SegmentCleanResult } from '../../models/SegmentCleanResult';
import type { SegmentDiscardResult } from '../../models/SegmentDiscardResult';
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

    async addIndex(options: SegmentAddIndexOptions): Promise<OkResult>;
    async addIndex(index: number): Promise<OkResult>;
    async addIndex(optionsOrIndex?: SegmentAddIndexOptions | number): Promise<OkResult> {
        const options: SegmentAddIndexOptions = isSegmentAddIndexOptions(optionsOrIndex)
            ? optionsOrIndex
            : ({ index: optionsOrIndex } as SegmentAddIndexOptions);
        return this.api.addIndex({ layer: this.handle, ...options });
    }

    async changePosition(options: SegmentChangePositionOptions): Promise<OkResult>;
    async changePosition(osbLayer: OsbLayer): Promise<OkResult>;
    async changePosition(optionsOrOsbLayer?: SegmentChangePositionOptions | OsbLayer): Promise<OkResult> {
        const options: SegmentChangePositionOptions = isSegmentChangePositionOptions(optionsOrOsbLayer)
            ? optionsOrOsbLayer
            : ({ osbLayer: optionsOrOsbLayer } as SegmentChangePositionOptions);
        return this.api.changePosition({ layer: this.handle, ...options });
    }

    async clean(): Promise<SegmentCleanResult> {
        return this.api.clean({ layer: this.handle });
    }

    async createAnimation(options: CreateAnimationOptions): Promise<Sprite>;
    async createAnimation(path: string, frames: number, frameDelay: number, loopType: OsbLoopType, origin: OsbOrigin, position: Vector2, layer?: string | null): Promise<Sprite>;
    async createAnimation(optionsOrPath?: CreateAnimationOptions | string, frames?: number, frameDelay?: number, loopType?: OsbLoopType, origin?: OsbOrigin, position?: Vector2, layer?: string | null): Promise<Sprite> {
        const options: CreateAnimationOptions = isCreateAnimationOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath, frames: frames, frameDelay: frameDelay, loopType: loopType, origin: origin, position: position, layer: layer } as CreateAnimationOptions);
        const handle = await this.api.createAnimation({ layerHandle: this.handle, ...options });
        return new Sprite(this.sprites, handle.layer ?? this.handle, handle.sprite ?? 0);
    }

    async createSample(options: CreateSampleOptions): Promise<OkResult>;
    async createSample(path: string, time: number, volume?: number | null, layer?: string | null): Promise<OkResult>;
    async createSample(optionsOrPath?: CreateSampleOptions | string, time?: number, volume?: number | null, layer?: string | null): Promise<OkResult> {
        const options: CreateSampleOptions = isCreateSampleOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath, time: time, volume: volume, layer: layer } as CreateSampleOptions);
        return this.api.createSample({ layerHandle: this.handle, ...options });
    }

    async createSegment(): Promise<Layer> {
        const handle = await this.api.createSegment({ layer: this.handle });
        return new Layer(this.api, this.sprites, handle.layer ?? this.handle);
    }

    async createSprite(options: CreateSpriteOptions): Promise<Sprite>;
    async createSprite(path: string, origin: OsbOrigin, position?: Vector2 | null, layer?: string | null): Promise<Sprite>;
    async createSprite(optionsOrPath?: CreateSpriteOptions | string, origin?: OsbOrigin, position?: Vector2 | null, layer?: string | null): Promise<Sprite> {
        const options: CreateSpriteOptions = isCreateSpriteOptions(optionsOrPath)
            ? optionsOrPath
            : ({ path: optionsOrPath, origin: origin, position: position, layer: layer } as CreateSpriteOptions);
        const handle = await this.api.createSprite({ layerHandle: this.handle, ...options });
        return new Sprite(this.sprites, handle.layer ?? this.handle, handle.sprite ?? 0);
    }

    async discard(): Promise<SegmentDiscardResult> {
        return this.api.discard({ layer: this.handle });
    }

    async discardSprite(sprite: number): Promise<BoolResult> {
        return this.api.discardSprite({ layer: this.handle, sprite });
    }

    async indexes(): Promise<SegmentIndexesResult> {
        return this.api.indexes({ layer: this.handle });
    }

    async moveToBottom(): Promise<OkResult> {
        return this.api.moveToBottom({ layer: this.handle });
    }

    async moveToTop(): Promise<OkResult> {
        return this.api.moveToTop({ layer: this.handle });
    }

    async reverseDepth(options: SegmentReverseDepthOptions): Promise<OkResult>;
    async reverseDepth(value: boolean): Promise<OkResult>;
    async reverseDepth(optionsOrValue?: SegmentReverseDepthOptions | boolean): Promise<OkResult> {
        const options: SegmentReverseDepthOptions = isSegmentReverseDepthOptions(optionsOrValue)
            ? optionsOrValue
            : ({ value: optionsOrValue } as SegmentReverseDepthOptions);
        return this.api.reverseDepth({ layer: this.handle, ...options });
    }
}
