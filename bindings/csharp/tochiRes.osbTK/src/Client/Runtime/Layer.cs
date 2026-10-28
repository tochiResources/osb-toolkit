#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Runtime;

public sealed class Layer
{
    private readonly LayerAPI api;
    private readonly SpriteAPI sprites;
    private readonly int handle;

    public Layer(LayerAPI api, SpriteAPI sprites, int handle)
    {
        this.api = api;
        this.sprites = sprites;
        this.handle = handle;
    }

    public static bool InstanceOf(object value) => value is Layer;

    /// <summary>
    /// Record a depth index on a segment.
    /// </summary>
    /// <param name="Index">The index to record.</param>
    public void AddIndex(double Index) =>
        AddIndex(new SegmentAddIndexOptions { Index = Index });

    /// <summary>
    /// Record a depth index on a segment.
    /// </summary>
    /// <param name="options">Record a depth index on a segment.</param>
    public void AddIndex(SegmentAddIndexOptions options)
    {
        api.AddIndex(new SegmentAddIndexRequest(options.Index, this.handle));
    }

    /// <summary>
    /// Move a segment to another osu! layer (`ChangePosition`).
    /// </summary>
    /// <param name="OsbLayer">The destination osu! layer.</param>
    public void ChangePosition(OsbLayer OsbLayer) =>
        ChangePosition(new SegmentChangePositionOptions { OsbLayer = OsbLayer });

    /// <summary>
    /// Move a segment to another osu! layer (`ChangePosition`).
    /// </summary>
    /// <param name="options">Move a segment to another osu! layer (`ChangePosition`).</param>
    public void ChangePosition(SegmentChangePositionOptions options)
    {
        api.ChangePosition(new SegmentChangePositionRequest(this.handle, options.OsbLayer));
    }

    /// <summary>
    /// A segment handle request.
    /// </summary>
    /// <returns>The number of sprites removed</returns>
    public int Clean() => api.Clean(new SegmentRequest(this.handle)).Removed;

    /// <summary>
    /// Create an animation sprite.
    /// </summary>
    /// <param name="Path">The texture path (relative to the mapset).</param>
    /// <param name="Frames">The number of frames.</param>
    /// <param name="FrameDelay">Delay between frames, in milliseconds.</param>
    /// <param name="LoopType">How the animation loops.</param>
    /// <param name="Origin">The sprite anchor origin.</param>
    /// <param name="Position">The initial position.</param>
    /// <returns>The new animation handle</returns>
    public Sprite CreateAnimation(string Path, int Frames, double FrameDelay, OsbLoopType LoopType, OsbOrigin Origin, Vector2 Position) =>
        CreateAnimation(new CreateAnimationOptions { Path = Path, Frames = Frames, FrameDelay = FrameDelay, LoopType = LoopType, Origin = Origin, Position = Position });

    /// <summary>
    /// Create an animation sprite.
    /// </summary>
    /// <param name="Path">The texture path (relative to the mapset).</param>
    /// <param name="Frames">The number of frames.</param>
    /// <param name="FrameDelay">Delay between frames, in milliseconds.</param>
    /// <param name="LoopType">How the animation loops.</param>
    /// <param name="Origin">The sprite anchor origin.</param>
    /// <param name="Position">The initial position.</param>
    /// <param name="Layer">The target layer name (omitted when `layerHandle` is set).</param>
    /// <returns>The new animation handle</returns>
    public Sprite CreateAnimation(string Path, int Frames, double FrameDelay, OsbLoopType LoopType, OsbOrigin Origin, Vector2 Position, string Layer) =>
        CreateAnimation(new CreateAnimationOptions { Path = Path, Frames = Frames, FrameDelay = FrameDelay, LoopType = LoopType, Origin = Origin, Position = Position, Layer = Layer });

    /// <summary>
    /// Create an animation sprite.
    /// </summary>
    /// <param name="options">Create an animation sprite.</param>
    /// <returns>The new animation handle</returns>
    public Sprite CreateAnimation(CreateAnimationOptions options)
    {
        var handle = api.CreateAnimation(new CreateAnimationRequest(options.FrameDelay, options.Frames, options.Layer, this.handle, options.LoopType, options.Origin, options.Path, options.Position));
        return new Sprite(this.sprites, handle.Layer, handle.Sprite);
    }

    /// <summary>
    /// Create a sound sample on a layer.
    /// </summary>
    /// <param name="Path">The sample path (relative to the mapset).</param>
    /// <param name="Time">The sample time in milliseconds.</param>
    public void CreateSample(string Path, double Time) =>
        CreateSample(new CreateSampleOptions { Path = Path, Time = Time });

    /// <summary>
    /// Create a sound sample on a layer.
    /// </summary>
    /// <param name="Path">The sample path (relative to the mapset).</param>
    /// <param name="Time">The sample time in milliseconds.</param>
    /// <param name="Volume">The sample volume (`0..=100`).</param>
    public void CreateSample(string Path, double Time, double? Volume) =>
        CreateSample(new CreateSampleOptions { Path = Path, Time = Time, Volume = Volume });

    /// <summary>
    /// Create a sound sample on a layer.
    /// </summary>
    /// <param name="Path">The sample path (relative to the mapset).</param>
    /// <param name="Time">The sample time in milliseconds.</param>
    /// <param name="Volume">The sample volume (`0..=100`).</param>
    /// <param name="Layer">The target layer name (omitted when `layerHandle` is set).</param>
    public void CreateSample(string Path, double Time, double? Volume, string Layer) =>
        CreateSample(new CreateSampleOptions { Path = Path, Time = Time, Volume = Volume, Layer = Layer });

    /// <summary>
    /// Create a sound sample on a layer.
    /// </summary>
    /// <param name="options">Create a sound sample on a layer.</param>
    public void CreateSample(CreateSampleOptions options)
    {
        api.CreateSample(new CreateSampleRequest(options.Layer, this.handle, options.Path, options.Time, options.Volume));
    }

    /// <summary>
    /// A segment handle request.
    /// </summary>
    /// <returns>The new segment handle</returns>
    public Layer CreateSegment()
    {
        var handle = api.CreateSegment(new SegmentRequest(this.handle));
        return new Layer(this.api, this.sprites, handle.Layer);
    }

    /// <summary>
    /// Create a sprite.
    /// </summary>
    /// <param name="Path">The texture path (relative to the mapset).</param>
    /// <param name="Origin">The sprite anchor origin.</param>
    /// <returns>The new sprite handle</returns>
    public Sprite CreateSprite(string Path, OsbOrigin Origin) =>
        CreateSprite(new CreateSpriteOptions { Path = Path, Origin = Origin });

    /// <summary>
    /// Create a sprite.
    /// </summary>
    /// <param name="Path">The texture path (relative to the mapset).</param>
    /// <param name="Origin">The sprite anchor origin.</param>
    /// <param name="Position">The initial position; omitted means the sprite's default.</param>
    /// <returns>The new sprite handle</returns>
    public Sprite CreateSprite(string Path, OsbOrigin Origin, Vector2 Position) =>
        CreateSprite(new CreateSpriteOptions { Path = Path, Origin = Origin, Position = Position });

    /// <summary>
    /// Create a sprite.
    /// </summary>
    /// <param name="Path">The texture path (relative to the mapset).</param>
    /// <param name="Origin">The sprite anchor origin.</param>
    /// <param name="Position">The initial position; omitted means the sprite's default.</param>
    /// <param name="Layer">The target layer name (omitted when `layerHandle` is set).</param>
    /// <returns>The new sprite handle</returns>
    public Sprite CreateSprite(string Path, OsbOrigin Origin, Vector2 Position, string Layer) =>
        CreateSprite(new CreateSpriteOptions { Path = Path, Origin = Origin, Position = Position, Layer = Layer });

    /// <summary>
    /// Create a sprite.
    /// </summary>
    /// <param name="options">Create a sprite.</param>
    /// <returns>The new sprite handle</returns>
    public Sprite CreateSprite(CreateSpriteOptions options)
    {
        var handle = api.CreateSprite(new CreateSpriteRequest(options.Layer, this.handle, options.Origin, options.Path, options.Position));
        return new Sprite(this.sprites, handle.Layer, handle.Sprite);
    }

    /// <summary>
    /// A segment handle request.
    /// </summary>
    /// <returns>The remaining top-level layer count</returns>
    public int Discard() => api.Discard(new SegmentRequest(this.handle)).LayerCount;

    /// <summary>
    /// A sprite handle pair.
    /// </summary>
    /// <param name="sprite">The sprite handle.</param>
    /// <returns>Whether the sprite was discarded</returns>
    public bool DiscardSprite(int sprite) => api.DiscardSprite(new HandleRequest(this.handle, sprite)).Value;

    /// <summary>
    /// A segment handle request.
    /// </summary>
    /// <returns>The segment's indexes and bounds</returns>
    public SegmentIndexesResult Indexes() => api.Indexes(new SegmentRequest(this.handle));

    /// <summary>
    /// A segment handle request.
    /// </summary>
    public void MoveToBottom()
    {
        api.MoveToBottom(new SegmentRequest(this.handle));
    }

    /// <summary>
    /// A segment handle request.
    /// </summary>
    public void MoveToTop()
    {
        api.MoveToTop(new SegmentRequest(this.handle));
    }

    /// <summary>
    /// Set a segment's reverse-depth flag.
    /// </summary>
    /// <param name="Value">Whether children are emitted in reverse order.</param>
    public void ReverseDepth(bool Value) =>
        ReverseDepth(new SegmentReverseDepthOptions { Value = Value });

    /// <summary>
    /// Set a segment's reverse-depth flag.
    /// </summary>
    /// <param name="options">Set a segment's reverse-depth flag.</param>
    public void ReverseDepth(SegmentReverseDepthOptions options)
    {
        api.ReverseDepth(new SegmentReverseDepthRequest(this.handle, options.Value));
    }
}
