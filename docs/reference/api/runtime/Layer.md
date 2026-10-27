# Layer

The clean `Layer` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `addIndex`

```ts
addIndex(index: number): void
addIndex(options: SegmentAddIndexOptions): void
```

**Options — `SegmentAddIndexOptions`**

Record a depth index on a segment.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `index` | `number` | yes | The index to record. |

### `changePosition`

```ts
changePosition(osbLayer: OsbLayer): void
changePosition(options: SegmentChangePositionOptions): void
```

**Options — `SegmentChangePositionOptions`**

Move a segment to another osu! layer (`ChangePosition`).

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `osbLayer` | `OsbLayer` | yes | The destination osu! layer. |

### `clean`

```ts
clean(): number
```

### `createAnimation`

```ts
createAnimation(path: string, frames: number, frameDelay: number, loopType: OsbLoopType, origin: OsbOrigin, position: Vector2): Sprite
createAnimation(path: string, frames: number, frameDelay: number, loopType: OsbLoopType, origin: OsbOrigin, position: Vector2, layer: string | null): Sprite
createAnimation(options: CreateAnimationOptions): Sprite
```

**Options — `CreateAnimationOptions`**

Create an animation sprite.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `path` | `string` | yes | The texture path (relative to the mapset). |
| `frames` | `number` | yes | The number of frames. |
| `frameDelay` | `number` | yes | Delay between frames, in milliseconds. |
| `loopType` | `OsbLoopType` | yes | How the animation loops. |
| `origin` | `OsbOrigin` | yes | The sprite anchor origin. |
| `position` | `Vector2` | yes | The initial position. |
| `layer` | `string | null` | no | The target layer name (omitted when `layerHandle` is set). |

### `createSample`

```ts
createSample(path: string, time: number): void
createSample(path: string, time: number, volume: number | null): void
createSample(path: string, time: number, volume: number | null, layer: string | null): void
createSample(options: CreateSampleOptions): void
```

**Options — `CreateSampleOptions`**

Create a sound sample on a layer.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `path` | `string` | yes | The sample path (relative to the mapset). |
| `time` | `number` | yes | The sample time in milliseconds. |
| `volume` | `number | null` | no | The sample volume (`0..=100`). |
| `layer` | `string | null` | no | The target layer name (omitted when `layerHandle` is set). |

### `createSegment`

```ts
createSegment(): Layer
```

### `createSprite`

```ts
createSprite(path: string, origin: OsbOrigin): Sprite
createSprite(path: string, origin: OsbOrigin, position: Vector2 | null): Sprite
createSprite(path: string, origin: OsbOrigin, position: Vector2 | null, layer: string | null): Sprite
createSprite(options: CreateSpriteOptions): Sprite
```

**Options — `CreateSpriteOptions`**

Create a sprite.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `path` | `string` | yes | The texture path (relative to the mapset). |
| `origin` | `OsbOrigin` | yes | The sprite anchor origin. |
| `position` | `Vector2 | null` | no | The initial position; omitted means the sprite's default. |
| `layer` | `string | null` | no | The target layer name (omitted when `layerHandle` is set). |

### `discard`

```ts
discard(): number
```

### `discardSprite`

```ts
discardSprite(sprite: number): boolean
```

### `indexes`

```ts
indexes(): SegmentIndexesResult
```

### `moveToBottom`

```ts
moveToBottom(): void
```

### `moveToTop`

```ts
moveToTop(): void
```

### `reverseDepth`

```ts
reverseDepth(value: boolean): void
reverseDepth(options: SegmentReverseDepthOptions): void
```

**Options — `SegmentReverseDepthOptions`**

Set a segment's reverse-depth flag.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `value` | `boolean` | yes | Whether children are emitted in reverse order. |
