"""The image library: decode/encode and processing."""

from __future__ import annotations

from typing import Any

from osbtk.api.image_api import ImageAPI as ImageApi
from osbtk.libraries._client import STATELESS
from osbtk.models.image_options import ImageOptions
from osbtk.models.image_process_request import ImageProcessRequest
from osbtk.models.path_request import PathRequest

_api = ImageApi(STATELESS)


class Image:
    """The image library: decode/encode and processing."""

    @staticmethod
    def info(path: str) -> Any:
        return _api.info(PathRequest(path=path))

    @staticmethod
    def process(path: str, output: str, options: ImageOptions) -> Any:
        return _api.process(
            ImageProcessRequest(path=path, output=output, options=options)
        )
