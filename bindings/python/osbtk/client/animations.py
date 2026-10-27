from __future__ import annotations

from ..api.animations_api import AnimationsAPI
from ..models.apply_easing_request import ApplyEasingRequest
from ..models.easing_result import EasingResult
from ..models.easing_value_request import EasingValueRequest


class Animations:
    def __init__(self, api: AnimationsAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Animations)

    def apply_easing(self, body: ApplyEasingRequest) -> EasingResult:
        return self._api.apply_easing(body)

    def easing_value(self, body: EasingValueRequest) -> EasingResult:
        return self._api.easing_value(body)
