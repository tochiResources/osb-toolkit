"""The 3D storyboard library: reuses the storyboarding surface and adds the
3D -> 2D bake and the motion-graphics keyframe helpers."""

from __future__ import annotations

import math
from typing import Any, Dict, List, Optional

from osbtk.api.helpers_api import HelpersAPI as HelpersApi
from osbtk.api.storyboarding3d_api import Storyboarding3dAPI
from osbtk.client.runtime.storyboarding3d import Storyboarding3d as Storyboarding3dClient
from osbtk.libraries._client import STATELESS
from osbtk.libraries.storyboarding import Storyboarding
from osbtk.models.helper_request import HelperRequest

_client = Storyboarding3dClient(Storyboarding3dAPI(STATELESS))
_helpers = HelpersApi(STATELESS)


class Storyboarding3d(Storyboarding):
    """The 3D storyboard library: a sibling that reuses the storyboarding surface
    and adds the 3D -> 2D bake and the keyframe helpers."""

    @staticmethod
    def bake3d(scene: Any) -> Any:
        return _client.bake3d(scene)

    class Helpers:
        """The 3D motion-graphics keyframe helpers: pure keyframe-channel
        utilities, canonical in ``osb_runtime::storyboarding3d::helpers``.

        A channel is ``{"keys": [{"time": <ms>, "value": [..components]}]}``."""

        @staticmethod
        def call(name: str, input: Optional[Dict[str, Any]] = None) -> Any:
            """Calls any helper by name with a raw input object."""
            return _helpers.keyframes(
                HelperRequest(input=input if input is not None else {}, name=name)
            ).value

        @staticmethod
        def flatten(channel: Any, axis: Optional[str] = None, mode: str = "mean") -> Any:
            return Storyboarding3d.Helpers.call(
                "flatten", {"channel": channel, "axis": axis, "mode": mode}
            )

        @staticmethod
        def scale_keyframes(channel: Any, factor: float) -> Any:
            return Storyboarding3d.Helpers.call(
                "scaleKeyframes", {"channel": channel, "factor": factor}
            )

        @staticmethod
        def offset_keyframes(channel: Any, delta: float) -> Any:
            return Storyboarding3d.Helpers.call(
                "offsetKeyframes", {"channel": channel, "delta": delta}
            )

        @staticmethod
        def mirror(channel: Any, axis: Optional[str] = None, pivot: float = 0.0) -> Any:
            return Storyboarding3d.Helpers.call(
                "mirror", {"channel": channel, "axis": axis, "pivot": pivot}
            )

        @staticmethod
        def randomize(
            channel: Any,
            axis: Optional[str] = None,
            min: float = -1.0,
            max: float = 1.0,
            seed: int = 0,
            step: float = 0.0,
        ) -> Any:
            return Storyboarding3d.Helpers.call(
                "randomize",
                {"channel": channel, "axis": axis, "min": min, "max": max, "seed": seed, "step": step},
            )

        @staticmethod
        def reverse_keyframes(channel: Any, start: float, end: float) -> Any:
            return Storyboarding3d.Helpers.call(
                "reverseKeyframes", {"channel": channel, "start": start, "end": end}
            )

        @staticmethod
        def repeat_keyframes(
            channel: Any, count: int, start: float, end: float, ping_pong: bool = False
        ) -> Any:
            return Storyboarding3d.Helpers.call(
                "repeatKeyframes",
                {"channel": channel, "count": count, "start": start, "end": end, "pingPong": ping_pong},
            )

        @staticmethod
        def resample_keyframes(channel: Any, step: float, count: int = 0) -> Any:
            return Storyboarding3d.Helpers.call(
                "resampleKeyframes", {"channel": channel, "step": step, "count": count}
            )

        @staticmethod
        def smooth_keyframes(channel: Any, window: int = 3) -> Any:
            return Storyboarding3d.Helpers.call(
                "smoothKeyframes", {"channel": channel, "window": window}
            )

        @staticmethod
        def simplify_keyframes(channel: Any, tolerance: float = 0.1, decimals: int = 3) -> Any:
            return Storyboarding3d.Helpers.call(
                "simplifyKeyframes", {"channel": channel, "tolerance": tolerance, "decimals": decimals}
            )

        @staticmethod
        def align_keyframes(
            channel: Any, beat: float = 500.0, offset: float = 0.0, subdivision: int = 1
        ) -> Any:
            return Storyboarding3d.Helpers.call(
                "alignKeyframes",
                {"channel": channel, "beat": beat, "offset": offset, "subdivision": subdivision},
            )

        @staticmethod
        def ease_keyframes(channel: Any, easing: str = "None", samples: int = 8) -> Any:
            return Storyboarding3d.Helpers.call(
                "easeKeyframes", {"channel": channel, "easing": easing, "samples": samples}
            )

        @staticmethod
        def merge_keyframes(channel: Any, other: Any, amount: float = 0.5) -> Any:
            return Storyboarding3d.Helpers.call(
                "mergeKeyframes", {"channel": channel, "other": other, "amount": amount}
            )

        @staticmethod
        def stagger(index: int, step: float) -> float:
            return Storyboarding3d.Helpers.call("stagger", {"index": index, "step": step})

        @staticmethod
        def orbit(
            center: Optional[List[float]] = None,
            radius_x: float = 100.0,
            radius_y: float = 100.0,
            start_angle: float = 0.0,
            sweep: float = math.tau,
            count: int = 32,
        ) -> Any:
            return Storyboarding3d.Helpers.call(
                "orbit",
                {
                    "center": center,
                    "radiusX": radius_x,
                    "radiusY": radius_y,
                    "startAngle": start_angle,
                    "sweep": sweep,
                    "count": count,
                },
            )

        @staticmethod
        def pulse(
            base: float = 1.0,
            amplitude: float = 0.5,
            frequency: float = 1.0,
            phase: float = 0.0,
            step: float = 0.0,
            duration: float = 1000.0,
        ) -> Any:
            return Storyboarding3d.Helpers.call(
                "pulse",
                {
                    "base": base,
                    "amplitude": amplitude,
                    "frequency": frequency,
                    "phase": phase,
                    "step": step,
                    "duration": duration,
                },
            )

        @staticmethod
        def scatter(
            seed: int = 0,
            count: int = 8,
            min: Optional[List[float]] = None,
            max: Optional[List[float]] = None,
            spread: float = 0.0,
        ) -> Any:
            return Storyboarding3d.Helpers.call(
                "scatter",
                {"seed": seed, "count": count, "min": min, "max": max, "spread": spread},
            )

        @staticmethod
        def spin(
            axis: str = "z", turns: float = 1.0, duration: float = 1000.0, count: int = 16
        ) -> Any:
            return Storyboarding3d.Helpers.call(
                "spin", {"axis": axis, "turns": turns, "duration": duration, "count": count}
            )

        @staticmethod
        def tumble(
            turns: Optional[List[float]] = None, duration: float = 1000.0, count: int = 16
        ) -> Any:
            return Storyboarding3d.Helpers.call(
                "tumble", {"turns": turns, "duration": duration, "count": count}
            )

        @staticmethod
        def path_follow(points: List[List[float]], count: int = 16, by_arc_length: bool = False) -> Any:
            return Storyboarding3d.Helpers.call(
                "pathFollow", {"points": points, "count": count, "byArcLength": by_arc_length}
            )
