using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Origin
{
    private readonly OriginAPI api;

    public Origin(OriginAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Origin;

    public Vector2 GetOriginVector(GetOriginVectorOptions options) => api.GetOriginVector(new GetOriginVectorRequest(options.Height, options.Origin, options.Width));

    public BoolResult InScreenBounds(InScreenBoundsOptions options) => api.InScreenBounds(new InScreenBoundsRequest(options.Origin, options.Position, options.Rotation, options.Size));
}
