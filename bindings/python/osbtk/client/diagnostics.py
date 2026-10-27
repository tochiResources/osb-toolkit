from __future__ import annotations

from ..api.diagnostics_api import DiagnosticsAPI
from ..models.diagnostic_report import DiagnosticReport
from ..models.text_request import TextRequest


class Diagnostics:
    def __init__(self, api: DiagnosticsAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Diagnostics)

    def diagnose_osb(self, body: TextRequest) -> DiagnosticReport:
        return self._api.diagnose_osb(body)

    def diagnose_osu(self, body: TextRequest) -> DiagnosticReport:
        return self._api.diagnose_osu(body)
