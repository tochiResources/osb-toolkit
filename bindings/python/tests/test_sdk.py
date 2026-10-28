"""The Python SDK: generated OpenAPI models and clients driven through the native
bridge transport, with the shared golden corpus for parity."""

from __future__ import annotations

import unittest

from osbtk import Generator, Runtime, osb
from osbtk.client import Layer as CleanLayer, Sprite as CleanSprite
from osbtk.client.options import CommandOptions, CreateSpriteOptions, SpriteTimeOptions
from osbtk.exceptions import ApiException
from osbtk.models import (
    CommandRequest,
    CreateSpriteRequest,
    CurveKind,
    CurveSpec,
    OptimizeOptionsInput,
    RgbaColor,
    Shape,
    ShapeKind,
    SpriteTimeRequest,
    Vector2,
)

OSU = (
    "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
    "[TimingPoints]\n0,500,4,2,1,100,1,0\n"
    "[HitObjects]\n256,192,1000,1,0\n"
)


class SdkTests(unittest.TestCase):
    def test_exposes_the_native_version(self) -> None:
        self.assertTrue(osb.version())

    def test_parses_a_beatmap_into_the_typed_model(self) -> None:
        beatmap = osb.storyboarding.parse_osu(OSU)
        self.assertEqual(beatmap.audio_filename, "song.mp3")
        self.assertEqual(beatmap.hit_objects[0].kind.value, "Circle")

    def test_storyboarding3d_reuses_the_storyboarding_surface(self) -> None:
        beatmap = osb.storyboarding3d.parse_osu(OSU)
        self.assertEqual(beatmap.audio_filename, "song.mp3")

    def test_computes_origin_vectors_and_screen_bounds(self) -> None:
        origin = osb.storyboarding.get_origin_vector("Centre", 100.0, 100.0)
        self.assertAlmostEqual(origin.x, 50.0, places=6)
        self.assertTrue(
            osb.storyboarding.in_screen_bounds(
                {"x": 427.0, "y": 240.0},
                {"x": 100.0, "y": 100.0},
                0.0,
                {"x": 50.0, "y": 50.0},
            )
        )

    def test_applies_easing(self) -> None:
        self.assertAlmostEqual(osb.animations.apply_easing("InQuad", 0.5), 0.25, places=6)

    def test_drives_the_storyboard_through_the_generated_client(self) -> None:
        runtime = Runtime()
        try:
            handle = runtime.layer.create_sprite(
                CreateSpriteRequest(layer="Foreground", path="sb/dot.png", origin="Centre")
            )
            runtime.sprite.command(
                CommandRequest(
                    kind="move",
                    layer=handle.layer,
                    sprite=handle.sprite,
                    start_time=0,
                    end_time=1000,
                    easing="None",
                    start={"x": 0, "y": 0},
                    end={"x": 100, "y": 0},
                )
            )
            state = runtime.sprite.state_at(
                SpriteTimeRequest(layer=handle.layer, sprite=handle.sprite, time=500)
            )
            self.assertAlmostEqual(state.position.x, 50.0, places=4)
        finally:
            runtime.close()

    def test_drives_the_handle_bound_clean_client(self) -> None:
        runtime = Runtime()
        try:
            handle = runtime.generator.layer_handle("Foreground")
            layer = CleanLayer(runtime.layer, runtime.sprite, handle)
            self.assertTrue(CleanLayer.instance_of(layer))

            sprite = layer.create_sprite(
                CreateSpriteOptions(path="sb/dot.png", origin="Centre")
            )
            self.assertTrue(CleanSprite.instance_of(sprite))

            sprite.command(
                CommandOptions(
                    kind="move",
                    startTime=0,
                    endTime=1000,
                    easing="None",
                    start={"x": 0, "y": 0},
                    end={"x": 100, "y": 0},
                )
            )
            state = sprite.state_at(SpriteTimeOptions(time=500))
            self.assertAlmostEqual(state.position.x, 50.0, places=4)
        finally:
            runtime.close()

    def test_surfaces_native_errors(self) -> None:
        with self.assertRaises(ApiException):
            osb.audio.fft("does-not-exist.wav")

    def test_clean_helpers_accept_structured_input(self) -> None:
        from osbtk.api.helpers_api import HelpersAPI
        from osbtk.client import Helpers
        from osbtk.client.options import HelperOptions
        from osbtk.libraries._client import STATELESS

        scaled = Helpers(HelpersAPI(STATELESS)).call(
            HelperOptions(
                name="keyframe.scaleKeyframes",
                input={
                    "channel": {
                        "keys": [
                            {"time": 0.0, "value": [1.0, 2.0]},
                            {"time": 100.0, "value": [3.0, 4.0]},
                        ]
                    },
                    "factor": 2.0,
                },
            )
        )
        self.assertAlmostEqual(scaled["keys"][1]["value"][0], 6.0, 6)
        self.assertAlmostEqual(scaled["keys"][1]["value"][1], 8.0, 6)

    def test_drives_the_curve_library(self) -> None:
        curve = CurveSpec(
            kind=CurveKind("Linear"),
            points=[Vector2(x=0.0, y=0.0), Vector2(x=100.0, y=0.0)],
        )
        self.assertAlmostEqual(osb.curve.length(curve), 100.0, places=6)
        self.assertAlmostEqual(osb.curve.point(curve, 0.5).x, 50.0, places=6)
        self.assertEqual(len(osb.curve.sample(3, curve).points), 3)
        self.assertAlmostEqual(osb.curve.bounding_box(curve).max.x, 100.0, places=6)
        simplified = osb.curve.simplify(
            [
                Vector2(x=0.0, y=0.0),
                Vector2(x=1.0, y=0.05),
                Vector2(x=2.0, y=0.0),
                Vector2(x=3.0, y=0.0),
            ],
            0.1,
        )
        self.assertEqual(len(simplified.points), 2)
        self.assertAlmostEqual(osb.curve.normal(curve, 0.5).x, 0.0, places=6)
        self.assertAlmostEqual(osb.curve.curvature(curve, 0.5), 0.0, places=6)
        self.assertAlmostEqual(
            osb.curve.distance_to(curve, Vector2(x=50.0, y=10.0)), 10.0, places=6
        )
        self.assertEqual(len(osb.curve.offset(curve, 5.0).points), 2)
        self.assertEqual(len(osb.curve.arc(Vector2(x=0.0, y=0.0), 10.0, 0.0, 180.0, 3).points), 3)
        self.assertEqual(len(osb.curve.ellipse(Vector2(x=0.0, y=0.0), 4.0, 2.0, 4).points), 4)
        hit = osb.curve.line_intersect(
            Vector2(x=0.0, y=0.0),
            Vector2(x=10.0, y=10.0),
            Vector2(x=0.0, y=10.0),
            Vector2(x=10.0, y=0.0),
        )
        self.assertIsNotNone(hit)
        self.assertAlmostEqual(hit.x, 5.0, places=6)

    def test_diagnoses_storyboards_and_mapsets(self) -> None:
        report = osb.diagnostics.diagnose_osu(
            "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
        )
        self.assertGreaterEqual(report.error_count, 1)
        osb_report = osb.diagnostics.diagnose_osb(
            '[Events]\nSprite,Foreground,Centre,"sb/dot.png",320,240\n F,0,0,100,0,1\n'
        )
        self.assertEqual(osb_report.diagnostics, [])

    def test_optimizes_storyboards(self) -> None:
        result = osb.optimize.optimize_osb(
            OptimizeOptionsInput(deduplicate=True),
            '[Events]\nSprite,Foreground,Centre,"sb/dot.png",320,240\n'
            " F,0,0,100,0,1\n F,0,0,100,0,1\n",
        )
        self.assertEqual(result.stats.deduplicated, 1)
        self.assertIn("Sprite", result.osb)

    def test_surfaces_image_errors(self) -> None:
        with self.assertRaises(ApiException):
            osb.image.info("does-not-exist.png")

    def test_surfaces_media_errors(self) -> None:
        with self.assertRaises(ApiException):
            osb.media.info("does-not-exist.mp4")

    def test_runs_a_generator_backed_effect(self) -> None:
        class Pulse(Generator):
            def generate(self) -> None:
                self.create_sprite("Foreground", "sb/dot.png", "Centre")

        effect = Pulse()
        try:
            effect.generate()
            self.assertEqual(effect.sprite_count(), 1)
        finally:
            effect.close()

    def test_authors_a_sprite_through_the_facade(self) -> None:
        captured = []

        class Effect(Generator):
            def generate(self) -> None:
                layer = self.get_layer("Foreground")
                sprite = layer.create_sprite("sb/dot.png", "Centre")
                sprite.move("None", 0, 1000, {"x": 0.0, "y": 0.0}, {"x": 100.0, "y": 0.0})
                sprite.start_loop_group(1000, 2)
                sprite.fade(0, 100, 0.0, 1.0)
                sprite.fade(150, 1.0)
                sprite.end_group()
                captured.append((sprite.position_at(500)["x"], self.sprite_count()))

        effect = Effect()
        try:
            effect.generate()
        finally:
            effect.close()
        self.assertAlmostEqual(captured[0][0], 50.0, places=4)
        self.assertEqual(captured[0][1], 1)

    def test_animates_and_samples_a_sprite_anchor(self) -> None:
        captured = []

        class Effect(Generator):
            def generate(self) -> None:
                layer = self.get_layer("Foreground")
                sprite = layer.create_sprite("sb/dot.png", "Centre")
                sprite.anchor("None", 0, 1000, {"x": 0.0, "y": 0.0}, {"x": 100.0, "y": 50.0})
                captured.append(sprite.anchor_at(500))

        effect = Effect()
        try:
            effect.generate()
        finally:
            effect.close()
        self.assertAlmostEqual(captured[0]["x"], 50.0, places=4)
        self.assertAlmostEqual(captured[0]["y"], 25.0, places=4)

    def test_reads_back_the_built_storyboard(self) -> None:
        captured = []

        class Effect(Generator):
            def generate(self) -> None:
                layer = self.get_layer("Foreground")
                sprite = layer.create_sprite("sb/dot.png", "Centre")
                sprite.fade(0, 1000, 0.0, 1.0)
                storyboard = self.storyboard()
                foreground = next(
                    entry for entry in storyboard.layers if entry.identifier == "Foreground"
                )
                captured.append(
                    (
                        foreground.sprites[0].texture_path,
                        foreground.sprites[0].commands[0].kind,
                    )
                )

        effect = Effect()
        try:
            effect.generate()
        finally:
            effect.close()
        self.assertEqual(captured[0][0], "sb/dot.png")
        self.assertEqual(captured[0][1], "fade")

    def test_nests_segments_through_the_facade(self) -> None:
        captured = []

        class Effect(Generator):
            def generate(self) -> None:
                group = self.get_layer("Group")
                group.create_sprite("sb/group.png", "Centre")
                child = group.create_segment()
                child.create_sprite("sb/child.png", "Centre")
                child.set_reverse_depth(True)
                child.add_index(3)
                child.add_index(-1)
                indexes = child.indexes()
                captured.append((indexes["highest"], indexes["lowest"], self.sprite_count()))
                child.discard()
                captured.append(self.sprite_count())

        effect = Effect()
        try:
            effect.generate()
        finally:
            effect.close()
        self.assertEqual(captured[0], (3.0, -1.0, 2))
        self.assertEqual(captured[1], 1)

    def test_runs_the_motion_graphics_helpers(self) -> None:
        channel = {
            "keys": [
                {"time": 0.0, "value": [1.0, 2.0]},
                {"time": 100.0, "value": [3.0, 4.0]},
            ]
        }
        scaled = osb.storyboarding3d.Helpers.scale_keyframes(channel, 2.0)
        self.assertAlmostEqual(scaled["keys"][1]["value"][0], 6.0, 6)
        self.assertAlmostEqual(scaled["keys"][1]["value"][1], 8.0, 6)

        orbit = osb.storyboarding3d.Helpers.orbit(radius_x=10.0, radius_y=10.0, count=5)
        self.assertEqual(len(orbit["keys"]), 5)

        points = osb.storyboarding3d.Helpers.scatter(seed=1, count=3)
        self.assertEqual(len(points), 3)

        self.assertAlmostEqual(osb.storyboarding3d.Helpers.stagger(3, 100.0), 300.0, 6)

    def test_runs_the_command_helpers(self) -> None:
        commands = [
            {
                "kind": "fade",
                "startTime": 0.0,
                "endTime": 100.0,
                "easing": "None",
                "start": {"kind": "scalar", "value": 0.0},
                "end": {"kind": "scalar", "value": 1.0},
            },
            {
                "kind": "fade",
                "startTime": 200.0,
                "endTime": 300.0,
                "easing": "None",
                "start": {"kind": "scalar", "value": 1.0},
                "end": {"kind": "scalar", "value": 0.0},
            },
        ]
        shifted = osb.storyboarding.Commands.shift_commands(commands, 50.0)
        self.assertEqual(shifted[0]["startTime"], 50.0)

        selected = osb.storyboarding.Commands.select_commands(commands, start=150.0)
        self.assertEqual(len(selected), 1)
        self.assertEqual(selected[0]["startTime"], 200.0)

        merged = osb.storyboarding.Commands.merge_commands(commands)
        self.assertEqual(len(merged), 2)

        duplicated = osb.storyboarding.Commands.duplicate_commands(commands, offset=1000.0, count=2)
        self.assertEqual(len(duplicated), 4)

    def test_shares_the_motion_graphics_helpers(self) -> None:
        scaled = osb.helpers.call(
            "keyframe.scaleKeyframes",
            {
                "channel": {
                    "keys": [
                        {"time": 0.0, "value": [1.0, 2.0]},
                        {"time": 100.0, "value": [3.0, 4.0]},
                    ]
                },
                "factor": 2.0,
            },
        )
        self.assertAlmostEqual(scaled["keys"][1]["value"][0], 6.0, 6)
        self.assertAlmostEqual(scaled["keys"][1]["value"][1], 8.0, 6)

        resolved = osb.helpers.resolve_anchor("Centre", {"x": 10.0, "y": -5.0}, 100.0, 100.0)
        self.assertAlmostEqual(resolved["x"], 60.0, 6)
        self.assertAlmostEqual(resolved["y"], 45.0, 6)

        commands = [
            {
                "kind": "fade",
                "startTime": 0.0,
                "endTime": 100.0,
                "easing": "None",
                "start": {"kind": "scalar", "value": 0.0},
                "end": {"kind": "scalar", "value": 1.0},
            },
            {
                "kind": "fade",
                "startTime": 100.0,
                "endTime": 200.0,
                "easing": "None",
                "start": {"kind": "scalar", "value": 1.0},
                "end": {"kind": "scalar", "value": 0.0},
            },
        ]
        flattened = osb.helpers.transform_commands(
            commands, "fade", "keyframe.flatten", {"mode": "mean"}
        )
        self.assertEqual(len(flattened), 1)
        self.assertEqual(flattened[0]["startTime"], 0.0)
        self.assertEqual(flattened[0]["endTime"], 200.0)

    def test_manipulates_easings_timelines_and_depth_anchors(self) -> None:
        # InQuad at 0.5 is 0.25; amplifying about linear gives 0.5 + (0.25-0.5)*2.
        self.assertAlmostEqual(osb.helpers.amplify_easing("InQuad", 0.5, 2.0), 0.0, 6)

        timeline = {
            "clips": [
                {"start": 0.0, "channel": {"keys": [{"time": 0.0, "value": [0.0]}, {"time": 100.0, "value": [1.0]}]}},
                {"start": 200.0, "channel": {"keys": [{"time": 0.0, "value": [0.0]}, {"time": 100.0, "value": [1.0]}]}},
            ]
        }
        self.assertAlmostEqual(osb.helpers.timeline_duration(timeline), 300.0, 6)
        flushed = osb.helpers.flush_timeline(timeline)
        self.assertEqual(flushed["clips"][1]["channel"]["keys"][0]["time"], 200.0)
        self.assertEqual(len(osb.helpers.merge_timelines(timeline, timeline)["clips"]), 4)

        anchor = osb.helpers.resolve_anchor_3d(
            "Centre", {"x": 10.0, "y": -5.0, "z": 42.0}, 100.0, 100.0
        )
        self.assertAlmostEqual(anchor["z"], 42.0, 6)

    def test_renders_shapes(self) -> None:
        info = osb.image_gen.render_shapes(
            32,
            32,
            [
                Shape(
                    kind=ShapeKind("Rect"),
                    center=Vector2(x=16.0, y=16.0),
                    size=Vector2(x=20.0, y=20.0),
                    fill=RgbaColor(r=10, g=20, b=30, a=255),
                )
            ],
            "osbtk-python-render.png",
        )
        self.assertEqual(info.width, 32)


if __name__ == "__main__":
    unittest.main()
