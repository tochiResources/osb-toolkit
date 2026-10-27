from __future__ import annotations

from ...api.storyboarding3d_api import Storyboarding3dAPI
from ...models.bake3d_request import Bake3dRequest
from ...models.baked_storyboard import BakedStoryboard
from ..options import Bake3dOptions, to_kwargs


class Storyboarding3d:
    def __init__(self, api: Storyboarding3dAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Storyboarding3d)

    def bake3d(self, options: Bake3dOptions) -> BakedStoryboard:
        return self._api.bake3d(Bake3dRequest(**to_kwargs(options)))
