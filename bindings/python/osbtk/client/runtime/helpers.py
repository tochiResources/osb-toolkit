from __future__ import annotations

from ...api.helpers_api import HelpersAPI
from ...models.helper_request import HelperRequest
from ..options import HelperOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Helpers:
    def __init__(self, api: HelpersAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Helpers)

    @overload
    def call(self, name: str) -> Any: ...

    @overload
    def call(self, name: str, input: Optional[str]) -> Any: ...

    @overload
    def call(self, options: HelperOptions) -> Any: ...

    def call(self, *args: object, **kwargs: object) -> Any:
        options = args[0] if args and isinstance(args[0], HelperOptions) else HelperOptions(*args, **kwargs)
        return self._api.call(HelperRequest(**to_kwargs(options))).value

    @overload
    def commands(self, name: str) -> Any: ...

    @overload
    def commands(self, name: str, input: Optional[str]) -> Any: ...

    @overload
    def commands(self, options: HelperOptions) -> Any: ...

    def commands(self, *args: object, **kwargs: object) -> Any:
        options = args[0] if args and isinstance(args[0], HelperOptions) else HelperOptions(*args, **kwargs)
        return self._api.commands(HelperRequest(**to_kwargs(options))).value

    @overload
    def keyframes(self, name: str) -> Any: ...

    @overload
    def keyframes(self, name: str, input: Optional[str]) -> Any: ...

    @overload
    def keyframes(self, options: HelperOptions) -> Any: ...

    def keyframes(self, *args: object, **kwargs: object) -> Any:
        options = args[0] if args and isinstance(args[0], HelperOptions) else HelperOptions(*args, **kwargs)
        return self._api.keyframes(HelperRequest(**to_kwargs(options))).value
