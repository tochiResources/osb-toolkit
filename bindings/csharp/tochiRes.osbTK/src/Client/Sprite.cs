using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Sprite
{
    private readonly SpriteAPI api;

    public Sprite(SpriteAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Sprite;

    public OkResult Command(CommandRequest body) => api.Command(body);

    public OkResult EndGroup(HandleRequest body) => api.EndGroup(body);

    public SpriteInfo Info(HandleRequest body) => api.Info(body);

    public OkResult StartLoopGroup(StartLoopGroupRequest body) => api.StartLoopGroup(body);

    public OkResult StartTriggerGroup(StartTriggerGroupRequest body) => api.StartTriggerGroup(body);

    public SpriteState StateAt(SpriteTimeRequest body) => api.StateAt(body);
}
