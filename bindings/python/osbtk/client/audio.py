from __future__ import annotations

from ..api.audio_api import AudioAPI
from ..models.audio_duration_result import AudioDurationResult
from ..models.fft_request import FftRequest
from ..models.fft_result import FftResult
from ..models.get_fft_request import GetFftRequest
from .options import FftOptions, GetFftOptions, to_kwargs


class Audio:
    def __init__(self, api: AudioAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Audio)

    def audio_duration(self) -> AudioDurationResult:
        return self._api.audio_duration()

    def fft(self, options: FftOptions) -> FftResult:
        return self._api.fft(FftRequest(**to_kwargs(options)))

    def get_fft(self, options: GetFftOptions) -> FftResult:
        return self._api.get_fft(GetFftRequest(**to_kwargs(options)))
