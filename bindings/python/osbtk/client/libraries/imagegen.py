from __future__ import annotations

from ...api.imagegen_api import ImagegenAPI
from ...models.image_info import ImageInfo
from ...models.render_shapes_request import RenderShapesRequest
from ...models.rgba_color import RgbaColor
from ...models.shape import Shape
from ..options import RenderShapesOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Imagegen:
    def __init__(self, api: ImagegenAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Imagegen)

    @overload
    def render_shapes(self, width: int, height: int, shapes: List[Shape], output: str) -> ImageInfo:
        """A shape-rendering request: a canvas, a background and a shape list.

        Args:
            width: The canvas width in pixels.
            height: The canvas height in pixels.
            shapes: The shapes to draw, in order.
            output: The output file path (its extension selects the format).

        Returns:
            The rendered image's metadata
        """
        ...

    @overload
    def render_shapes(self, width: int, height: int, shapes: List[Shape], output: str, background: Optional[RgbaColor]) -> ImageInfo:
        """A shape-rendering request: a canvas, a background and a shape list.

        Args:
            width: The canvas width in pixels.
            height: The canvas height in pixels.
            shapes: The shapes to draw, in order.
            output: The output file path (its extension selects the format).
            background: The background colour (default transparent).

        Returns:
            The rendered image's metadata
        """
        ...

    @overload
    def render_shapes(self, options: RenderShapesOptions) -> ImageInfo:
        """A shape-rendering request: a canvas, a background and a shape list.

        Args:
            options: A shape-rendering request: a canvas, a background and a shape list.

        Returns:
            The rendered image's metadata
        """
        ...

    def render_shapes(self, *args: object, **kwargs: object) -> ImageInfo:
        """A shape-rendering request: a canvas, a background and a shape list.

        Returns:
            The rendered image's metadata
        """
        options = args[0] if args and isinstance(args[0], RenderShapesOptions) else RenderShapesOptions(*args, **kwargs)
        return self._api.render_shapes(RenderShapesRequest(**to_kwargs(options)))
