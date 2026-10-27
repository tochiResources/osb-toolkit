# Imagegen

The clean `Imagegen` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `renderShapes`

```ts
renderShapes(width: number, height: number, shapes: Shape[], output: string): ImageInfo
renderShapes(width: number, height: number, shapes: Shape[], output: string, background: RgbaColor | null): ImageInfo
renderShapes(options: RenderShapesOptions): ImageInfo
```

**Options — `RenderShapesOptions`**

A shape-rendering request: a canvas, a background and a shape list.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `width` | `number` | yes | The canvas width in pixels. |
| `height` | `number` | yes | The canvas height in pixels. |
| `shapes` | `Shape[]` | yes | The shapes to draw, in order. |
| `output` | `string` | yes | The output file path (its extension selects the format). |
| `background` | `RgbaColor | null` | no | The background colour (default transparent). |
