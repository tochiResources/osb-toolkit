# Image

The clean `Image` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `info`

```ts
info(path: string): ImageInfo
info(options: PathOptions): ImageInfo
```

**Options — `PathOptions`**

A request carrying a single file path.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `path` | `string` | yes | The input file path. |

### `process`

```ts
process(options: ImageOptions, output: string, path: string): ImageInfo
process(options: ImageProcessOptions): ImageInfo
```

**Options — `ImageProcessOptions`**

An image processing request: read `path`, apply `options`, write `output`.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `options` | `ImageOptions` | yes | The processing options. |
| `output` | `string` | yes | The output file path (its extension selects the format). |
| `path` | `string` | yes | The input file path. |
