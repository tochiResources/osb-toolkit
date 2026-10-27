from __future__ import annotations

from ..api.generator_api import GeneratorAPI
from ..models.beatmap import Beatmap
from ..models.control_point import ControlPoint
from ..models.import_osb_result import ImportOsbResult
from ..models.layer_handle import LayerHandle
from ..models.layer_handle_request import LayerHandleRequest
from ..models.load_beatmap_result import LoadBeatmapResult
from ..models.random_request import RandomRequest
from ..models.random_result import RandomResult
from ..models.sprite_count_result import SpriteCountResult
from ..models.storyboard import Storyboard
from ..models.text_request import TextRequest
from ..models.time_request import TimeRequest


class Generator:
    def __init__(self, api: GeneratorAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Generator)

    def beatmap(self) -> Beatmap:
        return self._api.beatmap()

    def control_point_at(self, body: TimeRequest) -> ControlPoint:
        return self._api.control_point_at(body)

    def import_osb(self, body: TextRequest) -> ImportOsbResult:
        return self._api.import_osb(body)

    def layer_handle(self, body: LayerHandleRequest) -> LayerHandle:
        return self._api.layer_handle(body)

    def load_beatmap(self, body: TextRequest) -> LoadBeatmapResult:
        return self._api.load_beatmap(body)

    def random(self, body: RandomRequest) -> RandomResult:
        return self._api.random(body)

    def sprite_count(self) -> SpriteCountResult:
        return self._api.sprite_count()

    def storyboard(self) -> Storyboard:
        return self._api.storyboard()

    def timing_point_at(self, body: TimeRequest) -> ControlPoint:
        return self._api.timing_point_at(body)
