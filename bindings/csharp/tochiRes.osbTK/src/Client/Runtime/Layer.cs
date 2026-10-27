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

    public OkResult AddIndex(SegmentAddIndexOptions options) => api.AddIndex(new SegmentAddIndexRequest(options.Index, this.handle));

    public OkResult ChangePosition(SegmentChangePositionOptions options) => api.ChangePosition(new SegmentChangePositionRequest(this.handle, options.OsbLayer));

    public SegmentCleanResult Clean() => api.Clean(new SegmentRequest(this.handle));

    public Sprite CreateAnimation(CreateAnimationOptions options)
    {
        var handle = api.CreateAnimation(new CreateAnimationRequest(options.FrameDelay, options.Frames, options.Layer, this.handle, options.LoopType, options.Origin, options.Path, options.Position));
        return new Sprite(this.sprites, handle.Layer, handle.Sprite);
    }

    public OkResult CreateSample(CreateSampleOptions options) => api.CreateSample(new CreateSampleRequest(options.Layer, this.handle, options.Path, options.Time, options.Volume));

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

    public SegmentDiscardResult Discard() => api.Discard(new SegmentRequest(this.handle));

    public BoolResult DiscardSprite(int sprite) => api.DiscardSprite(new HandleRequest(this.handle, sprite));

    public SegmentIndexesResult Indexes() => api.Indexes(new SegmentRequest(this.handle));

    public OkResult MoveToBottom() => api.MoveToBottom(new SegmentRequest(this.handle));

    public OkResult MoveToTop() => api.MoveToTop(new SegmentRequest(this.handle));

    public OkResult ReverseDepth(SegmentReverseDepthOptions options) => api.ReverseDepth(new SegmentReverseDepthRequest(this.handle, options.Value));
}
