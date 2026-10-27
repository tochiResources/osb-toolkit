"""The optimization library: storyboard optimization and `.osb` round-trip (M30)."""

from __future__ import annotations

from typing import Any

from osbtk.api.optimize_api import OptimizeAPI as OptimizeApi
from osbtk.libraries._client import STATELESS
from osbtk.models.optimize_options_input import OptimizeOptionsInput
from osbtk.models.optimize_request import OptimizeRequest

_api = OptimizeApi(STATELESS)


class Optimize:
    """The optimization library: storyboard optimization and `.osb` round-trip (M30)."""

    @staticmethod
    def optimize_osb(text: str, options: OptimizeOptionsInput) -> Any:
        return _api.optimize_osb(OptimizeRequest(text=text, options=options))
