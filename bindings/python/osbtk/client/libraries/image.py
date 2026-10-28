from __future__ import annotations

from ...api.image_api import ImageAPI
from ...models.image_info import ImageInfo
from ...models.image_options import ImageOptions
from ...models.image_process_request import ImageProcessRequest
from ...models.path_request import PathRequest
from ..options import ImageProcessOptions, PathOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Image:
    """The clean `Image` client."""

    def __init__(self, api: ImageAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Image)

    @overload
    def info(self, path: str) -> ImageInfo:
        """A request carrying a single file path.

        Args:
            path: The input file path.

        Returns:
            The image's metadata
        """
        ...

    @overload
    def info(self, options: PathOptions) -> ImageInfo:
        """A request carrying a single file path.

        Args:
            options: A request carrying a single file path.

        Returns:
            The image's metadata
        """
        ...

    def info(self, *args: object, **kwargs: object) -> ImageInfo:
        """A request carrying a single file path.

        Returns:
            The image's metadata
        """
        options = args[0] if args and isinstance(args[0], PathOptions) else PathOptions(*args, **kwargs)
        return self._api.info(PathRequest(**to_kwargs(options)))

    @overload
    def process(self, options: ImageOptions, output: str, path: str) -> ImageInfo:
        """An image processing request: read `path`, apply `options`, write `output`.

        Args:
            options: The processing options.
            output: The output file path (its extension selects the format).
            path: The input file path.

        Returns:
            The processed image's metadata
        """
        ...

    @overload
    def process(self, options: ImageProcessOptions) -> ImageInfo:
        """An image processing request: read `path`, apply `options`, write `output`.

        Args:
            options: An image processing request: read `path`, apply `options`, write `output`.

        Returns:
            The processed image's metadata
        """
        ...

    def process(self, *args: object, **kwargs: object) -> ImageInfo:
        """An image processing request: read `path`, apply `options`, write `output`.

        Returns:
            The processed image's metadata
        """
        options = args[0] if args and isinstance(args[0], ImageProcessOptions) else ImageProcessOptions(*args, **kwargs)
        return self._api.process(ImageProcessRequest(**to_kwargs(options)))
