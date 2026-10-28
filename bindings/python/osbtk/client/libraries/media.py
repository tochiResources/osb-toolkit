from __future__ import annotations

from ...api.media_api import MediaAPI
from ...models.encode_video_request import EncodeVideoRequest
from ...models.extract_audio_request import ExtractAudioRequest
from ...models.extract_frames_request import ExtractFramesRequest
from ...models.frame_result import FrameResult
from ...models.media_info import MediaInfo
from ...models.path_request import PathRequest
from ..options import EncodeVideoOptions, ExtractAudioOptions, ExtractFramesOptions, PathOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Media:
    def __init__(self, api: MediaAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Media)

    @overload
    def encode_video(self, inputPattern: str, output: str, fps: float) -> MediaInfo:
        """An image-sequence encoding request (frames -> video).

        Args:
            inputPattern: The input filename pattern (e.g. `frames/frame_%04d.png`).
            output: The output video path.
            fps: The frame rate.

        Returns:
            The encoded video's metadata
        """
        ...

    @overload
    def encode_video(self, inputPattern: str, output: str, fps: float, codec: Optional[str]) -> MediaInfo:
        """An image-sequence encoding request (frames -> video).

        Args:
            inputPattern: The input filename pattern (e.g. `frames/frame_%04d.png`).
            output: The output video path.
            fps: The frame rate.
            codec: The video codec (default `libx264`).

        Returns:
            The encoded video's metadata
        """
        ...

    @overload
    def encode_video(self, inputPattern: str, output: str, fps: float, codec: Optional[str], crf: Optional[int]) -> MediaInfo:
        """An image-sequence encoding request (frames -> video).

        Args:
            inputPattern: The input filename pattern (e.g. `frames/frame_%04d.png`).
            output: The output video path.
            fps: The frame rate.
            codec: The video codec (default `libx264`).
            crf: The constant-rate-factor quality (`0`-`51`; lower is better).

        Returns:
            The encoded video's metadata
        """
        ...

    @overload
    def encode_video(self, options: EncodeVideoOptions) -> MediaInfo:
        """An image-sequence encoding request (frames -> video).

        Args:
            options: An image-sequence encoding request (frames -> video).

        Returns:
            The encoded video's metadata
        """
        ...

    def encode_video(self, *args: object, **kwargs: object) -> MediaInfo:
        """An image-sequence encoding request (frames -> video).

        Returns:
            The encoded video's metadata
        """
        options = args[0] if args and isinstance(args[0], EncodeVideoOptions) else EncodeVideoOptions(*args, **kwargs)
        return self._api.encode_video(EncodeVideoRequest(**to_kwargs(options)))

    @overload
    def extract_audio(self, input: str, output: str) -> None:
        """An audio-extraction request (media -> audio file).

        Args:
            input: The input media path.
            output: The output audio path (its extension selects the format).

        Returns:
            Acknowledged
        """
        ...

    @overload
    def extract_audio(self, options: ExtractAudioOptions) -> None:
        """An audio-extraction request (media -> audio file).

        Args:
            options: An audio-extraction request (media -> audio file).

        Returns:
            Acknowledged
        """
        ...

    def extract_audio(self, *args: object, **kwargs: object) -> None:
        """An audio-extraction request (media -> audio file).

        Returns:
            Acknowledged
        """
        options = args[0] if args and isinstance(args[0], ExtractAudioOptions) else ExtractAudioOptions(*args, **kwargs)
        self._api.extract_audio(ExtractAudioRequest(**to_kwargs(options)))

    @overload
    def extract_frames(self, input: str, outputDir: str) -> FrameResult:
        """A frame-extraction request (video -> image sequence).

        Args:
            input: The input media path.
            outputDir: The directory to write frames into.

        Returns:
            The extracted frame count
        """
        ...

    @overload
    def extract_frames(self, input: str, outputDir: str, count: Optional[int]) -> FrameResult:
        """A frame-extraction request (video -> image sequence).

        Args:
            input: The input media path.
            outputDir: The directory to write frames into.
            count: The maximum number of frames to extract.

        Returns:
            The extracted frame count
        """
        ...

    @overload
    def extract_frames(self, input: str, outputDir: str, count: Optional[int], fps: Optional[float]) -> FrameResult:
        """A frame-extraction request (video -> image sequence).

        Args:
            input: The input media path.
            outputDir: The directory to write frames into.
            count: The maximum number of frames to extract.
            fps: The sampling rate (frames per second); defaults to the source rate.

        Returns:
            The extracted frame count
        """
        ...

    @overload
    def extract_frames(self, input: str, outputDir: str, count: Optional[int], fps: Optional[float], height: Optional[int]) -> FrameResult:
        """A frame-extraction request (video -> image sequence).

        Args:
            input: The input media path.
            outputDir: The directory to write frames into.
            count: The maximum number of frames to extract.
            fps: The sampling rate (frames per second); defaults to the source rate.
            height: Scale frames to this height.

        Returns:
            The extracted frame count
        """
        ...

    @overload
    def extract_frames(self, input: str, outputDir: str, count: Optional[int], fps: Optional[float], height: Optional[int], start: Optional[float]) -> FrameResult:
        """A frame-extraction request (video -> image sequence).

        Args:
            input: The input media path.
            outputDir: The directory to write frames into.
            count: The maximum number of frames to extract.
            fps: The sampling rate (frames per second); defaults to the source rate.
            height: Scale frames to this height.
            start: The start time in seconds.

        Returns:
            The extracted frame count
        """
        ...

    @overload
    def extract_frames(self, input: str, outputDir: str, count: Optional[int], fps: Optional[float], height: Optional[int], start: Optional[float], width: Optional[int]) -> FrameResult:
        """A frame-extraction request (video -> image sequence).

        Args:
            input: The input media path.
            outputDir: The directory to write frames into.
            count: The maximum number of frames to extract.
            fps: The sampling rate (frames per second); defaults to the source rate.
            height: Scale frames to this height.
            start: The start time in seconds.
            width: Scale frames to this width.

        Returns:
            The extracted frame count
        """
        ...

    @overload
    def extract_frames(self, options: ExtractFramesOptions) -> FrameResult:
        """A frame-extraction request (video -> image sequence).

        Args:
            options: A frame-extraction request (video -> image sequence).

        Returns:
            The extracted frame count
        """
        ...

    def extract_frames(self, *args: object, **kwargs: object) -> FrameResult:
        """A frame-extraction request (video -> image sequence).

        Returns:
            The extracted frame count
        """
        options = args[0] if args and isinstance(args[0], ExtractFramesOptions) else ExtractFramesOptions(*args, **kwargs)
        return self._api.extract_frames(ExtractFramesRequest(**to_kwargs(options)))

    @overload
    def info(self, path: str) -> MediaInfo:
        """A request carrying a single file path.

        Args:
            path: The input file path.

        Returns:
            The media metadata
        """
        ...

    @overload
    def info(self, options: PathOptions) -> MediaInfo:
        """A request carrying a single file path.

        Args:
            options: A request carrying a single file path.

        Returns:
            The media metadata
        """
        ...

    def info(self, *args: object, **kwargs: object) -> MediaInfo:
        """A request carrying a single file path.

        Returns:
            The media metadata
        """
        options = args[0] if args and isinstance(args[0], PathOptions) else PathOptions(*args, **kwargs)
        return self._api.info(PathRequest(**to_kwargs(options)))
