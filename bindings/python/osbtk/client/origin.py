from __future__ import annotations

from ..api.origin_api import OriginAPI
from ..models.bool_result import BoolResult
from ..models.get_origin_vector_request import GetOriginVectorRequest
from ..models.in_screen_bounds_request import InScreenBoundsRequest
from ..models.vector2 import Vector2
from .options import GetOriginVectorOptions, InScreenBoundsOptions, to_kwargs


class Origin:
    def __init__(self, api: OriginAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Origin)

    def get_origin_vector(self, options: GetOriginVectorOptions) -> Vector2:
        return self._api.get_origin_vector(GetOriginVectorRequest(**to_kwargs(options)))

    def in_screen_bounds(self, options: InScreenBoundsOptions) -> BoolResult:
        return self._api.in_screen_bounds(InScreenBoundsRequest(**to_kwargs(options)))
