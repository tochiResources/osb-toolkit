from __future__ import annotations

from ...api.generator_api import GeneratorAPI
from ...models.beatmap import Beatmap
from ...models.control_point import ControlPoint
from ...models.import_osb_result import ImportOsbResult
from ...models.layer_handle import LayerHandle
from ...models.layer_handle_request import LayerHandleRequest
from ...models.load_beatmap_result import LoadBeatmapResult
from ...models.random_request import RandomRequest
from ...models.random_result import RandomResult
from ...models.sprite_count_result import SpriteCountResult
from ...models.storyboard import Storyboard
from ...models.text_request import TextRequest
from ...models.time_request import TimeRequest
from ..options import LayerHandleOptions, RandomOptions, TextOptions, TimeOptions, to_kwargs


class Generator:
    def __init__(self, api: GeneratorAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Generator)

    def beatmap(self) -> Beatmap:
        return self._api.beatmap()

    def control_point_at(self, options: TimeOptions) -> ControlPoint:
        return self._api.control_point_at(TimeRequest(**to_kwargs(options)))

    def import_osb(self, options: TextOptions) -> ImportOsbResult:
        return self._api.import_osb(TextRequest(**to_kwargs(options)))

    def layer_handle(self, options: LayerHandleOptions) -> LayerHandle:
        return self._api.layer_handle(LayerHandleRequest(**to_kwargs(options)))

    def load_beatmap(self, options: TextOptions) -> LoadBeatmapResult:
        return self._api.load_beatmap(TextRequest(**to_kwargs(options)))

    def random(self, options: RandomOptions) -> RandomResult:
        return self._api.random(RandomRequest(**to_kwargs(options)))

    def sprite_count(self) -> SpriteCountResult:
        return self._api.sprite_count()

    def storyboard(self) -> Storyboard:
        return self._api.storyboard()

    def timing_point_at(self, options: TimeOptions) -> ControlPoint:
        return self._api.timing_point_at(TimeRequest(**to_kwargs(options)))
