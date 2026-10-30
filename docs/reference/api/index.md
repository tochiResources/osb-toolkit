# SDK reference

The typed SDK surface, generated from the native bridge contract. The
non-runtime `Core` classes have no handles or storyboard state; the
runtime classes are bound to a live runtime; the libraries are the
stateless media/image/etc. helpers.

Every options-based method takes the individual values positionally
(ordered by ascending arity), then the `<X>Options` form. TypeScript, C#,
Java and Python emit real overloads; Go has no overloading and uses the
options form only. Single-field results are unwrapped: `OkResult` is
`void`, `BoolResult` a boolean, scalar wrappers a number, and so on.

## Runtime

- [Audio](./runtime/Audio.md)
- [Generator](./runtime/Generator.md)
- [Helpers](./runtime/Helpers.md)
- [Layer](./runtime/Layer.md)
- [Sprite](./runtime/Sprite.md)
- [Storyboarding3d](./runtime/Storyboarding3d.md)

## Core

- [Core](./core/Core.md)

## Libraries

- [Diagnostics](./libraries/Diagnostics.md)
- [Image](./libraries/Image.md)
- [Imagegen](./libraries/Imagegen.md)
- [Media](./libraries/Media.md)
- [Optimize](./libraries/Optimize.md)

## Models

The shared request/response models are documented in the
[models index](./models/index.md).
