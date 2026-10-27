using Newtonsoft.Json.Linq;

using tochiRes.osbTK.Model;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>
    /// The 2D scene graph (M40): the 2D counterpart of the 3D scene tree. A node
    /// can be controlled from the top and the whole set cascades; <c>Resolve</c>
    /// samples the composed transforms into per-sprite command sets.
    /// </summary>
    public static partial class Helpers
    {
        private static JToken RunScene(string name, JToken scene, JObject? extra = null)
        {
            JObject input = extra ?? new JObject();
            input["scene"] = scene;
            return Call(name, input);
        }

        /// <summary>Resolves the scene into per-sprite command sets.</summary>
        public static JToken ResolveScene2d(JToken scene, JObject? options = null) =>
            Call("scene2d.resolve", new JObject
            {
                ["scene"] = scene,
                ["options"] = options ?? new JObject(),
            });

        /// <summary>Bakes the scene into a full storyboard model (per-node layers).</summary>
        public static Storyboard BakeScene2d(JToken scene, JObject? options = null) =>
            Call("scene2d.bake", new JObject
            {
                ["scene"] = scene,
                ["options"] = options ?? new JObject(),
            }).ToObject<Storyboard>()!;

        /// <summary>The scene's occupied [start, end] range, or null.</summary>
        public static JToken Scene2dTimeRange(JToken scene) =>
            Call("scene2d.timeRange", new JObject { ["scene"] = scene });

        /// <summary>Translates the whole scene (children inherit).</summary>
        public static JToken TranslateScene2d(JToken scene, double x, double y) =>
            RunScene("scene2d.translate", scene, new JObject { ["delta"] = new JArray(x, y) });

        /// <summary>Scales the whole scene about its origin.</summary>
        public static JToken ScaleScene2d(JToken scene, double factor) =>
            RunScene("scene2d.scale", scene, new JObject { ["factor"] = factor });

        /// <summary>Rotates the whole scene (radians).</summary>
        public static JToken RotateScene2d(JToken scene, double radians) =>
            RunScene("scene2d.rotate", scene, new JObject { ["radians"] = radians });

        /// <summary>Sets a constant opacity on the root (children inherit).</summary>
        public static JToken SetScene2dOpacity(JToken scene, double value) =>
            RunScene("scene2d.setOpacity", scene, new JObject { ["value"] = value });

        /// <summary>Sets a constant tint on the root (children inherit).</summary>
        public static JToken SetScene2dColor(JToken scene, Color4 color) =>
            RunScene("scene2d.setColor", scene, new JObject { ["color"] = JToken.FromObject(color) });

        /// <summary>Flattens the whole scene.</summary>
        public static JToken FlattenScene2d(JToken scene, string? axis = null, string mode = "mean")
        {
            JObject extra = new() { ["mode"] = mode };
            if (axis is not null)
            {
                extra["axis"] = axis;
            }
            return RunScene("scene2d.flatten", scene, extra);
        }

        /// <summary>Shifts the whole scene in time.</summary>
        public static JToken ShiftScene2d(JToken scene, double delta) =>
            RunScene("scene2d.shift", scene, new JObject { ["delta"] = delta });

        /// <summary>Scales the whole scene in time.</summary>
        public static JToken ScaleScene2dTime(JToken scene, double factor, double anchor = 0.0) =>
            RunScene("scene2d.scaleTime", scene, new JObject { ["factor"] = factor, ["anchor"] = anchor });

        /// <summary>Reverses the whole scene in time.</summary>
        public static JToken ReverseScene2d(JToken scene, double start = 0.0, double end = 0.0) =>
            RunScene("scene2d.reverse", scene, new JObject { ["start"] = start, ["end"] = end });
    }
}
