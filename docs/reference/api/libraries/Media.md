# Media

The clean `Media` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `encodeVideo`

```ts
encodeVideo(inputPattern: string, output: string, fps: number): MediaInfo
encodeVideo(inputPattern: string, output: string, fps: number, codec: string | null): MediaInfo
encodeVideo(inputPattern: string, output: string, fps: number, codec: string | null, crf: number | null): MediaInfo
encodeVideo(options: EncodeVideoOptions): MediaInfo
```

**Options — `EncodeVideoOptions`**

An image-sequence encoding request (frames -> video).

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `inputPattern` | `string` | yes | The input filename pattern (e.g. `frames/frame_%04d.png`). |
| `output` | `string` | yes | The output video path. |
| `fps` | `number` | yes | The frame rate. |
| `codec` | `string | null` | no | The video codec (default `libx264`). |
| `crf` | `number | null` | no | The constant-rate-factor quality (`0`-`51`; lower is better). |

### `extractAudio`

```ts
extractAudio(input: string, output: string): void
extractAudio(options: ExtractAudioOptions): void
```

**Options — `ExtractAudioOptions`**

An audio-extraction request (media -> audio file).

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | `string` | yes | The input media path. |
| `output` | `string` | yes | The output audio path (its extension selects the format). |

### `extractFrames`

```ts
extractFrames(input: string, outputDir: string): FrameResult
extractFrames(input: string, outputDir: string, count: number | null): FrameResult
extractFrames(input: string, outputDir: string, count: number | null, fps: number | null): FrameResult
extractFrames(input: string, outputDir: string, count: number | null, fps: number | null, height: number | null): FrameResult
extractFrames(input: string, outputDir: string, count: number | null, fps: number | null, height: number | null, start: number | null): FrameResult
extractFrames(input: string, outputDir: string, count: number | null, fps: number | null, height: number | null, start: number | null, width: number | null): FrameResult
extractFrames(options: ExtractFramesOptions): FrameResult
```

**Options — `ExtractFramesOptions`**

A frame-extraction request (video -> image sequence).

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `input` | `string` | yes | The input media path. |
| `outputDir` | `string` | yes | The directory to write frames into. |
| `count` | `number | null` | no | The maximum number of frames to extract. |
| `fps` | `number | null` | no | The sampling rate (frames per second); defaults to the source rate. |
| `height` | `number | null` | no | Scale frames to this height. |
| `start` | `number | null` | no | The start time in seconds. |
| `width` | `number | null` | no | Scale frames to this width. |

### `info`

```ts
info(path: string): MediaInfo
info(options: PathOptions): MediaInfo
```

**Options — `PathOptions`**

A request carrying a single file path.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `path` | `string` | yes | The input file path. |
