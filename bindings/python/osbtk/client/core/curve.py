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
    def arc(self, center: Vector2, radius: float, startAngle: float, sweepAngle: float) -> PointListResult:
        """A circular-arc sampling request.

        Args:
            center: The arc centre.
            radius: The radius.
            startAngle: The start angle in degrees.
            sweepAngle: The sweep angle in degrees.

        Returns:
            Points along a circular arc
        """
        ...

    @overload
    def arc(self, center: Vector2, radius: float, startAngle: float, sweepAngle: float, count: Optional[int]) -> PointListResult:
        """A circular-arc sampling request.

        Args:
            center: The arc centre.
            radius: The radius.
            startAngle: The start angle in degrees.
            sweepAngle: The sweep angle in degrees.
            count: The number of points (default `64`).

        Returns:
            Points along a circular arc
        """
        ...

    @overload
    def arc(self, options: ArcOptions) -> PointListResult:
        """A circular-arc sampling request.

        Args:
            options: A circular-arc sampling request.

        Returns:
            Points along a circular arc
        """
        ...

    def arc(self, *args: object, **kwargs: object) -> PointListResult:
        """A circular-arc sampling request.

        Returns:
            Points along a circular arc
        """
        options = args[0] if args and isinstance(args[0], ArcOptions) else ArcOptions(*args, **kwargs)
        return self._api.arc(ArcRequest(**to_kwargs(options)))

    @overload
    def bounding_box(self, curve: CurveSpec) -> Box2:
        """A curve query.

        Args:
            curve: The curve.

        Returns:
            The curve's axis-aligned bounding box
        """
        ...

    @overload
    def bounding_box(self, options: CurveOptions) -> Box2:
        """A curve query.

        Args:
            options: A curve query.

        Returns:
            The curve's axis-aligned bounding box
        """
        ...

    def bounding_box(self, *args: object, **kwargs: object) -> Box2:
        """A curve query.

        Returns:
            The curve's axis-aligned bounding box
        """
        options = args[0] if args and isinstance(args[0], CurveOptions) else CurveOptions(*args, **kwargs)
        return self._api.bounding_box(CurveRequest(**to_kwargs(options)))

    @overload
    def curvature(self, curve: CurveSpec, t: float) -> float:
        """A curve query at a parametric or arc-length position.

        Args:
            curve: The curve.
            t: The position in `0.0..=1.0`.

        Returns:
            The curvature at `t`
        """
        ...

    @overload
    def curvature(self, curve: CurveSpec, t: float, byArcLength: Optional[bool]) -> float:
        """A curve query at a parametric or arc-length position.

        Args:
            curve: The curve.
            t: The position in `0.0..=1.0`.
            byArcLength: Sample by arc length instead of the raw parameter (default `false`).

        Returns:
            The curvature at `t`
        """
        ...

    @overload
    def curvature(self, options: CurvePointOptions) -> float:
        """A curve query at a parametric or arc-length position.

        Args:
            options: A curve query at a parametric or arc-length position.

        Returns:
            The curvature at `t`
        """
        ...

    def curvature(self, *args: object, **kwargs: object) -> float:
        """A curve query at a parametric or arc-length position.

        Returns:
            The curvature at `t`
        """
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.curvature(CurvePointRequest(**to_kwargs(options))).value

    @overload
    def distance_to(self, curve: CurveSpec, point: Vector2) -> float:
        """A curve hit-test request.

        Args:
            curve: The curve.
            point: The query point.

        Returns:
            The distance from a point to the curve
        """
        ...

    @overload
    def distance_to(self, options: CurveDistanceOptions) -> float:
        """A curve hit-test request.

        Args:
            options: A curve hit-test request.

        Returns:
            The distance from a point to the curve
        """
        ...

    def distance_to(self, *args: object, **kwargs: object) -> float:
        """A curve hit-test request.

        Returns:
            The distance from a point to the curve
        """
        options = args[0] if args and isinstance(args[0], CurveDistanceOptions) else CurveDistanceOptions(*args, **kwargs)
        return self._api.distance_to(CurveDistanceRequest(**to_kwargs(options))).value

    @overload
    def ellipse(self, center: Vector2, radiusX: float, radiusY: float) -> PointListResult:
        """An ellipse sampling request.

        Args:
            center: The ellipse centre.
            radiusX: The horizontal radius.
            radiusY: The vertical radius.

        Returns:
            Points around an ellipse
        """
        ...

    @overload
    def ellipse(self, center: Vector2, radiusX: float, radiusY: float, count: Optional[int]) -> PointListResult:
        """An ellipse sampling request.

        Args:
            center: The ellipse centre.
            radiusX: The horizontal radius.
            radiusY: The vertical radius.
            count: The number of points (default `64`).

        Returns:
            Points around an ellipse
        """
        ...

    @overload
    def ellipse(self, options: EllipseOptions) -> PointListResult:
        """An ellipse sampling request.

        Args:
            options: An ellipse sampling request.

        Returns:
            Points around an ellipse
        """
        ...

    def ellipse(self, *args: object, **kwargs: object) -> PointListResult:
        """An ellipse sampling request.

        Returns:
            Points around an ellipse
        """
        options = args[0] if args and isinstance(args[0], EllipseOptions) else EllipseOptions(*args, **kwargs)
        return self._api.ellipse(EllipseRequest(**to_kwargs(options)))

    @overload
    def length(self, curve: CurveSpec) -> float:
        """A curve query.

        Args:
            curve: The curve.

        Returns:
            The curve's arc length
        """
        ...

    @overload
    def length(self, options: CurveOptions) -> float:
        """A curve query.

        Args:
            options: A curve query.

        Returns:
            The curve's arc length
        """
        ...

    def length(self, *args: object, **kwargs: object) -> float:
        """A curve query.

        Returns:
            The curve's arc length
        """
        options = args[0] if args and isinstance(args[0], CurveOptions) else CurveOptions(*args, **kwargs)
        return self._api.length(CurveRequest(**to_kwargs(options))).value

    @overload
    def line_intersect(self, a1: Vector2, a2: Vector2, b1: Vector2, b2: Vector2) -> Optional[Vector2]:
        """A line-segment intersection request.

        Args:
            a1: The first segment's start.
            a2: The first segment's end.
            b1: The second segment's start.
            b2: The second segment's end.

        Returns:
            The intersection point, or null
        """
        ...

    @overload
    def line_intersect(self, options: LineIntersectOptions) -> Optional[Vector2]:
        """A line-segment intersection request.

        Args:
            options: A line-segment intersection request.

        Returns:
            The intersection point, or null
        """
        ...

    def line_intersect(self, *args: object, **kwargs: object) -> Optional[Vector2]:
        """A line-segment intersection request.

        Returns:
            The intersection point, or null
        """
        options = args[0] if args and isinstance(args[0], LineIntersectOptions) else LineIntersectOptions(*args, **kwargs)
        return self._api.line_intersect(LineIntersectRequest(**to_kwargs(options))).point

    @overload
    def normal(self, curve: CurveSpec, t: float) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Args:
            curve: The curve.
            t: The position in `0.0..=1.0`.

        Returns:
            The unit normal at `t`
        """
        ...

    @overload
    def normal(self, curve: CurveSpec, t: float, byArcLength: Optional[bool]) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Args:
            curve: The curve.
            t: The position in `0.0..=1.0`.
            byArcLength: Sample by arc length instead of the raw parameter (default `false`).

        Returns:
            The unit normal at `t`
        """
        ...

    @overload
    def normal(self, options: CurvePointOptions) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Args:
            options: A curve query at a parametric or arc-length position.

        Returns:
            The unit normal at `t`
        """
        ...

    def normal(self, *args: object, **kwargs: object) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Returns:
            The unit normal at `t`
        """
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.normal(CurvePointRequest(**to_kwargs(options)))

    @overload
    def offset(self, curve: CurveSpec, distance: float) -> PointListResult:
        """A curve offset request.

        Args:
            curve: The curve.
            distance: The signed offset along the curve's local normal.

        Returns:
            The polyline offset along the curve's normal
        """
        ...

    @overload
    def offset(self, options: CurveOffsetOptions) -> PointListResult:
        """A curve offset request.

        Args:
            options: A curve offset request.

        Returns:
            The polyline offset along the curve's normal
        """
        ...

    def offset(self, *args: object, **kwargs: object) -> PointListResult:
        """A curve offset request.

        Returns:
            The polyline offset along the curve's normal
        """
        options = args[0] if args and isinstance(args[0], CurveOffsetOptions) else CurveOffsetOptions(*args, **kwargs)
        return self._api.offset(CurveOffsetRequest(**to_kwargs(options)))

    @overload
    def point(self, curve: CurveSpec, t: float) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Args:
            curve: The curve.
            t: The position in `0.0..=1.0`.

        Returns:
            The point at `t`, parametrically or by arc length
        """
        ...

    @overload
    def point(self, curve: CurveSpec, t: float, byArcLength: Optional[bool]) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Args:
            curve: The curve.
            t: The position in `0.0..=1.0`.
            byArcLength: Sample by arc length instead of the raw parameter (default `false`).

        Returns:
            The point at `t`, parametrically or by arc length
        """
        ...

    @overload
    def point(self, options: CurvePointOptions) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Args:
            options: A curve query at a parametric or arc-length position.

        Returns:
            The point at `t`, parametrically or by arc length
        """
        ...

    def point(self, *args: object, **kwargs: object) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Returns:
            The point at `t`, parametrically or by arc length
        """
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.point(CurvePointRequest(**to_kwargs(options)))

    @overload
    def sample(self, count: int, curve: CurveSpec) -> PointListResult:
        """A curve sampling request.

        Args:
            count: The number of points to sample.
            curve: The curve.

        Returns:
            `count` points along the curve
        """
        ...

    @overload
    def sample(self, count: int, curve: CurveSpec, byArcLength: Optional[bool]) -> PointListResult:
        """A curve sampling request.

        Args:
            count: The number of points to sample.
            curve: The curve.
            byArcLength: Sample by arc length instead of the raw parameter (default `false`).

        Returns:
            `count` points along the curve
        """
        ...

    @overload
    def sample(self, options: CurveSampleOptions) -> PointListResult:
        """A curve sampling request.

        Args:
            options: A curve sampling request.

        Returns:
            `count` points along the curve
        """
        ...

    def sample(self, *args: object, **kwargs: object) -> PointListResult:
        """A curve sampling request.

        Returns:
            `count` points along the curve
        """
        options = args[0] if args and isinstance(args[0], CurveSampleOptions) else CurveSampleOptions(*args, **kwargs)
        return self._api.sample(CurveSampleRequest(**to_kwargs(options)))

    @overload
    def simplify(self, points: List[Vector2], tolerance: float) -> PointListResult:
        """A Douglas-Peucker simplification request.

        Args:
            points: The polyline to simplify.
            tolerance: The maximum deviation to allow.

        Returns:
            The simplified polyline
        """
        ...

    @overload
    def simplify(self, options: SimplifyOptions) -> PointListResult:
        """A Douglas-Peucker simplification request.

        Args:
            options: A Douglas-Peucker simplification request.

        Returns:
            The simplified polyline
        """
        ...

    def simplify(self, *args: object, **kwargs: object) -> PointListResult:
        """A Douglas-Peucker simplification request.

        Returns:
            The simplified polyline
        """
        options = args[0] if args and isinstance(args[0], SimplifyOptions) else SimplifyOptions(*args, **kwargs)
        return self._api.simplify(SimplifyRequest(**to_kwargs(options)))

    @overload
    def tangent(self, curve: CurveSpec, t: float) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Args:
            curve: The curve.
            t: The position in `0.0..=1.0`.

        Returns:
            The unit tangent at `t`
        """
        ...

    @overload
    def tangent(self, curve: CurveSpec, t: float, byArcLength: Optional[bool]) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Args:
            curve: The curve.
            t: The position in `0.0..=1.0`.
            byArcLength: Sample by arc length instead of the raw parameter (default `false`).

        Returns:
            The unit tangent at `t`
        """
        ...

    @overload
    def tangent(self, options: CurvePointOptions) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Args:
            options: A curve query at a parametric or arc-length position.

        Returns:
            The unit tangent at `t`
        """
        ...

    def tangent(self, *args: object, **kwargs: object) -> Vector2:
        """A curve query at a parametric or arc-length position.

        Returns:
            The unit tangent at `t`
        """
        options = args[0] if args and isinstance(args[0], CurvePointOptions) else CurvePointOptions(*args, **kwargs)
        return self._api.tangent(CurvePointRequest(**to_kwargs(options)))
