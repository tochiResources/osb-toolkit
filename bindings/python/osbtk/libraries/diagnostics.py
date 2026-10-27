"""The diagnostics library: storyboard and mapset analysis (M29)."""

from __future__ import annotations

from typing import Any

from osbtk.api.diagnostics_api import DiagnosticsAPI as DiagnosticsApi
from osbtk.libraries._client import STATELESS
from osbtk.models.text_request import TextRequest

_api = DiagnosticsApi(STATELESS)


class Diagnostics:
    """The diagnostics library: storyboard and mapset analysis (M29)."""

    @staticmethod
    def diagnose_osb(text: str) -> Any:
        return _api.diagnose_osb(TextRequest(text=text))

    @staticmethod
    def diagnose_osu(text: str) -> Any:
        return _api.diagnose_osu(TextRequest(text=text))
