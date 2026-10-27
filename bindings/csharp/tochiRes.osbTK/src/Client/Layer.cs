using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Layer
{
    private readonly LayerAPI api;

    public Layer(LayerAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Layer;

    public OkResult AddIndex(SegmentAddIndexRequest body) => api.AddIndex(body);

    public OkResult ChangePosition(SegmentChangePositionRequest body) => api.ChangePosition(body);

    public SegmentCleanResult Clean(SegmentRequest body) => api.Clean(body);

    public Handle CreateAnimation(CreateAnimationRequest body) => api.CreateAnimation(body);

    public OkResult CreateSample(CreateSampleRequest body) => api.CreateSample(body);

    public LayerHandle CreateSegment(SegmentRequest body) => api.CreateSegment(body);

    public Handle CreateSprite(CreateSpriteRequest body) => api.CreateSprite(body);

    public SegmentDiscardResult Discard(SegmentRequest body) => api.Discard(body);

    public BoolResult DiscardSprite(HandleRequest body) => api.DiscardSprite(body);

    public SegmentIndexesResult Indexes(SegmentRequest body) => api.Indexes(body);

    public OkResult MoveToBottom(SegmentRequest body) => api.MoveToBottom(body);

    public OkResult MoveToTop(SegmentRequest body) => api.MoveToTop(body);

    public OkResult ReverseDepth(SegmentReverseDepthRequest body) => api.ReverseDepth(body);
}
