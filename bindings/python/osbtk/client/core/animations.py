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
