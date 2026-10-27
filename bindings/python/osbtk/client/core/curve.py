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
from ...models.point_list_result import PointListResult
from ...models.simplify_request import SimplifyRequest
from ...models.vector2 import Vector2
from ..options import ArcOptions, CurveDistanceOptions, CurveOffsetOptions, CurveOptions, CurvePointOptions, CurveSampleOptions, EllipseOptions, LineIntersectOptions, SimplifyOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Curve:
    def __init__(self, api: CurveAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Curve)

    @overload
    def arc(self, center: Vector2, radius: float, startAngle: float, sweepAngle: float) -> PointListResult: ...

    @overload
    def arc(self, center: Vector2, radius: float, startAngle: float, sweepAngle: float, count: Optional[int]) -> PointListResult: ...

    @overload
    def arc(self, options: ArcOptions) -> PointListResult: ...

    def arc(self, *args: object, **kwargs: object) -> PointListResult:
        options = args[0] if args and isinstance(args[0], ArcOptions) else ArcOptions(*args, **kwargs)
        return self._api.arc(ArcRequest(**to_kwargs(options)))

    @overload
    def bounding_box(self, curve: CurveSpec) -> Box2: ...

    @overload
    def bounding_box(self, options: CurveOptions) -> Box2: ...

    def bounding_box(self, *args: object, **kwargs: object) -> Box2:
        options = args[0] if args and isinstance(args[0], CurveOptions) else CurveOptions(*args, **kwargs)
        return self._api.bounding_box(CurveRequest(**to_kwargs(options)))

    @overload
    def curvature(self, curve: CurveSpec, t: float) -> float: ...

    @overload
    def curvature(self, curve: CurveSpec, t: float, byArcLength: Optional[bool]) -> float: ...

    @overload
    def curvature(self, options: CurvePointOptions) -> float: ...

    def curvature(self, *args: object, **kwargs: object) -> float:
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.curvature(CurvePointRequest(**to_kwargs(options))).value

    @overload
    def distance_to(self, curve: CurveSpec, point: Vector2) -> float: ...

    @overload
    def distance_to(self, options: CurveDistanceOptions) -> float: ...

    def distance_to(self, *args: object, **kwargs: object) -> float:
        options = args[0] if args and isinstance(args[0], CurveDistanceOptions) else CurveDistanceOptions(*args, **kwargs)
        return self._api.distance_to(CurveDistanceRequest(**to_kwargs(options))).value

    @overload
    def ellipse(self, center: Vector2, radiusX: float, radiusY: float) -> PointListResult: ...

    @overload
    def ellipse(self, center: Vector2, radiusX: float, radiusY: float, count: Optional[int]) -> PointListResult: ...

    @overload
    def ellipse(self, options: EllipseOptions) -> PointListResult: ...

    def ellipse(self, *args: object, **kwargs: object) -> PointListResult:
        options = args[0] if args and isinstance(args[0], EllipseOptions) else EllipseOptions(*args, **kwargs)
        return self._api.ellipse(EllipseRequest(**to_kwargs(options)))

    @overload
    def length(self, curve: CurveSpec) -> float: ...

    @overload
    def length(self, options: CurveOptions) -> float: ...

    def length(self, *args: object, **kwargs: object) -> float:
        options = args[0] if args and isinstance(args[0], CurveOptions) else CurveOptions(*args, **kwargs)
        return self._api.length(CurveRequest(**to_kwargs(options))).value

    @overload
    def line_intersect(self, a1: Vector2, a2: Vector2, b1: Vector2, b2: Vector2) -> Optional[Vector2]: ...

    @overload
    def line_intersect(self, options: LineIntersectOptions) -> Optional[Vector2]: ...

    def line_intersect(self, *args: object, **kwargs: object) -> Optional[Vector2]:
        options = args[0] if args and isinstance(args[0], LineIntersectOptions) else LineIntersectOptions(*args, **kwargs)
        return self._api.line_intersect(LineIntersectRequest(**to_kwargs(options))).point

    @overload
    def normal(self, curve: CurveSpec, t: float) -> Vector2: ...

    @overload
    def normal(self, curve: CurveSpec, t: float, byArcLength: Optional[bool]) -> Vector2: ...

    @overload
    def normal(self, options: CurvePointOptions) -> Vector2: ...

    def normal(self, *args: object, **kwargs: object) -> Vector2:
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.normal(CurvePointRequest(**to_kwargs(options)))

    @overload
    def offset(self, curve: CurveSpec, distance: float) -> PointListResult: ...

    @overload
    def offset(self, options: CurveOffsetOptions) -> PointListResult: ...

    def offset(self, *args: object, **kwargs: object) -> PointListResult:
        options = args[0] if args and isinstance(args[0], CurveOffsetOptions) else CurveOffsetOptions(*args, **kwargs)
        return self._api.offset(CurveOffsetRequest(**to_kwargs(options)))

    @overload
    def point(self, curve: CurveSpec, t: float) -> Vector2: ...

    @overload
    def point(self, curve: CurveSpec, t: float, byArcLength: Optional[bool]) -> Vector2: ...

    @overload
    def point(self, options: CurvePointOptions) -> Vector2: ...

    def point(self, *args: object, **kwargs: object) -> Vector2:
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.point(CurvePointRequest(**to_kwargs(options)))

    @overload
    def sample(self, count: int, curve: CurveSpec) -> PointListResult: ...

    @overload
    def sample(self, count: int, curve: CurveSpec, byArcLength: Optional[bool]) -> PointListResult: ...

    @overload
    def sample(self, options: CurveSampleOptions) -> PointListResult: ...

    def sample(self, *args: object, **kwargs: object) -> PointListResult:
        options = args[0] if args and isinstance(args[0], CurveSampleOptions) else CurveSampleOptions(*args, **kwargs)
        return self._api.sample(CurveSampleRequest(**to_kwargs(options)))

    @overload
    def simplify(self, points: List[Vector2], tolerance: float) -> PointListResult: ...

    @overload
    def simplify(self, options: SimplifyOptions) -> PointListResult: ...

    def simplify(self, *args: object, **kwargs: object) -> PointListResult:
        options = args[0] if args and isinstance(args[0], SimplifyOptions) else SimplifyOptions(*args, **kwargs)
        return self._api.simplify(SimplifyRequest(**to_kwargs(options)))

    @overload
    def tangent(self, curve: CurveSpec, t: float) -> Vector2: ...

    @overload
    def tangent(self, curve: CurveSpec, t: float, byArcLength: Optional[bool]) -> Vector2: ...

    @overload
    def tangent(self, options: CurvePointOptions) -> Vector2: ...

    def tangent(self, *args: object, **kwargs: object) -> Vector2:
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.tangent(CurvePointRequest(**to_kwargs(options)))
