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

    public void AddIndex(SegmentAddIndexOptions options)
    {
        api.AddIndex(new SegmentAddIndexRequest(options.Index, this.handle));
    }

    public void AddIndex(double Index) =>
        AddIndex(new SegmentAddIndexOptions { Index = Index });

    public void ChangePosition(SegmentChangePositionOptions options)
    {
        api.ChangePosition(new SegmentChangePositionRequest(this.handle, options.OsbLayer));
    }

    public void ChangePosition(OsbLayer OsbLayer) =>
        ChangePosition(new SegmentChangePositionOptions { OsbLayer = OsbLayer });

    public int Clean() => api.Clean(new SegmentRequest(this.handle)).Removed;

    public Sprite CreateAnimation(CreateAnimationOptions options)
    {
        var handle = api.CreateAnimation(new CreateAnimationRequest(options.FrameDelay, options.Frames, options.Layer, this.handle, options.LoopType, options.Origin, options.Path, options.Position));
        return new Sprite(this.sprites, handle.Layer, handle.Sprite);
    }

    public Sprite CreateAnimation(string Path, int Frames, double FrameDelay, OsbLoopType LoopType, OsbOrigin Origin, Vector2 Position, string Layer = default) =>
        CreateAnimation(new CreateAnimationOptions { Path = Path, Frames = Frames, FrameDelay = FrameDelay, LoopType = LoopType, Origin = Origin, Position = Position, Layer = Layer });

    public void CreateSample(CreateSampleOptions options)
    {
        api.CreateSample(new CreateSampleRequest(options.Layer, this.handle, options.Path, options.Time, options.Volume));
    }

    public void CreateSample(string Path, double Time, double? Volume = default, string Layer = default) =>
        CreateSample(new CreateSampleOptions { Path = Path, Time = Time, Volume = Volume, Layer = Layer });

    public Layer CreateSegment()
    {
        var handle = api.CreateSegment(new SegmentRequest(this.handle));
        return new Layer(this.api, this.sprites, handle.Layer);
    }

    public Sprite CreateSprite(CreateSpriteOptions options)
    {
        var handle = api.CreateSprite(new CreateSpriteRequest(options.Layer, this.handle, options.Origin, options.Path, options.Position));
        return new Sprite(this.sprites, handle.Layer, handle.Sprite);
    }

    public Sprite CreateSprite(string Path, OsbOrigin Origin, Vector2 Position = default, string Layer = default) =>
        CreateSprite(new CreateSpriteOptions { Path = Path, Origin = Origin, Position = Position, Layer = Layer });

    public int Discard() => api.Discard(new SegmentRequest(this.handle)).LayerCount;

    public bool DiscardSprite(int sprite) => api.DiscardSprite(new HandleRequest(this.handle, sprite)).Value;

    public SegmentIndexesResult Indexes() => api.Indexes(new SegmentRequest(this.handle));

    public void MoveToBottom()
    {
        api.MoveToBottom(new SegmentRequest(this.handle));
    }

    public void MoveToTop()
    {
        api.MoveToTop(new SegmentRequest(this.handle));
    }

    public void ReverseDepth(SegmentReverseDepthOptions options)
    {
        api.ReverseDepth(new SegmentReverseDepthRequest(this.handle, options.Value));
    }

    public void ReverseDepth(bool Value) =>
        ReverseDepth(new SegmentReverseDepthOptions { Value = Value });
}
