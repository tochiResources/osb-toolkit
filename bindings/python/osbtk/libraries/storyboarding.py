"""The storyboarding library: `.osu`/`.osb` parsing and the 2D command
utilities."""

from __future__ import annotations

from typing import Any, Dict, List, Optional

from osbtk.api.helpers_api import HelpersAPI as HelpersApi
from osbtk.api.origin_api import OriginAPI
from osbtk.api.parsing_api import ParsingAPI
from osbtk.client.core.origin import Origin
from osbtk.client.core.parsing import Parsing
from osbtk.libraries._client import STATELESS
from osbtk.models.helper_request import HelperRequest

_parsing = Parsing(ParsingAPI(STATELESS))
_origin = Origin(OriginAPI(STATELESS))
_helpers = HelpersApi(STATELESS)


class Storyboarding:
    """The storyboarding library: `.osu`/`.osb` parsing and the command utilities."""

    @staticmethod
    def parse_osu(text: str) -> Any:
        return _parsing.parse_osu(text)

    @staticmethod
    def parse_osb(text: str) -> Any:
        return _parsing.parse_osb(text)

    @staticmethod
    def get_origin_vector(origin: str, width: float, height: float) -> Any:
        return _origin.get_origin_vector(origin, width, height)

    @staticmethod
    def in_screen_bounds(position: Any, size: Any, rotation: float, origin: Any) -> bool:
        return _origin.in_screen_bounds(position, size, rotation, origin)

    class Commands:
        """The 2D storyboard command utilities: pure transforms over a
        ``StoryboardCommand[]``, canonical in
        ``osb_runtime::storyboarding::helpers``."""

        @staticmethod
        def call(name: str, input: Optional[Dict[str, Any]] = None) -> Any:
            """Calls any helper by name with a raw input object."""
            return _helpers.commands(
                HelperRequest(input=input if input is not None else {}, name=name)
            ).value

        @staticmethod
        def select_commands(
            commands: List[Any],
            kind: Optional[str] = None,
            start: Optional[float] = None,
            end: Optional[float] = None,
        ) -> Any:
            return Storyboarding.Commands.call(
                "selectCommands",
                {"commands": commands, "kind": kind, "start": start, "end": end},
            )

        @staticmethod
        def shift_commands(commands: List[Any], delta: float) -> Any:
            return Storyboarding.Commands.call("shiftCommands", {"commands": commands, "delta": delta})

        @staticmethod
        def scale_commands(commands: List[Any], factor: float, anchor: float = 0.0) -> Any:
            return Storyboarding.Commands.call(
                "scaleCommands", {"commands": commands, "factor": factor, "anchor": anchor}
            )

        @staticmethod
        def reverse_commands(commands: List[Any], start: float, end: float) -> Any:
            return Storyboarding.Commands.call(
                "reverseCommands", {"commands": commands, "start": start, "end": end}
            )

        @staticmethod
        def merge_commands(commands: List[Any], tolerance: float = 0.0) -> Any:
            return Storyboarding.Commands.call(
                "mergeCommands", {"commands": commands, "tolerance": tolerance}
            )

        @staticmethod
        def split_commands(commands: List[Any], at: float) -> Any:
            return Storyboarding.Commands.call("splitCommands", {"commands": commands, "at": at})

        @staticmethod
        def flatten_commands(
            commands: List[Any], kind: Optional[str] = None, mode: str = "mean"
        ) -> Any:
            return Storyboarding.Commands.call(
                "flattenCommands", {"commands": commands, "kind": kind, "mode": mode}
            )

        @staticmethod
        def randomize_commands(
            commands: List[Any], min: float = -1.0, max: float = 1.0, seed: int = 0
        ) -> Any:
            return Storyboarding.Commands.call(
                "randomizeCommands",
                {"commands": commands, "min": min, "max": max, "seed": seed},
            )

        @staticmethod
        def filter_commands(
            commands: List[Any],
            kind: Optional[str] = None,
            start: Optional[float] = None,
            end: Optional[float] = None,
        ) -> Any:
            return Storyboarding.Commands.call(
                "filterCommands",
                {"commands": commands, "kind": kind, "start": start, "end": end},
            )

        @staticmethod
        def remove_commands(
            commands: List[Any],
            kind: Optional[str] = None,
            start: Optional[float] = None,
            end: Optional[float] = None,
        ) -> Any:
            return Storyboarding.Commands.call(
                "removeCommands",
                {"commands": commands, "kind": kind, "start": start, "end": end},
            )

        @staticmethod
        def map_commands(
            commands: List[Any], value_scale: float = 1.0, value_offset: float = 0.0
        ) -> Any:
            return Storyboarding.Commands.call(
                "mapCommands",
                {"commands": commands, "valueScale": value_scale, "valueOffset": value_offset},
            )

        @staticmethod
        def duplicate_commands(commands: List[Any], offset: float = 0.0, count: int = 1) -> Any:
            return Storyboarding.Commands.call(
                "duplicateCommands", {"commands": commands, "offset": offset, "count": count}
            )

        @staticmethod
        def align_commands(
            commands: List[Any],
            beat: float = 500.0,
            offset: float = 0.0,
            subdivision: int = 1,
            value_step: float = 0.0,
        ) -> Any:
            return Storyboarding.Commands.call(
                "alignCommands",
                {
                    "commands": commands,
                    "beat": beat,
                    "offset": offset,
                    "subdivision": subdivision,
                    "valueStep": value_step,
                },
            )

        @staticmethod
        def quantize_commands(commands: List[Any], time_step: float = 0.0, value_step: float = 0.0) -> Any:
            return Storyboarding.Commands.call(
                "quantizeCommands",
                {"commands": commands, "timeStep": time_step, "valueStep": value_step},
            )
