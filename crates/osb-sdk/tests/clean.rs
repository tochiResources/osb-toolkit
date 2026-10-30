use osb_sdk::clean::*;
use osb_sdk::models;

#[test]
fn drives_the_clean_core_and_library_clients() {
    let client = Client::new();
    assert!(!client.core.version().unwrap().is_empty());
    assert_eq!(
        client
            .core
            .apply_easing(ApplyEasingOptions {
                easing: models::Easing::InQuad,
                t: 0.5,
            })
            .unwrap(),
        0.25
    );

    let curve = models::CurveSpec {
        kind: models::CurveKind::Linear,
        points: vec![
            models::Vector2 { x: 0.0, y: 0.0 },
            models::Vector2 { x: 100.0, y: 0.0 },
        ],
    };
    assert_eq!(
        client
            .core
            .length(CurveOptions {
                curve: curve.clone()
            })
            .unwrap(),
        100.0
    );
    assert_eq!(
        client
            .core
            .point(CurvePointOptions {
                curve: curve.clone(),
                t: 0.5,
                by_arc_length: None,
            })
            .unwrap()
            .x,
        50.0
    );
    assert_eq!(
        client
            .core
            .sample(CurveSampleOptions {
                count: 3,
                curve,
                by_arc_length: None,
            })
            .unwrap()
            .points
            .len(),
        3
    );
}

#[test]
fn accepts_structured_helper_input() {
    let client = Client::new();
    let result = client
        .helpers
        .call(HelperOptions {
            name: "keyframe.scaleKeyframes".to_string(),
            input: Some(serde_json::json!({
                "channel": {
                    "keys": [
                        {"time": 0.0, "value": [1.0, 2.0]},
                        {"time": 100.0, "value": [3.0, 4.0]}
                    ]
                },
                "factor": 2.0
            })),
        })
        .unwrap();
    assert_eq!(result["keys"][1]["value"][0], 6.0);
    assert_eq!(result["keys"][1]["value"][1], 8.0);
}
