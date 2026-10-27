"""Golden-vector parity for the Python binding (run against the cdylib)."""

from __future__ import annotations

import json
import unittest
from pathlib import Path

import osbtk


def golden_corpus() -> dict:
    directory = Path(__file__).resolve().parent
    while True:
        candidate = (
            directory / "crates" / "osb-runtime" / "tests" / "golden" / "storyboarding3d" / "basic.json"
        )
        if candidate.is_file():
            return json.loads(candidate.read_text(encoding="utf-8"))
        if directory.parent == directory:
            raise FileNotFoundError("golden corpus not found")
        directory = directory.parent


class osbTKTests(unittest.TestCase):
    def test_version(self) -> None:
        self.assertTrue(osbtk.version())

    def test_bakes_the_shared_golden_corpus(self) -> None:
        cases = golden_corpus()["cases"]
        self.assertTrue(cases)
        for case in cases:
            baked = osbtk.bake3d(json.dumps(case["input"]))
            expect = case["expect"]
            if "spriteCount" in expect:
                self.assertEqual(expect["spriteCount"], len(baked["sprites"]))
            if "firstSpritePath" in expect:
                self.assertEqual(expect["firstSpritePath"], baked["sprites"][0]["spritePath"])

    def test_parses_a_beatmap_summary(self) -> None:
        text = (
            "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
            "[TimingPoints]\n0,500,4,2,1,100,1,0\n"
            "[HitObjects]\n256,192,1000,1,0\n"
        )
        beatmap = osbtk.parse_osu(text)
        self.assertEqual(beatmap["audioFilename"], "song.mp3")
        self.assertEqual(beatmap["hitObjects"][0]["kind"], "Circle")

    def test_parses_an_osb_summary(self) -> None:
        summary = osbtk.parse_osb('[Events]\nSprite,Foreground,Centre,"sb/dot.png",320,240\n')
        self.assertEqual(summary["spriteCount"], 1)

    def test_reports_an_error_for_invalid_input(self) -> None:
        self.assertIn("error", osbtk.exec("not json"))

    def test_drives_the_storyboard_runtime(self) -> None:
        from osbtk._native import Runtime

        runtime = Runtime()
        try:
            handle = runtime.create_sprite("Foreground", "sb/dot.png")
            runtime.command(handle, "move", 0, 1000, start={"x": 0, "y": 0}, end={"x": 100, "y": 0})
            state = runtime.sprite_state_at(handle, 500)
            self.assertEqual(state["position"]["x"], 50.0)
        finally:
            runtime.close()


if __name__ == "__main__":
    unittest.main()
