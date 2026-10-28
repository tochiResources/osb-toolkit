from __future__ import annotations

from ...api.layer_api import LayerAPI
from ...models.create_animation_request import CreateAnimationRequest
from ...models.create_sample_request import CreateSampleRequest
from ...models.create_sprite_request import CreateSpriteRequest
from ...models.handle_request import HandleRequest
from ...models.osb_layer import OsbLayer
from ...models.osb_loop_type import OsbLoopType
from ...models.osb_origin import OsbOrigin
from ...models.segment_add_index_request import SegmentAddIndexRequest
from ...models.segment_change_position_request import SegmentChangePositionRequest
from ...models.segment_indexes_result import SegmentIndexesResult
from ...models.segment_request import SegmentRequest
from ...models.segment_reverse_depth_request import SegmentReverseDepthRequest
from ...models.vector2 import Vector2
from ...api.sprite_api import SpriteAPI
from .sprite import Sprite
from ..options import CreateAnimationOptions, CreateSampleOptions, CreateSpriteOptions, SegmentAddIndexOptions, SegmentChangePositionOptions, SegmentReverseDepthOptions, to_kwargs
from typing import Any, Dict, List, Optional
from typing import overload


class Layer:
    def __init__(self, api: LayerAPI, sprites: SpriteAPI, handle: int) -> None:
        self._api = api
        self._sprites = sprites
        self._handle = handle

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Layer)

    @overload
    def add_index(self, index: float) -> None:
        """Record a depth index on a segment.

        Args:
            index: The index to record.

        Returns:
            Acknowledged
        """
        ...

    @overload
    def add_index(self, options: SegmentAddIndexOptions) -> None:
        """Record a depth index on a segment.

        Args:
            options: Record a depth index on a segment.

        Returns:
            Acknowledged
        """
        ...

    def add_index(self, *args: object, **kwargs: object) -> None:
        """Record a depth index on a segment.

        Returns:
            Acknowledged
        """
        options = args[0] if args and isinstance(args[0], SegmentAddIndexOptions) else SegmentAddIndexOptions(*args, **kwargs)
        self._api.add_index(SegmentAddIndexRequest(layer=self._handle, **to_kwargs(options)))

    @overload
    def change_position(self, osbLayer: OsbLayer) -> None:
        """Move a segment to another osu! layer (`ChangePosition`).

        Args:
            osbLayer: The destination osu! layer.

        Returns:
            Acknowledged
        """
        ...

    @overload
    def change_position(self, options: SegmentChangePositionOptions) -> None:
        """Move a segment to another osu! layer (`ChangePosition`).

        Args:
            options: Move a segment to another osu! layer (`ChangePosition`).

        Returns:
            Acknowledged
        """
        ...

    def change_position(self, *args: object, **kwargs: object) -> None:
        """Move a segment to another osu! layer (`ChangePosition`).

        Returns:
            Acknowledged
        """
        options = args[0] if args and isinstance(args[0], SegmentChangePositionOptions) else SegmentChangePositionOptions(*args, **kwargs)
        self._api.change_position(SegmentChangePositionRequest(layer=self._handle, **to_kwargs(options)))

    def clean(self) -> int:
        """A segment handle request.

        Returns:
            The number of sprites removed
        """
        return self._api.clean(SegmentRequest(layer=self._handle)).removed

    @overload
    def create_animation(self, path: str, frames: int, frameDelay: float, loopType: OsbLoopType, origin: OsbOrigin, position: Vector2) -> Sprite:
        """Create an animation sprite.

        Args:
            path: The texture path (relative to the mapset).
            frames: The number of frames.
            frameDelay: Delay between frames, in milliseconds.
            loopType: How the animation loops.
            origin: The sprite anchor origin.
            position: The initial position.

        Returns:
            The new animation handle
        """
        ...

    @overload
    def create_animation(self, path: str, frames: int, frameDelay: float, loopType: OsbLoopType, origin: OsbOrigin, position: Vector2, layer: Optional[str]) -> Sprite:
        """Create an animation sprite.

        Args:
            path: The texture path (relative to the mapset).
            frames: The number of frames.
            frameDelay: Delay between frames, in milliseconds.
            loopType: How the animation loops.
            origin: The sprite anchor origin.
            position: The initial position.
            layer: The target layer name (omitted when `layerHandle` is set).

        Returns:
            The new animation handle
        """
        ...

    @overload
    def create_animation(self, options: CreateAnimationOptions) -> Sprite:
        """Create an animation sprite.

        Args:
            options: Create an animation sprite.

        Returns:
            The new animation handle
        """
        ...

    def create_animation(self, *args: object, **kwargs: object) -> Sprite:
        """Create an animation sprite.

        Returns:
            The new animation handle
        """
        options = args[0] if args and isinstance(args[0], CreateAnimationOptions) else CreateAnimationOptions(*args, **kwargs)
        handle = self._api.create_animation(CreateAnimationRequest(layer_handle=self._handle, **to_kwargs(options)))
        return Sprite(self._sprites, handle.layer, handle.sprite)

    @overload
    def create_sample(self, path: str, time: float) -> None:
        """Create a sound sample on a layer.

        Args:
            path: The sample path (relative to the mapset).
            time: The sample time in milliseconds.

        Returns:
            Acknowledged
        """
        ...

    @overload
    def create_sample(self, path: str, time: float, volume: Optional[float]) -> None:
        """Create a sound sample on a layer.

        Args:
            path: The sample path (relative to the mapset).
            time: The sample time in milliseconds.
            volume: The sample volume (`0..=100`).

        Returns:
            Acknowledged
        """
        ...

    @overload
    def create_sample(self, path: str, time: float, volume: Optional[float], layer: Optional[str]) -> None:
        """Create a sound sample on a layer.

        Args:
            path: The sample path (relative to the mapset).
            time: The sample time in milliseconds.
            volume: The sample volume (`0..=100`).
            layer: The target layer name (omitted when `layerHandle` is set).

        Returns:
            Acknowledged
        """
        ...

    @overload
    def create_sample(self, options: CreateSampleOptions) -> None:
        """Create a sound sample on a layer.

        Args:
            options: Create a sound sample on a layer.

        Returns:
            Acknowledged
        """
        ...

    def create_sample(self, *args: object, **kwargs: object) -> None:
        """Create a sound sample on a layer.

        Returns:
            Acknowledged
        """
        options = args[0] if args and isinstance(args[0], CreateSampleOptions) else CreateSampleOptions(*args, **kwargs)
        self._api.create_sample(CreateSampleRequest(layer_handle=self._handle, **to_kwargs(options)))

    def create_segment(self) -> Layer:
        """A segment handle request.

        Returns:
            The new segment handle
        """
        handle = self._api.create_segment(SegmentRequest(layer=self._handle))
        return Layer(self._api, self._sprites, handle.layer)

    @overload
    def create_sprite(self, path: str, origin: OsbOrigin) -> Sprite:
        """Create a sprite.

        Args:
            path: The texture path (relative to the mapset).
            origin: The sprite anchor origin.

        Returns:
            The new sprite handle
        """
        ...

    @overload
    def create_sprite(self, path: str, origin: OsbOrigin, position: Optional[Vector2]) -> Sprite:
        """Create a sprite.

        Args:
            path: The texture path (relative to the mapset).
            origin: The sprite anchor origin.
            position: The initial position; omitted means the sprite's default.

        Returns:
            The new sprite handle
        """
        ...

    @overload
    def create_sprite(self, path: str, origin: OsbOrigin, position: Optional[Vector2], layer: Optional[str]) -> Sprite:
        """Create a sprite.

        Args:
            path: The texture path (relative to the mapset).
            origin: The sprite anchor origin.
            position: The initial position; omitted means the sprite's default.
            layer: The target layer name (omitted when `layerHandle` is set).

        Returns:
            The new sprite handle
        """
        ...

    @overload
    def create_sprite(self, options: CreateSpriteOptions) -> Sprite:
        """Create a sprite.

        Args:
            options: Create a sprite.

        Returns:
            The new sprite handle
        """
        ...

    def create_sprite(self, *args: object, **kwargs: object) -> Sprite:
        """Create a sprite.

        Returns:
            The new sprite handle
        """
        options = args[0] if args and isinstance(args[0], CreateSpriteOptions) else CreateSpriteOptions(*args, **kwargs)
        handle = self._api.create_sprite(CreateSpriteRequest(layer_handle=self._handle, **to_kwargs(options)))
        return Sprite(self._sprites, handle.layer, handle.sprite)

    def discard(self) -> int:
        """A segment handle request.

        Returns:
            The remaining top-level layer count
        """
        return self._api.discard(SegmentRequest(layer=self._handle)).layer_count

    def discard_sprite(self, sprite: int) -> bool:
        """A sprite handle pair.

        Args:
            sprite: The sprite handle.

        Returns:
            Whether the sprite was discarded
        """
        return self._api.discard_sprite(HandleRequest(layer=self._handle, sprite=sprite)).value

    def indexes(self) -> SegmentIndexesResult:
        """A segment handle request.

        Returns:
            The segment's indexes and bounds
        """
        return self._api.indexes(SegmentRequest(layer=self._handle))

    def move_to_bottom(self) -> None:
        """A segment handle request.

        Returns:
            Acknowledged
        """
        self._api.move_to_bottom(SegmentRequest(layer=self._handle))

    def move_to_top(self) -> None:
        """A segment handle request.

        Returns:
            Acknowledged
        """
        self._api.move_to_top(SegmentRequest(layer=self._handle))

    @overload
    def reverse_depth(self, value: bool) -> None:
        """Set a segment's reverse-depth flag.

        Args:
            value: Whether children are emitted in reverse order.

        Returns:
            Acknowledged
        """
        ...

    @overload
    def reverse_depth(self, options: SegmentReverseDepthOptions) -> None:
        """Set a segment's reverse-depth flag.

        Args:
            options: Set a segment's reverse-depth flag.

        Returns:
            Acknowledged
        """
        ...

    def reverse_depth(self, *args: object, **kwargs: object) -> None:
        """Set a segment's reverse-depth flag.

        Returns:
            Acknowledged
        """
        options = args[0] if args and isinstance(args[0], SegmentReverseDepthOptions) else SegmentReverseDepthOptions(*args, **kwargs)
        self._api.reverse_depth(SegmentReverseDepthRequest(layer=self._handle, **to_kwargs(options)))
