"""The 3D world and unified storyboard bake (M40)."""

from __future__ import annotations

from typing import Any, Dict, Optional

from osbtk.libraries.helpers._base import call


class _Scene3d:
    """The 3D world and unified storyboard bake."""

    @staticmethod
    def bake_world(world: Any, options2d: Optional[Dict[str, Any]] = None) -> Any:
        """Bakes a 3D world (`{ nodes, camera, options }`) into a storyboard model."""
        return call("scene3d.bakeWorld", {"world": world, "options2d": options2d or {}})

    @staticmethod
    def bake_scene(input: Dict[str, Any]) -> Any:
        """Bakes a set of 2D and 3D worlds (`{ worlds2d, worlds3d, ... }`) into one storyboard."""
        return call("scene3d.bakeScene", input)
