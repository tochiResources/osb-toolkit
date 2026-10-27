package moe.tochi.osbtk;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

import com.google.gson.reflect.TypeToken;

import moe.tochi.osbtk.api.HelpersAPI;

import moe.tochi.osbtk.bridge.BridgeInterceptor;
import moe.tochi.osbtk.bridge.NativeBridge;
import moe.tochi.osbtk.model.ApplyEasingRequest;
import moe.tochi.osbtk.model.ArcRequest;
import moe.tochi.osbtk.model.Bake3dRequest;
import moe.tochi.osbtk.model.BakedStoryboard;
import moe.tochi.osbtk.model.Beatmap;
import moe.tochi.osbtk.model.Box2;
import moe.tochi.osbtk.model.ControlPoint;
import moe.tochi.osbtk.model.CreateSampleRequest;
import moe.tochi.osbtk.model.CreateSpriteRequest;
import moe.tochi.osbtk.model.CurveDistanceRequest;
import moe.tochi.osbtk.model.CurveOffsetRequest;
import moe.tochi.osbtk.model.CurvePointRequest;
import moe.tochi.osbtk.model.CurveRequest;
import moe.tochi.osbtk.model.CurveSampleRequest;
import moe.tochi.osbtk.model.CurveSpec;
import moe.tochi.osbtk.model.DiagnosticReport;
import moe.tochi.osbtk.model.Easing;
import moe.tochi.osbtk.model.EasingValueRequest;
import moe.tochi.osbtk.model.EllipseRequest;
import moe.tochi.osbtk.model.EncodeVideoRequest;
import moe.tochi.osbtk.model.ExtractAudioRequest;
import moe.tochi.osbtk.model.ExtractFramesRequest;
import moe.tochi.osbtk.model.FftRequest;
import moe.tochi.osbtk.model.FftResult;
import moe.tochi.osbtk.model.FrameResult;
import moe.tochi.osbtk.model.GetFftRequest;
import moe.tochi.osbtk.model.GetOriginVectorRequest;
import moe.tochi.osbtk.model.Handle;
import moe.tochi.osbtk.model.HelperRequest;
import moe.tochi.osbtk.model.ImageInfo;
import moe.tochi.osbtk.model.ImageOptions;
import moe.tochi.osbtk.model.ImageProcessRequest;
import moe.tochi.osbtk.model.InScreenBoundsRequest;
import moe.tochi.osbtk.model.LineIntersectRequest;
import moe.tochi.osbtk.model.MediaInfo;
import moe.tochi.osbtk.model.OkResult;
import moe.tochi.osbtk.model.OptimizeOptionsInput;
import moe.tochi.osbtk.model.OptimizeRequest;
import moe.tochi.osbtk.model.OptimizeResult;
import moe.tochi.osbtk.model.OptionalPointResult;
import moe.tochi.osbtk.model.OsbOrigin;
import moe.tochi.osbtk.model.ParseOsbResult;
import moe.tochi.osbtk.model.PathRequest;
import moe.tochi.osbtk.model.PointListResult;
import moe.tochi.osbtk.model.RenderShapesRequest;
import moe.tochi.osbtk.model.Scene3dInput;
import moe.tochi.osbtk.model.SimplifyRequest;
import moe.tochi.osbtk.model.TextRequest;
import moe.tochi.osbtk.model.TimeRequest;
import moe.tochi.osbtk.model.Vector2;
import moe.tochi.osbtk.model.CommandRequest;
import moe.tochi.osbtk.model.CreateAnimationRequest;
import moe.tochi.osbtk.model.Color4;
import moe.tochi.osbtk.model.HandleRequest;
import moe.tochi.osbtk.model.LayerHandle;
import moe.tochi.osbtk.model.OsbLayer;
import moe.tochi.osbtk.model.SegmentAddIndexRequest;
import moe.tochi.osbtk.model.SegmentChangePositionRequest;
import moe.tochi.osbtk.model.SegmentCleanResult;
import moe.tochi.osbtk.model.SegmentDiscardResult;
import moe.tochi.osbtk.model.SegmentIndexesResult;
import moe.tochi.osbtk.model.SegmentRequest;
import moe.tochi.osbtk.model.SegmentReverseDepthRequest;
import moe.tochi.osbtk.model.LayerHandleRequest;
import moe.tochi.osbtk.model.ParameterType;
import moe.tochi.osbtk.model.SpriteInfo;
import moe.tochi.osbtk.model.SpriteState;
import moe.tochi.osbtk.model.SpriteTimeRequest;
import moe.tochi.osbtk.model.StartLoopGroupRequest;
import moe.tochi.osbtk.model.Storyboard;
import moe.tochi.osbtk.model.StoryboardCommand;
import moe.tochi.osbtk.model.StoryboardCommandKind;
import moe.tochi.osbtk.model.Vector3;
import okhttp3.OkHttpClient;

/**
 * The osb SDK entry point, organized into libraries: {@link Storyboarding},
 * {@link Animations}, {@link Audio} and {@link Storyboarding3d}.
 */
public final class osb {
    private osb() {}

    /** The native runtime version string. */
    public static String version() {
        return NativeBridge.version();
    }

    private static ApiClient statelessClient() {
        OkHttpClient httpClient =
                new OkHttpClient.Builder()
                        .addInterceptor(new BridgeInterceptor(NativeBridge::exec))
                        .build();
        ApiClient apiClient = new ApiClient(httpClient);
        apiClient.setBasePath("http://osbtk.invalid");
        return apiClient;
    }

    /**
     * The shared motion-graphics helpers: one {@code helper} op namespaced
     * by library ({@code keyframe.*}, {@code scene3d.*}, {@code command.*},
     * {@code origin.*}). The keyframe helpers work for 2D and 3D (the Z axis is
     * optional), and {@code transformCommands} applies any of them to a 2D
     * command channel.
     */
    public static final class Helpers {
        private static final moe.tochi.osbtk.api.HelpersAPI API = new moe.tochi.osbtk.api.HelpersAPI(statelessClient());

        private Helpers() {}

        /** Calls any shared helper by name with a raw input object. */
        public static Object call(String name, Map<String, Object> input) throws ApiException {
            return API.call(new HelperRequest().name(name).input(input)).getValue();
        }

        /** Applies any keyframe transform (e.g. {@code keyframe.flatten}) to a 2D command channel. */
        public static List<StoryboardCommand> transformCommands(
                List<StoryboardCommand> commands,
                StoryboardCommandKind kind,
                String name,
                Map<String, Object> options)
                throws ApiException {
            Map<String, Object> input = new HashMap<>(options == null ? Map.of() : options);
            input.put("commands", commands);
            input.put("kind", kind.getValue());
            return JSON.getGson()
                    .fromJson(
                            JSON.getGson().toJson(call(name, input)),
                            new TypeToken<List<StoryboardCommand>>() {}.getType());
        }

        /** The anchor offset for an enum origin, in texture pixels. */
        public static Vector2 originVector(OsbOrigin origin, double width, double height)
                throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("origin", origin.getValue());
            input.put("width", width);
            input.put("height", height);
            return JSON.getGson()
                    .fromJson(
                            JSON.getGson().toJson(call("origin.getOriginVector", input)),
                            Vector2.class);
        }

        /** The effective pivot: the enum origin vector plus an explicit anchor offset. */
        public static Vector2 resolveAnchor(
                OsbOrigin origin, Vector2 anchor, double width, double height)
                throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("origin", origin.getValue());
            input.put("anchor", anchor);
            input.put("width", width);
            input.put("height", height);
            return JSON.getGson()
                    .fromJson(
                            JSON.getGson().toJson(call("origin.resolveAnchor", input)),
                            Vector2.class);
        }

        /** Whether a rotated sprite's bounds intersect the screen. */
        public static boolean inScreenBounds(
                Vector2 position, Vector2 size, double rotation, Vector2 origin)
                throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("position", position);
            input.put("size", size);
            input.put("rotation", rotation);
            input.put("origin", origin);
            return Boolean.TRUE.equals(call("origin.inScreenBounds", input));
        }

        /** The 3D effective pivot: a {@link Vector3} anchor whose z anchors depth. */
        public static Vector3 resolveAnchor3d(
                OsbOrigin origin, Vector3 anchor, double width, double height)
                throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("origin", origin.getValue());
            input.put("anchor", anchor);
            input.put("width", width);
            input.put("height", height);
            return JSON.getGson()
                    .fromJson(
                            JSON.getGson().toJson(call("origin.resolveAnchor3d", input)),
                            Vector3.class);
        }

        /** Amplifies an easing about linear by {@code amount} ({@code >1} stronger). */
        public static double amplifyEasing(Easing easing, double t, double amount)
                throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("easing", easing.getValue());
            input.put("t", t);
            input.put("amount", amount);
            return ((Number) call("easing.amplifyEasing", input)).doubleValue();
        }

        /** Samples an easing at {@code count} points in [0, 1]. */
        public static List<Double> sampleEasing(Easing easing, int count) throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("easing", easing.getValue());
            input.put("count", count);
            return JSON.getGson()
                    .fromJson(
                            JSON.getGson().toJson(call("easing.sampleEasing", input)),
                            new TypeToken<List<Double>>() {}.getType());
        }

        private static List<StoryboardCommand> commandsOf(String name, Map<String, Object> input)
                throws ApiException {
            return JSON.getGson()
                    .fromJson(
                            JSON.getGson().toJson(call(name, input)),
                            new TypeToken<List<StoryboardCommand>>() {}.getType());
        }

        /** Sets an easing on every command (apply an easing to a whole group/scene). */
        public static List<StoryboardCommand> setEasing(
                List<StoryboardCommand> commands, Easing easing) throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("commands", commands);
            input.put("easing", easing.getValue());
            return commandsOf("command.setEasing", input);
        }

        /** Re-eases a command channel with a named easing, resolving it into commands. */
        public static List<StoryboardCommand> applyEasing(
                List<StoryboardCommand> commands,
                StoryboardCommandKind kind,
                Easing easing,
                int samples)
                throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("commands", commands);
            input.put("kind", kind.getValue());
            input.put("easing", easing.getValue());
            input.put("samples", samples);
            return commandsOf("command.applyEasing", input);
        }

        /** Re-eases a command channel with a cubic-bezier, resolving it into commands. */
        public static List<StoryboardCommand> applyBezier(
                List<StoryboardCommand> commands,
                StoryboardCommandKind kind,
                double x1,
                double y1,
                double x2,
                double y2,
                int samples)
                throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("commands", commands);
            input.put("kind", kind.getValue());
            input.put("x1", x1);
            input.put("y1", y1);
            input.put("x2", x2);
            input.put("y2", y2);
            input.put("samples", samples);
            return commandsOf("command.applyBezier", input);
        }

        /** The timeline's total duration. */
        public static double timelineDuration(Object timeline) throws ApiException {
            return ((Number) call("timeline.durationOf", Map.of("timeline", timeline)))
                    .doubleValue();
        }

        /** Resolves a timeline tree to flat absolute-time clips. */
        public static Object flushTimeline(Object timeline) throws ApiException {
            return call("timeline.flushTimeline", Map.of("timeline", timeline));
        }

        /** Overlays two timelines' clips and children. */
        public static Object mergeTimelines(Object a, Object b) throws ApiException {
            return call("timeline.mergeTimelines", Map.of("timeline", a, "other", b));
        }

        /** Adds a nested child timeline (a group). */
        public static Object groupTimeline(Object timeline, Object child) throws ApiException {
            return call("timeline.addGroup", Map.of("timeline", timeline, "child", child));
        }

        /** The total clip count across the tree. */
        public static int timelineClipCount(Object timeline) throws ApiException {
            return ((Number) call("timeline.clipCount", Map.of("timeline", timeline))).intValue();
        }

        private static Color4 colorOf(String name, Map<String, Object> input) throws ApiException {
            return JSON.getGson().fromJson(JSON.getGson().toJson(call(name, input)), Color4.class);
        }

        /** Converts a color into its (x, y, z, w) components in {@code space}. */
        public static List<Double> colorToSpace(Color4 color, String space) throws ApiException {
            return JSON.getGson()
                    .fromJson(
                            JSON.getGson()
                                    .toJson(
                                            call(
                                                    "color.toSpace",
                                                    Map.of("color", color, "space", space))),
                            new TypeToken<List<Double>>() {}.getType());
        }

        /** Builds a color from (x, y, z, w) components in {@code space}. */
        public static Color4 colorFromSpace(List<Double> value, String space) throws ApiException {
            return colorOf("color.fromSpace", Map.of("value", value, "space", space));
        }

        /** Adds a per-component {@code delta} in {@code space}. */
        public static Color4 colorOffset(Color4 color, List<Double> delta, String space)
                throws ApiException {
            return colorOf("color.offset", Map.of("color", color, "delta", delta, "space", space));
        }

        /** Multiplies each component by a per-component {@code factor} in {@code space}. */
        public static Color4 colorScale(Color4 color, List<Double> factor, String space)
                throws ApiException {
            return colorOf("color.scale", Map.of("color", color, "factor", factor, "space", space));
        }

        /** Applies a named CSS/Sass helper with extra options. */
        public static Color4 colorApply(Color4 color, String op, Map<String, Object> extra)
                throws ApiException {
            Map<String, Object> input = new HashMap<>(extra == null ? Map.of() : extra);
            input.put("color", color);
            input.put("op", op);
            return colorOf("color.apply", input);
        }

        /** Raises HSL lightness. */
        public static Color4 lighten(Color4 color, double amount) throws ApiException {
            return colorApply(color, "lighten", Map.of("amount", amount));
        }

        /** Lowers HSL lightness. */
        public static Color4 darken(Color4 color, double amount) throws ApiException {
            return colorApply(color, "darken", Map.of("amount", amount));
        }

        /** Raises HSL saturation. */
        public static Color4 saturate(Color4 color, double amount) throws ApiException {
            return colorApply(color, "saturate", Map.of("amount", amount));
        }

        /** Lowers HSL saturation. */
        public static Color4 desaturate(Color4 color, double amount) throws ApiException {
            return colorApply(color, "desaturate", Map.of("amount", amount));
        }

        /** Rotates the hue by {@code degrees}. */
        public static Color4 adjustHue(Color4 color, double degrees) throws ApiException {
            return colorApply(color, "adjustHue", Map.of("degrees", degrees));
        }

        /** The 180-degree hue rotation. */
        public static Color4 complement(Color4 color) throws ApiException {
            return colorApply(color, "complement", null);
        }

        /** Drops HSL saturation to zero. */
        public static Color4 grayscale(Color4 color) throws ApiException {
            return colorApply(color, "grayscale", null);
        }

        /** The RGB inverse, keeping alpha. */
        public static Color4 invert(Color4 color) throws ApiException {
            return colorApply(color, "invert", null);
        }

        /** The Sass weighted mix, accounting for alpha. */
        public static Color4 mix(Color4 a, Color4 b, double weight) throws ApiException {
            return colorApply(a, "mix", Map.of("other", b, "weight", weight));
        }

        /** Mixes toward white. */
        public static Color4 tint(Color4 color, double weight) throws ApiException {
            return colorApply(color, "tint", Map.of("weight", weight));
        }

        /** Mixes toward black. */
        public static Color4 shade(Color4 color, double weight) throws ApiException {
            return colorApply(color, "shade", Map.of("weight", weight));
        }

        /** Raises alpha. */
        public static Color4 opacify(Color4 color, double amount) throws ApiException {
            return colorApply(color, "opacify", Map.of("amount", amount));
        }

        /** Lowers alpha. */
        public static Color4 transparentize(Color4 color, double amount) throws ApiException {
            return colorApply(color, "transparentize", Map.of("amount", amount));
        }

        /** Sets the alpha channel. */
        public static Color4 withAlpha(Color4 color, double alpha) throws ApiException {
            return colorApply(color, "withAlpha", Map.of("alpha", alpha));
        }

        /** Samples a multi-stop gradient at {@code t} (0..=1). */
        public static Color4 gradient(
                List<Object> stops, double t, String space, boolean repeat) throws ApiException {
            return colorOf(
                    "color.gradient",
                    Map.of("stops", stops, "t", t, "space", space, "repeat", repeat));
        }

        /** Builds a color channel by sampling a gradient {@code count} times. */
        public static Object gradientChannel(
                List<Object> stops,
                int count,
                double start,
                double end,
                String space,
                boolean repeat)
                throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("stops", stops);
            input.put("count", count);
            input.put("start", start);
            input.put("end", end);
            input.put("space", space);
            input.put("repeat", repeat);
            return call("color.gradientChannel", input);
        }

        /** Resolves a 2D scene into per-sprite command sets. */
        public static List<Object> resolveScene2d(Object scene, Map<String, Object> options)
                throws ApiException {
            Object value =
                    call(
                            "scene2d.resolve",
                            Map.of("scene", scene, "options", options == null ? Map.of() : options));
            return JSON.getGson()
                    .fromJson(JSON.getGson().toJson(value), new TypeToken<List<Object>>() {}.getType());
        }

        /** Bakes a 2D scene into a full storyboard model (per-node layers). */
        public static Storyboard bakeScene2d(Object scene, Map<String, Object> options)
                throws ApiException {
            Object value =
                    call(
                            "scene2d.bake",
                            Map.of("scene", scene, "options", options == null ? Map.of() : options));
            return JSON.getGson().fromJson(JSON.getGson().toJson(value), Storyboard.class);
        }

        /** The scene's occupied [start, end] range. */
        public static Object scene2dTimeRange(Object scene) throws ApiException {
            return call("scene2d.timeRange", Map.of("scene", scene));
        }

        /** Translates the whole scene (children inherit). */
        public static Object translateScene2d(Object scene, List<Double> delta) throws ApiException {
            return call("scene2d.translate", Map.of("scene", scene, "delta", delta));
        }

        /** Scales the whole scene about its origin. */
        public static Object scaleScene2d(Object scene, double factor) throws ApiException {
            return call("scene2d.scale", Map.of("scene", scene, "factor", factor));
        }

        /** Rotates the whole scene (radians). */
        public static Object rotateScene2d(Object scene, double radians) throws ApiException {
            return call("scene2d.rotate", Map.of("scene", scene, "radians", radians));
        }

        /** Flattens the whole scene. */
        public static Object flattenScene2d(Object scene, String axis, String mode)
                throws ApiException {
            Map<String, Object> input = new HashMap<>();
            input.put("scene", scene);
            if (axis != null) {
                input.put("axis", axis);
            }
            input.put("mode", mode);
            return call("scene2d.flatten", input);
        }

        /** Shifts the whole scene in time. */
        public static Object shiftScene2d(Object scene, double delta) throws ApiException {
            return call("scene2d.shift", Map.of("scene", scene, "delta", delta));
        }

        /** Reverses the whole scene in time. */
        public static Object reverseScene2d(Object scene, double start, double end)
                throws ApiException {
            return call("scene2d.reverse", Map.of("scene", scene, "start", start, "end", end));
        }

        /** Bakes a 3D world ({@code { nodes, camera, options }}) into a storyboard. */
        public static Storyboard bakeWorld(Object world, Map<String, Object> options2d)
                throws ApiException {
            Object value =
                    call(
                            "scene3d.bakeWorld",
                            Map.of("world", world, "options2d", options2d == null ? Map.of() : options2d));
            return JSON.getGson().fromJson(JSON.getGson().toJson(value), Storyboard.class);
        }

        /** Bakes a set of 2D and 3D worlds into one storyboard. */
        public static Storyboard bakeScene(Map<String, Object> input) throws ApiException {
            return JSON.getGson()
                    .fromJson(
                            JSON.getGson().toJson(call("scene3d.bakeScene", input)),
                            Storyboard.class);
        }
    }

    /** The storyboarding library: `.osu`/`.osb` parsing. */
    public static final class Storyboarding {
        private static final moe.tochi.osbtk.client.core.Parsing Parsing =
                new moe.tochi.osbtk.client.core.Parsing(
                        new moe.tochi.osbtk.api.ParsingAPI(statelessClient()));
        private static final moe.tochi.osbtk.client.core.Origin Origin =
                new moe.tochi.osbtk.client.core.Origin(
                        new moe.tochi.osbtk.api.OriginAPI(statelessClient()));
        private static final moe.tochi.osbtk.api.HelpersAPI Helpers = new moe.tochi.osbtk.api.HelpersAPI(statelessClient());

        private Storyboarding() {}

        /** Parses a `.osu` beatmap into the typed model. */
        public static Beatmap parseOsu(String text) throws ApiException {
            return Parsing.parseOsu(text);
        }

        /** Parses a `.osb` storyboard into a summary. */
        public static ParseOsbResult parseOsb(String text) throws ApiException {
            return Parsing.parseOsb(text);
        }

        /** {@code OsbSprite.GetOriginVector}. */
        public static Vector2 getOriginVector(OsbOrigin origin, double width, double height)
                throws ApiException {
            return Origin.getOriginVector(origin, width, height);
        }

        /** {@code OsbSprite.InScreenBounds}. */
        public static boolean inScreenBounds(
                Vector2 position, Vector2 size, double rotation, Vector2 origin)
                throws ApiException {
            return Origin.inScreenBounds(position, size, rotation, origin);
        }

        /**
         * The 2D storyboard command utilities: pure transforms over a
         * {@code List<StoryboardCommand>}, canonical in
         * {@code osb_runtime::storyboarding::helpers}.
         */
        public static final class Commands {
            private Commands() {}

            /** Calls any helper by name with a raw input object. */
            public static Object call(String name, Map<String, Object> input) throws ApiException {
                return Helpers.commands(new HelperRequest().name(name).input(input)).getValue();
            }

            private static List<StoryboardCommand> commands(String name, Map<String, Object> input)
                    throws ApiException {
                return JSON.getGson()
                        .fromJson(
                                JSON.getGson().toJson(call(name, input)),
                                new TypeToken<List<StoryboardCommand>>() {}.getType());
            }

            private static String kind(StoryboardCommandKind kind) {
                return kind == null ? null : kind.getValue();
            }

            /** Selects a timeframe (and/or kind) of commands without mutating the source. */
            public static List<StoryboardCommand> selectCommands(
                    List<StoryboardCommand> commands,
                    StoryboardCommandKind kind,
                    Double start,
                    Double end)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("kind", kind(kind));
                input.put("start", start);
                input.put("end", end);
                return commands("selectCommands", input);
            }

            /** Moves a command set in time by {@code delta} ms. */
            public static List<StoryboardCommand> shiftCommands(
                    List<StoryboardCommand> commands, double delta) throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("delta", delta);
                return commands("shiftCommands", input);
            }

            /** Stretches a command set in time about {@code anchor} by {@code factor}. */
            public static List<StoryboardCommand> scaleCommands(
                    List<StoryboardCommand> commands, double factor, double anchor)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("factor", factor);
                input.put("anchor", anchor);
                return commands("scaleCommands", input);
            }

            /** Reverses a command set in time about {@code [start, end]}. */
            public static List<StoryboardCommand> reverseCommands(
                    List<StoryboardCommand> commands, double start, double end)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("start", start);
                input.put("end", end);
                return commands("reverseCommands", input);
            }

            /** Merges adjacent commands that are continuous in time and value. */
            public static List<StoryboardCommand> mergeCommands(
                    List<StoryboardCommand> commands, double tolerance) throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("tolerance", tolerance);
                return commands("mergeCommands", input);
            }

            /** Splits commands spanning {@code at} into two linear pieces. */
            public static List<StoryboardCommand> splitCommands(
                    List<StoryboardCommand> commands, double at) throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("at", at);
                return commands("splitCommands", input);
            }

            /** Collapses a command channel to a constant. */
            public static List<StoryboardCommand> flattenCommands(
                    List<StoryboardCommand> commands, StoryboardCommandKind kind, String mode)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("kind", kind(kind));
                input.put("mode", mode);
                return commands("flattenCommands", input);
            }

            /** Seeded jitter of command values. */
            public static List<StoryboardCommand> randomizeCommands(
                    List<StoryboardCommand> commands, double min, double max, long seed)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("min", min);
                input.put("max", max);
                input.put("seed", seed);
                return commands("randomizeCommands", input);
            }

            /** Keeps the commands matching a kind/time selector. */
            public static List<StoryboardCommand> filterCommands(
                    List<StoryboardCommand> commands,
                    StoryboardCommandKind kind,
                    Double start,
                    Double end)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("kind", kind(kind));
                input.put("start", start);
                input.put("end", end);
                return commands("filterCommands", input);
            }

            /** Removes the commands matching a kind/time selector. */
            public static List<StoryboardCommand> removeCommands(
                    List<StoryboardCommand> commands,
                    StoryboardCommandKind kind,
                    Double start,
                    Double end)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("kind", kind(kind));
                input.put("start", start);
                input.put("end", end);
                return commands("removeCommands", input);
            }

            /** Applies a scalar value transform ({@code value * scale + offset}) to every command. */
            public static List<StoryboardCommand> mapCommands(
                    List<StoryboardCommand> commands, double valueScale, double valueOffset)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("valueScale", valueScale);
                input.put("valueOffset", valueOffset);
                return commands("mapCommands", input);
            }

            /** Clones a command set {@code count} times, each copy shifted by {@code offset} ms. */
            public static List<StoryboardCommand> duplicateCommands(
                    List<StoryboardCommand> commands, double offset, int count) throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("offset", offset);
                input.put("count", count);
                return commands("duplicateCommands", input);
            }

            /** Snaps command times to a beat grid (and values to {@code valueStep}). */
            public static List<StoryboardCommand> alignCommands(
                    List<StoryboardCommand> commands,
                    double beat,
                    double offset,
                    int subdivision,
                    double valueStep)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("beat", beat);
                input.put("offset", offset);
                input.put("subdivision", subdivision);
                input.put("valueStep", valueStep);
                return commands("alignCommands", input);
            }

            /** Rounds command times/values to a step. */
            public static List<StoryboardCommand> quantizeCommands(
                    List<StoryboardCommand> commands, double timeStep, double valueStep)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("commands", commands);
                input.put("timeStep", timeStep);
                input.put("valueStep", valueStep);
                return commands("quantizeCommands", input);
            }
        }
    }

    /** The animations library: easing evaluation. */
    public static final class Animations {
        private static final moe.tochi.osbtk.client.core.Animations CLIENT =
                new moe.tochi.osbtk.client.core.Animations(
                        new moe.tochi.osbtk.api.AnimationsAPI(statelessClient()));

        private Animations() {}

        /** Applies a named easing to a normalized time in `0..1`. */
        public static double applyEasing(Easing easing, double t) throws ApiException {
            return CLIENT.applyEasing(easing, t);
        }

        /** Evaluates a CSS cubic-bezier easing ({@code CustomEasing}). */
        public static double applyCubicBezier(double x1, double y1, double x2, double y2, double t)
                throws ApiException {
            return CLIENT.easingValue(t, null, List.of(x1, y1, x2, y2));
        }
    }

    /** The audio library: FFT. */
    public static final class Audio {
        private static final moe.tochi.osbtk.client.runtime.Audio CLIENT =
                new moe.tochi.osbtk.client.runtime.Audio(
                        new moe.tochi.osbtk.api.AudioAPI(statelessClient()));

        private Audio() {}

        /** Reads FFT magnitudes from a `.wav` file. */
        public static FftResult fft(FftRequest request) throws ApiException {
            return CLIENT.fft(request.getPath(), request.getMagnitudes(), request.getTime());
        }
    }

    /**
     * The 3D storyboard library: a sibling that reuses the whole
     * {@link Storyboarding} surface and adds the 3D -> 2D bake, so the 3D API is
     * exposed alongside {@link Storyboarding}.
     */
    public static final class Storyboarding3d {
        private static final moe.tochi.osbtk.client.runtime.Storyboarding3d CLIENT =
                new moe.tochi.osbtk.client.runtime.Storyboarding3d(
                        new moe.tochi.osbtk.api.Storyboarding3dAPI(statelessClient()));
        private static final moe.tochi.osbtk.api.HelpersAPI HelpersClient = new moe.tochi.osbtk.api.HelpersAPI(statelessClient());

        private Storyboarding3d() {}

        /** Parses a `.osu` beatmap (shared with {@link Storyboarding}). */
        public static Beatmap parseOsu(String text) throws ApiException {
            return Storyboarding.parseOsu(text);
        }

        /** Parses a `.osb` storyboard (shared with {@link Storyboarding}). */
        public static ParseOsbResult parseOsb(String text) throws ApiException {
            return Storyboarding.parseOsb(text);
        }

        /** Bakes a 3D scene into a 2D storyboard. */
        public static BakedStoryboard bake3d(Scene3dInput scene) throws ApiException {
            return CLIENT.bake3d(scene);
        }

        /**
         * The 3D motion-graphics keyframe helpers: pure keyframe-channel
         * utilities, canonical in {@code osb_runtime::storyboarding3d::helpers}.
         */
        public static final class Helpers {
            private Helpers() {}

            /** A single keyframe: a time and a value of one, two or three components. */
            public static final class ChannelKey {
                private double time;
                private List<Double> value;

                public ChannelKey() {}

                public ChannelKey(double time, List<Double> value) {
                    this.time = time;
                    this.value = value;
                }

                public double getTime() {
                    return time;
                }

                public List<Double> getValue() {
                    return value;
                }
            }

            /** A keyframed channel: a list of {@code (time, value)} samples. */
            public static final class Channel {
                private List<ChannelKey> keys;

                public Channel() {}

                public Channel(List<ChannelKey> keys) {
                    this.keys = keys;
                }

                public List<ChannelKey> getKeys() {
                    return keys;
                }
            }

            /** Calls any helper by name with a raw input object. */
            public static Object call(String name, Map<String, Object> input) throws ApiException {
                return HelpersClient.keyframes(new HelperRequest().name(name).input(input))
                        .getValue();
            }

            private static Channel channel(String name, Map<String, Object> input)
                    throws ApiException {
                return JSON.getGson()
                        .fromJson(JSON.getGson().toJson(call(name, input)), Channel.class);
            }

            private static Map<String, Object> withChannel(Channel channel) {
                Map<String, Object> input = new HashMap<>();
                input.put("channel", channel);
                return input;
            }

            /** Collapses an axis (or every axis) to a constant. */
            public static Channel flatten(Channel channel, String axis, String mode)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("axis", axis);
                input.put("mode", mode);
                return channel("flatten", input);
            }

            /** Multiplies every component by {@code factor}. */
            public static Channel scaleKeyframes(Channel channel, double factor)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("factor", factor);
                return channel("scaleKeyframes", input);
            }

            /** Adds a constant {@code delta} to every component. */
            public static Channel offsetKeyframes(Channel channel, double delta)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("delta", delta);
                return channel("offsetKeyframes", input);
            }

            /** Reflects an axis (or every axis) about {@code pivot}. */
            public static Channel mirror(Channel channel, String axis, double pivot)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("axis", axis);
                input.put("pivot", pivot);
                return channel("mirror", input);
            }

            /** Seeded jitter in {@code min..max}, preserving keyframe times. */
            public static Channel randomize(
                    Channel channel, String axis, double min, double max, long seed, double step)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("axis", axis);
                input.put("min", min);
                input.put("max", max);
                input.put("seed", seed);
                input.put("step", step);
                return channel("randomize", input);
            }

            /** Reverses the channel in time about {@code [start, end]}. */
            public static Channel reverseKeyframes(Channel channel, double start, double end)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("start", start);
                input.put("end", end);
                return channel("reverseKeyframes", input);
            }

            /** Tiles the {@code [start, end]} span {@code count} times. */
            public static Channel repeatKeyframes(
                    Channel channel, int count, double start, double end, boolean pingPong)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("count", count);
                input.put("start", start);
                input.put("end", end);
                input.put("pingPong", pingPong);
                return channel("repeatKeyframes", input);
            }

            /** Resamples to a fixed {@code step} (or to exactly {@code count} samples). */
            public static Channel resampleKeyframes(Channel channel, double step, int count)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("step", step);
                input.put("count", count);
                return channel("resampleKeyframes", input);
            }

            /** Moving-average smoothing over a window of neighbouring keyframes. */
            public static Channel smoothKeyframes(Channel channel, int window)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("window", window);
                return channel("smoothKeyframes", input);
            }

            /** Douglas-Peucker simplification of each component. */
            public static Channel simplifyKeyframes(
                    Channel channel, double tolerance, int decimals) throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("tolerance", tolerance);
                input.put("decimals", decimals);
                return channel("simplifyKeyframes", input);
            }

            /** Snaps keyframe times to a beat grid. */
            public static Channel alignKeyframes(
                    Channel channel, double beat, double offset, int subdivision)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("beat", beat);
                input.put("offset", offset);
                input.put("subdivision", subdivision);
                return channel("alignKeyframes", input);
            }

            /** Re-eases each span (named easing). */
            public static Channel easeKeyframes(Channel channel, String easing, int samples)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("easing", easing);
                input.put("samples", samples);
                return channel("easeKeyframes", input);
            }

            /** Blends the channel toward {@code other} by {@code amount} ({@code 0..=1}). */
            public static Channel mergeKeyframes(Channel channel, Channel other, double amount)
                    throws ApiException {
                Map<String, Object> input = withChannel(channel);
                input.put("other", other);
                input.put("amount", amount);
                return channel("mergeKeyframes", input);
            }

            /** A stagger delay for element {@code index}: {@code index * step}. */
            public static double stagger(int index, double step) throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("index", index);
                input.put("step", step);
                return ((Number) call("stagger", input)).doubleValue();
            }

            /** A circular orbit position channel. */
            public static Channel orbit(
                    List<Double> center,
                    double radiusX,
                    double radiusY,
                    double startAngle,
                    double sweep,
                    int count)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("center", center);
                input.put("radiusX", radiusX);
                input.put("radiusY", radiusY);
                input.put("startAngle", startAngle);
                input.put("sweep", sweep);
                input.put("count", count);
                return channel("orbit", input);
            }

            /** A pulse oscillation channel. */
            public static Channel pulse(
                    double base,
                    double amplitude,
                    double frequency,
                    double phase,
                    double step,
                    double duration)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("base", base);
                input.put("amplitude", amplitude);
                input.put("frequency", frequency);
                input.put("phase", phase);
                input.put("step", step);
                input.put("duration", duration);
                return channel("pulse", input);
            }

            /** Seeded random placements in a box. */
            public static List<Vector3> scatter(
                    long seed, int count, List<Double> min, List<Double> max, double spread)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("seed", seed);
                input.put("count", count);
                input.put("min", min);
                input.put("max", max);
                input.put("spread", spread);
                return JSON.getGson()
                        .fromJson(
                                JSON.getGson().toJson(call("scatter", input)),
                                new TypeToken<List<Vector3>>() {}.getType());
            }

            /** A procedural spin about {@code axis}. */
            public static Channel spin(String axis, double turns, double duration, int count)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("axis", axis);
                input.put("turns", turns);
                input.put("duration", duration);
                input.put("count", count);
                return channel("spin", input);
            }

            /** Independent spins about X, Y and Z. */
            public static Channel tumble(List<Double> turns, double duration, int count)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("turns", turns);
                input.put("duration", duration);
                input.put("count", count);
                return channel("tumble", input);
            }

            /** A position channel sampled along a polyline. */
            public static Channel pathFollow(
                    List<List<Double>> points, int count, boolean byArcLength)
                    throws ApiException {
                Map<String, Object> input = new HashMap<>();
                input.put("points", points);
                input.put("count", count);
                input.put("byArcLength", byArcLength);
                return channel("pathFollow", input);
            }
        }
    }

    /**
     * The curve library: Bezier/Catmull-Rom curves, sampling, arc length, tangents,
     * bounding boxes and Douglas-Peucker simplification.
     */
    public static final class Curve {
        private static final moe.tochi.osbtk.client.core.Curve CLIENT =
                new moe.tochi.osbtk.client.core.Curve(
                        new moe.tochi.osbtk.api.CurveAPI(statelessClient()));

        private Curve() {}

        /** The arc length of a curve. */
        public static double length(CurveSpec curve) throws ApiException {
            return CLIENT.length(curve);
        }

        /** The axis-aligned bounding box of a curve. */
        public static Box2 boundingBox(CurveSpec curve) throws ApiException {
            return CLIENT.boundingBox(curve);
        }

        /** The point at {@code t}, parametrically or by arc length. */
        public static Vector2 point(CurveSpec curve, double t, boolean byArcLength)
                throws ApiException {
            return CLIENT.point(curve, t, byArcLength);
        }

        /** The unit tangent at {@code t}. */
        public static Vector2 tangent(CurveSpec curve, double t) throws ApiException {
            return CLIENT.tangent(curve, t);
        }

        /** {@code count} points along the curve, parametrically or by arc length. */
        public static PointListResult sample(CurveSpec curve, int count, boolean byArcLength)
                throws ApiException {
            return CLIENT.sample(count, curve, byArcLength);
        }

        /** Simplifies a polyline with the Douglas-Peucker algorithm. */
        public static PointListResult simplify(List<Vector2> points, double tolerance)
                throws ApiException {
            return CLIENT.simplify(points, tolerance);
        }

        /** The unit normal at {@code t}. */
        public static Vector2 normal(CurveSpec curve, double t) throws ApiException {
            return CLIENT.normal(curve, t);
        }

        /** The curvature at {@code t}. */
        public static double curvature(CurveSpec curve, double t) throws ApiException {
            return CLIENT.curvature(curve, t);
        }

        /** The distance from {@code point} to the curve. */
        public static double distanceTo(CurveSpec curve, Vector2 point) throws ApiException {
            return CLIENT.distanceTo(curve, point);
        }

        /** The polyline offset along the curve's local normal. */
        public static PointListResult offset(CurveSpec curve, double distance) throws ApiException {
            return CLIENT.offset(curve, distance);
        }

        /** Points along a circular arc. */
        public static PointListResult arc(
                Vector2 center, double radius, double startAngle, double sweepAngle, int count)
                throws ApiException {
            return CLIENT.arc(center, radius, startAngle, sweepAngle, count);
        }

        /** Points around an ellipse. */
        public static PointListResult ellipse(
                Vector2 center, double radiusX, double radiusY, int count) throws ApiException {
            return CLIENT.ellipse(center, radiusX, radiusY, count);
        }

        /** The intersection of two line segments, if any. */
        public static Vector2 lineIntersect(
                Vector2 a1, Vector2 a2, Vector2 b1, Vector2 b2) throws ApiException {
            return CLIENT.lineIntersect(a1, a2, b1, b2);
        }
    }

    /** The diagnostics library: storyboard and mapset analysis. */
    public static final class Diagnostics {
        private static final moe.tochi.osbtk.client.libraries.Diagnostics CLIENT =
                new moe.tochi.osbtk.client.libraries.Diagnostics(
                        new moe.tochi.osbtk.api.DiagnosticsAPI(statelessClient()));

        private Diagnostics() {}

        /** Diagnoses an `.osb` storyboard. */
        public static DiagnosticReport diagnoseOsb(String text) throws ApiException {
            return CLIENT.diagnoseOsb(text);
        }

        /** Diagnoses an `.osu` beatmap. */
        public static DiagnosticReport diagnoseOsu(String text) throws ApiException {
            return CLIENT.diagnoseOsu(text);
        }
    }

    /** The optimization library: storyboard optimization and `.osb` round-trip. */
    public static final class Optimize {
        private static final moe.tochi.osbtk.client.libraries.Optimize CLIENT =
                new moe.tochi.osbtk.client.libraries.Optimize(
                        new moe.tochi.osbtk.api.OptimizeAPI(statelessClient()));

        private Optimize() {}

        /** Optimizes an `.osb` storyboard, returning the text and stats. */
        public static OptimizeResult optimizeOsb(String text, OptimizeOptionsInput options)
                throws ApiException {
            return CLIENT.optimizeOsb(options, text);
        }
    }

    /** The image library: decode/encode and processing. */
    public static final class Image {
        private static final moe.tochi.osbtk.client.libraries.Image CLIENT =
                new moe.tochi.osbtk.client.libraries.Image(
                        new moe.tochi.osbtk.api.ImageAPI(statelessClient()));

        private Image() {}

        /** Reads image metadata. */
        public static ImageInfo info(String path) throws ApiException {
            return CLIENT.info(path);
        }

        /** Processes an image (read, apply options, write). */
        public static ImageInfo process(String path, String output, ImageOptions options)
                throws ApiException {
            return CLIENT.process(options, output, path);
        }
    }

    /** The image-generation library: draw primitive shapes. */
    public static final class ImageGen {
        private static final moe.tochi.osbtk.client.libraries.Imagegen CLIENT =
                new moe.tochi.osbtk.client.libraries.Imagegen(
                        new moe.tochi.osbtk.api.ImagegenAPI(statelessClient()));

        private ImageGen() {}

        /** Renders primitive shapes to a new image. */
        public static ImageInfo renderShapes(RenderShapesRequest request) throws ApiException {
            if (request.getBackground() == null) {
                return CLIENT.renderShapes(
                        request.getWidth(),
                        request.getHeight(),
                        request.getShapes(),
                        request.getOutput());
            }
            return CLIENT.renderShapes(
                    request.getWidth(),
                    request.getHeight(),
                    request.getShapes(),
                    request.getOutput(),
                    request.getBackground());
        }
    }

    /** The media library: probing, frame extraction/encoding and audio. */
    public static final class Media {
        private static final moe.tochi.osbtk.client.libraries.Media CLIENT =
                new moe.tochi.osbtk.client.libraries.Media(
                        new moe.tochi.osbtk.api.MediaAPI(statelessClient()));

        private Media() {}

        /** Probes a media file. */
        public static MediaInfo info(String path) throws ApiException {
            return CLIENT.info(path);
        }

        /** Extracts frames from a video into a directory. */
        public static FrameResult extractFrames(ExtractFramesRequest request) throws ApiException {
            return CLIENT.extractFrames(
                    request.getInput(),
                    request.getOutputDir(),
                    request.getCount(),
                    request.getFps(),
                    request.getHeight(),
                    request.getStart(),
                    request.getWidth());
        }

        /** Encodes an image sequence into a video. */
        public static MediaInfo encodeVideo(EncodeVideoRequest request) throws ApiException {
            return CLIENT.encodeVideo(
                    request.getInputPattern(),
                    request.getOutput(),
                    request.getFps(),
                    request.getCodec(),
                    request.getCrf());
        }

        /** Extracts the audio track of a media file. */
        public static void extractAudio(ExtractAudioRequest request) throws ApiException {
            CLIENT.extractAudio(request.getInput(), request.getOutput());
        }
    }

    /**
     * A storyboard generator (the {@code StoryboardObjectGenerator} equivalent).
     * Subclass it and override {@link #generate()}.
     */
    public abstract static class Generator {
        /** The live storyboard runtime. */
        public final Runtime runtime;

        protected Generator() {
            this(640.0, 480.0);
        }

        protected Generator(double width, double height) {
            this.runtime = new Runtime(width, height);
        }

        /** {@code CreateSprite}. */
        public Handle createSprite(String layer, String path, OsbOrigin origin) throws ApiException {
            return runtime.layer()
                    .createSprite(new CreateSpriteRequest().layer(layer).path(path).origin(origin));
        }

        /** {@code CreateSample}. */
        public void createSample(String layer, String path, double time, double volume)
                throws ApiException {
            runtime.layer()
                    .createSample(
                            new CreateSampleRequest().layer(layer).path(path).time(time).volume(volume));
        }

        /** {@code GetBeatmap}. */
        public Beatmap getBeatmap() throws ApiException {
            return runtime.generator().beatmap();
        }

        /** {@code GetFft}. */
        public FftResult getFft(double time, int magnitudes) throws ApiException {
            return runtime.audio().getFft(time, magnitudes);
        }

        /** {@code GetTimingPointAt}. */
        public ControlPoint getTimingPointAt(double time) throws ApiException {
            return runtime.generator().timingPointAt(time);
        }

        /** The number of sprites on the storyboard. */
        public int spriteCount() throws ApiException {
            return runtime.generator().spriteCount();
        }

        /** The built storyboard model, so {@code generate()}'s result can be read back. */
        public moe.tochi.osbtk.model.Storyboard storyboard() throws ApiException {
            return runtime.generator().storyboard();
        }

        /** Frees the underlying runtime. */
        public void close() {
            runtime.close();
        }

        /** {@code GetLayer}: a layer that creates sprites/animations/samples. */
        public Layer getLayer(String name) throws ApiException {
            int handle = runtime.generator().layerHandle(name);
            return new Layer(runtime.layer(), runtime.sprite(), name, handle);
        }

        /** {@code Generate}: build the storyboard. */
        public abstract void generate() throws ApiException;
    }

    /** A storyboard segment ({@code StoryboardSegment}). The handle is implicit. */
    public static final class Layer {
        private final moe.tochi.osbtk.api.LayerAPI api;
        private final moe.tochi.osbtk.api.SpriteAPI sprites;

        /** The layer name/identifier. */
        public final String name;

        /** The segment handle. */
        public final int handle;

        Layer(moe.tochi.osbtk.api.LayerAPI api, moe.tochi.osbtk.api.SpriteAPI sprites, String name, int handle) {
            this.api = api;
            this.sprites = sprites;
            this.name = name;
            this.handle = handle;
        }

        /** {@code CreateSprite}. */
        public Sprite createSprite(String texturePath, OsbOrigin origin) throws ApiException {
            Handle handle =
                    api.createSprite(
                            new CreateSpriteRequest()
                                    .layerHandle(this.handle)
                                    .path(texturePath)
                                    .origin(origin));
            return new Sprite(sprites, handle.getLayer(), handle.getSprite());
        }

        /** {@code CreateAnimation}. */
        public Sprite createAnimation(CreateAnimationRequest request) throws ApiException {
            Handle handle = api.createAnimation(request);
            return new Sprite(sprites, handle.getLayer(), handle.getSprite());
        }

        /** {@code CreateSample}. */
        public Layer createSample(String samplePath, double time, double volume) throws ApiException {
            api.createSample(
                    new CreateSampleRequest()
                            .layerHandle(this.handle)
                            .path(samplePath)
                            .time(time)
                            .volume(volume));
            return this;
        }

        /** {@code CreateSegment}: a segment nested in this one. */
        public Layer createSegment() throws ApiException {
            LayerHandle handle = api.createSegment(new SegmentRequest().layer(this.handle));
            return new Layer(api, sprites, "", handle.getLayer());
        }

        /** {@code ReverseDepth}. */
        public Layer setReverseDepth(boolean value) throws ApiException {
            api.reverseDepth(
                    new SegmentReverseDepthRequest().layer(this.handle).value(value));
            return this;
        }

        /** {@code AddIndex}. */
        public Layer addIndex(double index) throws ApiException {
            api.addIndex(new SegmentAddIndexRequest().layer(this.handle).index(index));
            return this;
        }

        /** {@code Indexes}/{@code LowestIndex}/{@code HighestIndex}. */
        public SegmentIndexesResult indexes() throws ApiException {
            return api.indexes(new SegmentRequest().layer(this.handle));
        }

        /** {@code ChangePosition}. */
        public Layer changePosition(OsbLayer osbLayer) throws ApiException {
            api.changePosition(
                    new SegmentChangePositionRequest().layer(this.handle).osbLayer(osbLayer));
            return this;
        }

        /** {@code MoveToTop}. */
        public Layer moveToTop() throws ApiException {
            api.moveToTop(new SegmentRequest().layer(this.handle));
            return this;
        }

        /** {@code MoveToBottom}. */
        public Layer moveToBottom() throws ApiException {
            api.moveToBottom(new SegmentRequest().layer(this.handle));
            return this;
        }

        /** {@code Clean}. */
        public SegmentCleanResult clean() throws ApiException {
            return api.clean(new SegmentRequest().layer(this.handle));
        }

        /** {@code Discard}. */
        public SegmentDiscardResult discard() throws ApiException {
            return api.discard(new SegmentRequest().layer(this.handle));
        }

        /** {@code DiscardSprite}. */
        public Layer discardSprite(Sprite sprite) throws ApiException {
            api.discardSprite(new HandleRequest().layer(sprite.layer).sprite(sprite.sprite));
            return this;
        }
    }

    /** A storyboard sprite ({@code OsbSprite}). */
    public static final class Sprite {
        private final moe.tochi.osbtk.api.SpriteAPI api;

        /** The layer handle. */
        public final int layer;

        /** The sprite handle. */
        public final int sprite;

        Sprite(moe.tochi.osbtk.api.SpriteAPI api, int layer, int sprite) {
            this.api = api;
            this.layer = layer;
            this.sprite = sprite;
        }

        /** The sprite's initial state and metadata. */
        public SpriteInfo info() throws ApiException {
            return api.info(new HandleRequest().layer(layer).sprite(sprite));
        }

        /** {@code Move}. */
        public Sprite move(
                double startTime, double endTime, Vector2 start, Vector2 end, Easing easing)
                throws ApiException {
            return add("move", startTime, endTime, easing, start, end, null);
        }

        /** {@code Move} (easing-first). */
        public Sprite move(Easing easing, double startTime, double endTime, Vector2 start, Vector2 end)
                throws ApiException {
            return add("move", startTime, endTime, easing, start, end, null);
        }

        /** {@code Move} (instant). */
        public Sprite move(double time, Vector2 position) throws ApiException {
            return add("move", time, time, Easing.NONE, position, position, null);
        }

        /** {@code Scale}. */
        public Sprite scale(
                double startTime, double endTime, double start, double end, Easing easing)
                throws ApiException {
            return add("scale", startTime, endTime, easing, start, end, null);
        }

        /** {@code Scale} (easing-first). */
        public Sprite scale(Easing easing, double startTime, double endTime, double start, double end)
                throws ApiException {
            return add("scale", startTime, endTime, easing, start, end, null);
        }

        /** {@code Scale} (instant). */
        public Sprite scale(double time, double scale) throws ApiException {
            return add("scale", time, time, Easing.NONE, scale, scale, null);
        }

        /** {@code Rotate} (radians). */
        public Sprite rotate(
                double startTime, double endTime, double start, double end, Easing easing)
                throws ApiException {
            return add("rotate", startTime, endTime, easing, start, end, null);
        }

        /** {@code Rotate} (easing-first). */
        public Sprite rotate(Easing easing, double startTime, double endTime, double start, double end)
                throws ApiException {
            return add("rotate", startTime, endTime, easing, start, end, null);
        }

        /** {@code Rotate} (instant). */
        public Sprite rotate(double time, double rotation) throws ApiException {
            return add("rotate", time, time, Easing.NONE, rotation, rotation, null);
        }

        /** {@code Fade}. */
        public Sprite fade(
                double startTime, double endTime, double start, double end, Easing easing)
                throws ApiException {
            return add("fade", startTime, endTime, easing, start, end, null);
        }

        /** {@code Fade} (easing-first). */
        public Sprite fade(Easing easing, double startTime, double endTime, double start, double end)
                throws ApiException {
            return add("fade", startTime, endTime, easing, start, end, null);
        }

        /** {@code Fade} (instant). */
        public Sprite fade(double time, double opacity) throws ApiException {
            return add("fade", time, time, Easing.NONE, opacity, opacity, null);
        }

        /** {@code Color}. */
        public Sprite color(
                double startTime, double endTime, Color4 start, Color4 end, Easing easing)
                throws ApiException {
            return add("color", startTime, endTime, easing, start, end, null);
        }

        /** {@code Color} (easing-first). */
        public Sprite color(Easing easing, double startTime, double endTime, Color4 start, Color4 end)
                throws ApiException {
            return add("color", startTime, endTime, easing, start, end, null);
        }

        /** {@code Color} (instant). */
        public Sprite color(double time, Color4 color) throws ApiException {
            return add("color", time, time, Easing.NONE, color, color, null);
        }

        /** {@code MoveX}. */
        public Sprite moveX(
                double startTime, double endTime, double start, double end, Easing easing)
                throws ApiException {
            return add("moveX", startTime, endTime, easing, start, end, null);
        }

        /** {@code MoveX} (easing-first). */
        public Sprite moveX(Easing easing, double startTime, double endTime, double start, double end)
                throws ApiException {
            return add("moveX", startTime, endTime, easing, start, end, null);
        }

        /** {@code MoveX} (instant). */
        public Sprite moveX(double time, double x) throws ApiException {
            return add("moveX", time, time, Easing.NONE, x, x, null);
        }

        /** {@code MoveY}. */
        public Sprite moveY(
                double startTime, double endTime, double start, double end, Easing easing)
                throws ApiException {
            return add("moveY", startTime, endTime, easing, start, end, null);
        }

        /** {@code MoveY} (easing-first). */
        public Sprite moveY(Easing easing, double startTime, double endTime, double start, double end)
                throws ApiException {
            return add("moveY", startTime, endTime, easing, start, end, null);
        }

        /** {@code MoveY} (instant). */
        public Sprite moveY(double time, double y) throws ApiException {
            return add("moveY", time, time, Easing.NONE, y, y, null);
        }

        /** {@code ScaleVec}. */
        public Sprite scaleVec(
                double startTime, double endTime, Vector2 start, Vector2 end, Easing easing)
                throws ApiException {
            return add("scaleVec", startTime, endTime, easing, start, end, null);
        }

        /** {@code ScaleVec} (easing-first). */
        public Sprite scaleVec(Easing easing, double startTime, double endTime, Vector2 start, Vector2 end)
                throws ApiException {
            return add("scaleVec", startTime, endTime, easing, start, end, null);
        }

        /** {@code ScaleVec} (instant). */
        public Sprite scaleVec(double time, Vector2 scale) throws ApiException {
            return add("scaleVec", time, time, Easing.NONE, scale, scale, null);
        }

        /** {@code ColorHsb}. */
        public Sprite colorHsb(
                double startTime, double endTime, Color4 start, Color4 end, Easing easing)
                throws ApiException {
            return add("colorHsb", startTime, endTime, easing, start, end, null);
        }

        /** {@code ColorHsb} (easing-first). */
        public Sprite colorHsb(Easing easing, double startTime, double endTime, Color4 start, Color4 end)
                throws ApiException {
            return add("colorHsb", startTime, endTime, easing, start, end, null);
        }

        /** {@code ColorHsb} (instant). */
        public Sprite colorHsb(double time, Color4 color) throws ApiException {
            return add("colorHsb", time, time, Easing.NONE, color, color, null);
        }

        /** {@code FlipH}. */
        public Sprite flipH(double startTime, double endTime) throws ApiException {
            return add("flipH", startTime, endTime, Easing.NONE, 0, 1, null);
        }

        /** {@code FlipH} (instant). */
        public Sprite flipH(double time) throws ApiException {
            return add("flipH", time, time, Easing.NONE, 0, 1, null);
        }

        /** {@code FlipV}. */
        public Sprite flipV(double startTime, double endTime) throws ApiException {
            return add("flipV", startTime, endTime, Easing.NONE, 0, 1, null);
        }

        /** {@code FlipV} (instant). */
        public Sprite flipV(double time) throws ApiException {
            return add("flipV", time, time, Easing.NONE, 0, 1, null);
        }

        /** {@code Additive}. */
        public Sprite additive(double startTime, double endTime) throws ApiException {
            return add("additive", startTime, endTime, Easing.NONE, 0, 1, null);
        }

        /** {@code Additive} (instant). */
        public Sprite additive(double time) throws ApiException {
            return add("additive", time, time, Easing.NONE, 0, 1, null);
        }

        /** {@code Parameter}. */
        public Sprite parameter(
                ParameterType parameter,
                double startTime,
                double endTime,
                double start,
                double end,
                Easing easing)
                throws ApiException {
            return add("parameter", startTime, endTime, easing, start, end, parameter);
        }

        /**
         * {@code Anchor}: an animatable origin offset in texture pixels, added to
         * the sprite's enum origin.
         */
        public Sprite anchor(
                double startTime, double endTime, Vector2 start, Vector2 end, Easing easing)
                throws ApiException {
            return add("anchor", startTime, endTime, easing, start, end, null);
        }

        /** {@code Anchor} (instant). */
        public Sprite anchor(double time, Vector2 point) throws ApiException {
            return add("anchor", time, time, Easing.NONE, point, point, null);
        }

        /** Sets a constant origin offset. */
        public Sprite setAnchor(Vector2 point) throws ApiException {
            return add("anchor", 0, 0, Easing.NONE, point, point, null);
        }

        /** The sampled origin offset at a time ({@code AnchorAt}). */
        public Vector2 anchorAt(double time) throws ApiException {
            return stateAt(time).getAnchor();
        }

        /** {@code StartLoopGroup}. */
        public Sprite startLoopGroup(double startTime, double count) throws ApiException {
            api.startLoopGroup(
                    new StartLoopGroupRequest()
                            .layer(layer)
                            .sprite(sprite)
                            .startTime(startTime)
                            .count(count));
            return this;
        }

        /** {@code EndGroup}. */
        public Sprite endGroup() throws ApiException {
            api.endGroup(new HandleRequest().layer(layer).sprite(sprite));
            return this;
        }

        /** The sampled position at a time. */
        public Vector2 positionAt(double time) throws ApiException {
            return stateAt(time).getPosition();
        }

        /** The sampled opacity at a time. */
        public double opacityAt(double time) throws ApiException {
            return stateAt(time).getOpacity();
        }

        private SpriteState stateAt(double time) throws ApiException {
            return api.stateAt(new SpriteTimeRequest().layer(layer).sprite(sprite).time(time));
        }

        private Sprite add(
                String kind,
                double startTime,
                double endTime,
                Easing easing,
                Object start,
                Object end,
                ParameterType parameter)
                throws ApiException {
            api.command(
                    new CommandRequest()
                            .kind(kind)
                            .layer(layer)
                            .sprite(sprite)
                            .startTime(startTime)
                            .endTime(endTime)
                            .easing(easing)
                            .start(start)
                            .end(end)
                            .parameter(parameter));
            return this;
        }
    }
}
