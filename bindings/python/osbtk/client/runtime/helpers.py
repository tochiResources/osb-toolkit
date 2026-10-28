from __future__ import annotations

from ...api.helpers_api import HelpersAPI
from ...models.helper_request import HelperRequest
from ..options import HelperOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Helpers:
    """The clean `Helpers` client."""

    def __init__(self, api: HelpersAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Helpers)

    @overload
    def call(self, name: str) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Args:
            name: The helper name, e.g. `flatten`, `orbit`, `selectCommands`.

        Returns:
            The shared motion-graphics helper's result
        """
        ...

    @overload
    def call(self, name: str, input: Optional[str]) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Args:
            name: The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
            input: The helper's input (a keyframe channel, a command list, or its parameters).

        Returns:
            The shared motion-graphics helper's result
        """
        ...

    @overload
    def call(self, options: HelperOptions) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Args:
            options: A motion-graphics helper call: a helper `name` plus its input.

        Returns:
            The shared motion-graphics helper's result
        """
        ...

    def call(self, *args: object, **kwargs: object) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Returns:
            The shared motion-graphics helper's result
        """
        options = args[0] if args and isinstance(args[0], HelperOptions) else HelperOptions(*args, **kwargs)
        return self._api.call(HelperRequest(**to_kwargs(options))).value

    @overload
    def commands(self, name: str) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Args:
            name: The helper name, e.g. `flatten`, `orbit`, `selectCommands`.

        Returns:
            The helper's result (a command list or scalar)
        """
        ...

    @overload
    def commands(self, name: str, input: Optional[str]) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Args:
            name: The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
            input: The helper's input (a keyframe channel, a command list, or its parameters).

        Returns:
            The helper's result (a command list or scalar)
        """
        ...

    @overload
    def commands(self, options: HelperOptions) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Args:
            options: A motion-graphics helper call: a helper `name` plus its input.

        Returns:
            The helper's result (a command list or scalar)
        """
        ...

    def commands(self, *args: object, **kwargs: object) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Returns:
            The helper's result (a command list or scalar)
        """
        options = args[0] if args and isinstance(args[0], HelperOptions) else HelperOptions(*args, **kwargs)
        return self._api.commands(HelperRequest(**to_kwargs(options))).value

    @overload
    def keyframes(self, name: str) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Args:
            name: The helper name, e.g. `flatten`, `orbit`, `selectCommands`.

        Returns:
            The helper's result (a keyframe channel or vectors)
        """
        ...

    @overload
    def keyframes(self, name: str, input: Optional[str]) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Args:
            name: The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
            input: The helper's input (a keyframe channel, a command list, or its parameters).

        Returns:
            The helper's result (a keyframe channel or vectors)
        """
        ...

    @overload
    def keyframes(self, options: HelperOptions) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Args:
            options: A motion-graphics helper call: a helper `name` plus its input.

        Returns:
            The helper's result (a keyframe channel or vectors)
        """
        ...

    def keyframes(self, *args: object, **kwargs: object) -> Any:
        """A motion-graphics helper call: a helper `name` plus its input.

        Returns:
            The helper's result (a keyframe channel or vectors)
        """
        options = args[0] if args and isinstance(args[0], HelperOptions) else HelperOptions(*args, **kwargs)
        return self._api.keyframes(HelperRequest(**to_kwargs(options))).value
