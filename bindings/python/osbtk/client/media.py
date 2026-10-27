from __future__ import annotations

from ..api.media_api import MediaAPI
from ..models.encode_video_request import EncodeVideoRequest
from ..models.extract_audio_request import ExtractAudioRequest
from ..models.extract_frames_request import ExtractFramesRequest
from ..models.frame_result import FrameResult
from ..models.media_info import MediaInfo
from ..models.ok_result import OkResult
from ..models.path_request import PathRequest
from .options import EncodeVideoOptions, ExtractAudioOptions, ExtractFramesOptions, PathOptions, to_kwargs


class Media:
    def __init__(self, api: MediaAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Media)

    def encode_video(self, options: EncodeVideoOptions) -> MediaInfo:
        return self._api.encode_video(EncodeVideoRequest(**to_kwargs(options)))

    def extract_audio(self, options: ExtractAudioOptions) -> OkResult:
        return self._api.extract_audio(ExtractAudioRequest(**to_kwargs(options)))

    def extract_frames(self, options: ExtractFramesOptions) -> FrameResult:
        return self._api.extract_frames(ExtractFramesRequest(**to_kwargs(options)))

    def info(self, options: PathOptions) -> MediaInfo:
        return self._api.info(PathRequest(**to_kwargs(options)))
