from __future__ import annotations

from ...api.sprite_api import SpriteAPI
from ...models.command_request import CommandRequest
from ...models.handle_request import HandleRequest
from ...models.ok_result import OkResult
from ...models.sprite_info import SpriteInfo
from ...models.sprite_state import SpriteState
from ...models.sprite_time_request import SpriteTimeRequest
from ...models.start_loop_group_request import StartLoopGroupRequest
from ...models.start_trigger_group_request import StartTriggerGroupRequest
from ..options import CommandOptions, SpriteTimeOptions, StartLoopGroupOptions, StartTriggerGroupOptions, to_kwargs


class Sprite:
    def __init__(self, api: SpriteAPI, layer: int, sprite: int) -> None:
        self._api = api
        self._layer = layer
        self._sprite = sprite

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Sprite)

    def command(self, options: CommandOptions) -> OkResult:
        return self._api.command(CommandRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))

    def end_group(self) -> OkResult:
        return self._api.end_group(HandleRequest(layer=self._layer, sprite=self._sprite))

    def info(self) -> SpriteInfo:
        return self._api.info(HandleRequest(layer=self._layer, sprite=self._sprite))

    def start_loop_group(self, options: StartLoopGroupOptions) -> OkResult:
        return self._api.start_loop_group(StartLoopGroupRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))

    def start_trigger_group(self, options: StartTriggerGroupOptions) -> OkResult:
        return self._api.start_trigger_group(StartTriggerGroupRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))

    def state_at(self, options: SpriteTimeOptions) -> SpriteState:
        return self._api.state_at(SpriteTimeRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))
