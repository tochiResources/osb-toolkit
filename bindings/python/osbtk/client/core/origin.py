from __future__ import annotations

from ...api.origin_api import OriginAPI
from ...models.get_origin_vector_request import GetOriginVectorRequest
from ...models.in_screen_bounds_request import InScreenBoundsRequest
from ...models.osb_origin import OsbOrigin
from ...models.vector2 import Vector2
from ..options import GetOriginVectorOptions, InScreenBoundsOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Origin:
    """The clean `Origin` client."""

    def __init__(self, api: OriginAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Origin)

    @overload
    def get_origin_vector(self, origin: OsbOrigin, width: float, height: float) -> Vector2:
        """The anchor offset for an origin.

        Args:
            origin: The sprite anchor origin.
            width: The sprite width.
            height: The sprite height.

        Returns:
            The anchor offset for the origin
        """
        ...

    @overload
    def get_origin_vector(self, options: GetOriginVectorOptions) -> Vector2:
        """The anchor offset for an origin.

        Args:
            options: The anchor offset for an origin.

        Returns:
            The anchor offset for the origin
        """
        ...

    def get_origin_vector(self, *args: object, **kwargs: object) -> Vector2:
        """The anchor offset for an origin.

        Returns:
            The anchor offset for the origin
        """
        options = args[0] if args and isinstance(args[0], GetOriginVectorOptions) else GetOriginVectorOptions(*args, **kwargs)
        return self._api.get_origin_vector(GetOriginVectorRequest(**to_kwargs(options)))

    @overload
    def in_screen_bounds(self, position: Vector2, size: Vector2, rotation: float, origin: Vector2) -> bool:
        """Whether a rotated sprite's bounding box intersects the widescreen storyboard area.

        Args:
            position: The sprite centre position.
            size: The sprite size.
            rotation: The rotation in radians.
            origin: The anchor offset (from `GetOriginVector`).

        Returns:
            Whether the sprite's bounds intersect the screen
        """
        ...

    @overload
    def in_screen_bounds(self, options: InScreenBoundsOptions) -> bool:
        """Whether a rotated sprite's bounding box intersects the widescreen storyboard area.

        Args:
            options: Whether a rotated sprite's bounding box intersects the widescreen storyboard area.

        Returns:
            Whether the sprite's bounds intersect the screen
        """
        ...

    def in_screen_bounds(self, *args: object, **kwargs: object) -> bool:
        """Whether a rotated sprite's bounding box intersects the widescreen storyboard area.

        Returns:
            Whether the sprite's bounds intersect the screen
        """
        options = args[0] if args and isinstance(args[0], InScreenBoundsOptions) else InScreenBoundsOptions(*args, **kwargs)
        return self._api.in_screen_bounds(InScreenBoundsRequest(**to_kwargs(options))).value
