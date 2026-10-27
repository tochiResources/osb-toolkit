// The curve library facade: Bezier/Catmull-Rom curves, sampling and simplification.

package osbtk

// curveLib is the curve library facade.
type curveLib struct{}

// Curve is the curve library.
var Curve = curveLib{}

// Length returns the curve's arc length.
func (curveLib) Length(curve CurveSpec) (float64, error) {
	result, err := decodeBridge[ScalarResult](map[string]any{"op": "Curve.length", "curve": curve})
	if err != nil {
		return 0, err
	}
	return result.Value, nil
}

// Point returns the point at `t`, parametrically or by arc length.
func (curveLib) Point(curve CurveSpec, t float64, byArcLength bool) (*Vector2, error) {
	return decodeBridge[*Vector2](map[string]any{
		"op": "Curve.point", "curve": curve, "t": t, "byArcLength": byArcLength,
	})
}

// Sample returns `count` points along the curve, parametrically or by arc length.
func (curveLib) Sample(curve CurveSpec, count int, byArcLength bool) (*PointListResult, error) {
	return decodeBridge[*PointListResult](map[string]any{
		"op": "Curve.sample", "curve": curve, "count": count, "byArcLength": byArcLength,
	})
}

// BoundingBox returns the curve's axis-aligned bounding box.
func (curveLib) BoundingBox(curve CurveSpec) (*Box2, error) {
	return decodeBridge[*Box2](map[string]any{"op": "Curve.boundingBox", "curve": curve})
}

// Simplify simplifies a polyline with the Douglas-Peucker algorithm.
func (curveLib) Simplify(points []Vector2, tolerance float64) (*PointListResult, error) {
	return decodeBridge[*PointListResult](map[string]any{
		"op": "Curve.simplify", "points": points, "tolerance": tolerance,
	})
}

// Normal returns the unit normal at `t`.
func (curveLib) Normal(curve CurveSpec, t float64) (*Vector2, error) {
	return decodeBridge[*Vector2](map[string]any{"op": "Curve.normal", "curve": curve, "t": t})
}

// Curvature returns the curvature at `t`.
func (curveLib) Curvature(curve CurveSpec, t float64) (float64, error) {
	result, err := decodeBridge[ScalarResult](map[string]any{"op": "Curve.curvature", "curve": curve, "t": t})
	if err != nil {
		return 0, err
	}
	return result.Value, nil
}

// DistanceTo returns the distance from `point` to the curve.
func (curveLib) DistanceTo(curve CurveSpec, point Vector2) (float64, error) {
	result, err := decodeBridge[ScalarResult](map[string]any{
		"op": "Curve.distanceTo", "curve": curve, "point": point,
	})
	if err != nil {
		return 0, err
	}
	return result.Value, nil
}

// Offset returns the polyline offset along the curve's local normal.
func (curveLib) Offset(curve CurveSpec, distance float64) (*PointListResult, error) {
	return decodeBridge[*PointListResult](map[string]any{
		"op": "Curve.offset", "curve": curve, "distance": distance,
	})
}

// Arc returns points along a circular arc.
func (curveLib) Arc(center Vector2, radius, startAngle, sweepAngle float64, count int) (*PointListResult, error) {
	return decodeBridge[*PointListResult](map[string]any{
		"op": "Curve.arc", "center": center, "radius": radius,
		"startAngle": startAngle, "sweepAngle": sweepAngle, "count": count,
	})
}

// Ellipse returns points around an ellipse.
func (curveLib) Ellipse(center Vector2, radiusX, radiusY float64, count int) (*PointListResult, error) {
	return decodeBridge[*PointListResult](map[string]any{
		"op": "Curve.ellipse", "center": center, "radiusX": radiusX, "radiusY": radiusY, "count": count,
	})
}

// LineIntersect returns the intersection of two line segments, if any.
func (curveLib) LineIntersect(a1, a2, b1, b2 Vector2) (*OptionalPointResult, error) {
	return decodeBridge[*OptionalPointResult](map[string]any{
		"op": "Curve.lineIntersect", "a1": a1, "a2": a2, "b1": b1, "b2": b2,
	})
}
