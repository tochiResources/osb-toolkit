using Newtonsoft.Json.Linq;

using tochiRes.osbTK.Model;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>
    /// The color-space and CSS/Sass-style helpers (M40): offset/scale/randomize in
    /// any space (RGB/HSL/HSB/HWB), <c>Lighten</c>/<c>Darken</c>, <c>Saturate</c>/
    /// <c>Desaturate</c>, <c>AdjustHue</c>/<c>Complement</c>, <c>Grayscale</c>/
    /// <c>Invert</c>, <c>Mix</c>/<c>Tint</c>/<c>Shade</c>, <c>Opacify</c>/
    /// <c>Transparentize</c>, and multi-stop gradients.
    /// </summary>
    public static partial class Helpers
    {
        private static Color4 ApplyColor(Color4 color, string op, JObject? extra = null)
        {
            JObject input = extra ?? new JObject();
            input["color"] = JToken.FromObject(color);
            input["op"] = op;
            return Call("color.apply", input).ToObject<Color4>()!;
        }

        /// <summary>Converts a color into its (x, y, z, w) components in <paramref name="space"/>.</summary>
        public static double[] ColorToSpace(Color4 color, string space = "rgb") =>
            Call("color.toSpace", new JObject { ["color"] = JToken.FromObject(color), ["space"] = space })
                .ToObject<double[]>()!;

        /// <summary>Adds a per-component <paramref name="delta"/> in <paramref name="space"/>.</summary>
        public static Color4 ColorOffset(Color4 color, double[] delta, string space = "rgb") =>
            Call("color.offset", new JObject
            {
                ["color"] = JToken.FromObject(color),
                ["space"] = space,
                ["delta"] = JArray.FromObject(delta),
            }).ToObject<Color4>()!;

        /// <summary>Multiplies each component by a per-component <paramref name="factor"/> in <paramref name="space"/>.</summary>
        public static Color4 ColorScale(Color4 color, double[] factor, string space = "rgb") =>
            Call("color.scale", new JObject
            {
                ["color"] = JToken.FromObject(color),
                ["space"] = space,
                ["factor"] = JArray.FromObject(factor),
            }).ToObject<Color4>()!;

        /// <summary>Raises HSL lightness.</summary>
        public static Color4 Lighten(Color4 color, double amount) =>
            ApplyColor(color, "lighten", new JObject { ["amount"] = amount });

        /// <summary>Lowers HSL lightness.</summary>
        public static Color4 Darken(Color4 color, double amount) =>
            ApplyColor(color, "darken", new JObject { ["amount"] = amount });

        /// <summary>Raises HSL saturation.</summary>
        public static Color4 Saturate(Color4 color, double amount) =>
            ApplyColor(color, "saturate", new JObject { ["amount"] = amount });

        /// <summary>Lowers HSL saturation.</summary>
        public static Color4 Desaturate(Color4 color, double amount) =>
            ApplyColor(color, "desaturate", new JObject { ["amount"] = amount });

        /// <summary>Rotates the hue by <paramref name="degrees"/>.</summary>
        public static Color4 AdjustHue(Color4 color, double degrees) =>
            ApplyColor(color, "adjustHue", new JObject { ["degrees"] = degrees });

        /// <summary>The 180-degree hue rotation.</summary>
        public static Color4 Complement(Color4 color) => ApplyColor(color, "complement");

        /// <summary>Drops HSL saturation to zero.</summary>
        public static Color4 Grayscale(Color4 color) => ApplyColor(color, "grayscale");

        /// <summary>The RGB inverse, keeping alpha.</summary>
        public static Color4 Invert(Color4 color) => ApplyColor(color, "invert");

        /// <summary>The Sass weighted mix, accounting for alpha.</summary>
        public static Color4 Mix(Color4 a, Color4 b, double weight = 0.5) =>
            ApplyColor(a, "mix", new JObject { ["other"] = JToken.FromObject(b), ["weight"] = weight });

        /// <summary>Mixes toward white.</summary>
        public static Color4 Tint(Color4 color, double weight = 0.5) =>
            ApplyColor(color, "tint", new JObject { ["weight"] = weight });

        /// <summary>Mixes toward black.</summary>
        public static Color4 Shade(Color4 color, double weight = 0.5) =>
            ApplyColor(color, "shade", new JObject { ["weight"] = weight });

        /// <summary>Raises alpha.</summary>
        public static Color4 Opacify(Color4 color, double amount) =>
            ApplyColor(color, "opacify", new JObject { ["amount"] = amount });

        /// <summary>Lowers alpha.</summary>
        public static Color4 Transparentize(Color4 color, double amount) =>
            ApplyColor(color, "transparentize", new JObject { ["amount"] = amount });

        /// <summary>Sets the alpha channel.</summary>
        public static Color4 WithAlpha(Color4 color, double alpha) =>
            ApplyColor(color, "withAlpha", new JObject { ["alpha"] = alpha });

        /// <summary>Samples a multi-stop gradient at <paramref name="t"/> (0..=1).</summary>
        public static Color4 Gradient(
            JArray stops,
            double t,
            string space = "rgb",
            bool repeat = false
        ) =>
            Call("color.gradient", new JObject
            {
                ["stops"] = stops,
                ["t"] = t,
                ["space"] = space,
                ["repeat"] = repeat,
            }).ToObject<Color4>()!;

        /// <summary>Builds a color channel by sampling a gradient <paramref name="count"/> times.</summary>
        public static JToken GradientChannel(
            JArray stops,
            int count = 16,
            double start = 0.0,
            double end = 1000.0,
            string space = "rgb",
            bool repeat = false
        ) =>
            Call("color.gradientChannel", new JObject
            {
                ["stops"] = stops,
                ["count"] = count,
                ["start"] = start,
                ["end"] = end,
                ["space"] = space,
                ["repeat"] = repeat,
            });

        /// <summary>Jitters each component by a seeded per-component range in <paramref name="space"/>.</summary>
        public static Color4 ColorRandomize(
            Color4 color,
            double[] min,
            double[] max,
            long seed,
            string space = "rgb"
        ) =>
            Call("color.randomize", new JObject
            {
                ["color"] = JToken.FromObject(color),
                ["space"] = space,
                ["seed"] = seed,
                ["min"] = JArray.FromObject(min),
                ["max"] = JArray.FromObject(max),
            }).ToObject<Color4>()!;

        /// <summary>Offsets a whole color channel in <paramref name="space"/>.</summary>
        public static JToken ColorOffsetChannel(JToken channel, double[] delta, string space = "rgb") =>
            Call("color.offsetChannel", new JObject
            {
                ["channel"] = channel,
                ["space"] = space,
                ["delta"] = JArray.FromObject(delta),
            });

        /// <summary>Scales a whole color channel in <paramref name="space"/>.</summary>
        public static JToken ColorScaleChannel(JToken channel, double[] factor, string space = "rgb") =>
            Call("color.scaleChannel", new JObject
            {
                ["channel"] = channel,
                ["space"] = space,
                ["factor"] = JArray.FromObject(factor),
            });

        /// <summary>Randomizes a whole color channel in <paramref name="space"/>.</summary>
        public static JToken ColorRandomizeChannel(
            JToken channel,
            double[] min,
            double[] max,
            long seed,
            string space = "rgb"
        ) =>
            Call("color.randomizeChannel", new JObject
            {
                ["channel"] = channel,
                ["space"] = space,
                ["seed"] = seed,
                ["min"] = JArray.FromObject(min),
                ["max"] = JArray.FromObject(max),
            });

        /// <summary>Applies a named helper to every keyframe of a color channel.</summary>
        public static JToken ColorApplyChannel(JToken channel, string op, JObject? extra = null)
        {
            JObject input = extra ?? new JObject();
            input["channel"] = channel;
            input["op"] = op;
            return Call("color.applyChannel", input);
        }
    }
}
