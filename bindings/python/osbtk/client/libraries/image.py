from __future__ import annotations

from ...api.image_api import ImageAPI
from ...models.image_info import ImageInfo
from ...models.image_options import ImageOptions
from ...models.image_process_request import ImageProcessRequest
from ...models.path_request import PathRequest
from ..options import ImageProcessOptions, PathOptions, to_kwargs
from typing import Any, Optional
from typing import overload


class Image:
    def __init__(self, api: ImageAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Image)

    @overload
    def info(self, options: PathOptions) -> ImageInfo: ...

    @overload
    def info(self, path: str) -> ImageInfo: ...

    def info(self, *args: object, **kwargs: object) -> ImageInfo:
        options = args[0] if args and isinstance(args[0], PathOptions) else PathOptions(*args, **kwargs)
        return self._api.info(PathRequest(**to_kwargs(options)))

    @overload
    def process(self, options: ImageProcessOptions) -> ImageInfo: ...

    @overload
    def process(self, options: ImageOptions, output: str, path: str) -> ImageInfo: ...

    def process(self, *args: object, **kwargs: object) -> ImageInfo:
        options = args[0] if args and isinstance(args[0], ImageProcessOptions) else ImageProcessOptions(*args, **kwargs)
        return self._api.process(ImageProcessRequest(**to_kwargs(options)))
