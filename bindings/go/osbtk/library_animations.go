// The animations library facade: easing evaluation.

package osbtk

// animationsLib is the animations library facade.
type animationsLib struct{}

// Animations is the animations library: easing evaluation.
var Animations = animationsLib{}

// ApplyEasing applies a named easing function to a normalized time in `0..1`.
func (animationsLib) ApplyEasing(easing Easing, t float64) (float64, error) {
	result, err := decodeBridge[EasingResult](map[string]any{"op": "Animations.applyEasing", "easing": easing, "t": t})
	if err != nil {
		return 0, err
	}
	return result.Value, nil
}

// ApplyCubicBezier evaluates a CSS-style cubic-bezier easing with control points
// `(x1, y1)`, `(x2, y2)` at a normalized time.
func (animationsLib) ApplyCubicBezier(x1, y1, x2, y2, t float64) (float64, error) {
	result, err := decodeBridge[EasingResult](map[string]any{
		"op": "Animations.easingValue", "cubicBezier": []float64{x1, y1, x2, y2}, "t": t,
	})
	if err != nil {
		return 0, err
	}
	return result.Value, nil
}
