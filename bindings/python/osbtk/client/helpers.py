from __future__ import annotations

from ..api.helpers_api import HelpersAPI
from ..models.helper_request import HelperRequest
from ..models.helper_result import HelperResult


class Helpers:
    def __init__(self, api: HelpersAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Helpers)

    def call(self, body: HelperRequest) -> HelperResult:
        return self._api.call(body)

    def commands(self, body: HelperRequest) -> HelperResult:
        return self._api.commands(body)

    def keyframes(self, body: HelperRequest) -> HelperResult:
        return self._api.keyframes(body)
