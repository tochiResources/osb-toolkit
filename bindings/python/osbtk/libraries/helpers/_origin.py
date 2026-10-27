"""The shared origin/anchor helpers."""

from __future__ import annotations

from typing import Any

from osbtk.libraries.helpers._base import call


class _Origin:
    """The shared origin/anchor helpers."""

    @staticmethod
    def origin_vector(origin: str, width: float, height: float) -> Any:
        """The anchor offset for an enum origin, in texture pixels."""
        return call(
            "origin.getOriginVector", {"origin": origin, "width": width, "height": height}
        )

    @staticmethod
    def resolve_anchor(origin: str, anchor: Any, width: float, height: float) -> Any:
        """The effective pivot: the enum origin vector plus an explicit anchor offset."""
        return call(
            "origin.resolveAnchor",
            {"origin": origin, "anchor": anchor, "width": width, "height": height},
        )

    @staticmethod
    def in_screen_bounds(position: Any, size: Any, rotation: float, origin: Any) -> bool:
        """Whether a rotated sprite's bounds intersect the screen."""
        return call(
            "origin.inScreenBounds",
            {"position": position, "size": size, "rotation": rotation, "origin": origin},
        )

    @staticmethod
    def resolve_anchor_3d(origin: str, anchor: Any, width: float, height: float) -> Any:
        """The 3D effective pivot: a Vector3 anchor whose z anchors depth."""
        return call(
            "origin.resolveAnchor3d",
            {"origin": origin, "anchor": anchor, "width": width, "height": height},
        )
