# Animations

The clean `Animations` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `applyEasing`

```ts
applyEasing(easing: Easing, t: number): number
applyEasing(options: ApplyEasingOptions): number
```

**Options — `ApplyEasingOptions`**

An easing evaluation request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `easing` | `Easing` | yes | The easing function. |
| `t` | `number` | yes | The normalized time in `0.0..=1.0`. |

### `easingValue`

```ts
easingValue(t: number): number
easingValue(t: number, easing: Easing | null): number
easingValue(t: number, easing: Easing | null, cubicBezier: number[] | null): number
easingValue(options: EasingValueOptions): number
```

**Options — `EasingValueOptions`**

A named-or-custom easing evaluation request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `t` | `number` | yes | The normalized time in `0.0..=1.0`. |
| `easing` | `Easing | null` | no | A named easing (used when `cubicBezier` is absent). |
| `cubicBezier` | `number[] | null` | no | A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`. |
