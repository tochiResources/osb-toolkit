#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Runtime;

public sealed class Sprite
{
    private readonly SpriteAPI api;
    private readonly int layer;
    private readonly int sprite;

    public Sprite(SpriteAPI api, int layer, int sprite)
    {
        this.api = api;
        this.layer = layer;
        this.sprite = sprite;
    }

    public static bool InstanceOf(object value) => value is Sprite;

    /// <summary>
    /// A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
    /// </summary>
    /// <param name="Kind">The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).</param>
    /// <param name="StartTime">The command start time in milliseconds.</param>
    /// <param name="EndTime">The command end time in milliseconds.</param>
    /// <param name="Easing">The easing function.</param>
    public void Command(string Kind, double StartTime, double EndTime, Easing Easing) =>
        Command(new CommandOptions { Kind = Kind, StartTime = StartTime, EndTime = EndTime, Easing = Easing });

    /// <summary>
    /// A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
    /// </summary>
    /// <param name="Kind">The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).</param>
    /// <param name="StartTime">The command start time in milliseconds.</param>
    /// <param name="EndTime">The command end time in milliseconds.</param>
    /// <param name="Easing">The easing function.</param>
    /// <param name="Start">The start value (scalar, point or color).</param>
    public void Command(string Kind, double StartTime, double EndTime, Easing Easing, object Start) =>
        Command(new CommandOptions { Kind = Kind, StartTime = StartTime, EndTime = EndTime, Easing = Easing, Start = Start });

    /// <summary>
    /// A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
    /// </summary>
    /// <param name="Kind">The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).</param>
    /// <param name="StartTime">The command start time in milliseconds.</param>
    /// <param name="EndTime">The command end time in milliseconds.</param>
    /// <param name="Easing">The easing function.</param>
    /// <param name="Start">The start value (scalar, point or color).</param>
    /// <param name="End">The end value (scalar, point or color).</param>
    public void Command(string Kind, double StartTime, double EndTime, Easing Easing, object Start, object End) =>
        Command(new CommandOptions { Kind = Kind, StartTime = StartTime, EndTime = EndTime, Easing = Easing, Start = Start, End = End });

    /// <summary>
    /// A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
    /// </summary>
    /// <param name="Kind">The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).</param>
    /// <param name="StartTime">The command start time in milliseconds.</param>
    /// <param name="EndTime">The command end time in milliseconds.</param>
    /// <param name="Easing">The easing function.</param>
    /// <param name="Start">The start value (scalar, point or color).</param>
    /// <param name="End">The end value (scalar, point or color).</param>
    /// <param name="Parameter">The parameter kind for a `parameter` command.</param>
    public void Command(string Kind, double StartTime, double EndTime, Easing Easing, object Start, object End, ParameterType? Parameter) =>
        Command(new CommandOptions { Kind = Kind, StartTime = StartTime, EndTime = EndTime, Easing = Easing, Start = Start, End = End, Parameter = Parameter });

    /// <summary>
    /// A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
    /// </summary>
    /// <param name="options">A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.</param>
    public void Command(CommandOptions options)
    {
        api.Command(new CommandRequest(options.Easing, options.End, options.EndTime, options.Kind, this.layer, options.Parameter, this.sprite, options.Start, options.StartTime));
    }

    /// <summary>
    /// A sprite handle pair.
    /// </summary>
    public void EndGroup()
    {
        api.EndGroup(new HandleRequest(this.layer, this.sprite));
    }

    /// <summary>
    /// A sprite handle pair.
    /// </summary>
    /// <returns>The sprite's initial state</returns>
    public SpriteInfo Info() => api.Info(new HandleRequest(this.layer, this.sprite));

    /// <summary>
    /// Start a loop group.
    /// </summary>
    /// <param name="StartTime">The loop start time in milliseconds.</param>
    /// <param name="Count">The number of iterations.</param>
    public void StartLoopGroup(double StartTime, double Count) =>
        StartLoopGroup(new StartLoopGroupOptions { StartTime = StartTime, Count = Count });

    /// <summary>
    /// Start a loop group.
    /// </summary>
    /// <param name="options">Start a loop group.</param>
    public void StartLoopGroup(StartLoopGroupOptions options)
    {
        api.StartLoopGroup(new StartLoopGroupRequest(options.Count, this.layer, this.sprite, options.StartTime));
    }

    /// <summary>
    /// Start a trigger group.
    /// </summary>
    /// <param name="Name">The trigger name (matches a hit object).</param>
    /// <param name="StartTime">The trigger start time in milliseconds.</param>
    /// <param name="EndTime">The trigger end time in milliseconds.</param>
    /// <param name="Group">The trigger group index.</param>
    public void StartTriggerGroup(string Name, double StartTime, double EndTime, int Group) =>
        StartTriggerGroup(new StartTriggerGroupOptions { Name = Name, StartTime = StartTime, EndTime = EndTime, Group = Group });

    /// <summary>
    /// Start a trigger group.
    /// </summary>
    /// <param name="options">Start a trigger group.</param>
    public void StartTriggerGroup(StartTriggerGroupOptions options)
    {
        api.StartTriggerGroup(new StartTriggerGroupRequest(options.EndTime, options.Group, this.layer, options.Name, this.sprite, options.StartTime));
    }

    /// <summary>
    /// A sprite handle pair plus a sample time.
    /// </summary>
    /// <param name="Time">The sample time in milliseconds.</param>
    /// <returns>The sampled sprite state</returns>
    public SpriteState StateAt(double Time) =>
        StateAt(new SpriteTimeOptions { Time = Time });

    /// <summary>
    /// A sprite handle pair plus a sample time.
    /// </summary>
    /// <param name="options">A sprite handle pair plus a sample time.</param>
    /// <returns>The sampled sprite state</returns>
    public SpriteState StateAt(SpriteTimeOptions options) => api.StateAt(new SpriteTimeRequest(this.layer, this.sprite, options.Time));
}
