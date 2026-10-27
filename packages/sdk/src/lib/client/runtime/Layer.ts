import { LayerAPI } from '../../api/LayerAPI';
import type { BoolResult } from '../../models/BoolResult';
import type { CreateAnimationRequest } from '../../models/CreateAnimationRequest';
import type { CreateSampleRequest } from '../../models/CreateSampleRequest';
import type { CreateSpriteRequest } from '../../models/CreateSpriteRequest';
import type { Handle } from '../../models/Handle';
import type { HandleRequest } from '../../models/HandleRequest';
import type { LayerHandle } from '../../models/LayerHandle';
import type { OkResult } from '../../models/OkResult';
import type { SegmentAddIndexRequest } from '../../models/SegmentAddIndexRequest';
import type { SegmentChangePositionRequest } from '../../models/SegmentChangePositionRequest';
import type { SegmentCleanResult } from '../../models/SegmentCleanResult';
import type { SegmentDiscardResult } from '../../models/SegmentDiscardResult';
import type { SegmentIndexesResult } from '../../models/SegmentIndexesResult';
import type { SegmentRequest } from '../../models/SegmentRequest';
import type { SegmentReverseDepthRequest } from '../../models/SegmentReverseDepthRequest';

export class Layer {
    constructor(private readonly api: LayerAPI) {}

    static instanceOf(value: unknown): value is Layer {
        return value instanceof Layer;
    }

    async addIndex(body: SegmentAddIndexRequest): Promise<OkResult> {
        return this.api.addIndex(body);
    }

    async changePosition(body: SegmentChangePositionRequest): Promise<OkResult> {
        return this.api.changePosition(body);
    }

    async clean(body: SegmentRequest): Promise<SegmentCleanResult> {
        return this.api.clean(body);
    }

    async createAnimation(body: CreateAnimationRequest): Promise<Handle> {
        return this.api.createAnimation(body);
    }

    async createSample(body: CreateSampleRequest): Promise<OkResult> {
        return this.api.createSample(body);
    }

    async createSegment(body: SegmentRequest): Promise<LayerHandle> {
        return this.api.createSegment(body);
    }

    async createSprite(body: CreateSpriteRequest): Promise<Handle> {
        return this.api.createSprite(body);
    }

    async discard(body: SegmentRequest): Promise<SegmentDiscardResult> {
        return this.api.discard(body);
    }

    async discardSprite(body: HandleRequest): Promise<BoolResult> {
        return this.api.discardSprite(body);
    }

    async indexes(body: SegmentRequest): Promise<SegmentIndexesResult> {
        return this.api.indexes(body);
    }

    async moveToBottom(body: SegmentRequest): Promise<OkResult> {
        return this.api.moveToBottom(body);
    }

    async moveToTop(body: SegmentRequest): Promise<OkResult> {
        return this.api.moveToTop(body);
    }

    async reverseDepth(body: SegmentReverseDepthRequest): Promise<OkResult> {
        return this.api.reverseDepth(body);
    }
}
