from __future__ import annotations

from ...api.audio_api import AudioAPI
from ...models.easing import Easing
from ...models.fft_request import FftRequest
from ...models.fft_result import FftResult
from ...models.get_fft_request import GetFftRequest
from ..options import FftOptions, GetFftOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Audio:
    def __init__(self, api: AudioAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Audio)

    def audio_duration(self) -> float:
        """Calls the Audio.audioDuration bridge operation.

        Returns:
            The loaded audio duration
        """
        return self._api.audio_duration().audio_duration

    @overload
    def fft(self, path: str) -> FftResult:
        """An FFT request against a `.wav` file.

        Args:
            path: Path to the audio file.

        Returns:
            The FFT magnitudes
        """
        ...

    @overload
    def fft(self, path: str, magnitudes: Optional[int]) -> FftResult:
        """An FFT request against a `.wav` file.

        Args:
            path: Path to the audio file.
            magnitudes: Number of frequency bins.

        Returns:
            The FFT magnitudes
        """
        ...

    @overload
    def fft(self, path: str, magnitudes: Optional[int], time: Optional[float]) -> FftResult:
        """An FFT request against a `.wav` file.

        Args:
            path: Path to the audio file.
            magnitudes: Number of frequency bins.
            time: Sample time in milliseconds.

        Returns:
            The FFT magnitudes
        """
        ...

    @overload
    def fft(self, options: FftOptions) -> FftResult:
        """An FFT request against a `.wav` file.

        Args:
            options: An FFT request against a `.wav` file.

        Returns:
            The FFT magnitudes
        """
        ...

    def fft(self, *args: object, **kwargs: object) -> FftResult:
        """An FFT request against a `.wav` file.

        Returns:
            The FFT magnitudes
        """
        options = args[0] if args and isinstance(args[0], FftOptions) else FftOptions(*args, **kwargs)
        return self._api.fft(FftRequest(**to_kwargs(options)))

    @overload
    def get_fft(self, time: float, magnitudes: int) -> FftResult:
        """An FFT request against the runtime's loaded audio.

        Args:
            time: Sample time in milliseconds.
            magnitudes: Number of frequency bins.

        Returns:
            The FFT magnitudes
        """
        ...

    @overload
    def get_fft(self, time: float, magnitudes: int, cutoff: Optional[float]) -> FftResult:
        """An FFT request against the runtime's loaded audio.

        Args:
            time: Sample time in milliseconds.
            magnitudes: Number of frequency bins.
            cutoff: A frequency cut-off.

        Returns:
            The FFT magnitudes
        """
        ...

    @overload
    def get_fft(self, time: float, magnitudes: int, cutoff: Optional[float], easing: Optional[Easing]) -> FftResult:
        """An FFT request against the runtime's loaded audio.

        Args:
            time: Sample time in milliseconds.
            magnitudes: Number of frequency bins.
            cutoff: A frequency cut-off.
            easing: The easing applied across the bins.

        Returns:
            The FFT magnitudes
        """
        ...

    @overload
    def get_fft(self, options: GetFftOptions) -> FftResult:
        """An FFT request against the runtime's loaded audio.

        Args:
            options: An FFT request against the runtime's loaded audio.

        Returns:
            The FFT magnitudes
        """
        ...

    def get_fft(self, *args: object, **kwargs: object) -> FftResult:
        """An FFT request against the runtime's loaded audio.

        Returns:
            The FFT magnitudes
        """
        options = args[0] if args and isinstance(args[0], GetFftOptions) else GetFftOptions(*args, **kwargs)
        return self._api.get_fft(GetFftRequest(**to_kwargs(options)))
