using Newtonsoft.Json.Linq;

using tochiRes.osbTK.Model;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>
    /// The 3D world and unified storyboard bake: a world
    /// (<c>{ nodes, camera, options }</c>) or a set of 2D + 3D worlds bakes into a
    /// full <see cref="Storyboard"/> model.
    /// </summary>
    public static partial class Helpers
    {
        /// <summary>Bakes a 3D world into a full storyboard model.</summary>
        public static Storyboard BakeWorld(JToken world, JObject? options2d = null) =>
            Call("scene3d.bakeWorld", new JObject
            {
                ["world"] = world,
                ["options2d"] = options2d ?? new JObject(),
            }).ToObject<Storyboard>()!;

        /// <summary>
        /// Bakes a set of 2D and 3D worlds into one storyboard. <paramref name="input"/>
        /// carries <c>{ worlds2d, worlds3d, options, options2d }</c>.
        /// </summary>
        public static Storyboard BakeScene(JObject input) =>
            Call("scene3d.bakeScene", input).ToObject<Storyboard>()!;
    }
}
