from __future__ import annotations

from ...api.animations_api import AnimationsAPI
from ...models.apply_easing_request import ApplyEasingRequest
from ...models.easing_result import EasingResult
from ...models.easing_value_request import EasingValueRequest
from ..options import ApplyEasingOptions, EasingValueOptions, to_kwargs


class Animations:
    def __init__(self, api: AnimationsAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Animations)

    def apply_easing(self, options: ApplyEasingOptions) -> EasingResult:
        return self._api.apply_easing(ApplyEasingRequest(**to_kwargs(options)))

    def easing_value(self, options: EasingValueOptions) -> EasingResult:
        return self._api.easing_value(EasingValueRequest(**to_kwargs(options)))
