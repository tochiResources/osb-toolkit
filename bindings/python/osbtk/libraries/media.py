"""The media library: probing, frame extraction/encoding and audio (M26)."""

from __future__ import annotations

from typing import Any

from osbtk.api.media_api import MediaAPI as MediaApi
from osbtk.libraries._client import STATELESS
from osbtk.models.encode_video_request import EncodeVideoRequest
from osbtk.models.extract_audio_request import ExtractAudioRequest
from osbtk.models.extract_frames_request import ExtractFramesRequest
from osbtk.models.path_request import PathRequest

_api = MediaApi(STATELESS)


class Media:
    """The media library: probing, frame extraction/encoding and audio (M26)."""

    @staticmethod
    def info(path: str) -> Any:
        return _api.info(PathRequest(path=path))

    @staticmethod
    def extract_frames(request: ExtractFramesRequest) -> Any:
        return _api.extract_frames(request)

    @staticmethod
    def encode_video(request: EncodeVideoRequest) -> Any:
        return _api.encode_video(request)

    @staticmethod
    def extract_audio(request: ExtractAudioRequest) -> Any:
        return _api.extract_audio(request)
