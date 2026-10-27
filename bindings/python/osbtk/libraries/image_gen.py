"""The image-generation library: draw primitive shapes (M25)."""

from __future__ import annotations

from typing import Any

from osbtk.api.imagegen_api import ImagegenAPI as ImagegenApi
from osbtk.libraries._client import STATELESS
from osbtk.models.render_shapes_request import RenderShapesRequest

_api = ImagegenApi(STATELESS)


class ImageGen:
    """The image-generation library: draw primitive shapes (M25)."""

    @staticmethod
    def render_shapes(request: RenderShapesRequest) -> Any:
        return _api.render_shapes(request)
