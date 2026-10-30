# Core

The clean `Core` client. Its overloads take positional values from the
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

### `version`

```ts
version(): string
```

### `arc`

```ts
arc(center: Vector2, radius: number, startAngle: number, sweepAngle: number): PointListResult
arc(center: Vector2, radius: number, startAngle: number, sweepAngle: number, count: number | null): PointListResult
arc(options: ArcOptions): PointListResult
```

**Options — `ArcOptions`**

A circular-arc sampling request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `center` | `Vector2` | yes | The arc centre. |
| `radius` | `number` | yes | The radius. |
| `startAngle` | `number` | yes | The start angle in degrees. |
| `sweepAngle` | `number` | yes | The sweep angle in degrees. |
| `count` | `number | null` | no | The number of points (default `64`). |

### `boundingBox`

```ts
boundingBox(curve: CurveSpec): Box2
boundingBox(options: CurveOptions): Box2
```

**Options — `CurveOptions`**

A curve query.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `curve` | `CurveSpec` | yes | The curve. |

### `curvature`

```ts
curvature(curve: CurveSpec, t: number): number
curvature(curve: CurveSpec, t: number, byArcLength: boolean | null): number
curvature(options: CurvePointOptions): number
```

**Options — `CurvePointOptions`**

A curve query at a parametric or arc-length position.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `curve` | `CurveSpec` | yes | The curve. |
| `t` | `number` | yes | The position in `0.0..=1.0`. |
| `byArcLength` | `boolean | null` | no | Sample by arc length instead of the raw parameter (default `false`). |

### `distanceTo`

```ts
distanceTo(curve: CurveSpec, point: Vector2): number
distanceTo(options: CurveDistanceOptions): number
```

**Options — `CurveDistanceOptions`**

A curve hit-test request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `curve` | `CurveSpec` | yes | The curve. |
| `point` | `Vector2` | yes | The query point. |

### `ellipse`

```ts
ellipse(center: Vector2, radiusX: number, radiusY: number): PointListResult
ellipse(center: Vector2, radiusX: number, radiusY: number, count: number | null): PointListResult
ellipse(options: EllipseOptions): PointListResult
```

**Options — `EllipseOptions`**

An ellipse sampling request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `center` | `Vector2` | yes | The ellipse centre. |
| `radiusX` | `number` | yes | The horizontal radius. |
| `radiusY` | `number` | yes | The vertical radius. |
| `count` | `number | null` | no | The number of points (default `64`). |

### `length`

```ts
length(curve: CurveSpec): number
length(options: CurveOptions): number
```

**Options — `CurveOptions`**

A curve query.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `curve` | `CurveSpec` | yes | The curve. |

### `lineIntersect`

```ts
lineIntersect(a1: Vector2, a2: Vector2, b1: Vector2, b2: Vector2): Vector2 | null
lineIntersect(options: LineIntersectOptions): Vector2 | null
```

**Options — `LineIntersectOptions`**

A line-segment intersection request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `a1` | `Vector2` | yes | The first segment's start. |
| `a2` | `Vector2` | yes | The first segment's end. |
| `b1` | `Vector2` | yes | The second segment's start. |
| `b2` | `Vector2` | yes | The second segment's end. |

### `normal`

```ts
normal(curve: CurveSpec, t: number): Vector2
normal(curve: CurveSpec, t: number, byArcLength: boolean | null): Vector2
normal(options: CurvePointOptions): Vector2
```

**Options — `CurvePointOptions`**

A curve query at a parametric or arc-length position.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `curve` | `CurveSpec` | yes | The curve. |
| `t` | `number` | yes | The position in `0.0..=1.0`. |
| `byArcLength` | `boolean | null` | no | Sample by arc length instead of the raw parameter (default `false`). |

### `offset`

```ts
offset(curve: CurveSpec, distance: number): PointListResult
offset(options: CurveOffsetOptions): PointListResult
```

**Options — `CurveOffsetOptions`**

A curve offset request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `curve` | `CurveSpec` | yes | The curve. |
| `distance` | `number` | yes | The signed offset along the curve's local normal. |

### `point`

```ts
point(curve: CurveSpec, t: number): Vector2
point(curve: CurveSpec, t: number, byArcLength: boolean | null): Vector2
point(options: CurvePointOptions): Vector2
```

**Options — `CurvePointOptions`**

A curve query at a parametric or arc-length position.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `curve` | `CurveSpec` | yes | The curve. |
| `t` | `number` | yes | The position in `0.0..=1.0`. |
| `byArcLength` | `boolean | null` | no | Sample by arc length instead of the raw parameter (default `false`). |

### `sample`

```ts
sample(count: number, curve: CurveSpec): PointListResult
sample(count: number, curve: CurveSpec, byArcLength: boolean | null): PointListResult
sample(options: CurveSampleOptions): PointListResult
```

**Options — `CurveSampleOptions`**

A curve sampling request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `count` | `number` | yes | The number of points to sample. |
| `curve` | `CurveSpec` | yes | The curve. |
| `byArcLength` | `boolean | null` | no | Sample by arc length instead of the raw parameter (default `false`). |

### `simplify`

```ts
simplify(points: Vector2[], tolerance: number): PointListResult
simplify(options: SimplifyOptions): PointListResult
```

**Options — `SimplifyOptions`**

A Douglas-Peucker simplification request.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `points` | `Vector2[]` | yes | The polyline to simplify. |
| `tolerance` | `number` | yes | The maximum deviation to allow. |

### `tangent`

```ts
tangent(curve: CurveSpec, t: number): Vector2
tangent(curve: CurveSpec, t: number, byArcLength: boolean | null): Vector2
tangent(options: CurvePointOptions): Vector2
```

**Options — `CurvePointOptions`**

A curve query at a parametric or arc-length position.

| Property | Type | Required | Description |
| --- | --- | --- | --- |
| `curve` | `CurveSpec` | yes | The curve. |
| `t` | `number` | yes | The position in `0.0..=1.0`. |
| `byArcLength` | `boolean | null` | no | Sample by arc length instead of the raw parameter (default `false`). |

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
