#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Core;

public sealed class Origin
{
    private readonly OriginAPI api;

    public Origin(OriginAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Origin;

    public Vector2 GetOriginVector(GetOriginVectorOptions options) => api.GetOriginVector(new GetOriginVectorRequest(options.Height, options.Origin, options.Width));

    public Vector2 GetOriginVector(OsbOrigin Origin, double Width, double Height) =>
        GetOriginVector(new GetOriginVectorOptions { Origin = Origin, Width = Width, Height = Height });

    public BoolResult InScreenBounds(InScreenBoundsOptions options) => api.InScreenBounds(new InScreenBoundsRequest(options.Origin, options.Position, options.Rotation, options.Size));

    public BoolResult InScreenBounds(Vector2 Position, Vector2 Size, double Rotation, Vector2 Origin) =>
        InScreenBounds(new InScreenBoundsOptions { Position = Position, Size = Size, Rotation = Rotation, Origin = Origin });
}
