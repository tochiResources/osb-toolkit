using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using LayerApi = tochiRes.osbTK.Api.LayerAPI;
using SpriteApi = tochiRes.osbTK.Api.SpriteAPI;

namespace tochiRes.osbTK;

/// <summary>
/// A storyboard segment (the <c>StoryboardSegment</c> surface): a layer that
/// creates sprites, animations and samples, can nest sub-segments, and carries
/// the depth-ordering operations. The handle is implicit.
/// </summary>
public sealed class Layer
{
    private readonly LayerApi _api;
    private readonly SpriteApi _sprites;

    internal Layer(LayerApi api, SpriteApi sprites, string name, int handle)
    {
        _api = api;
        _sprites = sprites;
        Name = name;
        Handle = handle;
    }

    /// <summary>The layer name/identifier.</summary>
    public string Name { get; }

    /// <summary>The native segment handle.</summary>
    public int Handle { get; }

    /// <summary>Creates a sprite on the segment.</summary>
    public Sprite CreateSprite(string texturePath, OsbOrigin origin = OsbOrigin.Centre)
    {
        ArgumentException.ThrowIfNullOrEmpty(texturePath);
        Handle handle = _api.CreateSprite(
            new CreateSpriteRequest(layerHandle: Handle, origin: origin, path: texturePath)
        );
        return new Sprite(_sprites, handle.Layer, handle.Sprite);
    }

    /// <summary>Creates a positioned sprite on the segment.</summary>
    public Sprite CreateSprite(string texturePath, OsbOrigin origin, Vector2 position)
    {
        ArgumentException.ThrowIfNullOrEmpty(texturePath);
        Handle handle = _api.CreateSprite(
            new CreateSpriteRequest(
                layerHandle: Handle,
                origin: origin,
                path: texturePath,
                position: position
            )
        );
        return new Sprite(_sprites, handle.Layer, handle.Sprite);
    }

    /// <summary>Creates an animation sprite on the segment.</summary>
    public Sprite CreateAnimation(
        string texturePath,
        int frames,
        double frameDelay,
        OsbLoopType loopType,
        OsbOrigin origin,
        Vector2 position
    )
    {
        ArgumentException.ThrowIfNullOrEmpty(texturePath);
        Handle handle = _api.CreateAnimation(
            new CreateAnimationRequest(
                frameDelay: frameDelay,
                frames: frames,
                layerHandle: Handle,
                loopType: loopType,
                origin: origin,
                path: texturePath,
                position: position
            )
        );
        return new Sprite(_sprites, handle.Layer, handle.Sprite);
    }

    /// <summary>Adds a sound sample to the segment.</summary>
    public Layer CreateSample(string samplePath, double time, double volume = 100)
    {
        ArgumentException.ThrowIfNullOrEmpty(samplePath);
        _api.CreateSample(
            new CreateSampleRequest(layerHandle: Handle, path: samplePath, time: time, volume: volume)
        );
        return this;
    }

    /// <summary>Creates a segment nested in this one (<c>CreateSegment</c>).</summary>
    public Layer CreateSegment()
    {
        LayerHandle handle = _api.CreateSegment(new SegmentRequest(layer: Handle));
        return new Layer(_api, _sprites, string.Empty, handle.Layer);
    }

    /// <summary>Sets the reverse-depth flag (<c>ReverseDepth</c>).</summary>
    public Layer SetReverseDepth(bool value)
    {
        _api.ReverseDepth(new SegmentReverseDepthRequest(layer: Handle, value: value));
        return this;
    }

    /// <summary>Records a depth index (<c>AddIndex</c>).</summary>
    public Layer AddIndex(double index)
    {
        _api.AddIndex(new SegmentAddIndexRequest(index: index, layer: Handle));
        return this;
    }

    /// <summary>The depth indexes and bounds.</summary>
    public SegmentIndexesResult Indexes() => _api.Indexes(new SegmentRequest(layer: Handle));

    /// <summary>Moves the segment to another osu! layer (<c>ChangePosition</c>).</summary>
    public Layer ChangePosition(OsbLayer osbLayer)
    {
        _api.ChangePosition(new SegmentChangePositionRequest(layer: Handle, osbLayer: osbLayer));
        return this;
    }

    /// <summary>Moves the segment on top of its siblings (<c>MoveToTop</c>).</summary>
    public Layer MoveToTop()
    {
        _api.MoveToTop(new SegmentRequest(layer: Handle));
        return this;
    }

    /// <summary>Moves the segment below its siblings (<c>MoveToBottom</c>).</summary>
    public Layer MoveToBottom()
    {
        _api.MoveToBottom(new SegmentRequest(layer: Handle));
        return this;
    }

    /// <summary>Removes zero-duration sprites (<c>Clean</c>).</summary>
    public SegmentCleanResult Clean() => _api.Clean(new SegmentRequest(layer: Handle));

    /// <summary>Removes the segment (<c>Discard</c>).</summary>
    public SegmentDiscardResult Discard() => _api.Discard(new SegmentRequest(layer: Handle));

    /// <summary>Removes a sprite from output (<c>DiscardSprite</c>).</summary>
    public Layer DiscardSprite(Sprite sprite)
    {
        _api.DiscardSprite(new HandleRequest(layer: sprite.LayerIndex, sprite: sprite.Index));
        return this;
    }
}
