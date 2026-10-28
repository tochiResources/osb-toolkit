from __future__ import annotations

from ...api.parsing_api import ParsingAPI
from ...models.beatmap import Beatmap
from ...models.parse_osb_result import ParseOsbResult
from ...models.text_request import TextRequest
from ..options import TextOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Parsing:
    """The clean `Parsing` client."""

    def __init__(self, api: ParsingAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Parsing)

    @overload
    def parse_osb(self, text: str) -> ParseOsbResult:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            text: The text payload.

        Returns:
            The storyboard summary
        """
        ...

    @overload
    def parse_osb(self, options: TextOptions) -> ParseOsbResult:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            options: A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            The storyboard summary
        """
        ...

    def parse_osb(self, *args: object, **kwargs: object) -> ParseOsbResult:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            The storyboard summary
        """
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.parse_osb(TextRequest(**to_kwargs(options)))

    @overload
    def parse_osu(self, text: str) -> Beatmap:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            text: The text payload.

        Returns:
            The parsed beatmap
        """
        ...

    @overload
    def parse_osu(self, options: TextOptions) -> Beatmap:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            options: A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            The parsed beatmap
        """
        ...

    def parse_osu(self, *args: object, **kwargs: object) -> Beatmap:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            The parsed beatmap
        """
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.parse_osu(TextRequest(**to_kwargs(options)))
