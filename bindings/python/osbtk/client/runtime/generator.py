from __future__ import annotations

from ...api.generator_api import GeneratorAPI
from ...models.beatmap import Beatmap
from ...models.control_point import ControlPoint
from ...models.layer_handle_request import LayerHandleRequest
from ...models.load_beatmap_result import LoadBeatmapResult
from ...models.random_request import RandomRequest
from ...models.storyboard import Storyboard
from ...models.text_request import TextRequest
from ...models.time_request import TimeRequest
from ..options import LayerHandleOptions, RandomOptions, TextOptions, TimeOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Generator:
    def __init__(self, api: GeneratorAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Generator)

    def beatmap(self) -> Beatmap:
        return self._api.beatmap()

    @overload
    def control_point_at(self, time: float) -> ControlPoint: ...

    @overload
    def control_point_at(self, options: TimeOptions) -> ControlPoint: ...

    def control_point_at(self, *args: object, **kwargs: object) -> ControlPoint:
        options = args[0] if args and isinstance(args[0], TimeOptions) else TimeOptions(*args, **kwargs)
        return self._api.control_point_at(TimeRequest(**to_kwargs(options)))

    @overload
    def import_osb(self, text: str) -> int: ...

    @overload
    def import_osb(self, options: TextOptions) -> int: ...

    def import_osb(self, *args: object, **kwargs: object) -> int:
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.import_osb(TextRequest(**to_kwargs(options))).sprite_count

    @overload
    def layer_handle(self, name: str) -> int: ...

    @overload
    def layer_handle(self, options: LayerHandleOptions) -> int: ...

    def layer_handle(self, *args: object, **kwargs: object) -> int:
        options = args[0] if args and isinstance(args[0], LayerHandleOptions) else LayerHandleOptions(*args, **kwargs)
        return self._api.layer_handle(LayerHandleRequest(**to_kwargs(options))).layer

    @overload
    def load_beatmap(self, text: str) -> LoadBeatmapResult: ...

    @overload
    def load_beatmap(self, options: TextOptions) -> LoadBeatmapResult: ...

    def load_beatmap(self, *args: object, **kwargs: object) -> LoadBeatmapResult:
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.load_beatmap(TextRequest(**to_kwargs(options)))

    @overload
    def random(self, max: float, min: float) -> float: ...

    @overload
    def random(self, options: RandomOptions) -> float: ...

    def random(self, *args: object, **kwargs: object) -> float:
        options = args[0] if args and isinstance(args[0], RandomOptions) else RandomOptions(*args, **kwargs)
        return self._api.random(RandomRequest(**to_kwargs(options))).value

    def sprite_count(self) -> int:
        return self._api.sprite_count().sprite_count

    def storyboard(self) -> Storyboard:
        return self._api.storyboard()

    @overload
    def timing_point_at(self, time: float) -> ControlPoint: ...

    @overload
    def timing_point_at(self, options: TimeOptions) -> ControlPoint: ...

    def timing_point_at(self, *args: object, **kwargs: object) -> ControlPoint:
        options = args[0] if args and isinstance(args[0], TimeOptions) else TimeOptions(*args, **kwargs)
        return self._api.timing_point_at(TimeRequest(**to_kwargs(options)))
