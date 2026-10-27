# Helpers

The clean `Helpers` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `call`

```ts
call(name: string): unknown
call(name: string, input: unknown): unknown
call(options: HelperOptions): unknown
```

**Options — `HelperOptions`**

A motion-graphics helper call: a helper `name` plus its input.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | `string` | yes | The helper name, e.g. `flatten`, `orbit`, `selectCommands`. |
| `input` | `unknown` | no | The helper's input (a keyframe channel, a command list, or its parameters). |

### `commands`

```ts
commands(name: string): unknown
commands(name: string, input: unknown): unknown
commands(options: HelperOptions): unknown
```

**Options — `HelperOptions`**

A motion-graphics helper call: a helper `name` plus its input.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | `string` | yes | The helper name, e.g. `flatten`, `orbit`, `selectCommands`. |
| `input` | `unknown` | no | The helper's input (a keyframe channel, a command list, or its parameters). |

### `keyframes`

```ts
keyframes(name: string): unknown
keyframes(name: string, input: unknown): unknown
keyframes(options: HelperOptions): unknown
```

**Options — `HelperOptions`**

A motion-graphics helper call: a helper `name` plus its input.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | `string` | yes | The helper name, e.g. `flatten`, `orbit`, `selectCommands`. |
| `input` | `unknown` | no | The helper's input (a keyframe channel, a command list, or its parameters). |
