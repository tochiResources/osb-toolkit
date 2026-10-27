using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

using System;
using System.Collections.Generic;

using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using HelpersApi = tochiRes.osbTK.Api.HelpersAPI;
using Storyboarding3dApi = tochiRes.osbTK.Api.Storyboarding3dAPI;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>
    /// The 3D storyboard library: a sibling that reuses the whole
    /// <see cref="Storyboarding"/> surface and adds the 3D -> 2D bake, so the 3D
    /// API is exposed alongside <see cref="Storyboarding"/> (M28).
    /// </summary>
    public static class Storyboarding3d
    {
        private static readonly Storyboarding3dApi Api = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );
        private static readonly HelpersApi HelpersClient = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>Parses a `.osu` beatmap into the typed model (shared with <see cref="Storyboarding"/>).</summary>
        public static Beatmap ParseOsu(string text) => Storyboarding.ParseOsu(text);

        /// <summary>Parses a `.osb` storyboard into a summary (shared with <see cref="Storyboarding"/>).</summary>
        public static ParseOsbResult ParseOsb(string text) => Storyboarding.ParseOsb(text);

        /// <summary>
        /// A live storyboard runtime, reusing <see cref="Storyboarding.Runtime"/>
        /// (3D effects drive the same storyboard).
        /// </summary>
        public sealed class Runtime : Storyboarding.Runtime
        {
            /// <summary>Creates a runtime of the given storyboard resolution.</summary>
            public Runtime(double width = 640, double height = 480)
                : base(width, height) { }
        }

        /// <summary>Bakes a 3D scene into a 2D storyboard.</summary>
        public static BakedStoryboard Bake3d(Scene3dInput scene)
        {
            ArgumentNullException.ThrowIfNull(scene);
            return Api.Bake3d(new Bake3dRequest(scene));
        }

        /// <summary>
        /// The 3D motion-graphics keyframe helpers (M38): pure keyframe-channel
        /// utilities, canonical in `osb_runtime::storyboarding3d::helpers`.
        /// </summary>
        public static class Helpers
        {
            /// <summary>A single keyframe: a time and a value of one, two or three components.</summary>
            public sealed class ChannelKey
            {
                /// <summary>The keyframe time in milliseconds.</summary>
                [JsonProperty("time")]
                public double Time { get; set; }

                /// <summary>The value components.</summary>
                [JsonProperty("value")]
                public double[] Value { get; set; } = Array.Empty<double>();
            }

            /// <summary>A keyframed channel: a list of `(time, value)` samples.</summary>
            public sealed class Channel
            {
                /// <summary>The channel's keyframes.</summary>
                [JsonProperty("keys")]
                public List<ChannelKey> Keys { get; set; } = new();
            }

            /// <summary>Calls any helper by name with a raw input object.</summary>
            public static JToken Call(string name, JObject? input = null)
            {
                ArgumentNullException.ThrowIfNull(name);
                HelperResult result = HelpersClient.Keyframes(new HelperRequest(input ?? new JObject(), name));
                return result.Value is JToken token ? token : JToken.FromObject(result.Value);
            }

            private static Channel ChannelCall(string name, JObject input) =>
                Call(name, input).ToObject<Channel>()!;

            private static JObject WithChannel(Channel channel) =>
                new() { ["channel"] = JToken.FromObject(channel) };

            /// <summary>Collapses an axis (or every axis) to a constant.</summary>
            public static Channel Flatten(Channel channel, string? axis = null, string mode = "mean") =>
                ChannelCall("flatten", new JObject
                {
                    ["channel"] = JToken.FromObject(channel),
                    ["axis"] = axis,
                    ["mode"] = mode,
                });

            /// <summary>Multiplies every component by `factor`.</summary>
            public static Channel ScaleKeyframes(Channel channel, double factor) =>
                ChannelCall("scaleKeyframes", Add(WithChannel(channel), ("factor", factor)));

            /// <summary>Adds a constant `delta` to every component.</summary>
            public static Channel OffsetKeyframes(Channel channel, double delta) =>
                ChannelCall("offsetKeyframes", Add(WithChannel(channel), ("delta", delta)));

            /// <summary>Reflects an axis (or every axis) about `pivot`.</summary>
            public static Channel Mirror(Channel channel, string? axis = null, double pivot = 0) =>
                ChannelCall("mirror", new JObject
                {
                    ["channel"] = JToken.FromObject(channel),
                    ["axis"] = axis,
                    ["pivot"] = pivot,
                });

            /// <summary>Seeded jitter in `min..max`, preserving keyframe times.</summary>
            public static Channel Randomize(
                Channel channel,
                string? axis = null,
                double min = -1,
                double max = 1,
                long seed = 0,
                double step = 0
            ) =>
                ChannelCall("randomize", new JObject
                {
                    ["channel"] = JToken.FromObject(channel),
                    ["axis"] = axis,
                    ["min"] = min,
                    ["max"] = max,
                    ["seed"] = seed,
                    ["step"] = step,
                });

            /// <summary>Reverses the channel in time about `[start, end]`.</summary>
            public static Channel ReverseKeyframes(Channel channel, double start, double end) =>
                ChannelCall("reverseKeyframes", Add(WithChannel(channel), ("start", start), ("end", end)));

            /// <summary>Tiles the `[start, end]` span `count` times.</summary>
            public static Channel RepeatKeyframes(
                Channel channel,
                int count,
                double start,
                double end,
                bool pingPong = false
            ) =>
                ChannelCall("repeatKeyframes", new JObject
                {
                    ["channel"] = JToken.FromObject(channel),
                    ["count"] = count,
                    ["start"] = start,
                    ["end"] = end,
                    ["pingPong"] = pingPong,
                });

            /// <summary>Resamples to a fixed `step` (or to exactly `count` samples when `step &lt;= 0`).</summary>
            public static Channel ResampleKeyframes(Channel channel, double step, int count = 0) =>
                ChannelCall("resampleKeyframes", Add(WithChannel(channel), ("step", step), ("count", count)));

            /// <summary>Moving-average smoothing over a window of neighbouring keyframes.</summary>
            public static Channel SmoothKeyframes(Channel channel, int window = 3) =>
                ChannelCall("smoothKeyframes", Add(WithChannel(channel), ("window", window)));

            /// <summary>Douglas-Peucker simplification of each component.</summary>
            public static Channel SimplifyKeyframes(Channel channel, double tolerance = 0.1, int decimals = 3) =>
                ChannelCall("simplifyKeyframes", Add(WithChannel(channel), ("tolerance", tolerance), ("decimals", decimals)));

            /// <summary>Snaps keyframe times to a beat grid.</summary>
            public static Channel AlignKeyframes(
                Channel channel,
                double beat = 500,
                double offset = 0,
                int subdivision = 1
            ) =>
                ChannelCall("alignKeyframes", Add(WithChannel(channel), ("beat", beat), ("offset", offset), ("subdivision", subdivision)));

            /// <summary>Re-eases each span (named easing).</summary>
            public static Channel EaseKeyframes(Channel channel, string easing = "None", int samples = 8) =>
                ChannelCall("easeKeyframes", Add(WithChannel(channel), ("easing", easing), ("samples", samples)));

            /// <summary>Blends the channel toward `other` by `amount` (`0..=1`).</summary>
            public static Channel MergeKeyframes(Channel channel, Channel other, double amount = 0.5) =>
                ChannelCall("mergeKeyframes", new JObject
                {
                    ["channel"] = JToken.FromObject(channel),
                    ["other"] = JToken.FromObject(other),
                    ["amount"] = amount,
                });

            /// <summary>A stagger delay for element `index`: `index * step`.</summary>
            public static double Stagger(int index, double step) =>
                Call("stagger", new JObject { ["index"] = index, ["step"] = step }).ToObject<double>();

            /// <summary>A circular orbit position channel.</summary>
            public static Channel Orbit(
                double[]? center = null,
                double radiusX = 100,
                double radiusY = 100,
                double startAngle = 0,
                double sweep = System.Math.Tau,
                int count = 32
            ) =>
                ChannelCall("orbit", new JObject
                {
                    ["center"] = center is null ? null : new JArray(center),
                    ["radiusX"] = radiusX,
                    ["radiusY"] = radiusY,
                    ["startAngle"] = startAngle,
                    ["sweep"] = sweep,
                    ["count"] = count,
                });

            /// <summary>A pulse oscillation channel.</summary>
            public static Channel Pulse(
                double @base = 1,
                double amplitude = 0.5,
                double frequency = 1,
                double phase = 0,
                double step = 0,
                double duration = 1000
            ) =>
                ChannelCall("pulse", new JObject
                {
                    ["base"] = @base,
                    ["amplitude"] = amplitude,
                    ["frequency"] = frequency,
                    ["phase"] = phase,
                    ["step"] = step,
                    ["duration"] = duration,
                });

            /// <summary>Seeded random placements in a box.</summary>
            public static List<Vector3> Scatter(
                long seed = 0,
                int count = 8,
                double[]? min = null,
                double[]? max = null,
                double spread = 0
            ) =>
                Call("scatter", new JObject
                {
                    ["seed"] = seed,
                    ["count"] = count,
                    ["min"] = min is null ? null : new JArray(min),
                    ["max"] = max is null ? null : new JArray(max),
                    ["spread"] = spread,
                }).ToObject<List<Vector3>>()!;

            /// <summary>A procedural spin about `axis`.</summary>
            public static Channel Spin(string axis = "z", double turns = 1, double duration = 1000, int count = 16) =>
                ChannelCall("spin", new JObject
                {
                    ["axis"] = axis,
                    ["turns"] = turns,
                    ["duration"] = duration,
                    ["count"] = count,
                });

            /// <summary>Independent spins about X, Y and Z.</summary>
            public static Channel Tumble(double[]? turns = null, double duration = 1000, int count = 16) =>
                ChannelCall("tumble", new JObject
                {
                    ["turns"] = turns is null ? null : new JArray(turns),
                    ["duration"] = duration,
                    ["count"] = count,
                });

            /// <summary>A position channel sampled along a polyline.</summary>
            public static Channel PathFollow(double[][] points, int count = 16, bool byArcLength = false) =>
                ChannelCall("pathFollow", new JObject
                {
                    ["points"] = new JArray(Array.ConvertAll(points, p => (object)new JArray(p))),
                    ["count"] = count,
                    ["byArcLength"] = byArcLength,
                });

            private static JObject Add(JObject input, params (string Key, object? Value)[] fields)
            {
                foreach ((string key, object? value) in fields)
                {
                    input[key] = value is null ? JValue.CreateNull() : JToken.FromObject(value);
                }
                return input;
            }
        }
    }
}
