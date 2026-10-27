# Sprite

The clean `Sprite` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `command`

```ts
command(kind: string, startTime: number, endTime: number, easing: Easing): void
command(kind: string, startTime: number, endTime: number, easing: Easing, start: unknown): void
command(kind: string, startTime: number, endTime: number, easing: Easing, start: unknown, end: unknown): void
command(kind: string, startTime: number, endTime: number, easing: Easing, start: unknown, end: unknown, parameter: ParameterType | null): void
command(options: CommandOptions): void
```

**Options — `CommandOptions`**

A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4`
depending on `kind`.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `kind` | `string` | yes | The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`,
`rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`,
`flipV`, `additive`). |
| `startTime` | `number` | yes | The command start time in milliseconds. |
| `endTime` | `number` | yes | The command end time in milliseconds. |
| `easing` | `Easing` | yes | The easing function. |
| `start` | `unknown` | no | The start value (scalar, point or color). |
| `end` | `unknown` | no | The end value (scalar, point or color). |
| `parameter` | `ParameterType | null` | no | The parameter kind for a `parameter` command. |

### `endGroup`

```ts
endGroup(): void
```

### `info`

```ts
info(): SpriteInfo
```

### `startLoopGroup`

```ts
startLoopGroup(startTime: number, count: number): void
startLoopGroup(options: StartLoopGroupOptions): void
```

**Options — `StartLoopGroupOptions`**

Start a loop group.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `startTime` | `number` | yes | The loop start time in milliseconds. |
| `count` | `number` | yes | The number of iterations. |

### `startTriggerGroup`

```ts
startTriggerGroup(name: string, startTime: number, endTime: number, group: number): void
startTriggerGroup(options: StartTriggerGroupOptions): void
```

**Options — `StartTriggerGroupOptions`**

Start a trigger group.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | `string` | yes | The trigger name (matches a hit object). |
| `startTime` | `number` | yes | The trigger start time in milliseconds. |
| `endTime` | `number` | yes | The trigger end time in milliseconds. |
| `group` | `number` | yes | The trigger group index. |

### `stateAt`

```ts
stateAt(time: number): SpriteState
stateAt(options: SpriteTimeOptions): SpriteState
```

**Options — `SpriteTimeOptions`**

A sprite handle pair plus a sample time.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `time` | `number` | yes | The sample time in milliseconds. |
