//! The typed wire contract for the osbTK native bridge, and the OpenAPI schema
//! generated from it ().
//!
//! Rust is the source of truth: these types mirror the `osb-runtime` public API
//! and carry the doc comments that become OpenAPI `description`s, which the
//! per-language SDK models (C#, C++, Python, TypeScript) are generated from.

pub mod curve;
pub mod diagnostics;
pub mod enums;
pub mod helper;
pub mod image;
pub mod image_gen;
pub mod mapset;
pub mod math;
pub mod math_ops;
pub mod media;
pub mod model;
pub mod optimize;
pub mod request;
pub mod response;
pub mod storyboard;
pub mod storyboarding3d;

mod doc;
mod paths;

pub use curve::{
    ArcRequest, CurveKind, CurveSpec, EllipseRequest, LineIntersectRequest, OptionalPointResult,
};
pub use diagnostics::{Diagnostic, DiagnosticCode, DiagnosticReport, DiagnosticSeverity};
pub use doc::{openapi, openapi_json, OsbtkApi};
pub use enums::{Easing, HitObjectKind, OsbLayer, OsbLoopType, OsbOrigin, ParameterType};
pub use helper::{HelperRequest, HelperResult};
pub use image::{ImageCrop, ImageInfo, ImageOptions, ImageProcessRequest, PathRequest};
pub use image_gen::{RenderShapesRequest, RgbaColor, Shape, ShapeKind};
pub use mapset::{Beatmap, ControlPoint, HitObject};
pub use math::{Box2, Color4, Matrix4, Quaternion, Vector2, Vector3, Vector4};
pub use math_ops::{MathRequest, MathResult};
pub use media::{
    EncodeVideoRequest, ExtractAudioRequest, ExtractFramesRequest, FrameResult, MediaInfo,
};
pub use model::{AnimationMeta, Handle, LayerHandle, SpriteInfo, SpriteState};
pub use optimize::{OptimizeOptionsInput, OptimizeRequest, OptimizeResult, OptimizeStats};
pub use request::{
    ApplyEasingRequest, Bake3dRequest, CommandRequest, CreateAnimationRequest, CreateSampleRequest,
    CreateSpriteRequest, CurveDistanceRequest, CurveOffsetRequest, CurvePointRequest, CurveRequest,
    CurveSampleRequest, EasingValueRequest, FftRequest, GetFftRequest, GetOriginVectorRequest,
    HandleRequest, InScreenBoundsRequest, LayerHandleRequest, RandomRequest,
    SegmentAddIndexRequest, SegmentChangePositionRequest, SegmentRequest,
    SegmentReverseDepthRequest, SimplifyRequest, SpriteTimeRequest, StartLoopGroupRequest,
    StartTriggerGroupRequest, TextRequest, TimeRequest,
};
pub use response::{
    AudioDurationResult, BoolResult, EasingResult, ErrorResult, FftResult, ImportOsbResult,
    LoadBeatmapResult, OkResult, ParseOsbResult, PointListResult, RandomResult, ScalarResult,
    SegmentCleanResult, SegmentDiscardResult, SegmentIndexesResult, SpriteCountResult,
    VersionResult,
};
pub use storyboard::{
    CommandValue, ParameterValue, Storyboard, StoryboardCommand, StoryboardCommandKind,
    StoryboardLayer, StoryboardLoop, StoryboardSample, StoryboardSprite, StoryboardTrigger,
};
pub use storyboarding3d::{
    BakeOptionsInput, BakedCommand, BakedSprite, BakedStoryboard, BakedValue, Camera3dInput,
    CommandKind, Keyframe1, Keyframe2, Keyframe3, Node3dInput, Scene3dInput,
};
