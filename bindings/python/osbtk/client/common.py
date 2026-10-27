from __future__ import annotations

from ..api.common_api import CommonAPI
from ..models.version_result import VersionResult


class Common:
    def __init__(self, api: CommonAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Common)

    def version(self) -> VersionResult:
        return self._api.version()
