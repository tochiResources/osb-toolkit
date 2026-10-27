//! The OpenAPI document assembled from the wire types and operations. Generated
//! per-language SDK models and clients come from this document; it is checked in
//! at `bindings/openapi/osbtk.openapi.json` and verified by CI so it never drifts.

use utoipa::OpenApi;

use crate::{curve, enums, mapset, math, model, request, response, storyboard};

/// The OpenAPI 3.1 document describing the osbTK native bridge.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "osbTK Native Bridge",
        version = env!("CARGO_PKG_VERSION"),
        description = "Typed wire contract for the osbTK native engine (section 5.13, M21/M22). \
                       Rust (`crates/osb-api`) is the source of truth; the C#, C++, Python, \
                       TypeScript, Go and Java SDK models and clients are generated from this \
                       document. Every `operationId` is the native bridge `op`."
    ),
    paths(
        crate::paths::version,
        crate::paths::bake3d,
        crate::paths::parse_osu,
        crate::paths::parse_osb,
        crate::paths::fft,
        crate::paths::import_osb,
        crate::paths::load_beatmap,
        crate::paths::beatmap,
        crate::paths::timing_point_at,
        crate::paths::control_point_at,
        crate::paths::layer_handle,
        crate::paths::create_sprite,
        crate::paths::create_animation,
        crate::paths::command,
        crate::paths::start_loop_group,
        crate::paths::end_group,
        crate::paths::start_trigger_group,
        crate::paths::sprite_state_at,
        crate::paths::sprite_info,
        crate::paths::create_sample,
        crate::paths::create_segment,
        crate::paths::segment_reverse_depth,
        crate::paths::segment_add_index,
        crate::paths::segment_indexes,
        crate::paths::segment_change_position,
        crate::paths::segment_move_to_top,
        crate::paths::segment_move_to_bottom,
        crate::paths::segment_clean,
        crate::paths::segment_discard,
        crate::paths::segment_discard_sprite,
        crate::paths::get_fft,
        crate::paths::random,
        crate::paths::apply_easing,
        crate::paths::easing_value,
        crate::paths::sprite_count,
        crate::paths::storyboard,
        crate::paths::audio_duration,
        crate::paths::curve_length,
        crate::paths::curve_bounding_box,
        crate::paths::curve_point,
        crate::paths::curve_tangent,
        crate::paths::curve_sample,
        crate::paths::simplify,
        crate::paths::curve_normal,
        crate::paths::curve_curvature,
        crate::paths::curve_distance_to,
        crate::paths::curve_offset,
        crate::paths::curve_arc,
        crate::paths::curve_ellipse,
        crate::paths::line_intersect,
        crate::paths::diagnose_osb,
        crate::paths::diagnose_osu,
        crate::paths::optimize_osb,
        crate::paths::image_info,
        crate::paths::image_process,
        crate::paths::render_shapes,
        crate::paths::media_info,
        crate::paths::extract_frames,
        crate::paths::encode_video,
        crate::paths::extract_audio,
        crate::paths::get_origin_vector,
        crate::paths::in_screen_bounds,
        crate::paths::storyboarding3d_helper,
        crate::paths::storyboard_helper,
        crate::paths::helper
    ),
    components(schemas(
        math::Vector2,
        math::Vector3,
        math::Vector4,
        math::Box2,
        math::Color4,
        model::SpriteState,
        model::AnimationMeta,
        model::SpriteInfo,
        model::Handle,
        model::LayerHandle,
        storyboard::CommandValue,
        storyboard::ParameterValue,
        storyboard::StoryboardCommandKind,
        storyboard::StoryboardCommand,
        storyboard::StoryboardLoop,
        storyboard::StoryboardTrigger,
        storyboard::StoryboardSprite,
        storyboard::StoryboardSample,
        storyboard::StoryboardLayer,
        storyboard::Storyboard,
        enums::OsbOrigin,
        enums::OsbLayer,
        enums::OsbLoopType,
        enums::ParameterType,
        enums::Easing,
        enums::HitObjectKind,
        mapset::ControlPoint,
        mapset::HitObject,
        mapset::Beatmap,
        request::Bake3dRequest,
        request::TextRequest,
        request::FftRequest,
        request::LayerHandleRequest,
        request::CreateSpriteRequest,
        request::CreateAnimationRequest,
        request::CommandRequest,
        request::StartLoopGroupRequest,
        request::StartTriggerGroupRequest,
        request::HandleRequest,
        request::SpriteTimeRequest,
        request::CreateSampleRequest,
        request::SegmentRequest,
        request::SegmentReverseDepthRequest,
        request::SegmentAddIndexRequest,
        request::SegmentChangePositionRequest,
        request::GetFftRequest,
        request::TimeRequest,
        request::RandomRequest,
        request::ApplyEasingRequest,
        request::EasingValueRequest,
        request::CurveRequest,
        request::CurvePointRequest,
        request::CurveSampleRequest,
        request::CurveOffsetRequest,
        request::CurveDistanceRequest,
        request::SimplifyRequest,
        request::GetOriginVectorRequest,
        request::InScreenBoundsRequest,
        curve::CurveKind,
        curve::CurveSpec,
        curve::ArcRequest,
        curve::EllipseRequest,
        curve::LineIntersectRequest,
        curve::OptionalPointResult,
        crate::storyboarding3d::Keyframe1,
        crate::storyboarding3d::Keyframe2,
        crate::storyboarding3d::Keyframe3,
        crate::storyboarding3d::Node3dInput,
        crate::storyboarding3d::Camera3dInput,
        crate::storyboarding3d::BakeOptionsInput,
        crate::storyboarding3d::Scene3dInput,
        crate::storyboarding3d::CommandKind,
        crate::storyboarding3d::BakedValue,
        crate::storyboarding3d::BakedCommand,
        crate::storyboarding3d::BakedSprite,
        crate::storyboarding3d::BakedStoryboard,
        response::VersionResult,
        response::ImportOsbResult,
        response::ParseOsbResult,
        response::LoadBeatmapResult,
        response::SpriteCountResult,
        response::SegmentIndexesResult,
        response::SegmentCleanResult,
        response::SegmentDiscardResult,
        response::AudioDurationResult,
        response::RandomResult,
        response::EasingResult,
        response::OkResult,
        response::FftResult,
        response::ScalarResult,
        response::BoolResult,
        response::PointListResult,
        response::ErrorResult,
    crate::diagnostics::DiagnosticSeverity,
    crate::diagnostics::DiagnosticCode,
    crate::diagnostics::Diagnostic,
        crate::diagnostics::DiagnosticReport,
        crate::optimize::OptimizeOptionsInput,
        crate::optimize::OptimizeStats,
        crate::optimize::OptimizeRequest,
        crate::optimize::OptimizeResult,
        crate::image::ImageInfo,
        crate::image::ImageCrop,
        crate::image::ImageOptions,
        crate::image::ImageProcessRequest,
        crate::image::PathRequest,
        crate::image_gen::RgbaColor,
        crate::image_gen::ShapeKind,
        crate::image_gen::Shape,
        crate::image_gen::RenderShapesRequest,
        crate::media::MediaInfo,
        crate::media::ExtractFramesRequest,
        crate::media::FrameResult,
        crate::media::EncodeVideoRequest,
        crate::media::ExtractAudioRequest,
        crate::helper::HelperRequest,
        crate::helper::HelperResult
    ))
)]
pub struct OsbtkApi;

/// The OpenAPI document as a `utoipa` value.
pub fn openapi() -> utoipa::openapi::OpenApi {
    OsbtkApi::openapi()
}

/// The OpenAPI document serialized as pretty JSON.
pub fn openapi_json() -> String {
    openapi()
        .to_pretty_json()
        .unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_lists_the_expected_schemas_and_paths() {
        let value: serde_json::Value = serde_json::from_str(&openapi_json()).unwrap();
        let schemas = &value["components"]["schemas"];
        for name in [
            "Beatmap",
            "SpriteState",
            "SpriteInfo",
            "Easing",
            "OsbOrigin",
        ] {
            assert!(schemas.get(name).is_some(), "missing schema: {name}");
        }
        assert_eq!(value["openapi"], "3.1.0");
        assert!(value["paths"]["/runtime/Layer.createSprite"]["post"].is_object());
        assert_eq!(
            value["paths"]["/runtime/Layer.createSprite"]["post"]["operationId"],
            "Layer.createSprite"
        );
    }

    #[test]
    fn doc_comments_become_descriptions() {
        let value: serde_json::Value = serde_json::from_str(&openapi_json()).unwrap();
        assert!(value["components"]["schemas"]["SpriteState"]["description"].is_string());
        assert!(
            value["components"]["schemas"]["SpriteState"]["properties"]["position"]["description"]
                .is_string()
        );
    }
}
