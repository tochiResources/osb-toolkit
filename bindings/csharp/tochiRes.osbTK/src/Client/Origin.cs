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

    public Vector2 GetOriginVector(GetOriginVectorRequest body) => api.GetOriginVector(body);

    public BoolResult InScreenBounds(InScreenBoundsRequest body) => api.InScreenBounds(body);
}
