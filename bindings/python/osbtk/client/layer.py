from __future__ import annotations

from ..api.layer_api import LayerAPI
from ..models.bool_result import BoolResult
from ..models.create_animation_request import CreateAnimationRequest
from ..models.create_sample_request import CreateSampleRequest
from ..models.create_sprite_request import CreateSpriteRequest
from ..models.handle_request import HandleRequest
from ..models.ok_result import OkResult
from ..models.segment_add_index_request import SegmentAddIndexRequest
from ..models.segment_change_position_request import SegmentChangePositionRequest
from ..models.segment_clean_result import SegmentCleanResult
from ..models.segment_discard_result import SegmentDiscardResult
from ..models.segment_indexes_result import SegmentIndexesResult
from ..models.segment_request import SegmentRequest
from ..models.segment_reverse_depth_request import SegmentReverseDepthRequest
from ..api.sprite_api import SpriteAPI
from .sprite import Sprite
from .options import CreateAnimationOptions, CreateSampleOptions, CreateSpriteOptions, SegmentAddIndexOptions, SegmentChangePositionOptions, SegmentReverseDepthOptions, to_kwargs


class Layer:
    def __init__(self, api: LayerAPI, sprites: SpriteAPI, handle: int) -> None:
        self._api = api
        self._sprites = sprites
        self._handle = handle

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Layer)

    def add_index(self, options: SegmentAddIndexOptions) -> OkResult:
        return self._api.add_index(SegmentAddIndexRequest(layer=self._handle, **to_kwargs(options)))

    def change_position(self, options: SegmentChangePositionOptions) -> OkResult:
        return self._api.change_position(SegmentChangePositionRequest(layer=self._handle, **to_kwargs(options)))

    def clean(self) -> SegmentCleanResult:
        return self._api.clean(SegmentRequest(layer=self._handle))

    def create_animation(self, options: CreateAnimationOptions) -> Sprite:
        handle = self._api.create_animation(CreateAnimationRequest(layer_handle=self._handle, **to_kwargs(options)))
        return Sprite(self._sprites, handle.layer, handle.sprite)

    def create_sample(self, options: CreateSampleOptions) -> OkResult:
        return self._api.create_sample(CreateSampleRequest(layer_handle=self._handle, **to_kwargs(options)))

    def create_segment(self) -> Layer:
        handle = self._api.create_segment(SegmentRequest(layer=self._handle))
        return Layer(self._api, self._sprites, handle.layer)

    def create_sprite(self, options: CreateSpriteOptions) -> Sprite:
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

    def reverse_depth(self, options: SegmentReverseDepthOptions) -> OkResult:
        return self._api.reverse_depth(SegmentReverseDepthRequest(layer=self._handle, **to_kwargs(options)))
