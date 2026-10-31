using Newtonsoft.Json.Linq;

using TochiRes.Errors;

using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

using Xunit;

using Entry = global::tochiRes.osbTK.osb;

namespace tochiRes.osbTK.Tests;

/// <summary>
/// The generated SDK surface: typed models and the generated clients driven
/// through the native bridge transport. The golden-corpus case proves the SDK
/// agrees with Rust, TypeScript, C++, Python and the other bindings.
/// </summary>
public class SdkTests
{
    [Fact]
    public void ExposesTheNativeVersion() => Assert.False(string.IsNullOrEmpty(Entry.Version));

    [Fact]
    public void ParsesABeatmapIntoTheTypedModel()
    {
        Beatmap beatmap = Entry.Storyboarding.ParseOsu(
            "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
                + "[TimingPoints]\n0,500,4,2,1,100,1,0\n"
                + "[HitObjects]\n256,192,1000,1,0\n"
        );
        Assert.Equal("song.mp3", beatmap.AudioFilename);
        Assert.Equal(120.0, beatmap.TimingPoints[0].Bpm);
        Assert.Equal(HitObjectKind.Circle, beatmap.HitObjects[0].Kind);
    }

    [Fact]
    public void AppliesEasing() => Assert.Equal(0.25, Entry.Core.ApplyEasing(Easing.InQuad, 0.5), 6);

    [Fact]
    public void DrivesTheStoryboardThroughTheGeneratedClient()
    {
        using Entry.Storyboarding.Runtime runtime = new();
        Sprite sprite = runtime
            .Layer("Foreground")
            .CreateSprite("sb/dot.png", OsbOrigin.Centre);
        sprite.Move(0, 1000, new Vector2 { X = 0, Y = 0 }, new Vector2 { X = 100, Y = 0 });

        Assert.Equal(50.0, sprite.PositionAt(500).X, 4);
        Assert.Equal(1, runtime.SpriteCount);
    }

    [Fact]
    public void ReadsBackTheBuiltStoryboard()
    {
        using Entry.Storyboarding.Runtime runtime = new();
        Sprite sprite = runtime
            .Layer("Foreground")
            .CreateSprite("sb/dot.png", OsbOrigin.Centre);
        sprite.Fade(0, 1000, 0, 1);

        Storyboard storyboard = runtime.Storyboard;
        StoryboardLayer foreground = storyboard.Layers.Single(layer => layer.Identifier == "Foreground");
        Assert.Equal("sb/dot.png", foreground.Sprites[0].TexturePath);
        Assert.Equal(StoryboardCommandKind.Fade, foreground.Sprites[0].Commands[0].Kind);
    }

    [Fact]
    public void AnimatesAndSamplesASpriteAnchor()
    {
        using Entry.Storyboarding.Runtime runtime = new();
        Sprite sprite = runtime
            .Layer("Foreground")
            .CreateSprite("sb/dot.png", OsbOrigin.Centre);
        sprite.Anchor(
            Easing.None,
            0,
            1000,
            new Vector2 { X = 0, Y = 0 },
            new Vector2 { X = 100, Y = 50 }
        );

        Assert.Equal(50.0, sprite.AnchorAt(500).X, 4);
        Assert.Equal(25.0, sprite.AnchorAt(500).Y, 4);
    }

    [Fact]
    public void SamplesAnimationsAndLoopGroups()
    {
        using Entry.Storyboarding.Runtime runtime = new();
        Sprite sprite = runtime
            .Layer(OsbLayer.Foreground)
            .CreateAnimation(
                "sb/frame.png",
                4,
                50,
                OsbLoopType.LoopForever,
                OsbOrigin.Centre,
                new Vector2 { X = 320, Y = 240 }
            );
        sprite.StartLoopGroup(0, 3).Fade(0, 100, 0, 1).EndGroup();

        Assert.Equal(4, sprite.Info.Animation!.Frames);
        Assert.Equal(OsbLoopType.LoopForever, sprite.Info.Animation.LoopType);
        Assert.Equal(1, sprite.Info.LoopCount);
    }

    [Fact]
    public void LoadsAndQueriesABeatmap()
    {
        using Entry.Storyboarding.Runtime runtime = new();
        runtime.LoadBeatmap(
            "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
                + "[TimingPoints]\n0,500,4,2,1,100,1,0\n"
                + "[HitObjects]\n256,192,1000,1,0\n"
        );
        Assert.Equal("song.mp3", runtime.Beatmap!.AudioFilename);
        Assert.Equal(120.0, runtime.TimingPointAt(1500).Bpm);
    }

    [Fact]
    public void SurfacesNativeErrorsAsSdkExceptions()
    {
        osbTKException error = Assert.Throws<osbTKException>(() =>
            Entry.Audio.Fft("does-not-exist.wav")
        );
        Assert.IsAssignableFrom<TochiResException>(error);
        Assert.Equal(ErrorCodes.NativeCallFailed, error.Code);
        Assert.Equal("native", error.Category);
    }

    [Fact]
    public void ExposesTheStoryboardFacade()
    {
        using Entry.Storyboarding.Runtime runtime = new();
        Layer layer = runtime.GetLayer(OsbLayer.Foreground);
        Sprite sprite = layer
            .CreateSprite("sb/dot.png", OsbOrigin.Centre)
            .Move(Easing.None, 0, 1000, new Vector2 { X = 0, Y = 0 }, new Vector2 { X = 100, Y = 0 })
            .Fade(0, 1.0);
        Assert.Equal(50.0, sprite.PositionAt(500).X, 4);

        layer.CreateSample("sb/hit.ogg", 1000, 80);

        FftResult fft = runtime.GetFft(0, 8);
        Assert.Equal(8, fft.Magnitudes.Count);
    }

    [Fact]
    public void NestsSegmentsThroughTheFacade()
    {
        using Entry.Storyboarding.Runtime runtime = new();
        Layer group = runtime.GetLayer("Group");
        group.CreateSprite("sb/group.png", OsbOrigin.Centre);
        Layer child = group.CreateSegment();
        child.CreateSprite("sb/child.png", OsbOrigin.Centre);
        child.SetReverseDepth(true).AddIndex(3).AddIndex(-1);

        SegmentIndexesResult indexes = child.Indexes();
        Assert.Equal(3.0, indexes.Highest);
        Assert.Equal(-1.0, indexes.Lowest);
        Assert.Equal(2, runtime.SpriteCount);

        child.Discard();
        Assert.Equal(1, runtime.SpriteCount);
    }

    [Fact]
    public void Storyboarding3dReusesTheStoryboardingSurface()
    {
        Beatmap beatmap = Entry.Storyboarding3d.ParseOsu(
            "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
                + "[TimingPoints]\n0,500,4,2,1,100,1,0\n"
                + "[HitObjects]\n256,192,1000,1,0\n"
        );
        Assert.Equal("song.mp3", beatmap.AudioFilename);

        using Entry.Storyboarding3d.Runtime runtime = new();
        Sprite sprite = runtime
            .Layer("Foreground")
            .CreateSprite("sb/dot.png", OsbOrigin.Centre);
        sprite.Move(0, 1000, new Vector2 { X = 0, Y = 0 }, new Vector2 { X = 100, Y = 0 });

        Assert.Equal(50.0, sprite.PositionAt(500).X, 4);
        Assert.Equal(1, runtime.SpriteCount);
    }

    [Fact]
    public void BakesTheSharedGoldenCorpus()
    {
        JObject corpus = JObject.Parse(File.ReadAllText(GoldenCorpusPath()));
        int cases = 0;
        foreach (JToken testCase in (JArray)corpus["cases"]!)
        {
            Scene3dInput scene = testCase["input"]!.ToObject<Scene3dInput>()!;
            BakedStoryboard baked = Entry.Storyboarding3d.Bake3d(scene);
            IReadOnlyList<BakedSprite> sprites = baked.Sprites;
            JToken expect = testCase["expect"]!;

            if (expect["spriteCount"] is not null)
            {
                Assert.Equal(expect["spriteCount"]!.Value<int>(), sprites.Count);
            }
            if (expect["firstSpritePath"] is not null)
            {
                Assert.Equal(expect["firstSpritePath"]!.Value<string>(), sprites[0].SpritePath);
            }
            cases++;
        }

        Assert.True(cases > 0, "the golden corpus must have cases");
    }

    [Fact]
    public void BakesATypedSceneAndEvaluatesCustomEasing()
    {
        Scene3dInput scene = new(
            nodes: [new Node3dInput(sprite: "sb/dot.png")],
            camera: new Camera3dInput(position: [new Keyframe3(time: 0, value: [0, 0, 5])]),
            options: new BakeOptionsInput(endTime: 0)
        );
        BakedStoryboard baked = Entry.Storyboarding3d.Bake3d(scene);
        Assert.Single(baked.Sprites);
        Assert.Equal("sb/dot.png", baked.Sprites[0].SpritePath);
        Assert.NotEmpty(baked.Sprites[0].Commands);

        Assert.Equal(0.0, Entry.Core.EasingValue(0.0, null, [0.42, 0.0, 0.58, 1.0]), 6);
        Assert.Equal(1.0, Entry.Core.EasingValue(1.0, null, [0.42, 0.0, 0.58, 1.0]), 6);
    }

    [Fact]
    public void DrivesTheCurveLibrary()
    {
        CurveSpec curve = new(
            kind: CurveKind.Linear,
            points: [new Vector2 { X = 0, Y = 0 }, new Vector2 { X = 100, Y = 0 }]
        );
        Assert.Equal(100.0, Entry.Core.Length(curve), 6);
        Assert.Equal(50.0, Entry.Core.Point(curve, 0.5).X, 6);
        Assert.Equal(3, Entry.Core.Sample(3, curve).Points.Count);
        Assert.Equal(100.0, Entry.Core.BoundingBox(curve).Max.X, 6);
        Assert.Equal(0.0, Entry.Core.Normal(curve, 0.5).X, 6);
        Assert.Equal(0.0, Entry.Core.Curvature(curve, 0.5), 6);
        Assert.Equal(10.0, Entry.Core.DistanceTo(curve, new Vector2 { X = 50, Y = 10 }), 6);
        Assert.Equal(2, Entry.Core.Offset(curve, 5.0).Points.Count);

        PointListResult simplified = Entry.Core.Simplify(
            [
                new Vector2 { X = 0, Y = 0 },
                new Vector2 { X = 1, Y = 0.05 },
                new Vector2 { X = 2, Y = 0 },
                new Vector2 { X = 3, Y = 0 },
            ],
            0.1
        );
        Assert.Equal(2, simplified.Points.Count);

        Assert.Equal(3, Entry.Core.Arc(new Vector2 { X = 0, Y = 0 }, 10, 0, 180, 3).Points.Count);
        Assert.Equal(4, Entry.Core.Ellipse(new Vector2 { X = 0, Y = 0 }, 4, 2, 4).Points.Count);
        Vector2 hit = Entry.Core.LineIntersect(
            new Vector2 { X = 0, Y = 0 },
            new Vector2 { X = 10, Y = 10 },
            new Vector2 { X = 0, Y = 10 },
            new Vector2 { X = 10, Y = 0 }
        );
        Assert.Equal(5.0, hit.X, 6);
    }

    [Fact]
    public void DiagnosesArtifacts()
    {
        DiagnosticReport report = Entry.Diagnostics.DiagnoseOsu(
            "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
        );
        Assert.True(report.ErrorCount >= 1);

        DiagnosticReport osb = Entry.Diagnostics.DiagnoseOsb(
            "[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n F,0,0,100,0,1\n"
        );
        Assert.Empty(osb.Diagnostics);
    }

    [Fact]
    public void OptimizesStoryboards()
    {
        OptimizeResult result = Entry.Optimize.OptimizeOsb(
            new OptimizeOptionsInput(deduplicate: true),
            "[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n F,0,0,100,0,1\n F,0,0,100,0,1\n"
        );
        Assert.Equal(1, result.Stats.Deduplicated);
        Assert.Contains("Sprite", result.Osb);
    }

    [Fact]
    public void SurfacesImageErrors() =>
        Assert.Throws<osbTKException>(() => Entry.Image.Info("does-not-exist.png"));

    [Fact]
    public void RendersShapes()
    {
        string output = Path.Combine(Path.GetTempPath(), "osbtk-csharp-render.png");
        ImageInfo info = Entry.ImageGen.RenderShapes(
            32,
            32,
            [
                new Shape(
                    kind: ShapeKind.Rect,
                    center: new Vector2 { X = 16, Y = 16 },
                    size: new Vector2 { X = 20, Y = 20 },
                    fill: new RgbaColor { R = 10, G = 20, B = 30, A = 255 }
                ),
            ],
            output
        );
        Assert.Equal(32, info.Width);
        Assert.True(File.Exists(output));
    }

    [Fact]
    public void SurfacesMediaErrors() =>
        Assert.Throws<osbTKException>(() => Entry.Media.Info("does-not-exist.mp4"));

    [Fact]
    public void RunsAnEffectScript()
    {
        Pulse effect = new();
        effect.Generate();
        Assert.Equal(1, effect.SpriteCount);
    }

    private sealed class Pulse : Generator
    {
        public override void Generate()
        {
            Layer layer = GetLayer(OsbLayer.Foreground);
            Sprite sprite = layer.CreateSprite("sb/dot.png", OsbOrigin.Centre);
            sprite.Fade(0, 1.0);
        }
    }

    [Fact]
    public void RunsTheMotionGraphicsHelpers()
    {
        var channel = new Entry.Storyboarding3d.Helpers.Channel
        {
            Keys =
            {
                new() { Time = 0, Value = [1, 2] },
                new() { Time = 100, Value = [3, 4] },
            },
        };
        var scaled = Entry.Storyboarding3d.Helpers.ScaleKeyframes(channel, 2);
        Assert.Equal(6.0, scaled.Keys[1].Value[0], 6);
        Assert.Equal(8.0, scaled.Keys[1].Value[1], 6);

        var orbit = Entry.Storyboarding3d.Helpers.Orbit(radiusX: 10, radiusY: 10, count: 5);
        Assert.Equal(5, orbit.Keys.Count);

        var points = Entry.Storyboarding3d.Helpers.Scatter(seed: 1, count: 3);
        Assert.Equal(3, points.Count);

        Assert.Equal(300.0, Entry.Storyboarding3d.Helpers.Stagger(3, 100), 6);
    }

    [Fact]
    public void RunsTheCommandHelpers()
    {
        var commands = JArray
            .Parse(
                """
                [
                  { "kind": "fade", "startTime": 0, "endTime": 100, "easing": "None",
                    "start": { "kind": "scalar", "value": 0 }, "end": { "kind": "scalar", "value": 1 } },
                  { "kind": "fade", "startTime": 200, "endTime": 300, "easing": "None",
                    "start": { "kind": "scalar", "value": 1 }, "end": { "kind": "scalar", "value": 0 } }
                ]
                """
            )
            .ToObject<List<StoryboardCommand>>()!;

        var shifted = Entry.Storyboarding.Commands.ShiftCommands(commands, 50);
        Assert.Equal(50.0, shifted[0].StartTime, 6);

        var selected = Entry.Storyboarding.Commands.SelectCommands(commands, start: 150);
        Assert.Single(selected);
        Assert.Equal(200.0, selected[0].StartTime, 6);

        var merged = Entry.Storyboarding.Commands.MergeCommands(commands);
        Assert.Equal(2, merged.Count);

        var duplicated = Entry.Storyboarding.Commands.DuplicateCommands(commands, offset: 1000, count: 2);
        Assert.Equal(4, duplicated.Count);
    }

    [Fact]
    public void SharesTheMotionGraphicsHelpers()
    {
        // A 2D channel (no Z): the Z axis is optional.
        var scaled = Entry.Helpers.Call(
            "keyframe.scaleKeyframes",
            new JObject
            {
                ["channel"] = JObject.Parse(
                    """{"keys":[{"time":0,"value":[1,2]},{"time":100,"value":[3,4]}]}"""
                ),
                ["factor"] = 2,
            }
        );
        Assert.Equal(6.0, scaled["keys"]![1]!["value"]![0]!.Value<double>(), 6);
        Assert.Equal(8.0, scaled["keys"]![1]!["value"]![1]!.Value<double>(), 6);

        // The shared origin/anchor helper.
        Vector2 resolved = Entry.Helpers.ResolveAnchor(
            OsbOrigin.Centre,
            new Vector2 { X = 10, Y = -5 },
            100,
            100
        );
        Assert.Equal(60.0, resolved.X, 6);
        Assert.Equal(45.0, resolved.Y, 6);

        // The same keyframe helper applied to a 2D command channel.
        var commands = JArray
            .Parse(
                """
                [
                  { "kind": "fade", "startTime": 0, "endTime": 100, "easing": "None",
                    "start": { "kind": "scalar", "value": 0 }, "end": { "kind": "scalar", "value": 1 } },
                  { "kind": "fade", "startTime": 100, "endTime": 200, "easing": "None",
                    "start": { "kind": "scalar", "value": 1 }, "end": { "kind": "scalar", "value": 0 } }
                ]
                """
            )
            .ToObject<List<StoryboardCommand>>()!;
        var flattened = Entry.Helpers.TransformCommands(
            commands,
            StoryboardCommandKind.Fade,
            "keyframe.flatten",
            new JObject { ["mode"] = "mean" }
        );
        Assert.Single(flattened);
        Assert.Equal(0.0, flattened[0].StartTime, 6);
        Assert.Equal(200.0, flattened[0].EndTime, 6);
    }

    [Fact]
    public void ManipulatesEasingsTimelinesAndDepthAnchors()
    {
        // InQuad at 0.5 is 0.25; amplifying about linear gives 0.5 + (0.25-0.5)*2.
        Assert.Equal(0.0, Entry.Helpers.AmplifyEasing(Easing.InQuad, 0.5, 2.0), 6);

        var timeline = JObject.Parse(
            """
            {"clips":[
              {"start":0,"channel":{"keys":[{"time":0,"value":[0]},{"time":100,"value":[1]}]}},
              {"start":200,"channel":{"keys":[{"time":0,"value":[0]},{"time":100,"value":[1]}]}}]}
            """
        );
        Assert.Equal(300.0, Entry.Helpers.TimelineDuration(timeline), 6);
        var flushed = Entry.Helpers.FlushTimeline(timeline);
        Assert.Equal(
            200.0,
            flushed["clips"]![1]!["channel"]!["keys"]![0]!["time"]!.Value<double>(),
            6
        );
        Assert.Equal(4, ((JArray)Entry.Helpers.MergeTimelines(timeline, timeline)["clips"]!).Count);

        Vector3 anchor = Entry.Helpers.ResolveAnchor3d(
            OsbOrigin.Centre,
            new Vector3 { X = 10, Y = -5, Z = 42 },
            100,
            100
        );
        Assert.Equal(42.0, anchor.Z, 6);
    }

    [Fact]
    public void CleanHelpersAcceptStructuredInput()
    {
        var options = new HelperOptions
        {
            Name = "keyframe.scaleKeyframes",
            Input = JObject.Parse(
                """{"channel":{"keys":[{"time":0,"value":[1,2]},{"time":100,"value":[3,4]}]},"factor":2}"""
            ),
        };
        JObject result = JObject.FromObject(Entry.HelpersClient.Call(options));
        Assert.Equal(6.0, result["keys"]![1]!["value"]![0]!.Value<double>(), 6);
        Assert.Equal(8.0, result["keys"]![1]!["value"]![1]!.Value<double>(), 6);
    }

    [Fact]
    public void ComputesOriginVectorsAndScreenBounds()
    {
        Vector2 origin = Entry.Storyboarding.GetOriginVector(OsbOrigin.Centre, 100, 100);
        Assert.Equal(50.0, origin.X, 6);
        Assert.True(
            Entry.Storyboarding.InScreenBounds(
                new Vector2 { X = 427, Y = 240 },
                new Vector2 { X = 100, Y = 100 },
                0,
                new Vector2 { X = 50, Y = 50 }
            )
        );
    }

    private static string GoldenCorpusPath()
    {
        string? directory = AppContext.BaseDirectory;
        for (int depth = 0; depth < 8 && directory is not null; depth++)
        {
            string candidate = Path.Combine(
                directory,
                "crates",
                "osb-runtime",
                "tests",
                "golden",
                "storyboarding3d",
                "basic.json"
            );
            if (File.Exists(candidate))
            {
                return candidate;
            }
            directory = Path.GetDirectoryName(directory);
        }
        throw new FileNotFoundException("golden corpus not found");
    }
}
