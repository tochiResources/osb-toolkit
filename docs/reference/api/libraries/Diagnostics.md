# Diagnostics

The clean `Diagnostics` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `diagnoseOsb`

```ts
diagnoseOsb(text: string): DiagnosticReport
diagnoseOsb(options: TextOptions): DiagnosticReport
```

**Options — `TextOptions`**

A request carrying a block of text (an effect script, `.osu` or `.osb`).

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `text` | `string` | yes | The text payload. |

### `diagnoseOsu`

```ts
diagnoseOsu(text: string): DiagnosticReport
diagnoseOsu(options: TextOptions): DiagnosticReport
```

**Options — `TextOptions`**

A request carrying a block of text (an effect script, `.osu` or `.osb`).

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `text` | `string` | yes | The text payload. |
