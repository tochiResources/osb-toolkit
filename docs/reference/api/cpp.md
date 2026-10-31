# C++ clean header

`bindings/cpp/osbtk_clean.hpp` is generated from the contract alongside
`osbtk_sdk.hpp`. It contains the `<X>Options` structs and one class per tag.

- Stateless clients (`Core`, `Audio`, `Diagnostics`, `Optimize`, `Image`,
  `Imagegen`, `Media`, `Storyboarding3d`, `Helpers`) are members of
  `osbtk::clean::Client`.
- `osbtk::clean::Runtime` owns an `sdk::Runtime` and exposes `generator()`,
  `audio()`, `layer(name)` and `sprite(layer, sprite)`.
- Complex option fields use JSON strings (`start_json`, `end_json`,
  `input_json`, `request_json`) while typed fields such as `position` are
  `std::optional` where the contract makes them optional.

```cpp
osbtk::Library library(path);
osbtk::clean::Runtime runtime(library);
osbtk::clean::Layer layer = runtime.layer("Foreground");
osbtk::clean::Sprite sprite =
    layer.create_sprite({"sb/dot.png", osbtk::sdk::OsbOrigin::Centre});
sprite.command(
    {"move", 0.0, 1000.0, "None", R"({"x":0,"y":0})", R"({"x":100,"y":0})", ""});
```
