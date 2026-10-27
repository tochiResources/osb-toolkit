from __future__ import annotations

from ..api.diagnostics_api import DiagnosticsAPI
from ..models.diagnostic_report import DiagnosticReport
from ..models.text_request import TextRequest
from .options import TextOptions, to_kwargs


class Diagnostics:
    def __init__(self, api: DiagnosticsAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Diagnostics)

    def diagnose_osb(self, options: TextOptions) -> DiagnosticReport:
        return self._api.diagnose_osb(TextRequest(**to_kwargs(options)))

    def diagnose_osu(self, options: TextOptions) -> DiagnosticReport:
        return self._api.diagnose_osu(TextRequest(**to_kwargs(options)))
