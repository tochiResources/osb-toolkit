from __future__ import annotations

from ..api.sprite_api import SpriteAPI
from ..models.command_request import CommandRequest
from ..models.handle_request import HandleRequest
from ..models.ok_result import OkResult
from ..models.sprite_info import SpriteInfo
from ..models.sprite_state import SpriteState
from ..models.sprite_time_request import SpriteTimeRequest
from ..models.start_loop_group_request import StartLoopGroupRequest
from ..models.start_trigger_group_request import StartTriggerGroupRequest


class Sprite:
    def __init__(self, api: SpriteAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Sprite)

    def command(self, body: CommandRequest) -> OkResult:
        return self._api.command(body)

    def end_group(self, body: HandleRequest) -> OkResult:
        return self._api.end_group(body)

    def info(self, body: HandleRequest) -> SpriteInfo:
        return self._api.info(body)

    def start_loop_group(self, body: StartLoopGroupRequest) -> OkResult:
        return self._api.start_loop_group(body)

    def start_trigger_group(self, body: StartTriggerGroupRequest) -> OkResult:
        return self._api.start_trigger_group(body)

    def state_at(self, body: SpriteTimeRequest) -> SpriteState:
        return self._api.state_at(body)
