"""osb native bindings: a thin `ctypes` facade over the `crates/osb-native`
C ABI ().

The shared library is located from ``OSBTK_NATIVE_LIB`` or by walking up to the
cargo ``target/{release,debug}`` directory, so no install step is required in a
checkout.
"""

from __future__ import annotations

import ctypes
import json
import os
import sys
from pathlib import Path
from typing import Any

__all__ = ["version", "bake3d", "exec", "parse_osu", "parse_osb", "fft", "Runtime"]


def _library_names() -> list[str]:
    if sys.platform == "win32":
        return ["osb_native.dll"]
    if sys.platform == "darwin":
        return ["libosb_native.dylib"]
    return ["libosb_native.so"]


def _find_library() -> str:
    override = os.environ.get("OSBTK_NATIVE_LIB")
    if override and Path(override).is_file():
        return override
    directory = Path(__file__).resolve().parent
    while True:
        for name in _library_names():
            for profile in ("release", "debug"):
                candidate = directory / "target" / profile / name
                if candidate.is_file():
                    return str(candidate)
        if directory.parent == directory:
            break
        directory = directory.parent
    raise FileNotFoundError("osb_native shared library not found; build with `cargo build -p osb-native`")


_lib = ctypes.CDLL(_find_library())
_lib.osbtk_version.restype = ctypes.c_void_p
_lib.osbtk_bake3d.argtypes = [ctypes.c_char_p]
_lib.osbtk_bake3d.restype = ctypes.c_void_p
_lib.osbtk_exec.argtypes = [ctypes.c_char_p]
_lib.osbtk_exec.restype = ctypes.c_void_p
_lib.osbtk_string_free.argtypes = [ctypes.c_void_p]
_lib.osbtk_runtime_new.argtypes = [ctypes.c_double, ctypes.c_double]
_lib.osbtk_runtime_new.restype = ctypes.c_void_p
_lib.osbtk_runtime_free.argtypes = [ctypes.c_void_p]
_lib.osbtk_runtime_exec.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
_lib.osbtk_runtime_exec.restype = ctypes.c_void_p


def _take(pointer: int) -> str:
    if not pointer:
        return ""
    try:
        return ctypes.cast(pointer, ctypes.c_char_p).value.decode("utf-8")  # type: ignore[union-attr]
    finally:
        _lib.osbtk_string_free(pointer)


def version() -> str:
    """The native runtime version string."""
    return _take(_lib.osbtk_version())


def bake3d(input_json: str) -> dict[str, Any]:
    """Bakes a plain 3D input JSON into baked command-span JSON."""
    return json.loads(_take(_lib.osbtk_bake3d(input_json.encode("utf-8"))))


def exec(request_json: str) -> dict[str, Any]:
    """Runs a JSON request (``{"op": "..."}``) against the runtime."""
    return json.loads(_take(_lib.osbtk_exec(request_json.encode("utf-8"))))


def parse_osu(text: str) -> dict[str, Any]:
    """Parses a `.osu` beatmap into a summary."""
    return exec(json.dumps({"op": "Core.parseOsu", "text": text}))


def parse_osb(text: str) -> dict[str, Any]:
    """Parses a `.osb` storyboard into a summary."""
    return exec(json.dumps({"op": "Core.parseOsb", "text": text}))


def fft(path: str, time: float = 0.0, magnitudes: int = 64) -> list[float]:
    """FFT magnitudes for a WAV file at ``time`` (ms)."""
    return exec(json.dumps({"op": "Audio.fft", "path": path, "time": time, "magnitudes": magnitudes}))[
        "magnitudes"
    ]


class Runtime:
    """A live osb runtime (storyboard + context) - the storyboard API."""

    def __init__(self, width: float = 640, height: float = 480) -> None:
        self._handle = _lib.osbtk_runtime_new(width, height)

    def exec(self, request_json: str) -> dict[str, Any]:
        """Runs a JSON request against this runtime."""
        return json.loads(_take(_lib.osbtk_runtime_exec(self._handle, request_json.encode("utf-8"))))

    def create_sprite(self, layer: str, path: str, origin: int = 4) -> dict[str, Any]:
        return self.exec(json.dumps({"op": "Layer.createSprite", "layer": layer, "path": path, "origin": origin}))

    def command(self, handle: dict[str, Any], kind: str, start_time: float, end_time: float, **values: Any) -> dict[str, Any]:
        request = {
            "op": "Sprite.command",
            "kind": kind,
            "layer": handle["layer"],
            "sprite": handle["sprite"],
            "startTime": start_time,
            "endTime": end_time,
        }
        request.update(values)
        return self.exec(json.dumps(request))

    def sprite_state_at(self, handle: dict[str, Any], time: float) -> dict[str, Any]:
        return self.exec(
            json.dumps(
                {
                    "op": "Sprite.stateAt",
                    "layer": handle["layer"],
                    "sprite": handle["sprite"],
                    "time": time,
                }
            )
        )

    def import_osb(self, text: str) -> dict[str, Any]:
        return self.exec(json.dumps({"op": "Generator.importOsb", "text": text}))

    def close(self) -> None:
        if self._handle:
            _lib.osbtk_runtime_free(self._handle)
            self._handle = None

    def __del__(self) -> None:
        self.close()
