"""The 2D scene graph (M40)."""

from __future__ import annotations

from typing import Any, Dict, Optional

from osbtk.libraries.helpers._base import call


class _Scene2d:
    """The 2D scene graph: the 2D counterpart of the 3D scene tree."""

    @staticmethod
    def resolve_scene2d(scene: Any, options: Optional[Dict[str, Any]] = None) -> Any:
        """Resolves the scene into per-sprite command sets."""
        return call("scene2d.resolve", {"scene": scene, "options": options or {}})

    @staticmethod
    def bake_scene2d(scene: Any, options: Optional[Dict[str, Any]] = None) -> Any:
        """Bakes the scene into a full storyboard model (per-node layers)."""
        return call("scene2d.bake", {"scene": scene, "options": options or {}})

    @staticmethod
    def scene2d_time_range(scene: Any) -> Any:
        """The scene's occupied [start, end] range, or None."""
        return call("scene2d.timeRange", {"scene": scene})

    @staticmethod
    def translate_scene2d(scene: Any, delta: Any) -> Any:
        """Translates the whole scene (children inherit)."""
        return call("scene2d.translate", {"scene": scene, "delta": delta})

    @staticmethod
    def scale_scene2d(scene: Any, factor: float) -> Any:
        """Scales the whole scene about its origin."""
        return call("scene2d.scale", {"scene": scene, "factor": factor})

    @staticmethod
    def rotate_scene2d(scene: Any, radians: float) -> Any:
        """Rotates the whole scene (radians)."""
        return call("scene2d.rotate", {"scene": scene, "radians": radians})

    @staticmethod
    def flatten_scene2d(scene: Any, axis: Optional[str] = None, mode: str = "mean") -> Any:
        """Flattens the whole scene."""
        return call("scene2d.flatten", {"scene": scene, "axis": axis, "mode": mode})

    @staticmethod
    def shift_scene2d(scene: Any, delta: float) -> Any:
        """Shifts the whole scene in time."""
        return call("scene2d.shift", {"scene": scene, "delta": delta})

    @staticmethod
    def reverse_scene2d(scene: Any, start: float = 0.0, end: float = 0.0) -> Any:
        """Reverses the whole scene in time."""
        return call("scene2d.reverse", {"scene": scene, "start": start, "end": end})
