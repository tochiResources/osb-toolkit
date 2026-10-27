from __future__ import annotations

from ...api.sprite_api import SpriteAPI
from ...models.command_request import CommandRequest
from ...models.easing import Easing
from ...models.handle_request import HandleRequest
from ...models.parameter_type import ParameterType
from ...models.sprite_info import SpriteInfo
from ...models.sprite_state import SpriteState
from ...models.sprite_time_request import SpriteTimeRequest
from ...models.start_loop_group_request import StartLoopGroupRequest
from ...models.start_trigger_group_request import StartTriggerGroupRequest
from ..options import CommandOptions, SpriteTimeOptions, StartLoopGroupOptions, StartTriggerGroupOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Sprite:
    def __init__(self, api: SpriteAPI, layer: int, sprite: int) -> None:
        self._api = api
        self._layer = layer
        self._sprite = sprite

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Sprite)

    @overload
    def command(self, kind: str, startTime: float, endTime: float, easing: Easing) -> None: ...

    @overload
    def command(self, kind: str, startTime: float, endTime: float, easing: Easing, start: Optional[Dict[str, Any]]) -> None: ...

    @overload
    def command(self, kind: str, startTime: float, endTime: float, easing: Easing, start: Optional[Dict[str, Any]], end: Optional[Dict[str, Any]]) -> None: ...

    @overload
    def command(self, kind: str, startTime: float, endTime: float, easing: Easing, start: Optional[Dict[str, Any]], end: Optional[Dict[str, Any]], parameter: Optional[ParameterType]) -> None: ...

    @overload
    def command(self, options: CommandOptions) -> None: ...

    def command(self, *args: object, **kwargs: object) -> None:
        options = args[0] if args and isinstance(args[0], CommandOptions) else CommandOptions(*args, **kwargs)
        self._api.command(CommandRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))

    def end_group(self) -> None:
        self._api.end_group(HandleRequest(layer=self._layer, sprite=self._sprite))

    def info(self) -> SpriteInfo:
        return self._api.info(HandleRequest(layer=self._layer, sprite=self._sprite))

    @overload
    def start_loop_group(self, startTime: float, count: float) -> None: ...

    @overload
    def start_loop_group(self, options: StartLoopGroupOptions) -> None: ...

    def start_loop_group(self, *args: object, **kwargs: object) -> None:
        options = args[0] if args and isinstance(args[0], StartLoopGroupOptions) else StartLoopGroupOptions(*args, **kwargs)
        self._api.start_loop_group(StartLoopGroupRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))

    @overload
    def start_trigger_group(self, name: str, startTime: float, endTime: float, group: int) -> None: ...

    @overload
    def start_trigger_group(self, options: StartTriggerGroupOptions) -> None: ...

    def start_trigger_group(self, *args: object, **kwargs: object) -> None:
        options = args[0] if args and isinstance(args[0], StartTriggerGroupOptions) else StartTriggerGroupOptions(*args, **kwargs)
        self._api.start_trigger_group(StartTriggerGroupRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))

    @overload
    def state_at(self, time: float) -> SpriteState: ...

    @overload
    def state_at(self, options: SpriteTimeOptions) -> SpriteState: ...

    def state_at(self, *args: object, **kwargs: object) -> SpriteState:
        options = args[0] if args and isinstance(args[0], SpriteTimeOptions) else SpriteTimeOptions(*args, **kwargs)
        return self._api.state_at(SpriteTimeRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))
