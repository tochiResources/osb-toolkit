// The 2D scene graph (M40).

package osbtk

// ResolveScene2d resolves the scene into per-sprite command sets.
func (sharedHelpersLib) ResolveScene2d(scene, options any) (any, error) {
	return sharedHelpersLib{}.Call("scene2d.resolve", map[string]any{
		"scene": scene, "options": options,
	})
}

// BakeScene2d bakes the scene into a full storyboard model (per-node layers).
func (sharedHelpersLib) BakeScene2d(scene, options any) (Storyboard, error) {
	value, err := sharedHelpersLib{}.Call("scene2d.bake", map[string]any{
		"scene": scene, "options": options,
	})
	if err != nil {
		return Storyboard{}, err
	}
	return remarshal[Storyboard](value)
}

// Scene2dTimeRange returns the scene's occupied [start, end] range.
func (sharedHelpersLib) Scene2dTimeRange(scene any) (any, error) {
	return sharedHelpersLib{}.Call("scene2d.timeRange", map[string]any{"scene": scene})
}

// TranslateScene2d translates the whole scene (children inherit).
func (sharedHelpersLib) TranslateScene2d(scene any, delta []float64) (any, error) {
	return sharedHelpersLib{}.Call("scene2d.translate", map[string]any{
		"scene": scene, "delta": delta,
	})
}

// ScaleScene2d scales the whole scene about its origin.
func (sharedHelpersLib) ScaleScene2d(scene any, factor float64) (any, error) {
	return sharedHelpersLib{}.Call("scene2d.scale", map[string]any{
		"scene": scene, "factor": factor,
	})
}

// RotateScene2d rotates the whole scene (radians).
func (sharedHelpersLib) RotateScene2d(scene any, radians float64) (any, error) {
	return sharedHelpersLib{}.Call("scene2d.rotate", map[string]any{
		"scene": scene, "radians": radians,
	})
}

// FlattenScene2d flattens the whole scene.
func (sharedHelpersLib) FlattenScene2d(scene any, axis, mode string) (any, error) {
	return sharedHelpersLib{}.Call("scene2d.flatten", map[string]any{
		"scene": scene, "axis": axis, "mode": mode,
	})
}

// ShiftScene2d shifts the whole scene in time.
func (sharedHelpersLib) ShiftScene2d(scene any, delta float64) (any, error) {
	return sharedHelpersLib{}.Call("scene2d.shift", map[string]any{
		"scene": scene, "delta": delta,
	})
}

// ReverseScene2d reverses the whole scene in time.
func (sharedHelpersLib) ReverseScene2d(scene any, start, end float64) (any, error) {
	return sharedHelpersLib{}.Call("scene2d.reverse", map[string]any{
		"scene": scene, "start": start, "end": end,
	})
}
