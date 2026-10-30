"""osb SDK: typed, OpenAPI-generated models and clients over the native engine.

Rust (``crates/osb-api``) is the source of truth. The generated clean clients in
``osbtk.client`` are the primary surface; ``osb`` composes them with the handful
of domain conveniences the contract cannot express - the effect runtime, the
parsers/command helpers, the 3D bake and the shared motion-graphics helpers.
"""

from __future__ import annotations

import json
from typing import Any

from osbtk import models  # noqa: F401  (re-export the generated models)
from osbtk._native import bake3d, exec, fft, parse_osb, parse_osu, version
from osbtk.api.audio_api import AudioAPI
from osbtk.api.core_api import CoreAPI
from osbtk.api.diagnostics_api import DiagnosticsAPI
from osbtk.api.generator_api import GeneratorAPI
from osbtk.api.image_api import ImageAPI
from osbtk.api.imagegen_api import ImagegenAPI
from osbtk.api.layer_api import LayerAPI
from osbtk.api.media_api import MediaAPI
from osbtk.api.optimize_api import OptimizeAPI
from osbtk.api.sprite_api import SpriteAPI
from osbtk.api_client import ApiClient
from osbtk.bridge import BridgeRestClient
from osbtk.client import (
    Audio as AudioClient,
    Core,
    Diagnostics,
    Generator as GeneratorClient,
    Image,
    Imagegen,
    Media,
    Optimize,
)
from osbtk.libraries import Helpers, Storyboarding, Storyboarding3d
from osbtk.libraries._client import STATELESS
from osbtk.models.create_sample_request import CreateSampleRequest
from osbtk.models.create_sprite_request import CreateSpriteRequest
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
    """The SDK entry point: the generated clean clients plus the domain surfaces."""

    storyboarding = Storyboarding
    storyboarding3d = Storyboarding3d
    helpers = Helpers

    core = Core(CoreAPI(STATELESS))
    audio = AudioClient(AudioAPI(STATELESS))
    diagnostics = Diagnostics(DiagnosticsAPI(STATELESS))
    optimize = Optimize(OptimizeAPI(STATELESS))
    image = Image(ImageAPI(STATELESS))
    image_gen = Imagegen(ImagegenAPI(STATELESS))
    media = Media(MediaAPI(STATELESS))

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
        self.generator = GeneratorClient(GeneratorAPI(client))
        self.layer = LayerAPI(client)
        self.sprite = SpriteAPI(client)
        self.audio = AudioClient(AudioAPI(client))

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
        handle = self.runtime.generator.layer_handle(name)
        return Layer(self.runtime.bridge, name, handle)

    def create_animation(self, request: Any) -> Any:
        return self.runtime.layer.create_animation(request)

    def create_sample(self, layer: str, path: str, time: float, volume: float) -> Any:
        return self.runtime.layer.create_sample(
            CreateSampleRequest(layer=layer, path=path, time=time, volume=volume)
        )

    def get_beatmap(self) -> Any:
        return self.runtime.generator.beatmap()

    def get_fft(self, time: float, magnitudes: int) -> Any:
        return self.runtime.audio.get_fft(time, magnitudes)

    def get_timing_point_at(self, time: float) -> Any:
        return self.runtime.generator.timing_point_at(time)

    def sprite_count(self) -> int:
        return self.runtime.generator.sprite_count()

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
