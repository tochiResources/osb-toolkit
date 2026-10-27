//! Parse the real AeToOsb test project, when it is present.

use osb_parser_ae::{Bpc, Property};

fn test_project() -> Option<std::path::PathBuf> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../templates/after-effects/2026/AeToOsb/AeToOsb-Test-Project.aep");
    path.exists().then_some(path)
}

#[test]
fn parses_the_aetoosb_test_project() {
    let Some(path) = test_project() else {
        eprintln!("skipping: test project not present");
        return;
    };
    let project = osb_parser_ae::open(&path).expect("parse project");

    assert_eq!(project.expression_engine, "javascript-1.0");
    assert_eq!(project.depth, Bpc::Eight);
    assert!(
        project.items().count() > 100,
        "expected a populated item tree"
    );

    let compositions: Vec<_> = project.compositions().collect();
    assert!(compositions.len() >= 5, "expected several compositions");

    // The storyboard composition is 1920x1080 at ~29.97 fps.
    let storyboard = compositions
        .iter()
        .find(|comp| comp.name == "storyboard")
        .expect("storyboard composition");
    assert_eq!(storyboard.footage_dimensions, [1920, 1080]);
    assert!((storyboard.footage_framerate - 29.97).abs() < 0.01);

    // Storyboard layer comps are the widescreen 854x480 at 60 fps.
    let layer_comp = compositions
        .iter()
        .find(|comp| comp.name.starts_with("(Background)"))
        .expect("a storyboard layer composition");
    assert_eq!(layer_comp.footage_dimensions, [854, 480]);
    assert!((layer_comp.footage_framerate - 60.0).abs() < 1e-6);
    assert!(!layer_comp.composition_layers.is_empty());

    // Layers inherit the name of their source (image/audio footage).
    let named = project
        .items()
        .flat_map(|item| item.composition_layers.iter())
        .any(|layer| layer.name.ends_with(".png") || layer.name.ends_with(".mp3"));
    assert!(named, "layers should inherit their source name");
}

fn find_property<'a>(properties: &'a [Property], match_name: &str) -> Option<&'a Property> {
    for property in properties {
        if property.match_name == match_name {
            return Some(property);
        }
        if let Some(found) = find_property(&property.properties, match_name) {
            return Some(found);
        }
    }
    None
}

#[test]
fn parses_keyframes_from_the_test_project() {
    let Some(path) = test_project() else {
        eprintln!("skipping: test project not present");
        return;
    };
    let project = osb_parser_ae::open(&path).expect("parse project");

    let comp = project
        .compositions()
        .find(|comp| comp.name.starts_with("(Background)"))
        .expect("(Background) composition");
    let layer = comp
        .composition_layers
        .iter()
        .find(|layer| layer.name == "bg_clean.jpg")
        .expect("bg_clean.jpg layer");

    // A centered 854x480 image: Position is stored in composition pixels.
    let position = find_property(&layer.properties, "ADBE Position").expect("position");
    let position_stream = position.stream.as_ref().expect("position stream");
    assert!(position_stream.spatial);
    assert_eq!(position_stream.dimensions, 3);
    assert!(!position_stream.keyframes.is_empty());
    assert_eq!(position_stream.keyframes[0].value, vec![427.0, 240.0, 0.0]);

    // Opacity is stored as a fraction (0.6 == 60%) and has several keyframes.
    let opacity = find_property(&layer.properties, "ADBE Opacity").expect("opacity");
    let opacity_stream = opacity.stream.as_ref().expect("opacity stream");
    assert!(opacity_stream.animated);
    assert!(opacity_stream.keyframes.len() >= 2);
    assert!(opacity_stream
        .keyframes
        .iter()
        .all(|keyframe| keyframe.time_seconds >= 0.0));
    assert!(
        opacity_stream
            .keyframes
            .windows(2)
            .all(|pair| pair[0].time_seconds <= pair[1].time_seconds),
        "keyframes should be sorted by time"
    );
}

#[test]
fn parses_layer_timing_from_the_test_project() {
    let Some(path) = test_project() else {
        eprintln!("skipping: test project not present");
        return;
    };
    let project = osb_parser_ae::open(&path).expect("parse project");

    let comp = project
        .compositions()
        .find(|comp| comp.name.starts_with("(Main)"))
        .expect("(Main) composition");
    let audio = comp
        .composition_layers
        .iter()
        .find(|layer| layer.name == "audio.mp3")
        .expect("audio.mp3 layer");

    // Values verified against the live project via ExtendScript.
    assert_eq!(audio.start_time, 0.0);
    assert_eq!(audio.in_point, 0.0);
    assert!((audio.out_point - 115.672_448_979_592).abs() < 1e-6);
    assert!((audio.stretch - 100.0).abs() < 1e-6);
    assert!((audio.composition_out_point() - audio.out_point).abs() < 1e-6);
    assert_ne!(audio.id, 0);
    assert_eq!(audio.layer_type, 0); // AV layer
}
