package moe.tochi.osbtk;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.List;
import java.util.Map;
import com.google.gson.reflect.TypeToken;
import moe.tochi.osbtk.model.Beatmap;
import moe.tochi.osbtk.model.CommandRequest;
import moe.tochi.osbtk.model.CreateSpriteRequest;
import moe.tochi.osbtk.model.CurveKind;
import moe.tochi.osbtk.model.CurveSpec;
import moe.tochi.osbtk.model.DiagnosticReport;
import moe.tochi.osbtk.model.Easing;
import moe.tochi.osbtk.model.Handle;
import moe.tochi.osbtk.model.HitObjectKind;
import moe.tochi.osbtk.model.ImageInfo;
import moe.tochi.osbtk.model.OsbOrigin;
import moe.tochi.osbtk.model.OptimizeOptionsInput;
import moe.tochi.osbtk.model.OptimizeResult;
import moe.tochi.osbtk.model.OptionalPointResult;
import moe.tochi.osbtk.model.PointListResult;
import moe.tochi.osbtk.model.RenderShapesRequest;
import moe.tochi.osbtk.model.RgbaColor;
import moe.tochi.osbtk.model.Shape;
import moe.tochi.osbtk.model.ShapeKind;
import moe.tochi.osbtk.model.SpriteState;
import moe.tochi.osbtk.model.SpriteTimeRequest;
import moe.tochi.osbtk.model.StoryboardCommand;
import moe.tochi.osbtk.model.StoryboardCommandKind;
import moe.tochi.osbtk.model.Vector2;
import org.junit.jupiter.api.Test;

/** The Java SDK: generated OpenAPI models and clients driven through the native bridge. */
class OsbTkSdkTest {
    private static final String OSU =
            "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
                    + "[TimingPoints]\n0,500,4,2,1,100,1,0\n"
                    + "[HitObjects]\n256,192,1000,1,0\n";

    @Test
    void exposesTheNativeVersion() {
        assertFalse(osb.version().isEmpty());
    }

    @Test
    void parsesABeatmapIntoTheTypedModel() throws Exception {
        Beatmap beatmap = osb.Storyboarding.parseOsu(OSU);
        assertEquals("song.mp3", beatmap.getAudioFilename());
        assertEquals(HitObjectKind.CIRCLE, beatmap.getHitObjects().get(0).getKind());
    }

    @Test
    void storyboarding3dReusesTheStoryboardingSurface() throws Exception {
        Beatmap beatmap = osb.Storyboarding3d.parseOsu(OSU);
        assertEquals("song.mp3", beatmap.getAudioFilename());
    }

    @Test
    void appliesEasing() throws Exception {
        assertEquals(0.25, osb.Animations.applyEasing(Easing.IN_QUAD, 0.5), 1e-6);
    }

    @Test
    void drivesTheStoryboardThroughTheGeneratedClient() throws Exception {
        try (Runtime runtime = new Runtime()) {
            Handle handle =
                    runtime.layer()
                            .createSprite(
                                    new CreateSpriteRequest()
                                            .layer("Foreground")
                                            .path("sb/dot.png")
                                            .origin(OsbOrigin.CENTRE));
            runtime.sprite()
                    .command(
                            new CommandRequest()
                                    .kind("move")
                                    .layer(handle.getLayer())
                                    .sprite(handle.getSprite())
                                    .startTime(0.0)
                                    .endTime(1000.0)
                                    .easing(Easing.NONE)
                                    .start(Map.of("x", 0.0, "y", 0.0))
                                    .end(Map.of("x", 100.0, "y", 0.0)));
            SpriteState state =
                    runtime.sprite()
                            .stateAt(
                                    new SpriteTimeRequest()
                                            .layer(handle.getLayer())
                                            .sprite(handle.getSprite())
                                            .time(500.0));
            assertEquals(50.0, state.getPosition().getX(), 0.1);
        }
    }

    @Test
    void drivesTheCurveLibrary() throws Exception {
        CurveSpec curve =
                new CurveSpec()
                        .kind(CurveKind.LINEAR)
                        .points(
                                List.of(
                                        new Vector2().x(0.0).y(0.0),
                                        new Vector2().x(100.0).y(0.0)));
        assertEquals(100.0, osb.Curve.length(curve), 1e-6);
        assertEquals(50.0, osb.Curve.point(curve, 0.5, false).getX(), 1e-6);
        assertEquals(3, osb.Curve.sample(curve, 3, false).getPoints().size());
        assertEquals(100.0, osb.Curve.boundingBox(curve).getMax().getX(), 1e-6);

        PointListResult simplified =
                osb.Curve.simplify(
                        List.of(
                                new Vector2().x(0.0).y(0.0),
                                new Vector2().x(1.0).y(0.05),
                                new Vector2().x(2.0).y(0.0),
                                new Vector2().x(3.0).y(0.0)),
                        0.1);
        assertEquals(2, simplified.getPoints().size());
        assertEquals(0.0, osb.Curve.normal(curve, 0.5).getX(), 1e-6);
        assertEquals(0.0, osb.Curve.curvature(curve, 0.5), 1e-6);
        assertEquals(10.0, osb.Curve.distanceTo(curve, new Vector2().x(50.0).y(10.0)), 1e-6);
        assertEquals(2, osb.Curve.offset(curve, 5.0).getPoints().size());
        assertEquals(3, osb.Curve.arc(new Vector2().x(0.0).y(0.0), 10.0, 0.0, 180.0, 3).getPoints().size());
        assertEquals(4, osb.Curve.ellipse(new Vector2().x(0.0).y(0.0), 4.0, 2.0, 4).getPoints().size());
        OptionalPointResult hit =
                osb.Curve.lineIntersect(
                        new Vector2().x(0.0).y(0.0),
                        new Vector2().x(10.0).y(10.0),
                        new Vector2().x(0.0).y(10.0),
                        new Vector2().x(10.0).y(0.0));
        assertEquals(5.0, hit.getPoint().getX(), 1e-6);
    }

    @Test
    void diagnosesArtifacts() throws Exception {
        DiagnosticReport report =
                osb.Diagnostics.diagnoseOsu(
                        "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n");
        assertTrue(report.getErrorCount() >= 1);
    }

    @Test
    void optimizesStoryboards() throws Exception {
        OptimizeResult result =
                osb.Optimize.optimizeOsb(
                        "[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n"
                                + " F,0,0,100,0,1\n F,0,0,100,0,1\n",
                        new OptimizeOptionsInput().deduplicate(true));
        assertEquals(1, result.getStats().getDeduplicated());
    }

    @Test
    void surfacesImageErrors() {
        assertThrows(ApiException.class, () -> osb.Image.info("does-not-exist.png"));
    }

    @Test
    void surfacesMediaErrors() {
        assertThrows(ApiException.class, () -> osb.Media.info("does-not-exist.mp4"));
    }

    @Test
    void runsAnEffectScript() throws Exception {
        Pulse effect = new Pulse();
        try {
            effect.generate();
            assertEquals(1, effect.spriteCount());
        } finally {
            effect.close();
        }
    }

    @Test
    void rendersShapes() throws Exception {
        ImageInfo info =
                osb.ImageGen.renderShapes(
                        new RenderShapesRequest()
                                .width(32)
                                .height(32)
                                .shapes(
                                        List.of(
                                                new Shape()
                                                        .kind(ShapeKind.RECT)
                                                        .center(new Vector2().x(16.0).y(16.0))
                                                        .size(new Vector2().x(20.0).y(20.0))
                                                        .fill(
                                                                new RgbaColor()
                                                                        .r(10)
                                                                        .g(20)
                                                                        .b(30)
                                                                        .a(255))))
                                .output("osbtk-java-render.png"));
        assertEquals(32, info.getWidth());
    }

    @Test
    void computesOriginVectorsAndScreenBounds() throws Exception {
        Vector2 origin = osb.Storyboarding.getOriginVector(OsbOrigin.CENTRE, 100.0, 100.0);
        assertEquals(50.0, origin.getX(), 1e-6);
        assertTrue(
                osb.Storyboarding.inScreenBounds(
                        new Vector2().x(427.0).y(240.0),
                        new Vector2().x(100.0).y(100.0),
                        0.0,
                        new Vector2().x(50.0).y(50.0)));
    }

    @Test
    void authorsASpriteThroughTheFacade() throws Exception {
        final double[] positionX = new double[1];

        class Effect extends osb.Generator {
            @Override
            public void generate() throws ApiException {
                osb.Layer layer = getLayer("Foreground");
                osb.Sprite sprite = layer.createSprite("sb/dot.png", OsbOrigin.CENTRE);
                sprite.move(
                        0,
                        1000,
                        new Vector2().x(0.0).y(0.0),
                        new Vector2().x(100.0).y(0.0),
                        Easing.NONE);
                sprite.startLoopGroup(1000, 2);
                sprite.fade(0, 100, 0.0, 1.0, Easing.NONE);
                sprite.endGroup();
                positionX[0] = sprite.positionAt(500).getX();
            }
        }

        Effect effect = new Effect();
        try {
            effect.generate();
            assertEquals(50.0, positionX[0], 0.1);
            assertEquals(1, effect.spriteCount());
        } finally {
            effect.close();
        }
    }

    @Test
    void readsBackTheBuiltStoryboard() throws Exception {
        final String[] captured = new String[2];

        class Effect extends osb.Generator {
            @Override
            public void generate() throws ApiException {
                osb.Layer layer = getLayer("Foreground");
                osb.Sprite sprite = layer.createSprite("sb/dot.png", OsbOrigin.CENTRE);
                sprite.fade(0, 1000, 0.0, 1.0, Easing.NONE);
                moe.tochi.osbtk.model.Storyboard storyboard = storyboard();
                moe.tochi.osbtk.model.StoryboardLayer foreground =
                        storyboard.getLayers().stream()
                                .filter(entry -> "Foreground".equals(entry.getIdentifier()))
                                .findFirst()
                                .orElseThrow();
                captured[0] = foreground.getSprites().get(0).getTexturePath();
                captured[1] = foreground.getSprites().get(0).getCommands().get(0).getKind().getValue();
            }
        }

        Effect effect = new Effect();
        try {
            effect.generate();
            assertEquals("sb/dot.png", captured[0]);
            assertEquals("fade", captured[1]);
        } finally {
            effect.close();
        }
    }

    @Test
    void nestsSegmentsThroughTheFacade() throws Exception {
        final double[] result = new double[3];

        class Effect extends osb.Generator {
            @Override
            public void generate() throws ApiException {
                osb.Layer group = getLayer("Group");
                group.createSprite("sb/group.png", OsbOrigin.CENTRE);
                osb.Layer child = group.createSegment();
                child.createSprite("sb/child.png", OsbOrigin.CENTRE);
                child.setReverseDepth(true).addIndex(3).addIndex(-1);
                var indexes = child.indexes();
                result[0] = indexes.getHighest();
                result[1] = indexes.getLowest();
                result[2] = spriteCount();
                child.discard();
            }
        }

        Effect effect = new Effect();
        try {
            effect.generate();
            assertEquals(3.0, result[0], 1e-9);
            assertEquals(-1.0, result[1], 1e-9);
            assertEquals(2.0, result[2], 1e-9);
            assertEquals(1, effect.spriteCount());
        } finally {
            effect.close();
        }
    }

    @Test
    void runsTheMotionGraphicsHelpers() throws Exception {
        var channel =
                new osb.Storyboarding3d.Helpers.Channel(
                        List.of(
                                new osb.Storyboarding3d.Helpers.ChannelKey(
                                        0, List.of(1.0, 2.0)),
                                new osb.Storyboarding3d.Helpers.ChannelKey(
                                        100, List.of(3.0, 4.0))));
        var scaled = osb.Storyboarding3d.Helpers.scaleKeyframes(channel, 2);
        assertEquals(6.0, scaled.getKeys().get(1).getValue().get(0), 1e-9);
        assertEquals(8.0, scaled.getKeys().get(1).getValue().get(1), 1e-9);

        var orbit = osb.Storyboarding3d.Helpers.orbit(null, 10, 10, 0, 6.283185, 5);
        assertEquals(5, orbit.getKeys().size());

        var points = osb.Storyboarding3d.Helpers.scatter(1, 3, null, null, 0);
        assertEquals(3, points.size());

        assertEquals(300.0, osb.Storyboarding3d.Helpers.stagger(3, 100), 1e-9);
    }

    @Test
    void runsTheCommandHelpers() throws Exception {
        String raw =
                "[{\"kind\":\"fade\",\"startTime\":0,\"endTime\":100,\"easing\":\"None\","
                        + "\"start\":{\"kind\":\"scalar\",\"value\":0},\"end\":{\"kind\":\"scalar\",\"value\":1}},"
                        + "{\"kind\":\"fade\",\"startTime\":200,\"endTime\":300,\"easing\":\"None\","
                        + "\"start\":{\"kind\":\"scalar\",\"value\":1},\"end\":{\"kind\":\"scalar\",\"value\":0}}]";
        List<StoryboardCommand> commands =
                JSON.getGson()
                        .fromJson(
                                raw, new TypeToken<List<StoryboardCommand>>() {}.getType());

        var shifted = osb.Storyboarding.Commands.shiftCommands(commands, 50);
        assertEquals(50.0, shifted.get(0).getStartTime(), 1e-9);

        var selected =
                osb.Storyboarding.Commands.selectCommands(commands, null, 150.0, null);
        assertEquals(1, selected.size());
        assertEquals(200.0, selected.get(0).getStartTime(), 1e-9);

        var merged = osb.Storyboarding.Commands.mergeCommands(commands, 0);
        assertEquals(2, merged.size());

        var duplicated = osb.Storyboarding.Commands.duplicateCommands(commands, 1000, 2);
        assertEquals(4, duplicated.size());
    }

    @Test
    @SuppressWarnings("unchecked")
    void sharesTheMotionGraphicsHelpers() throws Exception {
        // A 2D channel (no Z): the Z axis is optional.
        var scaled =
                (Map<String, Object>)
                        osb.Helpers.call(
                                "keyframe.scaleKeyframes",
                                Map.of(
                                        "channel",
                                        Map.of(
                                                "keys",
                                                List.of(
                                                        Map.of("time", 0.0, "value", List.of(1.0, 2.0)),
                                                        Map.of("time", 100.0, "value", List.of(3.0, 4.0)))),
                                        "factor",
                                        2.0));
        var keys = (List<Map<String, Object>>) scaled.get("keys");
        var values = (List<Double>) keys.get(1).get("value");
        assertEquals(6.0, values.get(0), 1e-9);
        assertEquals(8.0, values.get(1), 1e-9);

        // The shared origin/anchor helper.
        var resolved =
                osb.Helpers.resolveAnchor(
                        OsbOrigin.CENTRE, new Vector2().x(10.0).y(-5.0), 100, 100);
        assertEquals(60.0, resolved.getX(), 1e-6);
        assertEquals(45.0, resolved.getY(), 1e-6);

        // The same keyframe helper applied to a 2D command channel.
        String raw =
                "[{\"kind\":\"fade\",\"startTime\":0,\"endTime\":100,\"easing\":\"None\","
                        + "\"start\":{\"kind\":\"scalar\",\"value\":0},\"end\":{\"kind\":\"scalar\",\"value\":1}},"
                        + "{\"kind\":\"fade\",\"startTime\":100,\"endTime\":200,\"easing\":\"None\","
                        + "\"start\":{\"kind\":\"scalar\",\"value\":1},\"end\":{\"kind\":\"scalar\",\"value\":0}}]";
        List<StoryboardCommand> commands =
                JSON.getGson().fromJson(raw, new TypeToken<List<StoryboardCommand>>() {}.getType());
        var flattened =
                osb.Helpers.transformCommands(
                        commands, StoryboardCommandKind.FADE, "keyframe.flatten", Map.of("mode", "mean"));
        assertEquals(1, flattened.size());
        assertEquals(0.0, flattened.get(0).getStartTime(), 1e-9);
        assertEquals(200.0, flattened.get(0).getEndTime(), 1e-9);
    }

    @Test
    void manipulatesEasingsTimelinesAndDepthAnchors() throws Exception {
        // InQuad at 0.5 is 0.25; amplifying about linear gives 0.5 + (0.25-0.5)*2.
        var amplified =
                osb.Helpers.call(
                        "easing.amplifyEasing",
                        Map.<String, Object>of("easing", "InQuad", "t", 0.5, "amount", 2.0));
        assertEquals(0.0, ((Number) amplified).doubleValue(), 1e-6);

        var clip =
                Map.<String, Object>of(
                        "channel",
                        Map.of(
                                "keys",
                                List.of(
                                        Map.of("time", 0.0, "value", List.of(0.0)),
                                        Map.of("time", 100.0, "value", List.of(1.0)))));
        var timeline =
                Map.<String, Object>of(
                        "clips", List.of(clipWithStart(clip, 0.0), clipWithStart(clip, 200.0)));
        var duration =
                osb.Helpers.call(
                        "timeline.durationOf", Map.<String, Object>of("timeline", timeline));
        assertEquals(300.0, ((Number) duration).doubleValue(), 1e-6);

        var anchor =
                osb.Helpers.call(
                        "origin.resolveAnchor3d",
                        Map.<String, Object>of(
                                "origin", "Centre",
                                "anchor", Map.of("x", 10.0, "y", -5.0, "z", 42.0),
                                "width", 100.0,
                                "height", 100.0));
        assertEquals(42.0, ((Number) ((Map<?, ?>) anchor).get("z")).doubleValue(), 1e-6);
    }

    private static Map<String, Object> clipWithStart(Map<String, Object> clip, double start) {
        return Map.of("start", start, "channel", clip.get("channel"));
    }

    private static final class Pulse extends osb.Generator {
        @Override
        public void generate() throws ApiException {
            createSprite("Foreground", "sb/dot.png", OsbOrigin.CENTRE);
        }
    }
}
