"""The color-space and CSS/Sass-style helpers."""

from __future__ import annotations

from typing import Any, Dict, List, Optional

from osbtk.libraries.helpers._base import call


class _Color:
    """The color-space and CSS/Sass-style helpers."""

    @staticmethod
    def color_to_space(color: Any, space: str = "rgb") -> List[float]:
        """Converts a color into its (x, y, z, w) components in `space`."""
        return call("color.toSpace", {"color": color, "space": space})

    @staticmethod
    def color_offset(color: Any, delta: List[float], space: str = "rgb") -> Any:
        """Adds a per-component `delta` in `space`."""
        return call("color.offset", {"color": color, "space": space, "delta": delta})

    @staticmethod
    def color_scale(color: Any, factor: List[float], space: str = "rgb") -> Any:
        """Multiplies each component by a per-component `factor` in `space`."""
        return call("color.scale", {"color": color, "space": space, "factor": factor})

    @staticmethod
    def color_randomize(
        color: Any,
        min: List[float],
        max: List[float],
        seed: int,
        space: str = "rgb",
    ) -> Any:
        """Jitters each component by a seeded per-component range in `space`."""
        return call(
            "color.randomize",
            {"color": color, "space": space, "seed": seed, "min": min, "max": max},
        )

    @staticmethod
    def _apply(color: Any, op: str, extra: Optional[Dict[str, Any]] = None) -> Any:
        payload: Dict[str, Any] = {"color": color, "op": op}
        payload.update(extra or {})
        return call("color.apply", payload)

    @staticmethod
    def lighten(color: Any, amount: float) -> Any:
        return _Color._apply(color, "lighten", {"amount": amount})

    @staticmethod
    def darken(color: Any, amount: float) -> Any:
        return _Color._apply(color, "darken", {"amount": amount})

    @staticmethod
    def adjust_hue(color: Any, degrees: float) -> Any:
        return _Color._apply(color, "adjustHue", {"degrees": degrees})

    @staticmethod
    def complement(color: Any) -> Any:
        return _Color._apply(color, "complement")

    @staticmethod
    def grayscale(color: Any) -> Any:
        return _Color._apply(color, "grayscale")

    @staticmethod
    def invert(color: Any) -> Any:
        return _Color._apply(color, "invert")

    @staticmethod
    def mix(a: Any, b: Any, weight: float = 0.5) -> Any:
        return _Color._apply(a, "mix", {"other": b, "weight": weight})

    @staticmethod
    def tint(color: Any, weight: float = 0.5) -> Any:
        return _Color._apply(color, "tint", {"weight": weight})

    @staticmethod
    def shade(color: Any, weight: float = 0.5) -> Any:
        return _Color._apply(color, "shade", {"weight": weight})

    @staticmethod
    def gradient(
        stops: List[Any], t: float, space: str = "rgb", repeat: bool = False
    ) -> Any:
        """Samples a multi-stop gradient at `t` (0..=1)."""
        return call("color.gradient", {"stops": stops, "t": t, "space": space, "repeat": repeat})

    @staticmethod
    def gradient_channel(
        stops: List[Any],
        count: int = 16,
        start: float = 0.0,
        end: float = 1000.0,
        space: str = "rgb",
        repeat: bool = False,
    ) -> Any:
        """Builds a color channel by sampling a gradient `count` times."""
        return call(
            "color.gradientChannel",
            {
                "stops": stops,
                "count": count,
                "start": start,
                "end": end,
                "space": space,
                "repeat": repeat,
            },
        )

    @staticmethod
    def color_offset_channel(channel: Any, delta: List[float], space: str = "rgb") -> Any:
        """Offsets a whole color channel in `space`."""
        return call("color.offsetChannel", {"channel": channel, "space": space, "delta": delta})

    @staticmethod
    def color_scale_channel(channel: Any, factor: List[float], space: str = "rgb") -> Any:
        """Scales a whole color channel in `space`."""
        return call("color.scaleChannel", {"channel": channel, "space": space, "factor": factor})

    @staticmethod
    def color_randomize_channel(
        channel: Any, min: List[float], max: List[float], seed: int, space: str = "rgb"
    ) -> Any:
        """Randomizes a whole color channel in `space`."""
        return call(
            "color.randomizeChannel",
            {"channel": channel, "space": space, "seed": seed, "min": min, "max": max},
        )

    @staticmethod
    def color_apply_channel(channel: Any, op: str, options: Optional[Dict[str, Any]] = None) -> Any:
        """Applies a named helper to every keyframe of a color channel."""
        payload: Dict[str, Any] = {"channel": channel, "op": op}
        payload.update(options or {})
        return call("color.applyChannel", payload)
