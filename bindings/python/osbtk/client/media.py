from __future__ import annotations

from ..api.media_api import MediaAPI
from ..models.encode_video_request import EncodeVideoRequest
from ..models.extract_audio_request import ExtractAudioRequest
from ..models.extract_frames_request import ExtractFramesRequest
from ..models.frame_result import FrameResult
from ..models.media_info import MediaInfo
from ..models.ok_result import OkResult
from ..models.path_request import PathRequest


class Media:
    def __init__(self, api: MediaAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Media)

    def encode_video(self, body: EncodeVideoRequest) -> MediaInfo:
        return self._api.encode_video(body)

    def extract_audio(self, body: ExtractAudioRequest) -> OkResult:
        return self._api.extract_audio(body)

    def extract_frames(self, body: ExtractFramesRequest) -> FrameResult:
        return self._api.extract_frames(body)

    def info(self, body: PathRequest) -> MediaInfo:
        return self._api.info(body)
