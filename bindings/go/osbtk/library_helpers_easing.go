// The easing-curve manipulation helpers (M39).

package osbtk

// AmplifyEasing amplifies an easing about linear by `amount` (>1 stronger).
func (sharedHelpersLib) AmplifyEasing(easing Easing, t, amount float64) (float64, error) {
	value, err := sharedHelpersLib{}.Call("easing.amplifyEasing", map[string]any{
		"easing": easing, "t": t, "amount": amount,
	})
	if err != nil {
		return 0, err
	}
	return remarshal[float64](value)
}

// SampleEasing samples an easing at `count` points in [0, 1].
func (sharedHelpersLib) SampleEasing(easing Easing, count int) ([]float64, error) {
	value, err := sharedHelpersLib{}.Call("easing.sampleEasing", map[string]any{
		"easing": easing, "count": count,
	})
	if err != nil {
		return nil, err
	}
	return remarshal[[]float64](value)
}
