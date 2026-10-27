from __future__ import annotations

from ..api.parsing_api import ParsingAPI
from ..models.beatmap import Beatmap
from ..models.parse_osb_result import ParseOsbResult
from ..models.text_request import TextRequest
from .options import TextOptions, to_kwargs


class Parsing:
    def __init__(self, api: ParsingAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Parsing)

    def parse_osb(self, options: TextOptions) -> ParseOsbResult:
        return self._api.parse_osb(TextRequest(**to_kwargs(options)))

    def parse_osu(self, options: TextOptions) -> Beatmap:
        return self._api.parse_osu(TextRequest(**to_kwargs(options)))
