using Newtonsoft.Json.Linq;

using System.Collections.Generic;

using tochiRes.osbTK.Model;

namespace tochiRes.osbTK;

public static partial class osb
{
    public static partial class Helpers
    {
        /// <summary>Sets an easing on every command (apply an easing to a whole group/scene).</summary>
        public static List<StoryboardCommand> SetEasing(
            IEnumerable<StoryboardCommand> commands,
            Easing easing
        ) =>
            Call("command.setEasing", new JObject
            {
                ["commands"] = JArray.FromObject(commands),
                ["easing"] = JToken.FromObject(easing),
            }).ToObject<List<StoryboardCommand>>()!;
    }
}
