using Newtonsoft.Json.Linq;

using System.Collections.Generic;

using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using AudioApi = tochiRes.osbTK.Api.AudioAPI;
using GeneratorApi = tochiRes.osbTK.Api.GeneratorAPI;
using HelpersApi = tochiRes.osbTK.Api.HelpersAPI;
using LayerApi = tochiRes.osbTK.Api.LayerAPI;
using SpriteApi = tochiRes.osbTK.Api.SpriteAPI;

using NativeRuntime = global::tochiRes.osbTK.Native.Runtime;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>
    /// The storyboarding library: `.osu`/`.osb` parsing (static) and the
    /// storyboard <see cref="Runtime"/>.
    /// </summary>
    public static class Storyboarding
    {
        private static readonly HelpersApi CommandsClient = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>Parses a `.osu` beatmap into the typed model.</summary>
        public static Beatmap ParseOsu(string text)
        {
            ArgumentException.ThrowIfNullOrEmpty(text);
            return osb.Parsing.ParseOsu(text);
        }

        /// <summary>Parses a `.osb` storyboard into a summary.</summary>
        public static ParseOsbResult ParseOsb(string text)
        {
            ArgumentException.ThrowIfNullOrEmpty(text);
            return osb.Parsing.ParseOsb(text);
        }

        /// <summary>The anchor offset for an origin.</summary>
        public static Vector2 GetOriginVector(OsbOrigin origin, double width, double height) =>
            osb.Origin.GetOriginVector(origin, width, height);

        /// <summary>Whether a rotated sprite's bounds intersect the screen.</summary>
        public static bool InScreenBounds(
            Vector2 position,
            Vector2 size,
            double rotation,
            Vector2 origin
        ) => osb.Origin.InScreenBounds(position, size, rotation, origin);

        /// <summary>
        /// A live storyboard runtime: the effect context (beatmap, audio,
        /// resolution) and the storyboard it builds. It wraps a native handle and
        /// drives the generated clients through the bridge transport. Dispose it
        /// when done (a finalizer frees it as a fallback).
        /// </summary>
        public class Runtime : IDisposable
        {
            private readonly NativeRuntime _native;
            private readonly Client.Runtime.Generator _generator;
            private readonly Client.Runtime.Audio _audio;
            private readonly LayerApi _layer;
            private readonly SpriteApi _sprite;
            private bool _disposed;

            /// <summary>Creates a runtime of the given storyboard resolution.</summary>
            public Runtime(double width = 640, double height = 480)
            {
                _native = new NativeRuntime(width, height);
                var transport = new OsbtkTransport(_native);
                _generator = new Client.Runtime.Generator(
                    new GeneratorApi(transport, transport, DefaultConfiguration)
                );
                _audio = new Client.Runtime.Audio(
                    new AudioApi(transport, transport, DefaultConfiguration)
                );
                _layer = new LayerApi(transport, transport, DefaultConfiguration);
                _sprite = new SpriteApi(transport, transport, DefaultConfiguration);
            }

            /// <summary>The clean audio client bound to this runtime.</summary>
            public Client.Runtime.Audio Audio => _audio;

            /// <summary>The number of sprites on the storyboard.</summary>
            public int SpriteCount => _generator.SpriteCount();

            /// <summary>The built storyboard model.</summary>
            public Storyboard Storyboard => _generator.Storyboard();

            /// <summary>The loaded audio duration in milliseconds.</summary>
            public double AudioDuration => _audio.AudioDuration();

            /// <summary>The loaded beatmap, or <c>null</c>.</summary>
            public Beatmap? Beatmap => _generator.Beatmap();

            /// <summary>Imports a `.osb` storyboard.</summary>
            public void ImportOsb(string textOrPath)
            {
                ArgumentException.ThrowIfNullOrEmpty(textOrPath);
                _generator.ImportOsb(textOrPath);
            }

            /// <summary>Loads a `.osu` beatmap into the runtime.</summary>
            public void LoadBeatmap(string osuText)
            {
                ArgumentException.ThrowIfNullOrEmpty(osuText);
                _generator.LoadBeatmap(osuText);
            }

            /// <summary>The active red timing point at a time.</summary>
            public ControlPoint TimingPointAt(double time) => _generator.TimingPointAt(time);

            /// <summary>The active control point at a time, or <c>null</c>.</summary>
            public ControlPoint? ControlPointAt(double time) => _generator.ControlPointAt(time);

            /// <summary>A deterministic random value in <c>[min, max)</c>.</summary>
            public double Random(double min, double max) => _generator.Random(max, min);

            /// <summary>Returns (creating on first use) a layer by name.</summary>
            public Layer Layer(string name)
            {
                ArgumentException.ThrowIfNullOrEmpty(name);
                int handle = _generator.LayerHandle(name);
                return new Layer(_layer, _sprite, name, handle);
            }

            /// <summary>Returns a standard osu! layer.</summary>
            public Layer Layer(OsbLayer layer) => Layer(layer.ToString());

            /// <summary>Returns (creating on first use) a layer by name.</summary>
            public Layer GetLayer(string name) => Layer(name);

            /// <summary>Returns a standard osu! layer.</summary>
            public Layer GetLayer(OsbLayer layer) => Layer(layer);

            /// <summary>Reads FFT magnitudes from the runtime's loaded audio.</summary>
            public FftResult GetFft(double time, int magnitudes = 64, Easing easing = Easing.None, double cutoff = 0) =>
                _audio.GetFft(time, magnitudes, cutoff, easing);

            /// <summary>Releases the native runtime.</summary>
            public void Dispose()
            {
                if (_disposed)
                {
                    return;
                }
                _native.Dispose();
                _disposed = true;
                GC.SuppressFinalize(this);
            }

            /// <summary>Frees the native runtime if <see cref="Dispose"/> was not called.</summary>
            ~Runtime() => Dispose();
        }

        /// <summary>
        /// The 2D storyboard command utilities: pure transforms over a
        /// `StoryboardCommand[]`, canonical in `osb_runtime::storyboarding::helpers`.
        /// </summary>
        public static class Commands
        {
            /// <summary>Calls any helper by name with a raw input object.</summary>
            public static JToken Call(string name, JObject? input = null)
            {
                ArgumentNullException.ThrowIfNull(name);
                HelperResult result = CommandsClient.Commands(new HelperRequest(input ?? new JObject(), name));
                return result.Value is JToken token ? token : JToken.FromObject(result.Value);
            }

            private static List<StoryboardCommand> Commands2(string name, JObject input) =>
                Call(name, input).ToObject<List<StoryboardCommand>>()!;

            private static JObject With(IEnumerable<StoryboardCommand> commands) =>
                new() { ["commands"] = JArray.FromObject(commands) };

            /// <summary>Selects a timeframe (and/or kind) of commands without mutating the source.</summary>
            public static List<StoryboardCommand> SelectCommands(
                IEnumerable<StoryboardCommand> commands,
                StoryboardCommandKind? kind = null,
                double? start = null,
                double? end = null
            ) =>
                Commands2("selectCommands", new JObject
                {
                    ["commands"] = JArray.FromObject(commands),
                    ["kind"] = kind?.ToString() is { } k ? Char.ToLowerInvariant(k[0]) + k[1..] : null,
                    ["start"] = start,
                    ["end"] = end,
                });

            /// <summary>Moves a command set in time by `delta` ms.</summary>
            public static List<StoryboardCommand> ShiftCommands(IEnumerable<StoryboardCommand> commands, double delta) =>
                Commands2("shiftCommands", Add(With(commands), ("delta", delta)));

            /// <summary>Stretches a command set in time about `anchor` by `factor`.</summary>
            public static List<StoryboardCommand> ScaleCommands(
                IEnumerable<StoryboardCommand> commands,
                double factor,
                double anchor = 0
            ) =>
                Commands2("scaleCommands", Add(With(commands), ("factor", factor), ("anchor", anchor)));

            /// <summary>Reverses a command set in time about `[start, end]`.</summary>
            public static List<StoryboardCommand> ReverseCommands(
                IEnumerable<StoryboardCommand> commands,
                double start,
                double end
            ) =>
                Commands2("reverseCommands", Add(With(commands), ("start", start), ("end", end)));

            /// <summary>Merges adjacent commands that are continuous in time and value.</summary>
            public static List<StoryboardCommand> MergeCommands(
                IEnumerable<StoryboardCommand> commands,
                double tolerance = 0
            ) =>
                Commands2("mergeCommands", Add(With(commands), ("tolerance", tolerance)));

            /// <summary>Splits commands spanning `at` into two linear pieces.</summary>
            public static List<StoryboardCommand> SplitCommands(IEnumerable<StoryboardCommand> commands, double at) =>
                Commands2("splitCommands", Add(With(commands), ("at", at)));

            /// <summary>Collapses a command channel to a constant.</summary>
            public static List<StoryboardCommand> FlattenCommands(
                IEnumerable<StoryboardCommand> commands,
                StoryboardCommandKind? kind = null,
                string mode = "mean"
            ) =>
                Commands2("flattenCommands", new JObject
                {
                    ["commands"] = JArray.FromObject(commands),
                    ["kind"] = kind?.ToString() is { } k ? Char.ToLowerInvariant(k[0]) + k[1..] : null,
                    ["mode"] = mode,
                });

            /// <summary>Seeded jitter of command values.</summary>
            public static List<StoryboardCommand> RandomizeCommands(
                IEnumerable<StoryboardCommand> commands,
                double min = -1,
                double max = 1,
                long seed = 0
            ) =>
                Commands2("randomizeCommands", Add(With(commands), ("min", min), ("max", max), ("seed", seed)));

            /// <summary>Keeps the commands matching a kind/time selector.</summary>
            public static List<StoryboardCommand> FilterCommands(
                IEnumerable<StoryboardCommand> commands,
                StoryboardCommandKind? kind = null,
                double? start = null,
                double? end = null
            ) =>
                Commands2("filterCommands", new JObject
                {
                    ["commands"] = JArray.FromObject(commands),
                    ["kind"] = kind?.ToString() is { } k ? Char.ToLowerInvariant(k[0]) + k[1..] : null,
                    ["start"] = start,
                    ["end"] = end,
                });

            /// <summary>Removes the commands matching a kind/time selector.</summary>
            public static List<StoryboardCommand> RemoveCommands(
                IEnumerable<StoryboardCommand> commands,
                StoryboardCommandKind? kind = null,
                double? start = null,
                double? end = null
            ) =>
                Commands2("removeCommands", new JObject
                {
                    ["commands"] = JArray.FromObject(commands),
                    ["kind"] = kind?.ToString() is { } k ? Char.ToLowerInvariant(k[0]) + k[1..] : null,
                    ["start"] = start,
                    ["end"] = end,
                });

            /// <summary>Applies a scalar value transform (`value * scale + offset`) to every command.</summary>
            public static List<StoryboardCommand> MapCommands(
                IEnumerable<StoryboardCommand> commands,
                double valueScale = 1,
                double valueOffset = 0
            ) =>
                Commands2("mapCommands", Add(With(commands), ("valueScale", valueScale), ("valueOffset", valueOffset)));

            /// <summary>Clones a command set `count` times, each copy shifted by `offset` ms.</summary>
            public static List<StoryboardCommand> DuplicateCommands(
                IEnumerable<StoryboardCommand> commands,
                double offset = 0,
                int count = 1
            ) =>
                Commands2("duplicateCommands", Add(With(commands), ("offset", offset), ("count", count)));

            /// <summary>Snaps command times to a beat grid (and values to `valueStep`).</summary>
            public static List<StoryboardCommand> AlignCommands(
                IEnumerable<StoryboardCommand> commands,
                double beat = 500,
                double offset = 0,
                int subdivision = 1,
                double valueStep = 0
            ) =>
                Commands2("alignCommands", Add(With(commands), ("beat", beat), ("offset", offset), ("subdivision", subdivision), ("valueStep", valueStep)));

            /// <summary>Rounds command times/values to a step.</summary>
            public static List<StoryboardCommand> QuantizeCommands(
                IEnumerable<StoryboardCommand> commands,
                double timeStep = 0,
                double valueStep = 0
            ) =>
                Commands2("quantizeCommands", Add(With(commands), ("timeStep", timeStep), ("valueStep", valueStep)));

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
