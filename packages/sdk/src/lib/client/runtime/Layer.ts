import { LayerAPI } from '../../api/LayerAPI';
import type { BoolResult } from '../../models/BoolResult';
import type { OkResult } from '../../models/OkResult';
import type { SegmentCleanResult } from '../../models/SegmentCleanResult';
import type { SegmentDiscardResult } from '../../models/SegmentDiscardResult';
import type { SegmentIndexesResult } from '../../models/SegmentIndexesResult';
import type { SpriteAPI } from '../../api/SpriteAPI';
import { Sprite } from './Sprite';
import type { CreateAnimationOptions, CreateSampleOptions, CreateSpriteOptions, SegmentAddIndexOptions, SegmentChangePositionOptions, SegmentReverseDepthOptions } from '../options';

export class Layer {
    constructor(
        private readonly api: LayerAPI,
        private readonly sprites: SpriteAPI,
        private readonly handle: number
    ) {}

    static instanceOf(value: unknown): value is Layer {
        return value instanceof Layer;
    }

    async addIndex(options: SegmentAddIndexOptions): Promise<OkResult> {
        return this.api.addIndex({ layer: this.handle, ...options });
    }

    async changePosition(options: SegmentChangePositionOptions): Promise<OkResult> {
        return this.api.changePosition({ layer: this.handle, ...options });
    }

    async clean(): Promise<SegmentCleanResult> {
        return this.api.clean({ layer: this.handle });
    }

    async createAnimation(options: CreateAnimationOptions): Promise<Sprite> {
        const handle = await this.api.createAnimation({ layerHandle: this.handle, ...options });
        return new Sprite(this.sprites, handle.layer ?? this.handle, handle.sprite ?? 0);
    }

    async createSample(options: CreateSampleOptions): Promise<OkResult> {
        return this.api.createSample({ layerHandle: this.handle, ...options });
    }

    async createSegment(): Promise<Layer> {
        const handle = await this.api.createSegment({ layer: this.handle });
        return new Layer(this.api, this.sprites, handle.layer ?? this.handle);
    }

    async createSprite(options: CreateSpriteOptions): Promise<Sprite> {
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

    async reverseDepth(options: SegmentReverseDepthOptions): Promise<OkResult> {
        return this.api.reverseDepth({ layer: this.handle, ...options });
    }
}
