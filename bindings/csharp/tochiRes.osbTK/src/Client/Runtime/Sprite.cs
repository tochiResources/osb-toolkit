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

    public void Command(string Kind, double StartTime, double EndTime, Easing Easing) =>
        Command(new CommandOptions { Kind = Kind, StartTime = StartTime, EndTime = EndTime, Easing = Easing });

    public void Command(string Kind, double StartTime, double EndTime, Easing Easing, object Start) =>
        Command(new CommandOptions { Kind = Kind, StartTime = StartTime, EndTime = EndTime, Easing = Easing, Start = Start });

    public void Command(string Kind, double StartTime, double EndTime, Easing Easing, object Start, object End) =>
        Command(new CommandOptions { Kind = Kind, StartTime = StartTime, EndTime = EndTime, Easing = Easing, Start = Start, End = End });

    public void Command(string Kind, double StartTime, double EndTime, Easing Easing, object Start, object End, ParameterType? Parameter) =>
        Command(new CommandOptions { Kind = Kind, StartTime = StartTime, EndTime = EndTime, Easing = Easing, Start = Start, End = End, Parameter = Parameter });

    public void Command(CommandOptions options)
    {
        api.Command(new CommandRequest(options.Easing, options.End, options.EndTime, options.Kind, this.layer, options.Parameter, this.sprite, options.Start, options.StartTime));
    }

    public void EndGroup()
    {
        api.EndGroup(new HandleRequest(this.layer, this.sprite));
    }

    public SpriteInfo Info() => api.Info(new HandleRequest(this.layer, this.sprite));

    public void StartLoopGroup(double StartTime, double Count) =>
        StartLoopGroup(new StartLoopGroupOptions { StartTime = StartTime, Count = Count });

    public void StartLoopGroup(StartLoopGroupOptions options)
    {
        api.StartLoopGroup(new StartLoopGroupRequest(options.Count, this.layer, this.sprite, options.StartTime));
    }

    public void StartTriggerGroup(string Name, double StartTime, double EndTime, int Group) =>
        StartTriggerGroup(new StartTriggerGroupOptions { Name = Name, StartTime = StartTime, EndTime = EndTime, Group = Group });

    public void StartTriggerGroup(StartTriggerGroupOptions options)
    {
        api.StartTriggerGroup(new StartTriggerGroupRequest(options.EndTime, options.Group, this.layer, options.Name, this.sprite, options.StartTime));
    }

    public SpriteState StateAt(double Time) =>
        StateAt(new SpriteTimeOptions { Time = Time });

    public SpriteState StateAt(SpriteTimeOptions options) => api.StateAt(new SpriteTimeRequest(this.layer, this.sprite, options.Time));
}
