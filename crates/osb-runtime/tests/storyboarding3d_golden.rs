//! Shared golden-vector parity test for the 3D bake (section 5.9). The corpus is
//! consumed across the SDKs, so all implementations must agree on it.

use osb_runtime::storyboarding3d::{bake_input, Bake3dInput, CommandKind};
use serde::Deserialize;

#[derive(Deserialize)]
struct Case {
    name: String,
    input: Bake3dInput,
    #[serde(default)]
    expect: Expect,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Expect {
    sprite_count: Option<usize>,
    position_at0: Option<[f64; 2]>,
    position_tolerance: Option<f64>,
    move_spans: Option<usize>,
    x_increasing: Option<bool>,
    first_sprite_path: Option<String>,
}

#[derive(Deserialize)]
struct Corpus {
    cases: Vec<Case>,
}

#[test]
fn golden_vectors() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/golden/storyboarding3d/basic.json"
    );
    let data = osb_core::io::read_string(path).expect("read golden corpus");
    let corpus: Corpus = serde_json::from_str(&data).expect("parse golden corpus");
    assert!(!corpus.cases.is_empty());

    for case in corpus.cases {
        let baked = bake_input(&case.input);

        if let Some(count) = case.expect.sprite_count {
            assert_eq!(baked.sprites.len(), count, "{}: sprite count", case.name);
        }
        if let Some(point) = case.expect.position_at0 {
            let position = baked.sprites[0]
                .position
                .value_at(0.0)
                .expect("position at 0");
            let tolerance = case.expect.position_tolerance.unwrap_or(0.5);
            assert!(
                (position.x - point[0]).abs() <= tolerance,
                "{}: x {} vs {}",
                case.name,
                position.x,
                point[0]
            );
            assert!(
                (position.y - point[1]).abs() <= tolerance,
                "{}: y {} vs {}",
                case.name,
                position.y,
                point[1]
            );
        }
        if let Some(spans) = case.expect.move_spans {
            let moves = baked.sprites[0]
                .commands()
                .iter()
                .filter(|command| command.kind == CommandKind::Move)
                .count();
            assert_eq!(moves, spans, "{}: move spans", case.name);
        }
        if case.expect.x_increasing == Some(true) {
            let start = baked.sprites[0].position.value_at(0.0).unwrap();
            let end = baked.sprites[0]
                .position
                .value_at(case.input.options.end_time.unwrap_or(1000.0))
                .unwrap();
            assert!(end.x > start.x, "{}: {} -> {}", case.name, start.x, end.x);
        }
        if let Some(path) = &case.expect.first_sprite_path {
            assert_eq!(
                &baked.sprites[0].sprite_path, path,
                "{}: first sprite",
                case.name
            );
        }
    }
}
