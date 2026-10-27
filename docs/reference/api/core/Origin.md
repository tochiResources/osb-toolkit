# Origin

The clean `Origin` client. Its overloads take positional values from the
smallest required arity upward, then the `<X>Options` form; results are
unwrapped into ordinary values.

## Methods

### `getOriginVector`

```ts
getOriginVector(origin: OsbOrigin, width: number, height: number): Vector2
getOriginVector(options: GetOriginVectorOptions): Vector2
```

**Options — `GetOriginVectorOptions`**

The anchor offset for an origin.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `origin` | `OsbOrigin` | yes | The sprite anchor origin. |
| `width` | `number` | yes | The sprite width. |
| `height` | `number` | yes | The sprite height. |

### `inScreenBounds`

```ts
inScreenBounds(position: Vector2, size: Vector2, rotation: number, origin: Vector2): boolean
inScreenBounds(options: InScreenBoundsOptions): boolean
```

**Options — `InScreenBoundsOptions`**

Whether a rotated sprite's bounding box intersects the widescreen
storyboard area.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `position` | `Vector2` | yes | The sprite centre position. |
| `size` | `Vector2` | yes | The sprite size. |
| `rotation` | `number` | yes | The rotation in radians. |
| `origin` | `Vector2` | yes | The anchor offset (from `GetOriginVector`). |
