using Newtonsoft.Json.Linq;

namespace tochiRes.osbTK;

public static partial class osb
{
    public static partial class Helpers
    {
        /// <summary>The timeline's total duration.</summary>
        public static double TimelineDuration(JToken timeline) =>
            Call("timeline.durationOf", new JObject { ["timeline"] = timeline }).Value<double>();

        /// <summary>Resolves a timeline tree to flat absolute-time clips.</summary>
        public static JToken FlushTimeline(JToken timeline) =>
            Call("timeline.flushTimeline", new JObject { ["timeline"] = timeline });

        /// <summary>Overlays two timelines' clips and children.</summary>
        public static JToken MergeTimelines(JToken a, JToken b) =>
            Call("timeline.mergeTimelines", new JObject { ["timeline"] = a, ["other"] = b });

        /// <summary>The total clip count across the tree.</summary>
        public static int TimelineClipCount(JToken timeline) =>
            Call("timeline.clipCount", new JObject { ["timeline"] = timeline }).Value<int>();

        /// <summary>Adds a nested child timeline (a group).</summary>
        public static JToken GroupTimeline(JToken timeline, JToken? child = null) =>
            Call("timeline.addGroup", new JObject
            {
                ["timeline"] = timeline,
                ["child"] = child ?? new JObject { ["clips"] = new JArray() },
            });

        /// <summary>Every clip in the tree, in the shared time coordinate.</summary>
        public static JToken TimelineAllClips(JToken timeline) =>
            Call("timeline.allClips", new JObject { ["timeline"] = timeline });
    }
}
