from __future__ import annotations

from ...api.core_api import CoreAPI
from ...models.apply_easing_request import ApplyEasingRequest
from ...models.arc_request import ArcRequest
from ...models.beatmap import Beatmap
from ...models.box2 import Box2
from ...models.curve_distance_request import CurveDistanceRequest
from ...models.curve_offset_request import CurveOffsetRequest
from ...models.curve_point_request import CurvePointRequest
from ...models.curve_request import CurveRequest
from ...models.curve_sample_request import CurveSampleRequest
from ...models.curve_spec import CurveSpec
from ...models.easing import Easing
from ...models.easing_value_request import EasingValueRequest
from ...models.ellipse_request import EllipseRequest
from ...models.get_origin_vector_request import GetOriginVectorRequest
from ...models.in_screen_bounds_request import InScreenBoundsRequest
from ...models.line_intersect_request import LineIntersectRequest
from ...models.osb_origin import OsbOrigin
from ...models.parse_osb_result import ParseOsbResult
from ...models.point_list_result import PointListResult
from ...models.simplify_request import SimplifyRequest
from ...models.text_request import TextRequest
from ...models.vector2 import Vector2
from ..options import ApplyEasingOptions, ArcOptions, CurveDistanceOptions, CurveOffsetOptions, CurveOptions, CurvePointOptions, CurveSampleOptions, EasingValueOptions, EllipseOptions, GetOriginVectorOptions, InScreenBoundsOptions, LineIntersectOptions, SimplifyOptions, TextOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Core:
    """The clean `Core` client."""

    def __init__(self, api: CoreAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Core)

    @overload
    def apply_easing(self, easing: Easing, t: float) -> float:
        """An easing evaluation request.

        Args:
            easing: The easing function.
            t: The normalized time in `0.0..=1.0`.

        Returns:
            The eased value
        """
        ...

    @overload
    def apply_easing(self, options: ApplyEasingOptions) -> float:
        """An easing evaluation request.

        Args:
            options: An easing evaluation request.

        Returns:
            The eased value
        """
        ...

    def apply_easing(self, *args: object, **kwargs: object) -> float:
        """An easing evaluation request.

        Returns:
            The eased value
        """
        options = args[0] if args and isinstance(args[0], ApplyEasingOptions) else ApplyEasingOptions(*args, **kwargs)
        return self._api.apply_easing(ApplyEasingRequest(**to_kwargs(options))).value

    @overload
    def easing_value(self, t: float) -> float:
        """A named-or-custom easing evaluation request.

        Args:
            t: The normalized time in `0.0..=1.0`.

        Returns:
            The eased value (named easing or cubic-bezier)
        """
        ...

    @overload
    def easing_value(self, t: float, easing: Optional[Easing]) -> float:
        """A named-or-custom easing evaluation request.

        Args:
            t: The normalized time in `0.0..=1.0`.
            easing: A named easing (used when `cubicBezier` is absent).

        Returns:
            The eased value (named easing or cubic-bezier)
        """
        ...

    @overload
    def easing_value(self, t: float, easing: Optional[Easing], cubicBezier: Optional[List[float]]) -> float:
        """A named-or-custom easing evaluation request.

        Args:
            t: The normalized time in `0.0..=1.0`.
            easing: A named easing (used when `cubicBezier` is absent).
            cubicBezier: A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.

        Returns:
            The eased value (named easing or cubic-bezier)
        """
        ...

    @overload
    def easing_value(self, options: EasingValueOptions) -> float:
        """A named-or-custom easing evaluation request.

        Args:
            options: A named-or-custom easing evaluation request.

        Returns:
            The eased value (named easing or cubic-bezier)
        """
        ...

    def easing_value(self, *args: object, **kwargs: object) -> float:
        """A named-or-custom easing evaluation request.

        Returns:
            The eased value (named easing or cubic-bezier)
        """
        options = args[0] if args and isinstance(args[0], EasingValueOptions) else EasingValueOptions(*args, **kwargs)
        return self._api.easing_value(EasingValueRequest(**to_kwargs(options))).value

    def version(self) -> str:
        """Calls the Core.version bridge operation.

        Returns:
            The runtime version
        """
        return self._api.version().version

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

    @overload
    def get_origin_vector(self, origin: OsbOrigin, width: float, height: float) -> Vector2:
        """The anchor offset for an origin.

        Args:
            origin: The sprite anchor origin.
            width: The sprite width.
            height: The sprite height.

        Returns:
            The anchor offset for the origin
        """
        ...

    @overload
    def get_origin_vector(self, options: GetOriginVectorOptions) -> Vector2:
        """The anchor offset for an origin.

        Args:
            options: The anchor offset for an origin.

        Returns:
            The anchor offset for the origin
        """
        ...

    def get_origin_vector(self, *args: object, **kwargs: object) -> Vector2:
        """The anchor offset for an origin.

        Returns:
            The anchor offset for the origin
        """
        options = args[0] if args and isinstance(args[0], GetOriginVectorOptions) else GetOriginVectorOptions(*args, **kwargs)
        return self._api.get_origin_vector(GetOriginVectorRequest(**to_kwargs(options)))

    @overload
    def in_screen_bounds(self, position: Vector2, size: Vector2, rotation: float, origin: Vector2) -> bool:
        """Whether a rotated sprite's bounding box intersects the widescreen storyboard area.

        Args:
            position: The sprite centre position.
            size: The sprite size.
            rotation: The rotation in radians.
            origin: The anchor offset (from `GetOriginVector`).

        Returns:
            Whether the sprite's bounds intersect the screen
        """
        ...

    @overload
    def in_screen_bounds(self, options: InScreenBoundsOptions) -> bool:
        """Whether a rotated sprite's bounding box intersects the widescreen storyboard area.

        Args:
            options: Whether a rotated sprite's bounding box intersects the widescreen storyboard area.

        Returns:
            Whether the sprite's bounds intersect the screen
        """
        ...

    def in_screen_bounds(self, *args: object, **kwargs: object) -> bool:
        """Whether a rotated sprite's bounding box intersects the widescreen storyboard area.

        Returns:
            Whether the sprite's bounds intersect the screen
        """
        options = args[0] if args and isinstance(args[0], InScreenBoundsOptions) else InScreenBoundsOptions(*args, **kwargs)
        return self._api.in_screen_bounds(InScreenBoundsRequest(**to_kwargs(options))).value

    @overload
    def parse_osb(self, text: str) -> ParseOsbResult:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            text: The text payload.

        Returns:
            The storyboard summary
        """
        ...

    @overload
    def parse_osb(self, options: TextOptions) -> ParseOsbResult:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            options: A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            The storyboard summary
        """
        ...

    def parse_osb(self, *args: object, **kwargs: object) -> ParseOsbResult:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            The storyboard summary
        """
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.parse_osb(TextRequest(**to_kwargs(options)))

    @overload
    def parse_osu(self, text: str) -> Beatmap:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            text: The text payload.

        Returns:
            The parsed beatmap
        """
        ...

    @overload
    def parse_osu(self, options: TextOptions) -> Beatmap:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            options: A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            The parsed beatmap
        """
        ...

    def parse_osu(self, *args: object, **kwargs: object) -> Beatmap:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            The parsed beatmap
        """
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.parse_osu(TextRequest(**to_kwargs(options)))
