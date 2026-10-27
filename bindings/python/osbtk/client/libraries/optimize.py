from __future__ import annotations

from ...api.optimize_api import OptimizeAPI
from ...models.optimize_options_input import OptimizeOptionsInput
from ...models.optimize_request import OptimizeRequest
from ...models.optimize_result import OptimizeResult
from ..options import OptimizeOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Optimize:
    def __init__(self, api: OptimizeAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Optimize)

    @overload
    def optimize_osb(self, options: OptimizeOptionsInput, text: str) -> OptimizeResult: ...

    @overload
    def optimize_osb(self, options: OptimizeOptions) -> OptimizeResult: ...

    def optimize_osb(self, *args: object, **kwargs: object) -> OptimizeResult:
        options = args[0] if args and isinstance(args[0], OptimizeOptions) else OptimizeOptions(*args, **kwargs)
        return self._api.optimize_osb(OptimizeRequest(**to_kwargs(options)))
