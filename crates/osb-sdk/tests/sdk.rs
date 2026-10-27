//! The generated Rust SDK: models from the OpenAPI contract driven through the
//! native bridge, with the shared golden corpus for parity.

use osb_sdk::models::{Easing, HitObjectKind, OsbOrigin, Vector2};
use osb_sdk::{Client, Runtime};

const OSU: &str = "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n\
[TimingPoints]\n0,500,4,2,1,100,1,0\n\
[HitObjects]\n256,192,1000,1,0\n";

#[test]
fn exposes_the_native_version() {
    assert!(!Client::version().unwrap().is_empty());
}

#[test]
fn parses_a_beatmap_into_the_typed_model() {
    let beatmap = Client::parse_beatmap(OSU).unwrap();
    assert_eq!(beatmap.audio_filename, "song.mp3");
    assert_eq!(beatmap.hit_objects[0].kind, HitObjectKind::Circle);
    assert!((beatmap.timing_points[0].bpm - 120.0).abs() < 1e-6);
}

#[test]
fn applies_easing() {
    assert!((Client::apply_easing(Easing::InQuad, 0.5).unwrap() - 0.25).abs() < 1e-6);
}

#[test]
fn nests_the_libraries() {
    let beatmap = osb_sdk::storyboarding::parse_osu(OSU).unwrap();
    assert_eq!(beatmap.audio_filename, "song.mp3");
    assert!((osb_sdk::animations::apply_easing(Easing::InQuad, 0.5).unwrap() - 0.25).abs() < 1e-6);
}

#[test]
fn storyboarding3d_reuses_the_storyboarding_surface() {
    // M28: the 3D library is a sibling that reuses the storyboarding surface.
    let beatmap = osb_sdk::storyboarding3d::parse_osu(OSU).unwrap();
    assert_eq!(beatmap.audio_filename, "song.mp3");
    let summary = osb_sdk::storyboarding3d::parse_osb("").unwrap();
    assert_eq!(summary.sprite_count, 0);
}

#[test]
fn drives_the_storyboard_through_the_bridge() {
    let mut runtime = Runtime::new(640.0, 480.0);
    let handle = runtime
        .create_sprite("Foreground", "sb/dot.png", OsbOrigin::Centre)
        .unwrap();
    runtime
        .move_(
            &handle,
            0.0,
            1000.0,
            Vector2::new(0.0, 0.0),
            Vector2::new(100.0, 0.0),
            Easing::None,
        )
        .unwrap();
    let state = runtime.sprite_state_at(&handle, 500.0).unwrap();
    assert!((state.position.x - 50.0).abs() < 0.1);
    assert_eq!(runtime.sprite_count().unwrap(), 1);
}

#[test]
fn nests_segments_through_the_object_facade() {
    let mut runtime = Runtime::new(640.0, 480.0);
    {
        let mut group = runtime.layer("Group").unwrap();
        group
            .create_sprite("sb/group.png", OsbOrigin::Centre)
            .unwrap();
        let mut child = group.create_segment().unwrap();
        child
            .create_sprite("sb/child.png", OsbOrigin::Centre)
            .unwrap();
        child.set_reverse_depth(true).unwrap();
        child.add_index(3.0).unwrap();
        child.add_index(-1.0).unwrap();
        let indexes = child.indexes().unwrap();
        assert_eq!(indexes.highest, 3.0);
        assert_eq!(indexes.lowest, -1.0);
    }
    assert_eq!(runtime.sprite_count().unwrap(), 2);

    {
        let group = runtime.layer("Group").unwrap();
        let child = group.handle();
        runtime.segment_discard(&child).unwrap();
    }
    assert_eq!(runtime.sprite_count().unwrap(), 1);
}

#[test]
fn computes_origin_vectors_and_screen_bounds() {
    let origin =
        osb_sdk::storyboarding::get_origin_vector(osb_sdk::models::OsbOrigin::Centre, 100.0, 100.0)
            .unwrap();
    assert!((origin.x - 50.0).abs() < 1e-6);
    assert!((origin.y - 50.0).abs() < 1e-6);

    let in_bounds = osb_sdk::storyboarding::in_screen_bounds(
        Vector2::new(427.0, 240.0),
        Vector2::new(100.0, 100.0),
        0.0,
        Vector2::new(50.0, 50.0),
    )
    .unwrap();
    assert!(in_bounds);
}

#[test]
fn surfaces_native_errors() {
    let error = Client::fft("does-not-exist.wav", 0.0, 64).unwrap_err();
    assert!(matches!(error, osb_sdk::Error::Bridge(_)));
}

#[test]
fn surfaces_image_errors() {
    let error = osb_sdk::image::info("does-not-exist.png").unwrap_err();
    assert!(matches!(error, osb_sdk::Error::Bridge(_)));
}

#[test]
fn probes_and_extracts_media() {
    use std::process::Command;
    let ffmpeg = Command::new("ffmpeg").arg("-version").output();
    if !ffmpeg
        .map(|output| output.status.success())
        .unwrap_or(false)
    {
        return;
    }
    let directory = osb_core::io::temp_dir().join("osbtk-sdk-media");
    osb_core::io::create_dir_all(&directory).unwrap();
    let clip = directory.join("clip.mp4");
    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-v",
            "error",
            "-f",
            "lavfi",
            "-i",
            "testsrc=duration=1:size=32x32:rate=10",
            clip.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(status.success());

    let info = osb_sdk::media::info(clip.to_str().unwrap()).unwrap();
    assert_eq!(info.width, 32);
    assert!(info.duration > 0.5);

    let frames = osb_sdk::media::extract_frames(osb_sdk::models::ExtractFramesRequest {
        input: clip.to_str().unwrap().to_string(),
        output_dir: directory.join("frames").to_str().unwrap().to_string(),
        fps: Some(Some(5.0)),
        start: None,
        count: Some(Some(2)),
        width: None,
        height: None,
    })
    .unwrap();
    assert_eq!(frames.count, 2);
}

#[test]
fn renders_shapes() {
    use osb_sdk::models::{RenderShapesRequest, RgbaColor, Shape, ShapeKind, Vector2};

    let output = osb_core::io::temp_dir().join("osbtk-sdk-render.png");
    let request = RenderShapesRequest {
        width: 32,
        height: 32,
        background: None,
        shapes: vec![Shape {
            kind: ShapeKind::Rect,
            center: Box::new(Vector2 { x: 16.0, y: 16.0 }),
            size: Box::new(Vector2 { x: 20.0, y: 20.0 }),
            corner_radius: None,
            points: None,
            fill: Some(Some(Box::new(RgbaColor {
                r: 10,
                g: 20,
                b: 30,
                a: 255,
            }))),
            stroke: None,
            stroke_width: None,
        }],
        output: output.to_str().unwrap().to_string(),
    };
    let info = osb_sdk::image_gen::render_shapes(request).unwrap();
    assert_eq!(info.width, 32);
    assert_eq!(info.height, 32);
    assert!(output.exists());
}

#[test]
fn diagnoses_storyboards_and_mapsets() {
    let report = osb_sdk::diagnostics::diagnose_osu(
        "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n",
    )
    .unwrap();
    assert!(report.error_count >= 1);

    let report = osb_sdk::diagnostics::diagnose_osb(
        "[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n F,0,0,100,0,1\n",
    )
    .unwrap();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
}

#[test]
fn optimizes_storyboards() {
    use osb_sdk::models::OptimizeOptionsInput;

    let result = osb_sdk::optimize::optimize_osb(
        "[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n F,0,0,100,0,1\n F,0,0,100,0,1\n",
        OptimizeOptionsInput {
            deduplicate: Some(Some(true)),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(result.stats.deduplicated, 1);
    assert!(result.osb.contains("Sprite,Foreground"));
}

#[test]
fn drives_the_curve_library() {
    use osb_sdk::models::{CurveKind, CurveSpec};

    let curve = CurveSpec::new(
        CurveKind::Linear,
        vec![Vector2::new(0.0, 0.0), Vector2::new(100.0, 0.0)],
    );
    assert!((osb_sdk::curve::length(curve.clone()).unwrap() - 100.0).abs() < 1e-6);
    assert!((osb_sdk::curve::point(curve.clone(), 0.5, false).unwrap().x - 50.0).abs() < 1e-6);
    assert_eq!(
        osb_sdk::curve::sample(curve.clone(), 3, false)
            .unwrap()
            .points
            .len(),
        3
    );
    let normal = osb_sdk::curve::normal(curve.clone(), 0.5).unwrap();
    assert!(normal.x.abs() < 1e-6);
    assert!(osb_sdk::curve::curvature(curve.clone(), 0.5).unwrap().abs() < 1e-6);
    assert!(
        (osb_sdk::curve::distance_to(curve.clone(), Vector2::new(50.0, 10.0)).unwrap() - 10.0)
            .abs()
            < 1e-6
    );
    assert_eq!(
        osb_sdk::curve::offset(curve.clone(), 5.0)
            .unwrap()
            .points
            .len(),
        2
    );
    assert!((osb_sdk::curve::bounding_box(curve).unwrap().max.x - 100.0).abs() < 1e-6);

    let simplified = osb_sdk::curve::simplify(
        vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.05),
            Vector2::new(2.0, 0.0),
            Vector2::new(3.0, 0.0),
        ],
        0.1,
    )
    .unwrap();
    assert_eq!(simplified.points.len(), 2);

    let arc = osb_sdk::curve::arc(Vector2::new(0.0, 0.0), 10.0, 0.0, 180.0, 3).unwrap();
    assert_eq!(arc.points.len(), 3);
    assert!((arc.points[0].x - 10.0).abs() < 1e-6);

    let ellipse = osb_sdk::curve::ellipse(Vector2::new(0.0, 0.0), 4.0, 2.0, 4).unwrap();
    assert_eq!(ellipse.points.len(), 4);

    let hit = osb_sdk::curve::line_intersect(
        Vector2::new(0.0, 0.0),
        Vector2::new(10.0, 10.0),
        Vector2::new(0.0, 10.0),
        Vector2::new(10.0, 0.0),
    )
    .unwrap();
    assert!((hit.point.unwrap().unwrap().x - 5.0).abs() < 1e-6);
}

#[test]
fn bakes_a_typed_scene_and_evaluates_custom_easing() {
    use osb_sdk::models::{BakeOptionsInput, Camera3dInput, Keyframe3, Node3dInput, Scene3dInput};

    let scene = Scene3dInput {
        nodes: vec![Node3dInput {
            sprite: "sb/dot.png".to_string(),
            ..Default::default()
        }],
        camera: Some(Box::new(Camera3dInput {
            position: Some(vec![Keyframe3 {
                time: 0.0,
                value: vec![0.0, 0.0, 5.0],
            }]),
            ..Default::default()
        })),
        options: Some(Box::new(BakeOptionsInput {
            end_time: Some(Some(0.0)),
            ..Default::default()
        })),
    };

    let baked = Client::bake3d(scene).unwrap();
    assert_eq!(baked.sprites.len(), 1);
    assert_eq!(baked.sprites[0].sprite_path, "sb/dot.png");
    assert!(!baked.sprites[0].commands.is_empty());

    assert!(
        Client::apply_cubic_bezier(0.42, 0.0, 0.58, 1.0, 0.0)
            .unwrap()
            .abs()
            < 1e-6
    );
    assert!((Client::apply_cubic_bezier(0.42, 0.0, 0.58, 1.0, 1.0).unwrap() - 1.0).abs() < 1e-6);
}

#[test]
fn runs_the_motion_graphics_helpers() {
    use osb_sdk::storyboarding3d::helpers::{self, Channel, ChannelKey};

    let channel = Channel::new(vec![
        ChannelKey {
            time: 0.0,
            value: vec![1.0, 2.0],
            easing: None,
        },
        ChannelKey {
            time: 100.0,
            value: vec![3.0, 4.0],
            easing: None,
        },
    ]);
    let scaled = helpers::scale_keyframes(channel, 2.0).unwrap();
    assert_eq!(scaled.keys[1].value, vec![6.0, 8.0]);

    let mirrored = helpers::mirror(
        Channel::new(vec![ChannelKey {
            time: 0.0,
            value: vec![10.0],
            easing: None,
        }]),
        None,
        100.0,
    )
    .unwrap();
    assert_eq!(mirrored.keys[0].value[0], 190.0);

    let orbit = helpers::orbit([0.0, 0.0, 0.0], 10.0, 10.0, 0.0, std::f64::consts::TAU, 5).unwrap();
    assert_eq!(orbit.keys.len(), 5);

    let points = helpers::scatter(1, 3, [-1.0, -1.0, 0.0], [1.0, 1.0, 0.0], 0.0).unwrap();
    assert_eq!(points.len(), 3);

    assert_eq!(helpers::stagger(3, 100.0).unwrap(), 300.0);
}

#[test]
fn runs_the_shared_helpers_surface() {
    use osb_sdk::helpers::{keyframe, origin, scene3d, transform_commands, Channel, ChannelKey};
    use osb_sdk::models::{OsbOrigin, StoryboardCommand, StoryboardCommandKind};

    // A 2D channel (no Z) and a 3D channel both work - Z is optional.
    let two_d = Channel::new(vec![
        ChannelKey {
            time: 0.0,
            value: vec![1.0, 2.0],
            easing: None,
        },
        ChannelKey {
            time: 100.0,
            value: vec![3.0, 4.0],
            easing: None,
        },
    ]);
    let scaled = keyframe::scale_keyframes(two_d, 2.0).unwrap();
    assert_eq!(scaled.keys[1].value, vec![6.0, 8.0]);

    let three_d = Channel::new(vec![ChannelKey {
        time: 0.0,
        value: vec![1.0, 2.0, 3.0],
        easing: None,
    }]);
    let moved = keyframe::offset_keyframes(three_d, 1.0).unwrap();
    assert_eq!(moved.keys[0].value, vec![2.0, 3.0, 4.0]);

    // The 3D constructor.
    let orbit = scene3d::orbit([0.0, 0.0, 0.0], 10.0, 10.0, 0.0, std::f64::consts::TAU, 5).unwrap();
    assert_eq!(orbit.keys.len(), 5);

    // Shared origin/anchor.
    let vector = origin::origin_vector(OsbOrigin::Centre, 100.0, 100.0).unwrap();
    assert_eq!(vector.x, 50.0);

    // The same keyframe helper applied to a 2D command channel.
    let commands: Vec<StoryboardCommand> = serde_json::from_value(serde_json::json!([
        { "kind": "fade", "startTime": 0.0, "endTime": 100.0, "easing": "None",
          "start": { "kind": "scalar", "value": 0.0 }, "end": { "kind": "scalar", "value": 1.0 } },
        { "kind": "fade", "startTime": 100.0, "endTime": 200.0, "easing": "None",
          "start": { "kind": "scalar", "value": 1.0 }, "end": { "kind": "scalar", "value": 0.0 } }
    ]))
    .unwrap();
    let flattened = transform_commands(
        commands,
        StoryboardCommandKind::Fade,
        "keyframe.flatten",
        serde_json::json!({ "mode": "mean" }),
    )
    .unwrap();
    assert_eq!(flattened.len(), 1);
    assert_eq!(flattened[0].start_time, 0.0);
    assert_eq!(flattened[0].end_time, 200.0);
}

#[test]
fn runs_the_easing_timeline_and_depth_helpers() {
    use osb_sdk::helpers::{easing, origin, timeline, Channel, ChannelKey};
    use osb_sdk::models::{Easing, OsbOrigin, Vector3};

    // InQuad at 0.5 is 0.25; amplifying about linear gives 0.5 + (0.25-0.5)*2 = 0.
    let amplified = easing::amplify(Easing::InQuad, 0.5, 2.0).unwrap();
    assert!(amplified.abs() < 1e-6);
    assert_eq!(easing::sample(Easing::OutQuad, 5).unwrap().len(), 5);

    let channel = Channel::new(vec![
        ChannelKey {
            time: 0.0,
            value: vec![0.0],
            easing: None,
        },
        ChannelKey {
            time: 100.0,
            value: vec![1.0],
            easing: None,
        },
    ]);
    let mut tl = timeline::Timeline::default();
    tl.clips.push(timeline::TimelineClip {
        start: 0.0,
        channel: channel.clone(),
    });
    tl.clips.push(timeline::TimelineClip {
        start: 200.0,
        channel,
    });
    assert_eq!(timeline::duration(&tl).unwrap(), 300.0);
    let flushed = timeline::flush(&tl).unwrap();
    assert_eq!(flushed.clips[1].channel.keys[0].time, 200.0);
    assert_eq!(timeline::merge(&tl, &tl).unwrap().clips.len(), 4);

    let anchor = origin::resolve_anchor_3d(
        OsbOrigin::Centre,
        Vector3 {
            x: 10.0,
            y: -5.0,
            z: 42.0,
        },
        100.0,
        100.0,
    )
    .unwrap();
    assert_eq!(anchor.z, 42.0);
}

#[test]
fn runs_the_command_easing_helpers() {
    use osb_sdk::models::{Easing, StoryboardCommand, StoryboardCommandKind};
    use osb_sdk::storyboarding::commands;

    let commands: Vec<StoryboardCommand> = serde_json::from_value(serde_json::json!([
        { "kind": "fade", "startTime": 0.0, "endTime": 100.0, "easing": "None",
          "start": { "kind": "scalar", "value": 0.0 }, "end": { "kind": "scalar", "value": 1.0 } }
    ]))
    .unwrap();

    let set = commands::set_easing(commands.clone(), Easing::InQuad).unwrap();
    assert_eq!(set[0].easing, Easing::InQuad);

    let applied =
        commands::apply_easing(commands, StoryboardCommandKind::Fade, Easing::InQuad, 4).unwrap();
    assert!(applied.len() > 1);
}

#[test]
fn runs_the_command_helpers() {
    use osb_sdk::models::StoryboardCommand;
    use osb_sdk::storyboarding::commands;

    let commands: Vec<StoryboardCommand> = serde_json::from_value(serde_json::json!([
        { "kind": "fade", "startTime": 0.0, "endTime": 100.0, "easing": "None",
          "start": { "kind": "scalar", "value": 0.0 }, "end": { "kind": "scalar", "value": 1.0 } },
        { "kind": "fade", "startTime": 200.0, "endTime": 300.0, "easing": "None",
          "start": { "kind": "scalar", "value": 1.0 }, "end": { "kind": "scalar", "value": 0.0 } }
    ]))
    .unwrap();

    let shifted = commands::shift_commands(commands.clone(), 50.0).unwrap();
    assert_eq!(shifted[0].start_time, 50.0);

    let selected = commands::select_commands(commands.clone(), None, Some(150.0), None).unwrap();
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].start_time, 200.0);

    let merged = commands::merge_commands(commands.clone(), 0.0).unwrap();
    assert_eq!(merged.len(), 2);

    let duplicated = commands::duplicate_commands(commands, 1000.0, 2).unwrap();
    assert_eq!(duplicated.len(), 4);
}

#[test]
fn runs_the_color_helpers() {
    use osb_sdk::helpers::color;
    use osb_sdk::models::Color4;

    let gray = Color4 {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    let lighter = color::lighten(gray.clone(), 0.25).unwrap();
    assert!((lighter.r - 0.75).abs() < 1e-6);

    let red = Color4 {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    let green = color::adjust_hue(red.clone(), 120.0).unwrap();
    assert!(green.g > 0.9 && green.r < 0.1);

    // A gradient sampled halfway is the RGB midpoint.
    let black = Color4 {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    let white = Color4 {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    let mid = color::gradient(
        vec![
            color::ColorStop {
                offset: 0.0,
                color: black,
            },
            color::ColorStop {
                offset: 1.0,
                color: white,
            },
        ],
        0.5,
        "rgb",
        false,
    )
    .unwrap();
    assert!((mid.r - 0.5).abs() < 1e-6);

    // A gradient channel has 4-component keys.
    let channel = color::gradient_channel(
        vec![
            color::ColorStop {
                offset: 0.0,
                color: Color4 {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            },
            color::ColorStop {
                offset: 1.0,
                color: Color4 {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            },
        ],
        "rgb",
        5,
        0.0,
        100.0,
        false,
    )
    .unwrap();
    assert_eq!(channel.keys.len(), 5);
    assert_eq!(channel.keys[0].value.len(), 4);
}

#[test]
fn resolves_a_2d_scene_from_the_top() {
    use osb_sdk::helpers::scene2d;
    use osb_sdk::models::OsbOrigin;

    let scene = scene2d::Scene2d {
        root: scene2d::Scene2dNode {
            name: "root".to_string(),
            children: vec![scene2d::Scene2dNode {
                name: "leaf".to_string(),
                sprite: Some(scene2d::Scene2dSprite {
                    texture_path: "sb/dot.png".to_string(),
                    origin: OsbOrigin::Centre,
                    additive: false,
                    transform: None,
                    commands: Vec::new(),
                }),
                ..Default::default()
            }],
            ..Default::default()
        },
    };
    let moved = scene2d::translate(&scene, [15.0, 25.0]).unwrap();
    let resolved = scene2d::resolve(&moved, &scene2d::Scene2dOptions::default()).unwrap();
    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].texture_path, "sb/dot.png");
    let move_command = resolved[0]
        .commands
        .iter()
        .find(|c| c.kind == osb_sdk::models::StoryboardCommandKind::Move)
        .unwrap();
    let json = serde_json::to_value(move_command).unwrap();
    assert_eq!(json["start"]["value"]["x"], 15.0);
    assert_eq!(json["start"]["value"]["y"], 25.0);
}
