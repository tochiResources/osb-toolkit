"""The audio library: FFT."""

from __future__ import annotations

from typing import Any

from osbtk.api.audio_api import AudioAPI as AudioApi
from osbtk.libraries._client import STATELESS
from osbtk.models.fft_request import FftRequest

_api = AudioApi(STATELESS)


class Audio:
    """The audio library: FFT."""

    @staticmethod
    def fft(request: FftRequest) -> Any:
        return _api.fft(request)
