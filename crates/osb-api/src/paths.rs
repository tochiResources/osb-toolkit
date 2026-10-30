//! The bridge operations as OpenAPI paths. Each `operation_id` is exactly the
//! native `op` string, so the generated SDK clients and the custom bridge
//! transport agree on one name. The functions are documentation-only: they are
//! never called, they exist so `utoipa` can derive the `paths` object.
//!
//! The stateless surface (version, parsing, easing, curves and origin math) is
//! tagged `Core` so every SDK emits one grouped `Core` class. The operation IDs
//! keep their historical `Common.*`/`Curve.*`/... bridge names until the
//! class-scoped naming pass (M31a) renames them.

use crate::curve::{ArcRequest, EllipseRequest, LineIntersectRequest, OptionalPointResult};
use crate::diagnostics::DiagnosticReport;
use crate::helper::{HelperRequest, HelperResult};
use crate::image::{ImageInfo, ImageProcessRequest, PathRequest};
use crate::image_gen::RenderShapesRequest;
use crate::mapset::{Beatmap, ControlPoint};
use crate::math::{Box2, Vector2};
use crate::media::{
    EncodeVideoRequest, ExtractAudioRequest, ExtractFramesRequest, FrameResult, MediaInfo,
};
use crate::model::{Handle, LayerHandle, SpriteInfo, SpriteState};
use crate::optimize::{OptimizeRequest, OptimizeResult};
use crate::request::{
    ApplyEasingRequest, Bake3dRequest, CommandRequest, CreateAnimationRequest, CreateSampleRequest,
    CreateSpriteRequest, CurveDistanceRequest, CurveOffsetRequest, CurvePointRequest, CurveRequest,
    CurveSampleRequest, EasingValueRequest, FftRequest, GetFftRequest, GetOriginVectorRequest,
    HandleRequest, InScreenBoundsRequest, LayerHandleRequest, RandomRequest,
    SegmentAddIndexRequest, SegmentChangePositionRequest, SegmentRequest,
    SegmentReverseDepthRequest, SimplifyRequest, SpriteTimeRequest, StartLoopGroupRequest,
    StartTriggerGroupRequest, TextRequest, TimeRequest,
};
use crate::response::{
    AudioDurationResult, BoolResult, EasingResult, FftResult, ImportOsbResult, LoadBeatmapResult,
    OkResult, ParseOsbResult, PointListResult, RandomResult, ScalarResult, SegmentCleanResult,
    SegmentDiscardResult, SegmentIndexesResult, SpriteCountResult, VersionResult,
};
use crate::storyboard::Storyboard;
use crate::storyboarding3d::BakedStoryboard;

#[utoipa::path(
    post,
    path = "/bridge/Common.version",
    tag = "Core",
    operation_id = "Common.version",
    responses((status = 200, description = "The runtime version", body = VersionResult))
)]
#[allow(dead_code)]
pub(crate) fn version() {}

#[utoipa::path(
    post,
    path = "/bridge/Storyboarding3d.bake3d",
    tag = "Storyboarding3d",
    operation_id = "Storyboarding3d.bake3d",
    request_body = Bake3dRequest,
    responses((status = 200, description = "Baked command spans", body = BakedStoryboard))
)]
#[allow(dead_code)]
pub(crate) fn bake3d() {}

#[utoipa::path(
    post,
    path = "/bridge/Parsing.parseOsu",
    tag = "Core",
    operation_id = "Parsing.parseOsu",
    request_body = TextRequest,
    responses((status = 200, description = "The parsed beatmap", body = Beatmap))
)]
#[allow(dead_code)]
pub(crate) fn parse_osu() {}

#[utoipa::path(
    post,
    path = "/bridge/Parsing.parseOsb",
    tag = "Core",
    operation_id = "Parsing.parseOsb",
    request_body = TextRequest,
    responses((status = 200, description = "The storyboard summary", body = ParseOsbResult))
)]
#[allow(dead_code)]
pub(crate) fn parse_osb() {}

#[utoipa::path(
    post,
    path = "/bridge/Audio.fft",
    tag = "Audio",
    operation_id = "Audio.fft",
    request_body = FftRequest,
    responses((status = 200, description = "The FFT magnitudes", body = FftResult))
)]
#[allow(dead_code)]
pub(crate) fn fft() {}

#[utoipa::path(
    post,
    path = "/runtime/Generator.importOsb",
    tag = "Generator",
    operation_id = "Generator.importOsb",
    request_body = TextRequest,
    responses((status = 200, description = "The sprite count after import", body = ImportOsbResult))
)]
#[allow(dead_code)]
pub(crate) fn import_osb() {}

#[utoipa::path(
    post,
    path = "/runtime/Generator.loadBeatmap",
    tag = "Generator",
    operation_id = "Generator.loadBeatmap",
    request_body = TextRequest,
    responses((status = 200, description = "A summary of the loaded beatmap", body = LoadBeatmapResult))
)]
#[allow(dead_code)]
pub(crate) fn load_beatmap() {}

#[utoipa::path(
    post,
    path = "/runtime/Generator.beatmap",
    tag = "Generator",
    operation_id = "Generator.beatmap",
    responses((status = 200, description = "The loaded beatmap, or null", body = Beatmap))
)]
#[allow(dead_code)]
pub(crate) fn beatmap() {}

#[utoipa::path(
    post,
    path = "/runtime/Generator.timingPointAt",
    tag = "Generator",
    operation_id = "Generator.timingPointAt",
    request_body = TimeRequest,
    responses((status = 200, description = "The active red timing point", body = ControlPoint))
)]
#[allow(dead_code)]
pub(crate) fn timing_point_at() {}

#[utoipa::path(
    post,
    path = "/runtime/Generator.controlPointAt",
    tag = "Generator",
    operation_id = "Generator.controlPointAt",
    request_body = TimeRequest,
    responses((status = 200, description = "The active control point, or null", body = ControlPoint))
)]
#[allow(dead_code)]
pub(crate) fn control_point_at() {}

#[utoipa::path(
    post,
    path = "/runtime/Generator.layerHandle",
    tag = "Generator",
    operation_id = "Generator.layerHandle",
    request_body = LayerHandleRequest,
    responses((status = 200, description = "The layer handle", body = LayerHandle))
)]
#[allow(dead_code)]
pub(crate) fn layer_handle() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.createSprite",
    tag = "Layer",
    operation_id = "Layer.createSprite",
    request_body = CreateSpriteRequest,
    responses((status = 200, description = "The new sprite handle", body = Handle))
)]
#[allow(dead_code)]
pub(crate) fn create_sprite() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.createAnimation",
    tag = "Layer",
    operation_id = "Layer.createAnimation",
    request_body = CreateAnimationRequest,
    responses((status = 200, description = "The new animation handle", body = Handle))
)]
#[allow(dead_code)]
pub(crate) fn create_animation() {}

#[utoipa::path(
    post,
    path = "/runtime/Sprite.command",
    tag = "Sprite",
    operation_id = "Sprite.command",
    request_body = CommandRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn command() {}

#[utoipa::path(
    post,
    path = "/runtime/Sprite.startLoopGroup",
    tag = "Sprite",
    operation_id = "Sprite.startLoopGroup",
    request_body = StartLoopGroupRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn start_loop_group() {}

#[utoipa::path(
    post,
    path = "/runtime/Sprite.endGroup",
    tag = "Sprite",
    operation_id = "Sprite.endGroup",
    request_body = HandleRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn end_group() {}

#[utoipa::path(
    post,
    path = "/runtime/Sprite.startTriggerGroup",
    tag = "Sprite",
    operation_id = "Sprite.startTriggerGroup",
    request_body = StartTriggerGroupRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn start_trigger_group() {}

#[utoipa::path(
    post,
    path = "/runtime/Sprite.stateAt",
    tag = "Sprite",
    operation_id = "Sprite.stateAt",
    request_body = SpriteTimeRequest,
    responses((status = 200, description = "The sampled sprite state", body = SpriteState))
)]
#[allow(dead_code)]
pub(crate) fn sprite_state_at() {}

#[utoipa::path(
    post,
    path = "/runtime/Sprite.info",
    tag = "Sprite",
    operation_id = "Sprite.info",
    request_body = HandleRequest,
    responses((status = 200, description = "The sprite's initial state", body = SpriteInfo))
)]
#[allow(dead_code)]
pub(crate) fn sprite_info() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.createSample",
    tag = "Layer",
    operation_id = "Layer.createSample",
    request_body = CreateSampleRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn create_sample() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.createSegment",
    tag = "Layer",
    operation_id = "Layer.createSegment",
    request_body = SegmentRequest,
    responses((status = 200, description = "The new segment handle", body = LayerHandle))
)]
#[allow(dead_code)]
pub(crate) fn create_segment() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.reverseDepth",
    tag = "Layer",
    operation_id = "Layer.reverseDepth",
    request_body = SegmentReverseDepthRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn segment_reverse_depth() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.addIndex",
    tag = "Layer",
    operation_id = "Layer.addIndex",
    request_body = SegmentAddIndexRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn segment_add_index() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.indexes",
    tag = "Layer",
    operation_id = "Layer.indexes",
    request_body = SegmentRequest,
    responses((status = 200, description = "The segment's indexes and bounds", body = SegmentIndexesResult))
)]
#[allow(dead_code)]
pub(crate) fn segment_indexes() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.changePosition",
    tag = "Layer",
    operation_id = "Layer.changePosition",
    request_body = SegmentChangePositionRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn segment_change_position() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.moveToTop",
    tag = "Layer",
    operation_id = "Layer.moveToTop",
    request_body = SegmentRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn segment_move_to_top() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.moveToBottom",
    tag = "Layer",
    operation_id = "Layer.moveToBottom",
    request_body = SegmentRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn segment_move_to_bottom() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.clean",
    tag = "Layer",
    operation_id = "Layer.clean",
    request_body = SegmentRequest,
    responses((status = 200, description = "The number of sprites removed", body = SegmentCleanResult))
)]
#[allow(dead_code)]
pub(crate) fn segment_clean() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.discard",
    tag = "Layer",
    operation_id = "Layer.discard",
    request_body = SegmentRequest,
    responses((status = 200, description = "The remaining top-level layer count", body = SegmentDiscardResult))
)]
#[allow(dead_code)]
pub(crate) fn segment_discard() {}

#[utoipa::path(
    post,
    path = "/runtime/Layer.discardSprite",
    tag = "Layer",
    operation_id = "Layer.discardSprite",
    request_body = HandleRequest,
    responses((status = 200, description = "Whether the sprite was discarded", body = BoolResult))
)]
#[allow(dead_code)]
pub(crate) fn segment_discard_sprite() {}

#[utoipa::path(
    post,
    path = "/bridge/Audio.getFft",
    tag = "Audio",
    operation_id = "Audio.getFft",
    request_body = GetFftRequest,
    responses((status = 200, description = "The FFT magnitudes", body = FftResult))
)]
#[allow(dead_code)]
pub(crate) fn get_fft() {}

#[utoipa::path(
    post,
    path = "/runtime/Generator.random",
    tag = "Generator",
    operation_id = "Generator.random",
    request_body = RandomRequest,
    responses((status = 200, description = "A deterministic random value", body = RandomResult))
)]
#[allow(dead_code)]
pub(crate) fn random() {}

#[utoipa::path(
    post,
    path = "/bridge/Animations.applyEasing",
    tag = "Core",
    operation_id = "Animations.applyEasing",
    request_body = ApplyEasingRequest,
    responses((status = 200, description = "The eased value", body = EasingResult))
)]
#[allow(dead_code)]
pub(crate) fn apply_easing() {}

#[utoipa::path(
    post,
    path = "/bridge/Animations.easingValue",
    tag = "Core",
    operation_id = "Animations.easingValue",
    request_body = EasingValueRequest,
    responses((status = 200, description = "The eased value (named easing or cubic-bezier)", body = EasingResult))
)]
#[allow(dead_code)]
pub(crate) fn easing_value() {}

#[utoipa::path(
    post,
    path = "/runtime/Generator.storyboard",
    tag = "Generator",
    operation_id = "Generator.storyboard",
    responses((status = 200, description = "The built storyboard model", body = Storyboard))
)]
#[allow(dead_code)]
pub(crate) fn storyboard() {}

#[utoipa::path(
    post,
    path = "/runtime/Generator.spriteCount",
    tag = "Generator",
    operation_id = "Generator.spriteCount",
    responses((status = 200, description = "The storyboard sprite count", body = SpriteCountResult))
)]
#[allow(dead_code)]
pub(crate) fn sprite_count() {}
#[utoipa::path(
    post,
    path = "/bridge/Audio.audioDuration",
    tag = "Audio",
    operation_id = "Audio.audioDuration",
    responses((status = 200, description = "The loaded audio duration", body = AudioDurationResult))
)]
#[allow(dead_code)]
pub(crate) fn audio_duration() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.length",
    tag = "Core",
    operation_id = "Curve.length",
    request_body = CurveRequest,
    responses((status = 200, description = "The curve's arc length", body = ScalarResult))
)]
#[allow(dead_code)]
pub(crate) fn curve_length() {}
#[utoipa::path(
    post,
    path = "/bridge/Curve.boundingBox",
    tag = "Core",
    operation_id = "Curve.boundingBox",
    request_body = CurveRequest,
    responses((status = 200, description = "The curve's axis-aligned bounding box", body = Box2))
)]
#[allow(dead_code)]
pub(crate) fn curve_bounding_box() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.point",
    tag = "Core",
    operation_id = "Curve.point",
    request_body = CurvePointRequest,
    responses((status = 200, description = "The point at `t`, parametrically or by arc length", body = Vector2))
)]
#[allow(dead_code)]
pub(crate) fn curve_point() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.tangent",
    tag = "Core",
    operation_id = "Curve.tangent",
    request_body = CurvePointRequest,
    responses((status = 200, description = "The unit tangent at `t`", body = Vector2))
)]
#[allow(dead_code)]
pub(crate) fn curve_tangent() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.sample",
    tag = "Core",
    operation_id = "Curve.sample",
    request_body = CurveSampleRequest,
    responses((status = 200, description = "`count` points along the curve", body = PointListResult))
)]
#[allow(dead_code)]
pub(crate) fn curve_sample() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.simplify",
    tag = "Core",
    operation_id = "Curve.simplify",
    request_body = SimplifyRequest,
    responses((status = 200, description = "The simplified polyline", body = PointListResult))
)]
#[allow(dead_code)]
pub(crate) fn simplify() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.normal",
    tag = "Core",
    operation_id = "Curve.normal",
    request_body = CurvePointRequest,
    responses((status = 200, description = "The unit normal at `t`", body = Vector2))
)]
#[allow(dead_code)]
pub(crate) fn curve_normal() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.curvature",
    tag = "Core",
    operation_id = "Curve.curvature",
    request_body = CurvePointRequest,
    responses((status = 200, description = "The curvature at `t`", body = ScalarResult))
)]
#[allow(dead_code)]
pub(crate) fn curve_curvature() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.distanceTo",
    tag = "Core",
    operation_id = "Curve.distanceTo",
    request_body = CurveDistanceRequest,
    responses((status = 200, description = "The distance from a point to the curve", body = ScalarResult))
)]
#[allow(dead_code)]
pub(crate) fn curve_distance_to() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.offset",
    tag = "Core",
    operation_id = "Curve.offset",
    request_body = CurveOffsetRequest,
    responses((status = 200, description = "The polyline offset along the curve's normal", body = PointListResult))
)]
#[allow(dead_code)]
pub(crate) fn curve_offset() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.arc",
    tag = "Core",
    operation_id = "Curve.arc",
    request_body = ArcRequest,
    responses((status = 200, description = "Points along a circular arc", body = PointListResult))
)]
#[allow(dead_code)]
pub(crate) fn curve_arc() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.ellipse",
    tag = "Core",
    operation_id = "Curve.ellipse",
    request_body = EllipseRequest,
    responses((status = 200, description = "Points around an ellipse", body = PointListResult))
)]
#[allow(dead_code)]
pub(crate) fn curve_ellipse() {}

#[utoipa::path(
    post,
    path = "/bridge/Curve.lineIntersect",
    tag = "Core",
    operation_id = "Curve.lineIntersect",
    request_body = LineIntersectRequest,
    responses((status = 200, description = "The intersection point, or null", body = OptionalPointResult))
)]
#[allow(dead_code)]
pub(crate) fn line_intersect() {}

#[utoipa::path(
    post,
    path = "/bridge/Diagnostics.diagnoseOsb",
    tag = "Diagnostics",
    operation_id = "Diagnostics.diagnoseOsb",
    request_body = TextRequest,
    responses((status = 200, description = "Diagnostics for an `.osb` storyboard", body = DiagnosticReport))
)]
#[allow(dead_code)]
pub(crate) fn diagnose_osb() {}

#[utoipa::path(
    post,
    path = "/bridge/Diagnostics.diagnoseOsu",
    tag = "Diagnostics",
    operation_id = "Diagnostics.diagnoseOsu",
    request_body = TextRequest,
    responses((status = 200, description = "Diagnostics for an `.osu` beatmap", body = DiagnosticReport))
)]
#[allow(dead_code)]
pub(crate) fn diagnose_osu() {}

#[utoipa::path(
    post,
    path = "/bridge/Optimize.optimizeOsb",
    tag = "Optimize",
    operation_id = "Optimize.optimizeOsb",
    request_body = OptimizeRequest,
    responses((status = 200, description = "The optimized `.osb` text and stats", body = OptimizeResult))
)]
#[allow(dead_code)]
pub(crate) fn optimize_osb() {}

#[utoipa::path(
    post,
    path = "/bridge/Image.info",
    tag = "Image",
    operation_id = "Image.info",
    request_body = PathRequest,
    responses((status = 200, description = "The image's metadata", body = ImageInfo))
)]
#[allow(dead_code)]
pub(crate) fn image_info() {}

#[utoipa::path(
    post,
    path = "/bridge/Image.process",
    tag = "Image",
    operation_id = "Image.process",
    request_body = ImageProcessRequest,
    responses((status = 200, description = "The processed image's metadata", body = ImageInfo))
)]
#[allow(dead_code)]
pub(crate) fn image_process() {}

#[utoipa::path(
    post,
    path = "/bridge/Imagegen.renderShapes",
    tag = "Imagegen",
    operation_id = "Imagegen.renderShapes",
    request_body = RenderShapesRequest,
    responses((status = 200, description = "The rendered image's metadata", body = ImageInfo))
)]
#[allow(dead_code)]
pub(crate) fn render_shapes() {}

#[utoipa::path(
    post,
    path = "/bridge/Media.info",
    tag = "Media",
    operation_id = "Media.info",
    request_body = PathRequest,
    responses((status = 200, description = "The media metadata", body = MediaInfo))
)]
#[allow(dead_code)]
pub(crate) fn media_info() {}

#[utoipa::path(
    post,
    path = "/bridge/Media.extractFrames",
    tag = "Media",
    operation_id = "Media.extractFrames",
    request_body = ExtractFramesRequest,
    responses((status = 200, description = "The extracted frame count", body = FrameResult))
)]
#[allow(dead_code)]
pub(crate) fn extract_frames() {}

#[utoipa::path(
    post,
    path = "/bridge/Media.encodeVideo",
    tag = "Media",
    operation_id = "Media.encodeVideo",
    request_body = EncodeVideoRequest,
    responses((status = 200, description = "The encoded video's metadata", body = MediaInfo))
)]
#[allow(dead_code)]
pub(crate) fn encode_video() {}

#[utoipa::path(
    post,
    path = "/bridge/Media.extractAudio",
    tag = "Media",
    operation_id = "Media.extractAudio",
    request_body = ExtractAudioRequest,
    responses((status = 200, description = "Acknowledged", body = OkResult))
)]
#[allow(dead_code)]
pub(crate) fn extract_audio() {}

#[utoipa::path(
    post,
    path = "/bridge/Origin.getOriginVector",
    tag = "Core",
    operation_id = "Origin.getOriginVector",
    request_body = GetOriginVectorRequest,
    responses((status = 200, description = "The anchor offset for the origin", body = Vector2))
)]
#[allow(dead_code)]
pub(crate) fn get_origin_vector() {}

#[utoipa::path(
    post,
    path = "/bridge/Origin.inScreenBounds",
    tag = "Core",
    operation_id = "Origin.inScreenBounds",
    request_body = InScreenBoundsRequest,
    responses((status = 200, description = "Whether the sprite's bounds intersect the screen", body = BoolResult))
)]
#[allow(dead_code)]
pub(crate) fn in_screen_bounds() {}

#[utoipa::path(
    post,
    path = "/bridge/Helpers.keyframes",
    tag = "Helpers",
    operation_id = "Helpers.keyframes",
    request_body = HelperRequest,
    responses((status = 200, description = "The helper's result (a keyframe channel or vectors)", body = HelperResult))
)]
#[allow(dead_code)]
pub(crate) fn storyboarding3d_helper() {}

#[utoipa::path(
    post,
    path = "/bridge/Helpers.call",
    tag = "Helpers",
    operation_id = "Helpers.call",
    request_body = HelperRequest,
    responses((status = 200, description = "The shared motion-graphics helper's result", body = HelperResult))
)]
#[allow(dead_code)]
pub(crate) fn helper() {}

#[utoipa::path(
    post,
    path = "/bridge/Helpers.commands",
    tag = "Helpers",
    operation_id = "Helpers.commands",
    request_body = HelperRequest,
    responses((status = 200, description = "The helper's result (a command list or scalar)", body = HelperResult))
)]
#[allow(dead_code)]
pub(crate) fn storyboard_helper() {}
