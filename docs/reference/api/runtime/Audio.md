# Audio

The clean `Audio` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `audioDuration`

```ts
audioDuration(): number
```

### `fft`

```ts
fft(path: string): FftResult
fft(path: string, magnitudes: number | null): FftResult
fft(path: string, magnitudes: number | null, time: number | null): FftResult
fft(options: FftOptions): FftResult
```

**Options — `FftOptions`**

An FFT request against a `.wav` file.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `path` | `string` | yes | Path to the audio file. |
| `magnitudes` | `number | null` | no | Number of frequency bins. |
| `time` | `number | null` | no | Sample time in milliseconds. |

### `getFft`

```ts
getFft(time: number, magnitudes: number): FftResult
getFft(time: number, magnitudes: number, cutoff: number | null): FftResult
getFft(time: number, magnitudes: number, cutoff: number | null, easing: Easing | null): FftResult
getFft(options: GetFftOptions): FftResult
```

**Options — `GetFftOptions`**

An FFT request against the runtime's loaded audio.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `time` | `number` | yes | Sample time in milliseconds. |
| `magnitudes` | `number` | yes | Number of frequency bins. |
| `cutoff` | `number | null` | no | A frequency cut-off. |
| `easing` | `Easing | null` | no | The easing applied across the bins. |
