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

/**
 * The clean `Layer` client.
 */
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

    /**
     * Record a depth index on a segment.
     *
     * @param index The index to record.
     */
    public void addIndex(Double index) throws ApiException {
        addIndex(SegmentAddIndexOptions.builder().index(index).build());
    }

    /**
     * Record a depth index on a segment.
     *
     * @param options Record a depth index on a segment.
     */
    public void addIndex(SegmentAddIndexOptions options) throws ApiException {
        api.addIndex(new SegmentAddIndexRequest()
            .index(options.index)
            .layer(this.handle));
    }

    /**
     * Move a segment to another osu! layer (`ChangePosition`).
     *
     * @param osbLayer The destination osu! layer.
     */
    public void changePosition(OsbLayer osbLayer) throws ApiException {
        changePosition(SegmentChangePositionOptions.builder().osbLayer(osbLayer).build());
    }

    /**
     * Move a segment to another osu! layer (`ChangePosition`).
     *
     * @param options Move a segment to another osu! layer (`ChangePosition`).
     */
    public void changePosition(SegmentChangePositionOptions options) throws ApiException {
        api.changePosition(new SegmentChangePositionRequest()
            .layer(this.handle)
            .osbLayer(options.osbLayer));
    }

    /**
     * A segment handle request.
     *
     * @return The number of sprites removed
     */
    public int clean() throws ApiException {
        return api.clean(new SegmentRequest()
            .layer(this.handle)).getRemoved();
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
     * @return The new animation handle
     */
    public Sprite createAnimation(String path, Integer frames, Double frameDelay, OsbLoopType loopType, OsbOrigin origin, Vector2 position) throws ApiException {
        return createAnimation(CreateAnimationOptions.builder().path(path).frames(frames).frameDelay(frameDelay).loopType(loopType).origin(origin).position(position).build());
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
     * @param layer The target layer name (omitted when `layerHandle` is set).
     *
     * @return The new animation handle
     */
    public Sprite createAnimation(String path, Integer frames, Double frameDelay, OsbLoopType loopType, OsbOrigin origin, Vector2 position, String layer) throws ApiException {
        return createAnimation(CreateAnimationOptions.builder().path(path).frames(frames).frameDelay(frameDelay).loopType(loopType).origin(origin).position(position).layer(layer).build());
    }

    /**
     * Create an animation sprite.
     *
     * @param options Create an animation sprite.
     *
     * @return The new animation handle
     */
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

    /**
     * Create a sound sample on a layer.
     *
     * @param path The sample path (relative to the mapset).
     * @param time The sample time in milliseconds.
     */
    public void createSample(String path, Double time) throws ApiException {
        createSample(CreateSampleOptions.builder().path(path).time(time).build());
    }

    /**
     * Create a sound sample on a layer.
     *
     * @param path The sample path (relative to the mapset).
     * @param time The sample time in milliseconds.
     * @param volume The sample volume (`0..=100`).
     */
    public void createSample(String path, Double time, Double volume) throws ApiException {
        createSample(CreateSampleOptions.builder().path(path).time(time).volume(volume).build());
    }

    /**
     * Create a sound sample on a layer.
     *
     * @param path The sample path (relative to the mapset).
     * @param time The sample time in milliseconds.
     * @param volume The sample volume (`0..=100`).
     * @param layer The target layer name (omitted when `layerHandle` is set).
     */
    public void createSample(String path, Double time, Double volume, String layer) throws ApiException {
        createSample(CreateSampleOptions.builder().path(path).time(time).volume(volume).layer(layer).build());
    }

    /**
     * Create a sound sample on a layer.
     *
     * @param options Create a sound sample on a layer.
     */
    public void createSample(CreateSampleOptions options) throws ApiException {
        api.createSample(new CreateSampleRequest()
            .layer(options.layer)
            .layerHandle(this.handle)
            .path(options.path)
            .time(options.time)
            .volume(options.volume));
    }

    /**
     * A segment handle request.
     *
     * @return The new segment handle
     */
    public Layer createSegment() throws ApiException {
        LayerHandle handle = api.createSegment(new SegmentRequest()
            .layer(this.handle));
        return new Layer(this.api, this.sprites, handle.getLayer());
    }

    /**
     * Create a sprite.
     *
     * @param path The texture path (relative to the mapset).
     * @param origin The sprite anchor origin.
     *
     * @return The new sprite handle
     */
    public Sprite createSprite(String path, OsbOrigin origin) throws ApiException {
        return createSprite(CreateSpriteOptions.builder().path(path).origin(origin).build());
    }

    /**
     * Create a sprite.
     *
     * @param path The texture path (relative to the mapset).
     * @param origin The sprite anchor origin.
     * @param position The initial position; omitted means the sprite's default.
     *
     * @return The new sprite handle
     */
    public Sprite createSprite(String path, OsbOrigin origin, Vector2 position) throws ApiException {
        return createSprite(CreateSpriteOptions.builder().path(path).origin(origin).position(position).build());
    }

    /**
     * Create a sprite.
     *
     * @param path The texture path (relative to the mapset).
     * @param origin The sprite anchor origin.
     * @param position The initial position; omitted means the sprite's default.
     * @param layer The target layer name (omitted when `layerHandle` is set).
     *
     * @return The new sprite handle
     */
    public Sprite createSprite(String path, OsbOrigin origin, Vector2 position, String layer) throws ApiException {
        return createSprite(CreateSpriteOptions.builder().path(path).origin(origin).position(position).layer(layer).build());
    }

    /**
     * Create a sprite.
     *
     * @param options Create a sprite.
     *
     * @return The new sprite handle
     */
    public Sprite createSprite(CreateSpriteOptions options) throws ApiException {
        Handle handle = api.createSprite(new CreateSpriteRequest()
            .layer(options.layer)
            .layerHandle(this.handle)
            .origin(options.origin)
            .path(options.path)
            .position(options.position));
        return new Sprite(this.sprites, handle.getLayer(), handle.getSprite());
    }

    /**
     * A segment handle request.
     *
     * @return The remaining top-level layer count
     */
    public int discard() throws ApiException {
        return api.discard(new SegmentRequest()
            .layer(this.handle)).getLayerCount();
    }

    /**
     * A sprite handle pair.
     *
     * @param sprite The sprite handle.
     *
     * @return Whether the sprite was discarded
     */
    public boolean discardSprite(int sprite) throws ApiException {
        return api.discardSprite(new HandleRequest()
            .layer(this.handle)
            .sprite(sprite)).getValue();
    }

    /**
     * A segment handle request.
     *
     * @return The segment's indexes and bounds
     */
    public SegmentIndexesResult indexes() throws ApiException {
        return api.indexes(new SegmentRequest()
            .layer(this.handle));
    }

    /**
     * A segment handle request.
     */
    public void moveToBottom() throws ApiException {
        api.moveToBottom(new SegmentRequest()
            .layer(this.handle));
    }

    /**
     * A segment handle request.
     */
    public void moveToTop() throws ApiException {
        api.moveToTop(new SegmentRequest()
            .layer(this.handle));
    }

    /**
     * Set a segment's reverse-depth flag.
     *
     * @param value Whether children are emitted in reverse order.
     */
    public void reverseDepth(Boolean value) throws ApiException {
        reverseDepth(SegmentReverseDepthOptions.builder().value(value).build());
    }

    /**
     * Set a segment's reverse-depth flag.
     *
     * @param options Set a segment's reverse-depth flag.
     */
    public void reverseDepth(SegmentReverseDepthOptions options) throws ApiException {
        api.reverseDepth(new SegmentReverseDepthRequest()
            .layer(this.handle)
            .value(options.value));
    }
}
