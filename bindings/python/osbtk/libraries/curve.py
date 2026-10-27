"""The curve library: Bezier/Catmull-Rom curves, sampling, arc length, tangents,
bounding boxes and Douglas-Peucker simplification (M24)."""

from __future__ import annotations

from typing import Any

from osbtk.api.curve_api import CurveAPI as CurveApi
from osbtk.libraries._client import STATELESS
from osbtk.models.arc_request import ArcRequest
from osbtk.models.curve_distance_request import CurveDistanceRequest
from osbtk.models.curve_offset_request import CurveOffsetRequest
from osbtk.models.curve_point_request import CurvePointRequest
from osbtk.models.curve_request import CurveRequest
from osbtk.models.curve_sample_request import CurveSampleRequest
from osbtk.models.curve_spec import CurveSpec
from osbtk.models.ellipse_request import EllipseRequest
from osbtk.models.line_intersect_request import LineIntersectRequest
from osbtk.models.simplify_request import SimplifyRequest
from osbtk.models.vector2 import Vector2

_api = CurveApi(STATELESS)


class Curve:
    """The curve library: Bezier/Catmull-Rom curves, sampling, arc length,
    tangents, bounding boxes and Douglas-Peucker simplification (M24)."""

    @staticmethod
    def length(curve: CurveSpec) -> float:
        return _api.length(CurveRequest(curve=curve)).value

    @staticmethod
    def bounding_box(curve: CurveSpec) -> Any:
        return _api.bounding_box(CurveRequest(curve=curve))

    @staticmethod
    def point(curve: CurveSpec, t: float, by_arc_length: bool = False) -> Any:
        return _api.point(
            CurvePointRequest(curve=curve, t=t, by_arc_length=by_arc_length)
        )

    @staticmethod
    def tangent(curve: CurveSpec, t: float) -> Any:
        return _api.tangent(CurvePointRequest(curve=curve, t=t))

    @staticmethod
    def sample(curve: CurveSpec, count: int, by_arc_length: bool = False) -> Any:
        return _api.sample(
            CurveSampleRequest(curve=curve, count=count, by_arc_length=by_arc_length)
        )

    @staticmethod
    def simplify(points: list[Any], tolerance: float) -> Any:
        return _api.simplify(SimplifyRequest(points=points, tolerance=tolerance))

    @staticmethod
    def normal(curve: CurveSpec, t: float) -> Any:
        return _api.normal(CurvePointRequest(curve=curve, t=t))

    @staticmethod
    def curvature(curve: CurveSpec, t: float) -> float:
        return _api.curvature(CurvePointRequest(curve=curve, t=t)).value

    @staticmethod
    def distance_to(curve: CurveSpec, point: Vector2) -> float:
        return _api.distance_to(
            CurveDistanceRequest(curve=curve, point=point)
        ).value

    @staticmethod
    def offset(curve: CurveSpec, distance: float) -> Any:
        return _api.offset(CurveOffsetRequest(curve=curve, distance=distance))

    @staticmethod
    def arc(
        center: Vector2, radius: float, start_angle: float, sweep_angle: float, count: int
    ) -> Any:
        return _api.arc(
            ArcRequest(
                center=center,
                radius=radius,
                start_angle=start_angle,
                sweep_angle=sweep_angle,
                count=count,
            )
        )

    @staticmethod
    def ellipse(center: Vector2, radius_x: float, radius_y: float, count: int) -> Any:
        return _api.ellipse(
            EllipseRequest(center=center, radius_x=radius_x, radius_y=radius_y, count=count)
        )

    @staticmethod
    def line_intersect(a1: Vector2, a2: Vector2, b1: Vector2, b2: Vector2) -> Any:
        return _api.line_intersect(LineIntersectRequest(a1=a1, a2=a2, b1=b1, b2=b2))
