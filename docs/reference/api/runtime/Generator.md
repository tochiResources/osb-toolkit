# Generator

The clean `Generator` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `beatmap`

```ts
beatmap(): Beatmap
```

### `controlPointAt`

```ts
controlPointAt(time: number): ControlPoint
controlPointAt(options: TimeOptions): ControlPoint
```

**Options — `TimeOptions`**

A time query.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `time` | `number` | yes | The time in milliseconds. |

### `importOsb`

```ts
importOsb(text: string): number
importOsb(options: TextOptions): number
```

**Options — `TextOptions`**

A request carrying a block of text (an effect script, `.osu` or `.osb`).

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `text` | `string` | yes | The text payload. |

### `layerHandle`

```ts
layerHandle(name: string): number
layerHandle(options: LayerHandleOptions): number
```

**Options — `LayerHandleOptions`**

Resolve (or create) a layer by name.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | `string` | yes | The layer name. |

### `loadBeatmap`

```ts
loadBeatmap(text: string): LoadBeatmapResult
loadBeatmap(options: TextOptions): LoadBeatmapResult
```

**Options — `TextOptions`**

A request carrying a block of text (an effect script, `.osu` or `.osb`).

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `text` | `string` | yes | The text payload. |

### `random`

```ts
random(max: number, min: number): number
random(options: RandomOptions): number
```

**Options — `RandomOptions`**

A deterministic random request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `max` | `number` | yes | The exclusive upper bound. |
| `min` | `number` | yes | The inclusive lower bound. |

### `spriteCount`

```ts
spriteCount(): number
```

### `storyboard`

```ts
storyboard(): Storyboard
```

### `timingPointAt`

```ts
timingPointAt(time: number): ControlPoint
timingPointAt(options: TimeOptions): ControlPoint
```

**Options — `TimeOptions`**

A time query.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `time` | `number` | yes | The time in milliseconds. |
