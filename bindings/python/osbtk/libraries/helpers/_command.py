"""The 2D command-list utilities."""

from __future__ import annotations

from typing import Any, List

from osbtk.libraries.helpers._base import call


class _Command:
    """The 2D command-list utilities."""

    @staticmethod
    def set_easing(commands: List[Any], easing: str) -> Any:
        """Sets an easing on every command (apply an easing to a whole group/scene)."""
        return call("command.setEasing", {"commands": commands, "easing": easing})

    @staticmethod
    def apply_easing(commands: List[Any], kind: str, easing: str, samples: int = 8) -> Any:
        """Re-eases a command channel with a named easing, resolving it into commands."""
        return call(
            "command.applyEasing",
            {"commands": commands, "kind": kind, "easing": easing, "samples": samples},
        )
