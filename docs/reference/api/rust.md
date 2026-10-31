# Rust clean module

The `osb-sdk` crate generates `crates/osb-sdk/src/clean.rs`: one `<X>Options`
struct per request and one client per tag.

- Stateless tags (`Core`, `Audio`, `Diagnostics`, `Helpers`, `Image`,
  `Imagegen`, `Media`, `Optimize`, `Storyboarding3d`) are fields on
  `clean::Client`.
- The handle-bound `Generator`, `Layer` and `Sprite` clients live behind
  `clean::Runtime`, which owns the hand-written `Runtime`.
- `Audio::fft` works without a runtime; `Runtime::audio()` binds the same
  client to a runtime for `get_fft` and `audio_duration`.

```rust
use osb_sdk::clean::*;
use osb_sdk::models;

let mut runtime = Runtime::new(640.0, 480.0);
let mut layer = runtime.layer("Foreground")?;
let mut sprite = layer.create_sprite(CreateSpriteOptions {
    path: "sb/dot.png".to_string(),
    origin: models::OsbOrigin::Centre,
    position: None,
    layer: None,
})?;
sprite.command(CommandOptions {
    kind: "move".to_string(),
    start_time: 0.0,
    end_time: 1000.0,
    easing: models::Easing::None,
    start: Some(serde_json::json!({ "x": 0.0, "y": 0.0 })),
    end: Some(serde_json::json!({ "x": 100.0, "y": 0.0 })),
    parameter: None,
})?;
```
