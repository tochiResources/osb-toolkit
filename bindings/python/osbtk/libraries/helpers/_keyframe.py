"""The shared keyframe channel transforms (2D, 3D and color)."""

from __future__ import annotations

from typing import Any, Dict, Optional

from osbtk.libraries.helpers._base import call


class _Keyframe:
    """The shared keyframe channel transforms."""

    @staticmethod
    def flatten(channel: Any, axis: Optional[str] = None, mode: str = "mean") -> Any:
        return call("keyframe.flatten", {"channel": channel, "axis": axis, "mode": mode})

    @staticmethod
    def scale_keyframes(channel: Any, factor: float) -> Any:
        return call("keyframe.scaleKeyframes", {"channel": channel, "factor": factor})

    @staticmethod
    def offset_keyframes(channel: Any, delta: float) -> Any:
        return call("keyframe.offsetKeyframes", {"channel": channel, "delta": delta})

    @staticmethod
    def mirror(channel: Any, axis: Optional[str] = None, pivot: float = 0.0) -> Any:
        return call("keyframe.mirror", {"channel": channel, "axis": axis, "pivot": pivot})

    @staticmethod
    def randomize(channel: Any, options: Optional[Dict[str, Any]] = None) -> Any:
        payload: Dict[str, Any] = {"channel": channel}
        payload.update(options or {})
        return call("keyframe.randomize", payload)

    @staticmethod
    def ease_keyframes(channel: Any, easing: str, samples: int = 8) -> Any:
        return call(
            "keyframe.easeKeyframes",
            {"channel": channel, "easing": easing, "samples": samples},
        )
