from __future__ import annotations

from ..api.imagegen_api import ImagegenAPI
from ..models.image_info import ImageInfo
from ..models.render_shapes_request import RenderShapesRequest


class Imagegen:
    def __init__(self, api: ImagegenAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Imagegen)

    def render_shapes(self, body: RenderShapesRequest) -> ImageInfo:
        return self._api.render_shapes(body)
