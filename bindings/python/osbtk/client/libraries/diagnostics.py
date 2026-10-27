from __future__ import annotations

from ...api.diagnostics_api import DiagnosticsAPI
from ...models.diagnostic_report import DiagnosticReport
from ...models.text_request import TextRequest
from ..options import TextOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Diagnostics:
    def __init__(self, api: DiagnosticsAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Diagnostics)

    @overload
    def diagnose_osb(self, text: str) -> DiagnosticReport: ...

    @overload
    def diagnose_osb(self, options: TextOptions) -> DiagnosticReport: ...

    def diagnose_osb(self, *args: object, **kwargs: object) -> DiagnosticReport:
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.diagnose_osb(TextRequest(**to_kwargs(options)))

    @overload
    def diagnose_osu(self, text: str) -> DiagnosticReport: ...

    @overload
    def diagnose_osu(self, options: TextOptions) -> DiagnosticReport: ...

    def diagnose_osu(self, *args: object, **kwargs: object) -> DiagnosticReport:
        options = args[0] if args and isinstance(args[0], TextOptions) else TextOptions(*args, **kwargs)
        return self._api.diagnose_osu(TextRequest(**to_kwargs(options)))
