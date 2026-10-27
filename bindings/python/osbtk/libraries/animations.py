"""The animations library: easing evaluation."""

from __future__ import annotations

from osbtk.api.animations_api import AnimationsAPI as AnimationsApi
from osbtk.libraries._client import STATELESS
from osbtk.models.apply_easing_request import ApplyEasingRequest
from osbtk.models.easing_value_request import EasingValueRequest

_api = AnimationsApi(STATELESS)


class Animations:
    """The animations library: easing evaluation."""

    @staticmethod
    def apply_easing(easing: str, t: float) -> float:
        return _api.apply_easing(ApplyEasingRequest(easing=easing, t=t)).value

    @staticmethod
    def apply_cubic_bezier(x1: float, y1: float, x2: float, y2: float, t: float) -> float:
        return _api.easing_value(
            EasingValueRequest(cubic_bezier=[x1, y1, x2, y2], t=t)
        ).value
