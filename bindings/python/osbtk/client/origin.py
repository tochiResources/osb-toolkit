from __future__ import annotations

from ..api.origin_api import OriginAPI
from ..models.bool_result import BoolResult
from ..models.get_origin_vector_request import GetOriginVectorRequest
from ..models.in_screen_bounds_request import InScreenBoundsRequest
from ..models.vector2 import Vector2


class Origin:
    def __init__(self, api: OriginAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Origin)

    def get_origin_vector(self, body: GetOriginVectorRequest) -> Vector2:
        return self._api.get_origin_vector(body)

    def in_screen_bounds(self, body: InScreenBoundsRequest) -> BoolResult:
        return self._api.in_screen_bounds(body)
