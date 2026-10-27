from __future__ import annotations

from ...api.optimize_api import OptimizeAPI
from ...models.optimize_request import OptimizeRequest
from ...models.optimize_result import OptimizeResult
from ..options import OptimizeOptions, to_kwargs


class Optimize:
    def __init__(self, api: OptimizeAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Optimize)

    def optimize_osb(self, options: OptimizeOptions) -> OptimizeResult:
        return self._api.optimize_osb(OptimizeRequest(**to_kwargs(options)))
