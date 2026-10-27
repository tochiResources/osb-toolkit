from __future__ import annotations

from ..api.image_api import ImageAPI
from ..models.image_info import ImageInfo
from ..models.image_process_request import ImageProcessRequest
from ..models.path_request import PathRequest
from .options import ImageProcessOptions, PathOptions, to_kwargs


class Image:
    def __init__(self, api: ImageAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Image)

    def info(self, options: PathOptions) -> ImageInfo:
        return self._api.info(PathRequest(**to_kwargs(options)))

    def process(self, options: ImageProcessOptions) -> ImageInfo:
        return self._api.process(ImageProcessRequest(**to_kwargs(options)))
