from __future__ import annotations

from ...api.sprite_api import SpriteAPI
from ...models.command_request import CommandRequest
from ...models.easing import Easing
from ...models.handle_request import HandleRequest
from ...models.parameter_type import ParameterType
from ...models.sprite_info import SpriteInfo
from ...models.sprite_state import SpriteState
from ...models.sprite_time_request import SpriteTimeRequest
from ...models.start_loop_group_request import StartLoopGroupRequest
from ...models.start_trigger_group_request import StartTriggerGroupRequest
from ..options import CommandOptions, SpriteTimeOptions, StartLoopGroupOptions, StartTriggerGroupOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Sprite:
    """The clean `Sprite` client."""

    def __init__(self, api: SpriteAPI, layer: int, sprite: int) -> None:
        self._api = api
        self._layer = layer
        self._sprite = sprite

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Sprite)

    @overload
    def command(self, kind: str, startTime: float, endTime: float, easing: Easing) -> None:
        """A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.

        Args:
            kind: The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
            startTime: The command start time in milliseconds.
            endTime: The command end time in milliseconds.
            easing: The easing function.

        Returns:
            Acknowledged
        """
        ...

    @overload
    def command(self, kind: str, startTime: float, endTime: float, easing: Easing, start: Optional[Dict[str, Any]]) -> None:
        """A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.

        Args:
            kind: The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
            startTime: The command start time in milliseconds.
            endTime: The command end time in milliseconds.
            easing: The easing function.
            start: The start value (scalar, point or color).

        Returns:
            Acknowledged
        """
        ...

    @overload
    def command(self, kind: str, startTime: float, endTime: float, easing: Easing, start: Optional[Dict[str, Any]], end: Optional[Dict[str, Any]]) -> None:
        """A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.

        Args:
            kind: The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
            startTime: The command start time in milliseconds.
            endTime: The command end time in milliseconds.
            easing: The easing function.
            start: The start value (scalar, point or color).
            end: The end value (scalar, point or color).

        Returns:
            Acknowledged
        """
        ...

    @overload
    def command(self, kind: str, startTime: float, endTime: float, easing: Easing, start: Optional[Dict[str, Any]], end: Optional[Dict[str, Any]], parameter: Optional[ParameterType]) -> None:
        """A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.

        Args:
            kind: The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
            startTime: The command start time in milliseconds.
            endTime: The command end time in milliseconds.
            easing: The easing function.
            start: The start value (scalar, point or color).
            end: The end value (scalar, point or color).
            parameter: The parameter kind for a `parameter` command.

        Returns:
            Acknowledged
        """
        ...

    @overload
    def command(self, options: CommandOptions) -> None:
        """A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.

        Args:
            options: A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.

        Returns:
            Acknowledged
        """
        ...

    def command(self, *args: object, **kwargs: object) -> None:
        """A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.

        Returns:
            Acknowledged
        """
        options = args[0] if args and isinstance(args[0], CommandOptions) else CommandOptions(*args, **kwargs)
        self._api.command(CommandRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))

    def end_group(self) -> None:
        """A sprite handle pair.

        Returns:
            Acknowledged
        """
        self._api.end_group(HandleRequest(layer=self._layer, sprite=self._sprite))

    def info(self) -> SpriteInfo:
        """A sprite handle pair.

        Returns:
            The sprite's initial state
        """
        return self._api.info(HandleRequest(layer=self._layer, sprite=self._sprite))

    @overload
    def start_loop_group(self, startTime: float, count: float) -> None:
        """Start a loop group.

        Args:
            startTime: The loop start time in milliseconds.
            count: The number of iterations.

        Returns:
            Acknowledged
        """
        ...

    @overload
    def start_loop_group(self, options: StartLoopGroupOptions) -> None:
        """Start a loop group.

        Args:
            options: Start a loop group.

        Returns:
            Acknowledged
        """
        ...

    def start_loop_group(self, *args: object, **kwargs: object) -> None:
        """Start a loop group.

        Returns:
            Acknowledged
        """
        options = args[0] if args and isinstance(args[0], StartLoopGroupOptions) else StartLoopGroupOptions(*args, **kwargs)
        self._api.start_loop_group(StartLoopGroupRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))

    @overload
    def start_trigger_group(self, name: str, startTime: float, endTime: float, group: int) -> None:
        """Start a trigger group.

        Args:
            name: The trigger name (matches a hit object).
            startTime: The trigger start time in milliseconds.
            endTime: The trigger end time in milliseconds.
            group: The trigger group index.

        Returns:
            Acknowledged
        """
        ...

    @overload
    def start_trigger_group(self, options: StartTriggerGroupOptions) -> None:
        """Start a trigger group.

        Args:
            options: Start a trigger group.

        Returns:
            Acknowledged
        """
        ...

    def start_trigger_group(self, *args: object, **kwargs: object) -> None:
        """Start a trigger group.

        Returns:
            Acknowledged
        """
        options = args[0] if args and isinstance(args[0], StartTriggerGroupOptions) else StartTriggerGroupOptions(*args, **kwargs)
        self._api.start_trigger_group(StartTriggerGroupRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))

    @overload
    def state_at(self, time: float) -> SpriteState:
        """A sprite handle pair plus a sample time.

        Args:
            time: The sample time in milliseconds.

        Returns:
            The sampled sprite state
        """
        ...

    @overload
    def state_at(self, options: SpriteTimeOptions) -> SpriteState:
        """A sprite handle pair plus a sample time.

        Args:
            options: A sprite handle pair plus a sample time.

        Returns:
            The sampled sprite state
        """
        ...

    def state_at(self, *args: object, **kwargs: object) -> SpriteState:
        """A sprite handle pair plus a sample time.

        Returns:
            The sampled sprite state
        """
        options = args[0] if args and isinstance(args[0], SpriteTimeOptions) else SpriteTimeOptions(*args, **kwargs)
        return self._api.state_at(SpriteTimeRequest(layer=self._layer, sprite=self._sprite, **to_kwargs(options)))
