from __future__ import annotations

from ...api.storyboarding3d_api import Storyboarding3dAPI
from ...models.bake3d_request import Bake3dRequest
from ...models.baked_storyboard import BakedStoryboard
from ...models.scene3d_input import Scene3dInput
from ..options import Bake3dOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Storyboarding3d:
    def __init__(self, api: Storyboarding3dAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Storyboarding3d)

    @overload
    def bake3d(self, input: Scene3dInput) -> BakedStoryboard:
        """A `bake3d` request: a plain 3D scene.

        Args:
            input: The 3D scene.

        Returns:
            Baked command spans
        """
        ...

    @overload
    def bake3d(self, options: Bake3dOptions) -> BakedStoryboard:
        """A `bake3d` request: a plain 3D scene.

        Args:
            options: A `bake3d` request: a plain 3D scene.

        Returns:
            Baked command spans
        """
        ...

    def bake3d(self, *args: object, **kwargs: object) -> BakedStoryboard:
        """A `bake3d` request: a plain 3D scene.

        Returns:
            Baked command spans
        """
        options = args[0] if args and isinstance(args[0], Bake3dOptions) else Bake3dOptions(*args, **kwargs)
        return self._api.bake3d(Bake3dRequest(**to_kwargs(options)))
