from __future__ import annotations

from ..api.curve_api import CurveAPI
from ..models.arc_request import ArcRequest
from ..models.box2 import Box2
from ..models.curve_distance_request import CurveDistanceRequest
from ..models.curve_offset_request import CurveOffsetRequest
from ..models.curve_point_request import CurvePointRequest
from ..models.curve_request import CurveRequest
from ..models.curve_sample_request import CurveSampleRequest
from ..models.ellipse_request import EllipseRequest
from ..models.line_intersect_request import LineIntersectRequest
from ..models.optional_point_result import OptionalPointResult
from ..models.point_list_result import PointListResult
from ..models.scalar_result import ScalarResult
from ..models.simplify_request import SimplifyRequest
from ..models.vector2 import Vector2


class Curve:
    def __init__(self, api: CurveAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Curve)

    def arc(self, body: ArcRequest) -> PointListResult:
        return self._api.arc(body)

    def bounding_box(self, body: CurveRequest) -> Box2:
        return self._api.bounding_box(body)

    def curvature(self, body: CurvePointRequest) -> ScalarResult:
        return self._api.curvature(body)

    def distance_to(self, body: CurveDistanceRequest) -> ScalarResult:
        return self._api.distance_to(body)

    def ellipse(self, body: EllipseRequest) -> PointListResult:
        return self._api.ellipse(body)

    def length(self, body: CurveRequest) -> ScalarResult:
        return self._api.length(body)

    def line_intersect(self, body: LineIntersectRequest) -> OptionalPointResult:
        return self._api.line_intersect(body)

    def normal(self, body: CurvePointRequest) -> Vector2:
        return self._api.normal(body)

    def offset(self, body: CurveOffsetRequest) -> PointListResult:
        return self._api.offset(body)

    def point(self, body: CurvePointRequest) -> Vector2:
        return self._api.point(body)

    def sample(self, body: CurveSampleRequest) -> PointListResult:
        return self._api.sample(body)

    def simplify(self, body: SimplifyRequest) -> PointListResult:
        return self._api.simplify(body)

    def tangent(self, body: CurvePointRequest) -> Vector2:
        return self._api.tangent(body)
