# Storyboarding3d

The clean `Storyboarding3d` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `bake3d`

```ts
bake3d(input: Scene3dInput): BakedStoryboard
bake3d(options: Bake3dOptions): BakedStoryboard
```

**Options — `Bake3dOptions`**

A `bake3d` request: a plain 3D scene.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | `Scene3dInput` | yes | The 3D scene. |
