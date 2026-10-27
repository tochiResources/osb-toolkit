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
from .options import ArcOptions, CurveDistanceOptions, CurveOffsetOptions, CurveOptions, CurvePointOptions, CurveSampleOptions, EllipseOptions, LineIntersectOptions, SimplifyOptions, to_kwargs


class Curve:
    def __init__(self, api: CurveAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Curve)

    def arc(self, options: ArcOptions) -> PointListResult:
        return self._api.arc(ArcRequest(**to_kwargs(options)))

    def bounding_box(self, options: CurveOptions) -> Box2:
        return self._api.bounding_box(CurveRequest(**to_kwargs(options)))

    def curvature(self, options: CurvePointOptions) -> ScalarResult:
        return self._api.curvature(CurvePointRequest(**to_kwargs(options)))

    def distance_to(self, options: CurveDistanceOptions) -> ScalarResult:
        return self._api.distance_to(CurveDistanceRequest(**to_kwargs(options)))

    def ellipse(self, options: EllipseOptions) -> PointListResult:
        return self._api.ellipse(EllipseRequest(**to_kwargs(options)))

    def length(self, options: CurveOptions) -> ScalarResult:
        return self._api.length(CurveRequest(**to_kwargs(options)))

    def line_intersect(self, options: LineIntersectOptions) -> OptionalPointResult:
        return self._api.line_intersect(LineIntersectRequest(**to_kwargs(options)))

    def normal(self, options: CurvePointOptions) -> Vector2:
        return self._api.normal(CurvePointRequest(**to_kwargs(options)))

    def offset(self, options: CurveOffsetOptions) -> PointListResult:
        return self._api.offset(CurveOffsetRequest(**to_kwargs(options)))

    def point(self, options: CurvePointOptions) -> Vector2:
        return self._api.point(CurvePointRequest(**to_kwargs(options)))

    def sample(self, options: CurveSampleOptions) -> PointListResult:
        return self._api.sample(CurveSampleRequest(**to_kwargs(options)))

    def simplify(self, options: SimplifyOptions) -> PointListResult:
        return self._api.simplify(SimplifyRequest(**to_kwargs(options)))

    def tangent(self, options: CurvePointOptions) -> Vector2:
        return self._api.tangent(CurvePointRequest(**to_kwargs(options)))
