from __future__ import annotations

from ..api.layer_api import LayerAPI
from ..models.bool_result import BoolResult
from ..models.create_animation_request import CreateAnimationRequest
from ..models.create_sample_request import CreateSampleRequest
from ..models.create_sprite_request import CreateSpriteRequest
from ..models.handle import Handle
from ..models.handle_request import HandleRequest
from ..models.layer_handle import LayerHandle
from ..models.ok_result import OkResult
from ..models.segment_add_index_request import SegmentAddIndexRequest
from ..models.segment_change_position_request import SegmentChangePositionRequest
from ..models.segment_clean_result import SegmentCleanResult
from ..models.segment_discard_result import SegmentDiscardResult
from ..models.segment_indexes_result import SegmentIndexesResult
from ..models.segment_request import SegmentRequest
from ..models.segment_reverse_depth_request import SegmentReverseDepthRequest


class Layer:
    def __init__(self, api: LayerAPI) -> None:
        self._api = api

    @staticmethod
    def instance_of(value: object) -> bool:
        return isinstance(value, Layer)

    def add_index(self, body: SegmentAddIndexRequest) -> OkResult:
        return self._api.add_index(body)

    def change_position(self, body: SegmentChangePositionRequest) -> OkResult:
        return self._api.change_position(body)

    def clean(self, body: SegmentRequest) -> SegmentCleanResult:
        return self._api.clean(body)

    def create_animation(self, body: CreateAnimationRequest) -> Handle:
        return self._api.create_animation(body)

    def create_sample(self, body: CreateSampleRequest) -> OkResult:
        return self._api.create_sample(body)

    def create_segment(self, body: SegmentRequest) -> LayerHandle:
        return self._api.create_segment(body)

    def create_sprite(self, body: CreateSpriteRequest) -> Handle:
        return self._api.create_sprite(body)

    def discard(self, body: SegmentRequest) -> SegmentDiscardResult:
        return self._api.discard(body)

    def discard_sprite(self, body: HandleRequest) -> BoolResult:
        return self._api.discard_sprite(body)

    def indexes(self, body: SegmentRequest) -> SegmentIndexesResult:
        return self._api.indexes(body)

    def move_to_bottom(self, body: SegmentRequest) -> OkResult:
        return self._api.move_to_bottom(body)

    def move_to_top(self, body: SegmentRequest) -> OkResult:
        return self._api.move_to_top(body)

    def reverse_depth(self, body: SegmentReverseDepthRequest) -> OkResult:
        return self._api.reverse_depth(body)
