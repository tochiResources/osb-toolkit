using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using SpriteApi = tochiRes.osbTK.Api.SpriteAPI;

namespace tochiRes.osbTK;

/// <summary>
/// A storyboard sprite, mirroring <c>OsbSprite</c>. Command methods are fluent
/// and come in three shapes - easing-first
/// (<c>Move(easing, start, end, ...)</c>), time-range (<c>Move(start, end, ...)</c>)
/// and time-only (<c>Move(time, ...)</c>); the <c>At</c> accessors sample them at a
/// time.
/// </summary>
public sealed class Sprite
{
    private readonly SpriteApi _api;

    internal Sprite(SpriteApi api, int layer, int sprite)
    {
        _api = api;
        LayerIndex = layer;
        Index = sprite;
    }

    /// <summary>The layer handle.</summary>
    public int LayerIndex { get; }

    /// <summary>The sprite handle within its layer.</summary>
    public int Index { get; }

    /// <summary>The sprite's initial state and metadata.</summary>
    public SpriteInfo Info => _api.Info(new HandleRequest(layer: LayerIndex, sprite: Index));

    /// <summary>Adds a move command.</summary>
    public Sprite Move(double startTime, double endTime, Vector2 start, Vector2 end, Easing easing = Easing.None) =>
        Add("move", startTime, endTime, easing, start, end);

    /// <summary>Adds a move command (easing-first).</summary>
    public Sprite Move(Easing easing, double startTime, double endTime, Vector2 start, Vector2 end) =>
        Move(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant move at a time.</summary>
    public Sprite Move(double time, Vector2 position) => Move(time, time, position, position);

    /// <summary>Adds a horizontal move command.</summary>
    public Sprite MoveX(double startTime, double endTime, double start, double end, Easing easing = Easing.None) =>
        Add("moveX", startTime, endTime, easing, start, end);

    /// <summary>Adds a horizontal move command (easing-first).</summary>
    public Sprite MoveX(Easing easing, double startTime, double endTime, double start, double end) =>
        MoveX(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant horizontal move.</summary>
    public Sprite MoveX(double time, double x) => MoveX(time, time, x, x);

    /// <summary>Adds a vertical move command.</summary>
    public Sprite MoveY(double startTime, double endTime, double start, double end, Easing easing = Easing.None) =>
        Add("moveY", startTime, endTime, easing, start, end);

    /// <summary>Adds a vertical move command (easing-first).</summary>
    public Sprite MoveY(Easing easing, double startTime, double endTime, double start, double end) =>
        MoveY(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant vertical move.</summary>
    public Sprite MoveY(double time, double y) => MoveY(time, time, y, y);

    /// <summary>Adds a uniform scale command.</summary>
    public Sprite Scale(double startTime, double endTime, double start, double end, Easing easing = Easing.None) =>
        Add("scale", startTime, endTime, easing, start, end);

    /// <summary>Adds a uniform scale command (easing-first).</summary>
    public Sprite Scale(Easing easing, double startTime, double endTime, double start, double end) =>
        Scale(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant uniform scale.</summary>
    public Sprite Scale(double time, double scale) => Scale(time, time, scale, scale);

    /// <summary>Adds a per-axis scale command.</summary>
    public Sprite ScaleVec(double startTime, double endTime, Vector2 start, Vector2 end, Easing easing = Easing.None) =>
        Add("scaleVec", startTime, endTime, easing, start, end);

    /// <summary>Adds a per-axis scale command (easing-first).</summary>
    public Sprite ScaleVec(Easing easing, double startTime, double endTime, Vector2 start, Vector2 end) =>
        ScaleVec(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant per-axis scale.</summary>
    public Sprite ScaleVec(double time, Vector2 scale) => ScaleVec(time, time, scale, scale);

    /// <summary>Adds a rotation command (radians).</summary>
    public Sprite Rotate(double startTime, double endTime, double start, double end, Easing easing = Easing.None) =>
        Add("rotate", startTime, endTime, easing, start, end);

    /// <summary>Adds a rotation command (easing-first).</summary>
    public Sprite Rotate(Easing easing, double startTime, double endTime, double start, double end) =>
        Rotate(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant rotation.</summary>
    public Sprite Rotate(double time, double rotation) => Rotate(time, time, rotation, rotation);

    /// <summary>Adds a fade command.</summary>
    public Sprite Fade(double startTime, double endTime, double start, double end, Easing easing = Easing.None) =>
        Add("fade", startTime, endTime, easing, start, end);

    /// <summary>Adds a fade command (easing-first).</summary>
    public Sprite Fade(Easing easing, double startTime, double endTime, double start, double end) =>
        Fade(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant opacity set.</summary>
    public Sprite Fade(double time, double opacity) => Fade(time, time, opacity, opacity);

    /// <summary>Adds a color command.</summary>
    public Sprite Color(double startTime, double endTime, Color4 start, Color4 end, Easing easing = Easing.None) =>
        Add("color", startTime, endTime, easing, start, end);

    /// <summary>Adds a color command (easing-first).</summary>
    public Sprite Color(Easing easing, double startTime, double endTime, Color4 start, Color4 end) =>
        Color(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant color set.</summary>
    public Sprite Color(double time, Color4 color) => Color(time, time, color, color);

    /// <summary>Adds a hue/saturation/brightness color command.</summary>
    public Sprite ColorHsb(double startTime, double endTime, Color4 start, Color4 end, Easing easing = Easing.None) =>
        Add("colorHsb", startTime, endTime, easing, start, end);

    /// <summary>Adds a hue/saturation/brightness color command (easing-first).</summary>
    public Sprite ColorHsb(Easing easing, double startTime, double endTime, Color4 start, Color4 end) =>
        ColorHsb(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant hue/saturation/brightness color set.</summary>
    public Sprite ColorHsb(double time, double hue, double saturation, double brightness) =>
        ColorHsb(
            time,
            time,
            new Color4(r: hue, g: saturation, b: brightness, a: 1),
            new Color4(r: hue, g: saturation, b: brightness, a: 1)
        );

    /// <summary>Adds a horizontal-flip parameter command.</summary>
    public Sprite FlipHorizontal(double startTime, double endTime, double start = 0, double end = 1, Easing easing = Easing.None) =>
        Add("flipH", startTime, endTime, easing, start, end);

    /// <summary>Adds a horizontal-flip parameter command (easing-first).</summary>
    public Sprite FlipHorizontal(Easing easing, double startTime, double endTime, double start = 0, double end = 1) =>
        FlipHorizontal(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant horizontal-flip toggle.</summary>
    public Sprite FlipHorizontal(double time, bool flipped = true) =>
        FlipHorizontal(time, time, flipped ? 0 : 1, flipped ? 1 : 0);

    /// <summary>Adds a vertical-flip parameter command.</summary>
    public Sprite FlipVertical(double startTime, double endTime, double start = 0, double end = 1, Easing easing = Easing.None) =>
        Add("flipV", startTime, endTime, easing, start, end);

    /// <summary>Adds a vertical-flip parameter command (easing-first).</summary>
    public Sprite FlipVertical(Easing easing, double startTime, double endTime, double start = 0, double end = 1) =>
        FlipVertical(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant vertical-flip toggle.</summary>
    public Sprite FlipVertical(double time, bool flipped = true) =>
        FlipVertical(time, time, flipped ? 0 : 1, flipped ? 1 : 0);

    /// <summary>Adds an additive-blend parameter command.</summary>
    public Sprite Additive(double startTime, double endTime, double start, double end, Easing easing = Easing.None) =>
        Add("additive", startTime, endTime, easing, start, end);

    /// <summary>Adds an additive-blend parameter command (easing-first).</summary>
    public Sprite Additive(Easing easing, double startTime, double endTime, double start, double end) =>
        Additive(startTime, endTime, start, end, easing);

    /// <summary>Adds an instant additive-blend toggle.</summary>
    public Sprite Additive(double time, bool additive = true) =>
        Additive(time, time, additive ? 0 : 1, additive ? 1 : 0);

    /// <summary>Adds a parameter command.</summary>
    public Sprite Parameter(ParameterType parameter, double startTime, double endTime, double start, double end, Easing easing = Easing.None) =>
        Add("parameter", startTime, endTime, easing, start, end, parameter);

    /// <summary>
    /// Adds an animatable origin-offset command (<c>Anchor</c>). The offset is in
    /// texture pixels and is added to the sprite's enum origin.
    /// </summary>
    public Sprite Anchor(double startTime, double endTime, Vector2 start, Vector2 end, Easing easing = Easing.None) =>
        Add("anchor", startTime, endTime, easing, start, end);

    /// <summary>Adds an animatable origin-offset command (easing-first).</summary>
    public Sprite Anchor(Easing easing, double startTime, double endTime, Vector2 start, Vector2 end) =>
        Anchor(startTime, endTime, start, end, easing);

    /// <summary>Sets a constant origin offset.</summary>
    public Sprite Anchor(double time, Vector2 point) => Anchor(time, time, point, point);

    /// <summary>Sets a constant origin offset.</summary>
    public Sprite SetAnchor(Vector2 point) => Anchor(0, 0, point, point);

    /// <summary>Samples the origin offset at a time.</summary>
    public Vector2 AnchorAt(double time) => StateAt(time).Anchor;

    /// <summary>Starts a loop group; subsequent commands join it until <see cref="EndGroup"/>.</summary>
    public Sprite StartLoopGroup(double startTime, double count)
    {
        _api.StartLoopGroup(
            new StartLoopGroupRequest(count: count, layer: LayerIndex, sprite: Index, startTime: startTime)
        );
        return this;
    }

    /// <summary>Starts a trigger group; subsequent commands join it until <see cref="EndGroup"/>.</summary>
    public Sprite StartTriggerGroup(string name, double startTime, double endTime, int group = 0)
    {
        _api.StartTriggerGroup(
            new StartTriggerGroupRequest(
                endTime: endTime,
                group: group,
                layer: LayerIndex,
                name: name,
                sprite: Index,
                startTime: startTime
            )
        );
        return this;
    }

    /// <summary>Closes the active loop or trigger group.</summary>
    public Sprite EndGroup()
    {
        _api.EndGroup(new HandleRequest(layer: LayerIndex, sprite: Index));
        return this;
    }

    /// <summary>Samples the sprite's full state at a time.</summary>
    public SpriteState StateAt(double time) =>
        _api.StateAt(new SpriteTimeRequest(layer: LayerIndex, sprite: Index, time: time));

    /// <summary>Samples the position at a time.</summary>
    public Vector2 PositionAt(double time) => StateAt(time).Position;

    /// <summary>Samples the scale at a time.</summary>
    public Vector2 ScaleAt(double time) => StateAt(time).Scale;

    /// <summary>Samples the rotation (radians) at a time.</summary>
    public double RotationAt(double time) => StateAt(time).Rotation;

    /// <summary>Samples the opacity at a time.</summary>
    public double OpacityAt(double time) => StateAt(time).Opacity;

    /// <summary>Samples the color at a time.</summary>
    public Color4 ColorAt(double time) => StateAt(time).Color;

    /// <summary>Whether the sprite is additive at a time.</summary>
    public bool AdditiveAt(double time) => StateAt(time).Additive;

    private Sprite Add(
        string kind,
        double startTime,
        double endTime,
        Easing easing,
        object? start,
        object? end,
        ParameterType? parameter = null
    )
    {
        _api.Command(
            new CommandRequest(
                easing: easing,
                end: end,
                endTime: endTime,
                kind: kind,
                layer: LayerIndex,
                varParameter: parameter,
                sprite: Index,
                start: start,
                startTime: startTime
            )
        );
        return this;
    }
}
