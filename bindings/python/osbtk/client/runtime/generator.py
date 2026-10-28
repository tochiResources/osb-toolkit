from __future__ import annotations

from ...api.generator_api import GeneratorAPI
from ...models.beatmap import Beatmap
from ...models.control_point import ControlPoint
from ...models.layer_handle_request import LayerHandleRequest
from ...models.load_beatmap_result import LoadBeatmapResult
from ...models.random_request import RandomRequest
from ...models.storyboard import Storyboard
from ...models.text_request import TextRequest
from ...models.time_request import TimeRequest
from ..options import LayerHandleOptions, RandomOptions, TextOptions, TimeOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Generator:
    """The clean `Generator` client."""

    def __init__(self, api: GeneratorAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Generator)

    def beatmap(self) -> Beatmap:
        """Calls the Generator.beatmap bridge operation.

        Returns:
            The loaded beatmap, or null
        """
        return self._api.beatmap()

    @overload
    def control_point_at(self, time: float) -> ControlPoint:
        """A time query.

        Args:
            time: The time in milliseconds.

        Returns:
            The active control point, or null
        """
        ...

    @overload
    def control_point_at(self, options: TimeOptions) -> ControlPoint:
        """A time query.

        Args:
            options: A time query.

        Returns:
            The active control point, or null
        """
        ...

    def control_point_at(self, *args: object, **kwargs: object) -> ControlPoint:
        """A time query.

        Returns:
            The active control point, or null
        """
        options = args[0] if args and isinstance(args[0], TimeOptions) else TimeOptions(*args, **kwargs)
        return self._api.control_point_at(TimeRequest(**to_kwargs(options)))

    @overload
    def import_osb(self, text: str) -> int:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            text: The text payload.

        Returns:
            The sprite count after import
        """
        ...

    @overload
    def import_osb(self, options: TextOptions) -> int:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            options: A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            The sprite count after import
        """
        ...

    def import_osb(self, *args: object, **kwargs: object) -> int:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            The sprite count after import
        """
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.import_osb(TextRequest(**to_kwargs(options))).sprite_count

    @overload
    def layer_handle(self, name: str) -> int:
        """Resolve (or create) a layer by name.

        Args:
            name: The layer name.

        Returns:
            The layer handle
        """
        ...

    @overload
    def layer_handle(self, options: LayerHandleOptions) -> int:
        """Resolve (or create) a layer by name.

        Args:
            options: Resolve (or create) a layer by name.

        Returns:
            The layer handle
        """
        ...

    def layer_handle(self, *args: object, **kwargs: object) -> int:
        """Resolve (or create) a layer by name.

        Returns:
            The layer handle
        """
        options = args[0] if args and isinstance(args[0], LayerHandleOptions) else LayerHandleOptions(*args, **kwargs)
        return self._api.layer_handle(LayerHandleRequest(**to_kwargs(options))).layer

    @overload
    def load_beatmap(self, text: str) -> LoadBeatmapResult:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            text: The text payload.

        Returns:
            A summary of the loaded beatmap
        """
        ...

    @overload
    def load_beatmap(self, options: TextOptions) -> LoadBeatmapResult:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Args:
            options: A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            A summary of the loaded beatmap
        """
        ...

    def load_beatmap(self, *args: object, **kwargs: object) -> LoadBeatmapResult:
        """A request carrying a block of text (an effect script, `.osu` or `.osb`).

        Returns:
            A summary of the loaded beatmap
        """
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.load_beatmap(TextRequest(**to_kwargs(options)))

    @overload
    def random(self, max: float, min: float) -> float:
        """A deterministic random request.

        Args:
            max: The exclusive upper bound.
            min: The inclusive lower bound.

        Returns:
            A deterministic random value
        """
        ...

    @overload
    def random(self, options: RandomOptions) -> float:
        """A deterministic random request.

        Args:
            options: A deterministic random request.

        Returns:
            A deterministic random value
        """
        ...

    def random(self, *args: object, **kwargs: object) -> float:
        """A deterministic random request.

        Returns:
            A deterministic random value
        """
        options = args[0] if args and isinstance(args[0], RandomOptions) else RandomOptions(*args, **kwargs)
        return self._api.random(RandomRequest(**to_kwargs(options))).value

    def sprite_count(self) -> int:
        """Calls the Generator.spriteCount bridge operation.

        Returns:
            The storyboard sprite count
        """
        return self._api.sprite_count().sprite_count

    def storyboard(self) -> Storyboard:
        """Calls the Generator.storyboard bridge operation.

        Returns:
            The built storyboard model
        """
        return self._api.storyboard()

    @overload
    def timing_point_at(self, time: float) -> ControlPoint:
        """A time query.

        Args:
            time: The time in milliseconds.

        Returns:
            The active red timing point
        """
        ...

    @overload
    def timing_point_at(self, options: TimeOptions) -> ControlPoint:
        """A time query.

        Args:
            options: A time query.

        Returns:
            The active red timing point
        """
        ...

    def timing_point_at(self, *args: object, **kwargs: object) -> ControlPoint:
        """A time query.

        Returns:
            The active red timing point
        """
        options = args[0] if args and isinstance(args[0], TimeOptions) else TimeOptions(*args, **kwargs)
        return self._api.timing_point_at(TimeRequest(**to_kwargs(options)))
