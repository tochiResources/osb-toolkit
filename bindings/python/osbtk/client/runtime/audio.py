from __future__ import annotations

from ...api.audio_api import AudioAPI
from ...models.easing import Easing
from ...models.fft_request import FftRequest
from ...models.fft_result import FftResult
from ...models.get_fft_request import GetFftRequest
from ..options import FftOptions, GetFftOptions, to_kwargs
from typing import Any, Optional
from typing import overload


class Audio:
    def __init__(self, api: AudioAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Audio)

    def audio_duration(self) -> float:
        return self._api.audio_duration().audio_duration

    @overload
    def fft(self, options: FftOptions) -> FftResult: ...

    @overload
    def fft(self, path: str, magnitudes: Optional[int] = None, time: Optional[float] = None) -> FftResult: ...

    def fft(self, *args: object, **kwargs: object) -> FftResult:
        options = args[0] if args and isinstance(args[0], FftOptions) else FftOptions(*args, **kwargs)
        return self._api.fft(FftRequest(**to_kwargs(options)))

    @overload
    def get_fft(self, options: GetFftOptions) -> FftResult: ...

    @overload
    def get_fft(self, time: float, magnitudes: int, cutoff: Optional[float] = None, easing: Optional[Easing] = None) -> FftResult: ...

    def get_fft(self, *args: object, **kwargs: object) -> FftResult:
        options = args[0] if args and isinstance(args[0], GetFftOptions) else GetFftOptions(*args, **kwargs)
        return self._api.get_fft(GetFftRequest(**to_kwargs(options)))
