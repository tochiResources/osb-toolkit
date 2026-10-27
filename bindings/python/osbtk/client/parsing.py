from __future__ import annotations

from ..api.parsing_api import ParsingAPI
from ..models.beatmap import Beatmap
from ..models.parse_osb_result import ParseOsbResult
from ..models.text_request import TextRequest


class Parsing:
    def __init__(self, api: ParsingAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Parsing)

    def parse_osb(self, body: TextRequest) -> ParseOsbResult:
        return self._api.parse_osb(body)

    def parse_osu(self, body: TextRequest) -> Beatmap:
        return self._api.parse_osu(body)
