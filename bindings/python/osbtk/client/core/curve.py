from __future__ import annotations

from ...api.curve_api import CurveAPI
from ...models.arc_request import ArcRequest
from ...models.box2 import Box2
from ...models.curve_distance_request import CurveDistanceRequest
from ...models.curve_offset_request import CurveOffsetRequest
from ...models.curve_point_request import CurvePointRequest
from ...models.curve_request import CurveRequest
from ...models.curve_sample_request import CurveSampleRequest
from ...models.curve_spec import CurveSpec
from ...models.ellipse_request import EllipseRequest
from ...models.line_intersect_request import LineIntersectRequest
from ...models.optional_point_result import OptionalPointResult
from ...models.point_list_result import PointListResult
from ...models.scalar_result import ScalarResult
from ...models.simplify_request import SimplifyRequest
from ...models.vector2 import Vector2
from ..options import ArcOptions, CurveDistanceOptions, CurveOffsetOptions, CurveOptions, CurvePointOptions, CurveSampleOptions, EllipseOptions, LineIntersectOptions, SimplifyOptions, to_kwargs
from typing import overload


class Curve:
    def __init__(self, api: CurveAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Curve)

    @overload
    def arc(self, options: ArcOptions) -> PointListResult: ...

    @overload
    def arc(self, center: Vector2, radius: float, startAngle: float, sweepAngle: float, count: Optional[int] = None) -> PointListResult: ...

    def arc(self, *args: object, **kwargs: object) -> PointListResult:
        options = args[0] if args and isinstance(args[0], ArcOptions) else ArcOptions(*args, **kwargs)
        return self._api.arc(ArcRequest(**to_kwargs(options)))

    @overload
    def bounding_box(self, options: CurveOptions) -> Box2: ...

    @overload
    def bounding_box(self, curve: CurveSpec) -> Box2: ...

    def bounding_box(self, *args: object, **kwargs: object) -> Box2:
        options = args[0] if args and isinstance(args[0], CurveOptions) else CurveOptions(*args, **kwargs)
        return self._api.bounding_box(CurveRequest(**to_kwargs(options)))

    @overload
    def curvature(self, options: CurvePointOptions) -> ScalarResult: ...

    @overload
    def curvature(self, curve: CurveSpec, t: float, byArcLength: Optional[bool] = None) -> ScalarResult: ...

    def curvature(self, *args: object, **kwargs: object) -> ScalarResult:
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.curvature(CurvePointRequest(**to_kwargs(options)))

    @overload
    def distance_to(self, options: CurveDistanceOptions) -> ScalarResult: ...

    @overload
    def distance_to(self, curve: CurveSpec, point: Vector2) -> ScalarResult: ...

    def distance_to(self, *args: object, **kwargs: object) -> ScalarResult:
        options = args[0] if args and isinstance(args[0], CurveDistanceOptions) else CurveDistanceOptions(*args, **kwargs)
        return self._api.distance_to(CurveDistanceRequest(**to_kwargs(options)))

    @overload
    def ellipse(self, options: EllipseOptions) -> PointListResult: ...

    @overload
    def ellipse(self, center: Vector2, radiusX: float, radiusY: float, count: Optional[int] = None) -> PointListResult: ...

    def ellipse(self, *args: object, **kwargs: object) -> PointListResult:
        options = args[0] if args and isinstance(args[0], EllipseOptions) else EllipseOptions(*args, **kwargs)
        return self._api.ellipse(EllipseRequest(**to_kwargs(options)))

    @overload
    def length(self, options: CurveOptions) -> ScalarResult: ...

    @overload
    def length(self, curve: CurveSpec) -> ScalarResult: ...

    def length(self, *args: object, **kwargs: object) -> ScalarResult:
        options = args[0] if args and isinstance(args[0], CurveOptions) else CurveOptions(*args, **kwargs)
        return self._api.length(CurveRequest(**to_kwargs(options)))

    @overload
    def line_intersect(self, options: LineIntersectOptions) -> OptionalPointResult: ...

    @overload
    def line_intersect(self, a1: Vector2, a2: Vector2, b1: Vector2, b2: Vector2) -> OptionalPointResult: ...

    def line_intersect(self, *args: object, **kwargs: object) -> OptionalPointResult:
        options = args[0] if args and isinstance(args[0], LineIntersectOptions) else LineIntersectOptions(*args, **kwargs)
        return self._api.line_intersect(LineIntersectRequest(**to_kwargs(options)))

    @overload
    def normal(self, options: CurvePointOptions) -> Vector2: ...

    @overload
    def normal(self, curve: CurveSpec, t: float, byArcLength: Optional[bool] = None) -> Vector2: ...

    def normal(self, *args: object, **kwargs: object) -> Vector2:
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.normal(CurvePointRequest(**to_kwargs(options)))

    @overload
    def offset(self, options: CurveOffsetOptions) -> PointListResult: ...

    @overload
    def offset(self, curve: CurveSpec, distance: float) -> PointListResult: ...

    def offset(self, *args: object, **kwargs: object) -> PointListResult:
        options = args[0] if args and isinstance(args[0], CurveOffsetOptions) else CurveOffsetOptions(*args, **kwargs)
        return self._api.offset(CurveOffsetRequest(**to_kwargs(options)))

    @overload
    def point(self, options: CurvePointOptions) -> Vector2: ...

    @overload
    def point(self, curve: CurveSpec, t: float, byArcLength: Optional[bool] = None) -> Vector2: ...

    def point(self, *args: object, **kwargs: object) -> Vector2:
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.point(CurvePointRequest(**to_kwargs(options)))

    @overload
    def sample(self, options: CurveSampleOptions) -> PointListResult: ...

    @overload
    def sample(self, count: int, curve: CurveSpec, byArcLength: Optional[bool] = None) -> PointListResult: ...

    def sample(self, *args: object, **kwargs: object) -> PointListResult:
        options = args[0] if args and isinstance(args[0], CurveSampleOptions) else CurveSampleOptions(*args, **kwargs)
        return self._api.sample(CurveSampleRequest(**to_kwargs(options)))

    @overload
    def simplify(self, options: SimplifyOptions) -> PointListResult: ...

    @overload
    def simplify(self, points: List[Vector2], tolerance: float) -> PointListResult: ...

    def simplify(self, *args: object, **kwargs: object) -> PointListResult:
        options = args[0] if args and isinstance(args[0], SimplifyOptions) else SimplifyOptions(*args, **kwargs)
        return self._api.simplify(SimplifyRequest(**to_kwargs(options)))

    @overload
    def tangent(self, options: CurvePointOptions) -> Vector2: ...

    @overload
    def tangent(self, curve: CurveSpec, t: float, byArcLength: Optional[bool] = None) -> Vector2: ...

    def tangent(self, *args: object, **kwargs: object) -> Vector2:
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.tangent(CurvePointRequest(**to_kwargs(options)))
