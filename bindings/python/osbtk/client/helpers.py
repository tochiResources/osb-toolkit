from __future__ import annotations

from ..api.helpers_api import HelpersAPI
from ..models.helper_request import HelperRequest
from ..models.helper_result import HelperResult
from .options import HelperOptions, to_kwargs


class Helpers:
    def __init__(self, api: HelpersAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Helpers)

    def call(self, options: HelperOptions) -> HelperResult:
        return self._api.call(HelperRequest(**to_kwargs(options)))

    def commands(self, options: HelperOptions) -> HelperResult:
        return self._api.commands(HelperRequest(**to_kwargs(options)))

    def keyframes(self, options: HelperOptions) -> HelperResult:
        return self._api.keyframes(HelperRequest(**to_kwargs(options)))
