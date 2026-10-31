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
fn drives_the_clean_runtime_clients() {
    let mut runtime = Runtime::new(640.0, 480.0);
    {
        let mut generator = runtime.generator();
        assert_eq!(generator.sprite_count().unwrap(), 0);
    }
    {
        let mut layer = runtime.layer("Foreground").unwrap();
        let layer_index = layer.handle().layer;
        let sprite_index = {
            let mut sprite = layer
                .create_sprite(CreateSpriteOptions {
                    path: "sb/dot.png".to_string(),
                    origin: models::OsbOrigin::Centre,
                    position: None,
                    layer: None,
                })
                .unwrap();
            sprite
                .command(CommandOptions {
                    kind: "move".to_string(),
                    start_time: 0.0,
                    end_time: 1000.0,
                    easing: models::Easing::None,
                    start: Some(serde_json::json!({ "x": 0.0, "y": 0.0 })),
                    end: Some(serde_json::json!({ "x": 100.0, "y": 0.0 })),
                    parameter: None,
                })
                .unwrap();
            let state = sprite.state_at(SpriteTimeOptions { time: 500.0 }).unwrap();
            assert!((state.position.x - 50.0).abs() < 0.1);
            assert_eq!(sprite.layer(), layer_index);
            sprite.sprite()
        };
        assert!(layer.discard_sprite(sprite_index).unwrap());
    }
    {
        let mut layer = runtime.layer("Foreground").unwrap();
        let _animation = layer
            .create_animation(CreateAnimationOptions {
                path: "sb/anim.png".to_string(),
                frames: 2,
                frame_delay: 100.0,
                loop_type: models::OsbLoopType::LoopForever,
                origin: models::OsbOrigin::Centre,
                position: models::Vector2 { x: 0.0, y: 0.0 },
                layer: None,
            })
            .unwrap();
        let mut child = layer.create_segment().unwrap();
        child
            .create_sprite(CreateSpriteOptions {
                path: "sb/child.png".to_string(),
                origin: models::OsbOrigin::Centre,
                position: None,
                layer: None,
            })
            .unwrap();
    }
    let mut generator = runtime.generator();
    assert_eq!(generator.sprite_count().unwrap(), 2);
    assert!(!generator.storyboard().unwrap().layers.is_empty());
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
