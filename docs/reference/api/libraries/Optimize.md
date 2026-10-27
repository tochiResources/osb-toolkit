# Optimize

The clean `Optimize` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `optimizeOsb`

```ts
optimizeOsb(options: OptimizeOptionsInput, text: string): OptimizeResult
optimizeOsb(options: OptimizeOptions): OptimizeResult
```

**Options — `OptimizeOptions`**

An `.osb` optimization request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `options` | `OptimizeOptionsInput` | yes | The optimization options. |
| `text` | `string` | yes | The `.osb` storyboard text. |
