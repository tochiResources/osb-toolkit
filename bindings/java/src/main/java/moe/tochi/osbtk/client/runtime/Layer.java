package moe.tochi.osbtk.client.runtime;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.LayerAPI;
import moe.tochi.osbtk.model.CreateAnimationRequest;
import moe.tochi.osbtk.model.CreateSampleRequest;
import moe.tochi.osbtk.model.CreateSpriteRequest;
import moe.tochi.osbtk.model.Handle;
import moe.tochi.osbtk.model.HandleRequest;
import moe.tochi.osbtk.model.LayerHandle;
import moe.tochi.osbtk.model.OsbLayer;
import moe.tochi.osbtk.model.OsbLoopType;
import moe.tochi.osbtk.model.OsbOrigin;
import moe.tochi.osbtk.model.SegmentAddIndexRequest;
import moe.tochi.osbtk.model.SegmentChangePositionRequest;
import moe.tochi.osbtk.model.SegmentIndexesResult;
import moe.tochi.osbtk.model.SegmentRequest;
import moe.tochi.osbtk.model.SegmentReverseDepthRequest;
import moe.tochi.osbtk.model.Vector2;
import moe.tochi.osbtk.client.CreateAnimationOptions;
import moe.tochi.osbtk.client.CreateSampleOptions;
import moe.tochi.osbtk.client.CreateSpriteOptions;
import moe.tochi.osbtk.client.SegmentAddIndexOptions;
import moe.tochi.osbtk.client.SegmentChangePositionOptions;
import moe.tochi.osbtk.client.SegmentReverseDepthOptions;
import moe.tochi.osbtk.api.SpriteAPI;
import java.util.List;

public final class Layer {
    private final LayerAPI api;
    private final SpriteAPI sprites;
    private final int handle;

    public Layer(LayerAPI api, SpriteAPI sprites, int handle) {
        this.api = api;
        this.sprites = sprites;
        this.handle = handle;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Layer;
    }

    public void addIndex(SegmentAddIndexOptions options) throws ApiException {
        api.addIndex(new SegmentAddIndexRequest()
            .index(options.index)
            .layer(this.handle));
    }

    public void addIndex(Double index) throws ApiException {
        addIndex(SegmentAddIndexOptions.builder().index(index).build());
    }

    public void changePosition(SegmentChangePositionOptions options) throws ApiException {
        api.changePosition(new SegmentChangePositionRequest()
            .layer(this.handle)
            .osbLayer(options.osbLayer));
    }

    public void changePosition(OsbLayer osbLayer) throws ApiException {
        changePosition(SegmentChangePositionOptions.builder().osbLayer(osbLayer).build());
    }

    public int clean() throws ApiException {
        return api.clean(new SegmentRequest()
            .layer(this.handle)).getRemoved();
    }

    public Sprite createAnimation(CreateAnimationOptions options) throws ApiException {
        Handle handle = api.createAnimation(new CreateAnimationRequest()
            .frameDelay(options.frameDelay)
            .frames(options.frames)
            .layer(options.layer)
            .layerHandle(this.handle)
            .loopType(options.loopType)
            .origin(options.origin)
            .path(options.path)
            .position(options.position));
        return new Sprite(this.sprites, handle.getLayer(), handle.getSprite());
    }

    public Sprite createAnimation(String path, Integer frames, Double frameDelay, OsbLoopType loopType, OsbOrigin origin, Vector2 position) throws ApiException {
        return createAnimation(CreateAnimationOptions.builder().path(path).frames(frames).frameDelay(frameDelay).loopType(loopType).origin(origin).position(position).build());
    }

    public Sprite createAnimation(String path, Integer frames, Double frameDelay, OsbLoopType loopType, OsbOrigin origin, Vector2 position, String layer) throws ApiException {
        return createAnimation(CreateAnimationOptions.builder().path(path).frames(frames).frameDelay(frameDelay).loopType(loopType).origin(origin).position(position).layer(layer).build());
    }

    public void createSample(CreateSampleOptions options) throws ApiException {
        api.createSample(new CreateSampleRequest()
            .layer(options.layer)
            .layerHandle(this.handle)
            .path(options.path)
            .time(options.time)
            .volume(options.volume));
    }

    public void createSample(String path, Double time) throws ApiException {
        createSample(CreateSampleOptions.builder().path(path).time(time).build());
    }

    public void createSample(String path, Double time, Double volume) throws ApiException {
        createSample(CreateSampleOptions.builder().path(path).time(time).volume(volume).build());
    }

    public void createSample(String path, Double time, Double volume, String layer) throws ApiException {
        createSample(CreateSampleOptions.builder().path(path).time(time).volume(volume).layer(layer).build());
    }

    public Layer createSegment() throws ApiException {
        LayerHandle handle = api.createSegment(new SegmentRequest()
            .layer(this.handle));
        return new Layer(this.api, this.sprites, handle.getLayer());
    }

    public Sprite createSprite(CreateSpriteOptions options) throws ApiException {
        Handle handle = api.createSprite(new CreateSpriteRequest()
            .layer(options.layer)
            .layerHandle(this.handle)
            .origin(options.origin)
            .path(options.path)
            .position(options.position));
        return new Sprite(this.sprites, handle.getLayer(), handle.getSprite());
    }

    public Sprite createSprite(String path, OsbOrigin origin) throws ApiException {
        return createSprite(CreateSpriteOptions.builder().path(path).origin(origin).build());
    }

    public Sprite createSprite(String path, OsbOrigin origin, Vector2 position) throws ApiException {
        return createSprite(CreateSpriteOptions.builder().path(path).origin(origin).position(position).build());
    }

    public Sprite createSprite(String path, OsbOrigin origin, Vector2 position, String layer) throws ApiException {
        return createSprite(CreateSpriteOptions.builder().path(path).origin(origin).position(position).layer(layer).build());
    }

    public int discard() throws ApiException {
        return api.discard(new SegmentRequest()
            .layer(this.handle)).getLayerCount();
    }

    public boolean discardSprite(int sprite) throws ApiException {
        return api.discardSprite(new HandleRequest()
            .layer(this.handle)
            .sprite(sprite)).getValue();
    }

    public SegmentIndexesResult indexes() throws ApiException {
        return api.indexes(new SegmentRequest()
            .layer(this.handle));
    }

    public void moveToBottom() throws ApiException {
        api.moveToBottom(new SegmentRequest()
            .layer(this.handle));
    }

    public void moveToTop() throws ApiException {
        api.moveToTop(new SegmentRequest()
            .layer(this.handle));
    }

    public void reverseDepth(SegmentReverseDepthOptions options) throws ApiException {
        api.reverseDepth(new SegmentReverseDepthRequest()
            .layer(this.handle)
            .value(options.value));
    }

    public void reverseDepth(Boolean value) throws ApiException {
        reverseDepth(SegmentReverseDepthOptions.builder().value(value).build());
    }
}
