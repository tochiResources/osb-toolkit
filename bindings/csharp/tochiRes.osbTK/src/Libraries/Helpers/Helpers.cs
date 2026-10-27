using Newtonsoft.Json.Linq;

using System;
using System.Collections.Generic;

using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using HelpersApi = tochiRes.osbTK.Api.HelpersAPI;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>
    /// The shared motion-graphics helpers (M38/M40): one <c>helper</c> op
    /// namespaced by library (<c>keyframe.*</c>, <c>scene3d.*</c>, <c>command.*</c>,
    /// <c>origin.*</c>, <c>easing.*</c>, <c>timeline.*</c>, <c>color.*</c>,
    /// <c>scene2d.*</c>). The surface is code-split by namespace into this folder.
    /// </summary>
    public static partial class Helpers
    {
        private static readonly HelpersApi Api = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>Calls any shared helper by name with a raw input object.</summary>
        public static JToken Call(string name, JObject? input = null)
        {
            ArgumentNullException.ThrowIfNull(name);
            HelperResult result = Api.Call(new HelperRequest(input ?? new JObject(), name));
            return result.Value is JToken token ? token : JToken.FromObject(result.Value);
        }

        /// <summary>
        /// Applies any keyframe transform (e.g. <c>keyframe.flatten</c>) to a 2D
        /// command channel: extract the <paramref name="kind"/> channel, apply
        /// <paramref name="name"/> with <paramref name="options"/>, and write it
        /// back as commands.
        /// </summary>
        public static List<StoryboardCommand> TransformCommands(
            IEnumerable<StoryboardCommand> commands,
            StoryboardCommandKind kind,
            string name,
            JObject? options = null
        )
        {
            JObject input = options ?? new JObject();
            input["commands"] = JArray.FromObject(commands);
            input["kind"] = KindString(kind);
            return Call(name, input).ToObject<List<StoryboardCommand>>()!;
        }

        private static string KindString(StoryboardCommandKind kind)
        {
            string value = kind.ToString();
            return char.ToLowerInvariant(value[0]) + value[1..];
        }
    }
}
