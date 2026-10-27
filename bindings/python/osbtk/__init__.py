"""osb SDK: typed, OpenAPI-generated models and clients over the native engine.

Rust (``crates/osb-api``) is the source of truth; the models in ``osbtk.models``
and the clients in ``osbtk.api`` are generated from the OpenAPI contract and
driven through :class:`osbtk.bridge.BridgeRestClient`. The SDK is organized into
libraries (``osbtk.libraries``): ``storyboarding``, ``storyboarding3d``,
``animations``, ``audio``, ``curve``, ``diagnostics``, ``optimize``, ``image``,
``image_gen`` and ``media``.
"""

from __future__ import annotations

import json
from typing import Any

from osbtk import models  # noqa: F401  (re-export the generated models)
from osbtk._native import bake3d, exec, fft, parse_osb, parse_osu, version
from osbtk.api.audio_api import AudioAPI as AudioApi
from osbtk.api.generator_api import GeneratorAPI as GeneratorApi
from osbtk.api.layer_api import LayerAPI as LayerApi
from osbtk.api.sprite_api import SpriteAPI as SpriteApi
from osbtk.api_client import ApiClient
from osbtk.bridge import BridgeRestClient
from osbtk.libraries import (
    Animations,
    Audio,
    Curve,
    Diagnostics,
    Helpers,
    Image,
    ImageGen,
    Media,
    Optimize,
    Storyboarding,
    Storyboarding3d,
)
from osbtk.models.create_sample_request import CreateSampleRequest
from osbtk.models.create_sprite_request import CreateSpriteRequest
from osbtk.models.get_fft_request import GetFftRequest
from osbtk.models.layer_handle_request import LayerHandleRequest
from osbtk.models.time_request import TimeRequest
from osbtk.facade import Layer, Sprite

__all__ = [
    "osb",
    "Runtime",
    "Generator",
    "Layer",
    "Sprite",
    "run",
    "version",
    "bake3d",
    "exec",
    "parse_osu",
    "parse_osb",
    "fft",
    "models",
]


class _Osb:
    """The SDK entry point, organized into libraries."""

    storyboarding = Storyboarding
    storyboarding3d = Storyboarding3d
    helpers = Helpers
    animations = Animations
    audio = Audio
    curve = Curve
    diagnostics = Diagnostics
    optimize = Optimize
    image = Image
    image_gen = ImageGen
    media = Media

    def version(self) -> str:
        return version()


osb = _Osb()


class Runtime:
    """A live storyboard runtime bound to a native handle."""

    def __init__(self, width: float = 640, height: float = 480) -> None:
        from osbtk._native import Runtime as _NativeRuntime

        self._native = _NativeRuntime(width, height)
        self.bridge = lambda request: self._native.exec(json.dumps(request))
        client = ApiClient()
        client.rest_client = BridgeRestClient(self._native.exec)
        self.generator = GeneratorApi(client)
        self.layer = LayerApi(client)
        self.sprite = SpriteApi(client)
        self.audio = AudioApi(client)

    def close(self) -> None:
        self._native.close()

    def __enter__(self) -> "Runtime":
        return self

    def __exit__(self, *_: object) -> None:
        self.close()

    def __del__(self) -> None:
        self.close()


class Generator:
    """A storyboard generator (the ``StoryboardObjectGenerator`` equivalent).
    Subclass it and override ``generate()``."""

    def __init__(self, width: float = 640.0, height: float = 480.0) -> None:
        self.runtime = Runtime(width, height)

    def create_sprite(self, layer: str, path: str, origin: str = "Centre") -> Any:
        return self.runtime.layer.create_sprite(
            CreateSpriteRequest(layer=layer, path=path, origin=origin)
        )

    def get_layer(self, name: str) -> Layer:
        """Returns a layer that creates sprites/animations/samples."""
        handle = self.runtime.generator.layer_handle(LayerHandleRequest(name=name))
        return Layer(self.runtime.bridge, name, handle.layer)

    def create_animation(self, request: Any) -> Any:
        return self.runtime.layer.create_animation(request)

    def create_sample(self, layer: str, path: str, time: float, volume: float) -> Any:
        return self.runtime.layer.create_sample(
            CreateSampleRequest(layer=layer, path=path, time=time, volume=volume)
        )

    def get_beatmap(self) -> Any:
        return self.runtime.generator.beatmap()

    def get_fft(self, time: float, magnitudes: int) -> Any:
        return self.runtime.audio.get_fft(GetFftRequest(time=time, magnitudes=magnitudes))

    def get_timing_point_at(self, time: float) -> Any:
        return self.runtime.generator.timing_point_at(TimeRequest(time=time))

    def sprite_count(self) -> int:
        return self.runtime.generator.sprite_count().sprite_count

    def storyboard(self) -> Any:
        """The built storyboard model, so ``generate()``'s result can be read back."""
        return self.runtime.generator.storyboard()

    def close(self) -> None:
        self.runtime.close()

    def generate(self) -> None:
        raise NotImplementedError


def run(effect: Generator) -> Generator:
    """Runs an effect's ``generate()`` against a fresh generator."""
    effect.generate()
    return effect
