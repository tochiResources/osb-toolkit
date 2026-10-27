using Newtonsoft.Json.Linq;

using tochiRes.osbTK.Model;

namespace tochiRes.osbTK;

public static partial class osb
{
    public static partial class Helpers
    {
        /// <summary>The anchor offset for an enum origin, in texture pixels.</summary>
        public static Vector2 OriginVector(OsbOrigin origin, double width, double height) =>
            Call("origin.getOriginVector", new JObject
            {
                ["origin"] = origin.ToString(),
                ["width"] = width,
                ["height"] = height,
            }).ToObject<Vector2>()!;

        /// <summary>The effective pivot: the enum origin vector plus an explicit anchor offset.</summary>
        public static Vector2 ResolveAnchor(
            OsbOrigin origin,
            Vector2 anchor,
            double width,
            double height
        ) =>
            Call("origin.resolveAnchor", new JObject
            {
                ["origin"] = origin.ToString(),
                ["anchor"] = JToken.FromObject(anchor),
                ["width"] = width,
                ["height"] = height,
            }).ToObject<Vector2>()!;

        /// <summary>Whether a rotated sprite's bounds intersect the screen.</summary>
        public static bool InScreenBounds(
            Vector2 position,
            Vector2 size,
            double rotation,
            Vector2 origin
        ) =>
            Call("origin.inScreenBounds", new JObject
            {
                ["position"] = JToken.FromObject(position),
                ["size"] = JToken.FromObject(size),
                ["rotation"] = rotation,
                ["origin"] = JToken.FromObject(origin),
            }).Value<bool>();

        /// <summary>The 3D effective pivot: a <see cref="Vector3"/> anchor whose z anchors depth.</summary>
        public static Vector3 ResolveAnchor3d(
            OsbOrigin origin,
            Vector3 anchor,
            double width,
            double height
        ) =>
            Call("origin.resolveAnchor3d", new JObject
            {
                ["origin"] = origin.ToString(),
                ["anchor"] = JToken.FromObject(anchor),
                ["width"] = width,
                ["height"] = height,
            }).ToObject<Vector3>()!;
    }
}
