# Parsing

The clean `Parsing` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `parseOsb`

```ts
parseOsb(text: string): ParseOsbResult
parseOsb(options: TextOptions): ParseOsbResult
```

**Options — `TextOptions`**

A request carrying a block of text (an effect script, `.osu` or `.osb`).

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `text` | `string` | yes | The text payload. |

### `parseOsu`

```ts
parseOsu(text: string): Beatmap
parseOsu(options: TextOptions): Beatmap
```

**Options — `TextOptions`**

A request carrying a block of text (an effect script, `.osu` or `.osb`).

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `text` | `string` | yes | The text payload. |
