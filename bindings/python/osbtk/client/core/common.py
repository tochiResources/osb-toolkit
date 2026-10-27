from __future__ import annotations

from ...api.common_api import CommonAPI
from typing import Any, Optional


class Common:
    def __init__(self, api: CommonAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Common)

    def version(self) -> str:
        return self._api.version().version
