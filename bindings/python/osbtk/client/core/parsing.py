from __future__ import annotations

from ...api.parsing_api import ParsingAPI
from ...models.beatmap import Beatmap
from ...models.parse_osb_result import ParseOsbResult
from ...models.text_request import TextRequest
from ..options import TextOptions, to_kwargs
from typing import overload


class Parsing:
    def __init__(self, api: ParsingAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Parsing)

    @overload
    def parse_osb(self, options: TextOptions) -> ParseOsbResult: ...

    @overload
    def parse_osb(self, text: str) -> ParseOsbResult: ...

    def parse_osb(self, *args: object, **kwargs: object) -> ParseOsbResult:
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.parse_osb(TextRequest(**to_kwargs(options)))

    @overload
    def parse_osu(self, options: TextOptions) -> Beatmap: ...

    @overload
    def parse_osu(self, text: str) -> Beatmap: ...

    def parse_osu(self, *args: object, **kwargs: object) -> Beatmap:
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.parse_osu(TextRequest(**to_kwargs(options)))
