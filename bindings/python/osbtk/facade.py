"""Storyboard facades.

``Layer`` and ``Sprite`` mirror the storyboard ``OsbLayer``/``OsbSprite`` API
over the native JSON bridge (so scalar and object command values both work).
Command methods accept three shapes - easing-first
(``move(easing, start, end, ...)``), time-range (``move(start, end, ...)``) and
time-only (``move(time, ...)``).
"""

from __future__ import annotations

from typing import Any, Callable

Bridge = Callable[[dict[str, Any]], Any]


def _normalize(args: tuple[Any, ...]) -> tuple[Any, Any, Any, Any, Any]:
    """Normalizes the three command shapes to (start, end, easing, startValue, endValue)."""
    if isinstance(args[0], str):
        return args[1], args[2], args[0], args[3], args[4]
    if len(args) == 2:
        return args[0], args[0], "None", args[1], args[1]
    easing = args[4] if len(args) > 4 else "None"
    return args[0], args[1], easing, args[2], args[3]


class Layer:
    """A storyboard segment (the ``StoryboardSegment`` surface)."""

    def __init__(self, bridge: Bridge, name: str, handle: int) -> None:
        self._bridge = bridge
        self.name = name
        self.handle = handle

    def create_sprite(
        self, texture_path: str, origin: str = "Centre", position: Any = None
    ) -> "Sprite":
        """``CreateSprite``."""
        request: dict[str, Any] = {
            "op": "Layer.createSprite",
            "layerHandle": self.handle,
            "path": texture_path,
            "origin": origin,
        }
        if position is not None:
            request["position"] = position
        handle = self._bridge(request)
        return Sprite(self._bridge, handle["layer"], handle["sprite"])

    def create_animation(
        self,
        texture_path: str,
        frames: int,
        frame_delay: float,
        loop_type: str,
        origin: str,
        position: Any,
    ) -> "Sprite":
        """``CreateAnimation``."""
        handle = self._bridge(
            {
                "op": "Layer.createAnimation",
                "layerHandle": self.handle,
                "path": texture_path,
                "frames": frames,
                "frameDelay": frame_delay,
                "loopType": loop_type,
                "origin": origin,
                "position": position,
            }
        )
        return Sprite(self._bridge, handle["layer"], handle["sprite"])

    def create_sample(self, sample_path: str, time: float, volume: float = 100.0) -> "Layer":
        """``CreateSample``."""
        self._bridge(
            {
                "op": "Layer.createSample",
                "layerHandle": self.handle,
                "path": sample_path,
                "time": time,
                "volume": volume,
            }
        )
        return self

    def create_segment(self) -> "Layer":
        """``CreateSegment``: a segment nested in this one."""
        handle = self._bridge({"op": "Layer.createSegment", "layer": self.handle})
        return Layer(self._bridge, "", handle["layer"])

    def set_reverse_depth(self, value: bool) -> "Layer":
        """``ReverseDepth``."""
        self._bridge({"op": "Layer.reverseDepth", "layer": self.handle, "value": value})
        return self

    def add_index(self, index: float) -> "Layer":
        """``AddIndex``."""
        self._bridge({"op": "Layer.addIndex", "layer": self.handle, "index": index})
        return self

    def indexes(self) -> Any:
        """``Indexes``/``LowestIndex``/``HighestIndex``."""
        return self._bridge({"op": "Layer.indexes", "layer": self.handle})

    def change_position(self, osb_layer: str) -> "Layer":
        """``ChangePosition``."""
        self._bridge(
            {"op": "Layer.changePosition", "layer": self.handle, "osbLayer": osb_layer}
        )
        return self

    def move_to_top(self) -> "Layer":
        """``MoveToTop``."""
        self._bridge({"op": "Layer.moveToTop", "layer": self.handle})
        return self

    def move_to_bottom(self) -> "Layer":
        """``MoveToBottom``."""
        self._bridge({"op": "Layer.moveToBottom", "layer": self.handle})
        return self

    def clean(self) -> Any:
        """``Clean``."""
        return self._bridge({"op": "Layer.clean", "layer": self.handle})

    def discard(self) -> Any:
        """``Discard``."""
        return self._bridge({"op": "Layer.discard", "layer": self.handle})

    def discard_sprite(self, sprite: "Sprite") -> "Layer":
        """``DiscardSprite``."""
        self._bridge(
            {"op": "Layer.discardSprite", "layer": self.handle, "sprite": sprite.sprite}
        )
        return self


class Sprite:
    """A storyboard sprite (the ``OsbSprite`` API)."""

    def __init__(self, bridge: Bridge, layer: int, sprite: int) -> None:
        self._bridge = bridge
        self.layer = layer
        self.sprite = sprite

    def info(self) -> Any:
        """The sprite's initial state and metadata."""
        return self._bridge({"op": "Sprite.info", "layer": self.layer, "sprite": self.sprite})

    def move(self, *args: Any) -> "Sprite":
        return self._command("move", args)

    def move_x(self, *args: Any) -> "Sprite":
        return self._command("moveX", args)

    def move_y(self, *args: Any) -> "Sprite":
        return self._command("moveY", args)

    def scale(self, *args: Any) -> "Sprite":
        return self._command("scale", args)

    def scale_vec(self, *args: Any) -> "Sprite":
        return self._command("scaleVec", args)

    def rotate(self, *args: Any) -> "Sprite":
        return self._command("rotate", args)

    def fade(self, *args: Any) -> "Sprite":
        return self._command("fade", args)

    def color(self, *args: Any) -> "Sprite":
        return self._command("color", args)

    def color_hsb(self, *args: Any) -> "Sprite":
        return self._command("colorHsb", args)

    def anchor(self, *args: Any) -> "Sprite":
        """``Anchor``: an animatable origin offset in texture pixels."""
        return self._command("anchor", args)

    def set_anchor(self, point: Any) -> "Sprite":
        """Sets a constant origin offset (a zero-length ``Anchor``)."""
        return self.anchor(0, 0, point, point)

    def parameter(
        self,
        parameter: str,
        start_time: float,
        end_time: float,
        start: float = 0.0,
        end: float = 1.0,
        easing: str = "None",
    ) -> "Sprite":
        return self._add("parameter", start_time, end_time, easing, start, end, parameter)

    def flip_h(self, start_time: float, end_time: float) -> "Sprite":
        return self.parameter("FlipHorizontal", start_time, end_time)

    def flip_v(self, start_time: float, end_time: float) -> "Sprite":
        return self.parameter("FlipVertical", start_time, end_time)

    def additive(self, start_time: float, end_time: float) -> "Sprite":
        return self.parameter("Additive", start_time, end_time)

    def start_loop_group(self, start_time: float, count: float) -> "Sprite":
        """``StartLoopGroup``."""
        self._bridge(
            {
                "op": "Sprite.startLoopGroup",
                "layer": self.layer,
                "sprite": self.sprite,
                "startTime": start_time,
                "count": count,
            }
        )
        return self

    def start_trigger_group(
        self, name: str, start_time: float, end_time: float, group: int = 0
    ) -> "Sprite":
        """``StartTriggerGroup``."""
        self._bridge(
            {
                "op": "Sprite.startTriggerGroup",
                "layer": self.layer,
                "sprite": self.sprite,
                "name": name,
                "startTime": start_time,
                "endTime": end_time,
                "group": group,
            }
        )
        return self

    def end_group(self) -> "Sprite":
        """``EndGroup``."""
        self._bridge({"op": "Sprite.endGroup", "layer": self.layer, "sprite": self.sprite})
        return self

    def state_at(self, time: float) -> Any:
        return self._bridge(
            {"op": "Sprite.stateAt", "layer": self.layer, "sprite": self.sprite, "time": time}
        )

    def position_at(self, time: float) -> Any:
        return self.state_at(time)["position"]

    def scale_at(self, time: float) -> Any:
        return self.state_at(time)["scale"]

    def rotation_at(self, time: float) -> float:
        return self.state_at(time)["rotation"]

    def opacity_at(self, time: float) -> float:
        return self.state_at(time)["opacity"]

    def color_at(self, time: float) -> Any:
        return self.state_at(time)["color"]

    def additive_at(self, time: float) -> bool:
        return self.state_at(time)["additive"]

    def anchor_at(self, time: float) -> Any:
        """``AnchorAt``: the sampled origin offset."""
        return self.state_at(time)["anchor"]

    def _command(self, kind: str, args: tuple[Any, ...]) -> "Sprite":
        start_time, end_time, easing, start, end = _normalize(args)
        return self._add(kind, start_time, end_time, easing, start, end)

    def _add(
        self,
        kind: str,
        start_time: float,
        end_time: float,
        easing: str,
        start: Any,
        end: Any,
        parameter: str | None = None,
    ) -> "Sprite":
        request: dict[str, Any] = {
            "op": "Sprite.command",
            "kind": kind,
            "layer": self.layer,
            "sprite": self.sprite,
            "startTime": start_time,
            "endTime": end_time,
            "easing": easing,
            "start": start,
            "end": end,
        }
        if parameter is not None:
            request["parameter"] = parameter
        self._bridge(request)
        return self
