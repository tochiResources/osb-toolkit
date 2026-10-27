from __future__ import annotations

from ...api.media_api import MediaAPI
from ...models.encode_video_request import EncodeVideoRequest
from ...models.extract_audio_request import ExtractAudioRequest
from ...models.extract_frames_request import ExtractFramesRequest
from ...models.frame_result import FrameResult
from ...models.media_info import MediaInfo
from ...models.ok_result import OkResult
from ...models.path_request import PathRequest
from ..options import EncodeVideoOptions, ExtractAudioOptions, ExtractFramesOptions, PathOptions, to_kwargs
from typing import overload


class Media:
    def __init__(self, api: MediaAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Media)

    @overload
    def encode_video(self, options: EncodeVideoOptions) -> MediaInfo: ...

    @overload
    def encode_video(self, inputPattern: str, output: str, fps: float, codec: Optional[str] = None, crf: Optional[int] = None) -> MediaInfo: ...

    def encode_video(self, *args: object, **kwargs: object) -> MediaInfo:
        options = args[0] if args and isinstance(args[0], EncodeVideoOptions) else EncodeVideoOptions(*args, **kwargs)
        return self._api.encode_video(EncodeVideoRequest(**to_kwargs(options)))

    @overload
    def extract_audio(self, options: ExtractAudioOptions) -> OkResult: ...

    @overload
    def extract_audio(self, input: str, output: str) -> OkResult: ...

    def extract_audio(self, *args: object, **kwargs: object) -> OkResult:
        options = args[0] if args and isinstance(args[0], ExtractAudioOptions) else ExtractAudioOptions(*args, **kwargs)
        return self._api.extract_audio(ExtractAudioRequest(**to_kwargs(options)))

    @overload
    def extract_frames(self, options: ExtractFramesOptions) -> FrameResult: ...

    @overload
    def extract_frames(self, input: str, outputDir: str, count: Optional[int] = None, fps: Optional[float] = None, height: Optional[int] = None, start: Optional[float] = None, width: Optional[int] = None) -> FrameResult: ...

    def extract_frames(self, *args: object, **kwargs: object) -> FrameResult:
        options = args[0] if args and isinstance(args[0], ExtractFramesOptions) else ExtractFramesOptions(*args, **kwargs)
        return self._api.extract_frames(ExtractFramesRequest(**to_kwargs(options)))

    @overload
    def info(self, options: PathOptions) -> MediaInfo: ...

    @overload
    def info(self, path: str) -> MediaInfo: ...

    def info(self, *args: object, **kwargs: object) -> MediaInfo:
        options = args[0] if args and isinstance(args[0], PathOptions) else PathOptions(*args, **kwargs)
        return self._api.info(PathRequest(**to_kwargs(options)))
