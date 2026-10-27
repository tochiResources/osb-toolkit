from __future__ import annotations

from ...api.layer_api import LayerAPI
from ...models.bool_result import BoolResult
from ...models.create_animation_request import CreateAnimationRequest
from ...models.create_sample_request import CreateSampleRequest
from ...models.create_sprite_request import CreateSpriteRequest
from ...models.handle_request import HandleRequest
from ...models.ok_result import OkResult
from ...models.osb_layer import OsbLayer
from ...models.osb_loop_type import OsbLoopType
from ...models.osb_origin import OsbOrigin
from ...models.segment_add_index_request import SegmentAddIndexRequest
from ...models.segment_change_position_request import SegmentChangePositionRequest
from ...models.segment_clean_result import SegmentCleanResult
from ...models.segment_discard_result import SegmentDiscardResult
from ...models.segment_indexes_result import SegmentIndexesResult
from ...models.segment_request import SegmentRequest
from ...models.segment_reverse_depth_request import SegmentReverseDepthRequest
from ...models.vector2 import Vector2
from ...api.sprite_api import SpriteAPI
from .sprite import Sprite
from ..options import CreateAnimationOptions, CreateSampleOptions, CreateSpriteOptions, SegmentAddIndexOptions, SegmentChangePositionOptions, SegmentReverseDepthOptions, to_kwargs
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
    def add_index(self, options: SegmentAddIndexOptions) -> OkResult: ...

    @overload
    def add_index(self, index: float) -> OkResult: ...

    def add_index(self, *args: object, **kwargs: object) -> OkResult:
        options = args[0] if args and isinstance(args[0], SegmentAddIndexOptions) else SegmentAddIndexOptions(*args, **kwargs)
        return self._api.add_index(SegmentAddIndexRequest(layer=self._handle, **to_kwargs(options)))

    @overload
    def change_position(self, options: SegmentChangePositionOptions) -> OkResult: ...

    @overload
    def change_position(self, osbLayer: OsbLayer) -> OkResult: ...

    def change_position(self, *args: object, **kwargs: object) -> OkResult:
        options = args[0] if args and isinstance(args[0], SegmentChangePositionOptions) else SegmentChangePositionOptions(*args, **kwargs)
        return self._api.change_position(SegmentChangePositionRequest(layer=self._handle, **to_kwargs(options)))

    def clean(self) -> SegmentCleanResult:
        return self._api.clean(SegmentRequest(layer=self._handle))

    @overload
    def create_animation(self, options: CreateAnimationOptions) -> Sprite: ...

    @overload
    def create_animation(self, path: str, frames: int, frameDelay: float, loopType: OsbLoopType, origin: OsbOrigin, position: Vector2, layer: Optional[str] = None) -> Sprite: ...

    def create_animation(self, *args: object, **kwargs: object) -> Sprite:
        options = args[0] if args and isinstance(args[0], CreateAnimationOptions) else CreateAnimationOptions(*args, **kwargs)
        handle = self._api.create_animation(CreateAnimationRequest(layer_handle=self._handle, **to_kwargs(options)))
        return Sprite(self._sprites, handle.layer, handle.sprite)

    @overload
    def create_sample(self, options: CreateSampleOptions) -> OkResult: ...

    @overload
    def create_sample(self, path: str, time: float, volume: Optional[float] = None, layer: Optional[str] = None) -> OkResult: ...

    def create_sample(self, *args: object, **kwargs: object) -> OkResult:
        options = args[0] if args and isinstance(args[0], CreateSampleOptions) else CreateSampleOptions(*args, **kwargs)
        return self._api.create_sample(CreateSampleRequest(layer_handle=self._handle, **to_kwargs(options)))

    def create_segment(self) -> Layer:
        handle = self._api.create_segment(SegmentRequest(layer=self._handle))
        return Layer(self._api, self._sprites, handle.layer)

    @overload
    def create_sprite(self, options: CreateSpriteOptions) -> Sprite: ...

    @overload
    def create_sprite(self, path: str, origin: OsbOrigin, position: Optional[Vector2] = None, layer: Optional[str] = None) -> Sprite: ...

    def create_sprite(self, *args: object, **kwargs: object) -> Sprite:
        options = args[0] if args and isinstance(args[0], CreateSpriteOptions) else CreateSpriteOptions(*args, **kwargs)
        handle = self._api.create_sprite(CreateSpriteRequest(layer_handle=self._handle, **to_kwargs(options)))
        return Sprite(self._sprites, handle.layer, handle.sprite)

    def discard(self) -> SegmentDiscardResult:
        return self._api.discard(SegmentRequest(layer=self._handle))

    def discard_sprite(self, sprite: int) -> BoolResult:
        return self._api.discard_sprite(HandleRequest(layer=self._handle, sprite=sprite))

    def indexes(self) -> SegmentIndexesResult:
        return self._api.indexes(SegmentRequest(layer=self._handle))

    def move_to_bottom(self) -> OkResult:
        return self._api.move_to_bottom(SegmentRequest(layer=self._handle))

    def move_to_top(self) -> OkResult:
        return self._api.move_to_top(SegmentRequest(layer=self._handle))

    @overload
    def reverse_depth(self, options: SegmentReverseDepthOptions) -> OkResult: ...

    @overload
    def reverse_depth(self, value: bool) -> OkResult: ...

    def reverse_depth(self, *args: object, **kwargs: object) -> OkResult:
        options = args[0] if args and isinstance(args[0], SegmentReverseDepthOptions) else SegmentReverseDepthOptions(*args, **kwargs)
        return self._api.reverse_depth(SegmentReverseDepthRequest(layer=self._handle, **to_kwargs(options)))
