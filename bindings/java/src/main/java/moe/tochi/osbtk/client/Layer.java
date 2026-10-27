package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.LayerAPI;
import moe.tochi.osbtk.model.BoolResult;
import moe.tochi.osbtk.model.CreateAnimationRequest;
import moe.tochi.osbtk.model.CreateSampleRequest;
import moe.tochi.osbtk.model.CreateSpriteRequest;
import moe.tochi.osbtk.model.Handle;
import moe.tochi.osbtk.model.HandleRequest;
import moe.tochi.osbtk.model.LayerHandle;
import moe.tochi.osbtk.model.OkResult;
import moe.tochi.osbtk.model.SegmentAddIndexRequest;
import moe.tochi.osbtk.model.SegmentChangePositionRequest;
import moe.tochi.osbtk.model.SegmentCleanResult;
import moe.tochi.osbtk.model.SegmentDiscardResult;
import moe.tochi.osbtk.model.SegmentIndexesResult;
import moe.tochi.osbtk.model.SegmentRequest;
import moe.tochi.osbtk.model.SegmentReverseDepthRequest;
import java.util.List;

public final class Layer {
    private final LayerAPI api;

    public Layer(LayerAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Layer;
    }

    public OkResult addIndex(SegmentAddIndexRequest body) throws ApiException {
        return api.addIndex(body);
    }

    public OkResult changePosition(SegmentChangePositionRequest body) throws ApiException {
        return api.changePosition(body);
    }

    public SegmentCleanResult clean(SegmentRequest body) throws ApiException {
        return api.clean(body);
    }

    public Handle createAnimation(CreateAnimationRequest body) throws ApiException {
        return api.createAnimation(body);
    }

    public OkResult createSample(CreateSampleRequest body) throws ApiException {
        return api.createSample(body);
    }

    public LayerHandle createSegment(SegmentRequest body) throws ApiException {
        return api.createSegment(body);
    }

    public Handle createSprite(CreateSpriteRequest body) throws ApiException {
        return api.createSprite(body);
    }

    public SegmentDiscardResult discard(SegmentRequest body) throws ApiException {
        return api.discard(body);
    }

    public BoolResult discardSprite(HandleRequest body) throws ApiException {
        return api.discardSprite(body);
    }

    public SegmentIndexesResult indexes(SegmentRequest body) throws ApiException {
        return api.indexes(body);
    }

    public OkResult moveToBottom(SegmentRequest body) throws ApiException {
        return api.moveToBottom(body);
    }

    public OkResult moveToTop(SegmentRequest body) throws ApiException {
        return api.moveToTop(body);
    }

    public OkResult reverseDepth(SegmentReverseDepthRequest body) throws ApiException {
        return api.reverseDepth(body);
    }
}
