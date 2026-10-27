"""The shared ``call`` entry point and command-channel bridging."""

from __future__ import annotations

from typing import Any, Dict, List, Optional

from osbtk.api.helpers_api import HelpersAPI
from osbtk.libraries._client import STATELESS
from osbtk.models.helper_request import HelperRequest

_api = HelpersAPI(STATELESS)


def call(name: str, input: Optional[Dict[str, Any]] = None) -> Any:
    """Calls any shared helper by name with a raw input object."""
    return _api.call(HelperRequest(input=input if input is not None else {}, name=name)).value


def transform_commands(
    commands: List[Any],
    kind: str,
    name: str,
    options: Optional[Dict[str, Any]] = None,
) -> Any:
    """Applies any keyframe transform (e.g. ``keyframe.flatten``) to a 2D command
    channel: extract the ``kind`` channel, apply ``name`` with ``options``, and
    write it back as commands."""
    payload: Dict[str, Any] = dict(options or {})
    payload["commands"] = commands
    payload["kind"] = kind
    return call(name, payload)


class _Base:
    """The shared entry point and the command-channel bridge."""

    call = staticmethod(call)
    transform_commands = staticmethod(transform_commands)
