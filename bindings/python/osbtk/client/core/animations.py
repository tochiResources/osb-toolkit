from __future__ import annotations

from ...api.animations_api import AnimationsAPI
from ...models.apply_easing_request import ApplyEasingRequest
from ...models.easing import Easing
from ...models.easing_value_request import EasingValueRequest
from ..options import ApplyEasingOptions, EasingValueOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Animations:
    def __init__(self, api: AnimationsAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Animations)

    @overload
    def apply_easing(self, easing: Easing, t: float) -> float: ...

    @overload
    def apply_easing(self, options: ApplyEasingOptions) -> float: ...

    def apply_easing(self, *args: object, **kwargs: object) -> float:
        options = args[0] if args and isinstance(args[0], ApplyEasingOptions) else ApplyEasingOptions(*args, **kwargs)
        return self._api.apply_easing(ApplyEasingRequest(**to_kwargs(options))).value

    @overload
    def easing_value(self, t: float) -> float: ...

    @overload
    def easing_value(self, t: float, easing: Optional[Easing]) -> float: ...

    @overload
    def easing_value(self, t: float, easing: Optional[Easing], cubicBezier: Optional[List[float]]) -> float: ...

    @overload
    def easing_value(self, options: EasingValueOptions) -> float: ...

    def easing_value(self, *args: object, **kwargs: object) -> float:
        options = args[0] if args and isinstance(args[0], EasingValueOptions) else EasingValueOptions(*args, **kwargs)
        return self._api.easing_value(EasingValueRequest(**to_kwargs(options))).value
