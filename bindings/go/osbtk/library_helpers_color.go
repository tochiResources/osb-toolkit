// The color-space and CSS/Sass-style helpers (M40).

package osbtk

// ColorToSpace converts a color into its (x, y, z, w) components in `space`.
func (sharedHelpersLib) ColorToSpace(color Color4, space string) ([]float64, error) {
	value, err := sharedHelpersLib{}.Call("color.toSpace", map[string]any{
		"color": color, "space": space,
	})
	if err != nil {
		return nil, err
	}
	return remarshal[[]float64](value)
}

// ColorOffset adds a per-component delta in `space`.
func (sharedHelpersLib) ColorOffset(color Color4, delta []float64, space string) (Color4, error) {
	value, err := sharedHelpersLib{}.Call("color.offset", map[string]any{
		"color": color, "space": space, "delta": delta,
	})
	if err != nil {
		return Color4{}, err
	}
	return remarshal[Color4](value)
}

// ColorScale multiplies each component by a per-component factor in `space`.
func (sharedHelpersLib) ColorScale(color Color4, factor []float64, space string) (Color4, error) {
	value, err := sharedHelpersLib{}.Call("color.scale", map[string]any{
		"color": color, "space": space, "factor": factor,
	})
	if err != nil {
		return Color4{}, err
	}
	return remarshal[Color4](value)
}

func (sharedHelpersLib) applyColor(color Color4, op string, extra map[string]any) (Color4, error) {
	input := map[string]any{"color": color, "op": op}
	for key, value := range extra {
		input[key] = value
	}
	value, err := sharedHelpersLib{}.Call("color.apply", input)
	if err != nil {
		return Color4{}, err
	}
	return remarshal[Color4](value)
}

// Lighten raises HSL lightness.
func (sharedHelpersLib) Lighten(color Color4, amount float64) (Color4, error) {
	return sharedHelpersLib{}.applyColor(color, "lighten", map[string]any{"amount": amount})
}

// Darken lowers HSL lightness.
func (sharedHelpersLib) Darken(color Color4, amount float64) (Color4, error) {
	return sharedHelpersLib{}.applyColor(color, "darken", map[string]any{"amount": amount})
}

// AdjustHue rotates the hue by `degrees`.
func (sharedHelpersLib) AdjustHue(color Color4, degrees float64) (Color4, error) {
	return sharedHelpersLib{}.applyColor(color, "adjustHue", map[string]any{"degrees": degrees})
}

// Complement is the 180-degree hue rotation.
func (sharedHelpersLib) Complement(color Color4) (Color4, error) {
	return sharedHelpersLib{}.applyColor(color, "complement", nil)
}

// Grayscale drops HSL saturation to zero.
func (sharedHelpersLib) Grayscale(color Color4) (Color4, error) {
	return sharedHelpersLib{}.applyColor(color, "grayscale", nil)
}

// Invert is the RGB inverse, keeping alpha.
func (sharedHelpersLib) Invert(color Color4) (Color4, error) {
	return sharedHelpersLib{}.applyColor(color, "invert", nil)
}

// Mix is the Sass weighted mix, accounting for alpha.
func (sharedHelpersLib) Mix(a, b Color4, weight float64) (Color4, error) {
	return sharedHelpersLib{}.applyColor(a, "mix", map[string]any{"other": b, "weight": weight})
}

// Tint mixes toward white.
func (sharedHelpersLib) Tint(color Color4, weight float64) (Color4, error) {
	return sharedHelpersLib{}.applyColor(color, "tint", map[string]any{"weight": weight})
}

// Shade mixes toward black.
func (sharedHelpersLib) Shade(color Color4, weight float64) (Color4, error) {
	return sharedHelpersLib{}.applyColor(color, "shade", map[string]any{"weight": weight})
}

// Gradient samples a multi-stop gradient at t (0..=1).
func (sharedHelpersLib) Gradient(stops []any, t float64, space string, repeat bool) (Color4, error) {
	value, err := sharedHelpersLib{}.Call("color.gradient", map[string]any{
		"stops": stops, "t": t, "space": space, "repeat": repeat,
	})
	if err != nil {
		return Color4{}, err
	}
	return remarshal[Color4](value)
}

// ColorRandomize jitters each component by a seeded per-component range in space.
func (sharedHelpersLib) ColorRandomize(color Color4, min, max []float64, seed int64, space string) (Color4, error) {
	value, err := sharedHelpersLib{}.Call("color.randomize", map[string]any{
		"color": color, "space": space, "seed": seed, "min": min, "max": max,
	})
	if err != nil {
		return Color4{}, err
	}
	return remarshal[Color4](value)
}

// ColorOffsetChannel offsets a whole color channel in space.
func (sharedHelpersLib) ColorOffsetChannel(channel any, delta []float64, space string) (any, error) {
	return sharedHelpersLib{}.Call("color.offsetChannel", map[string]any{
		"channel": channel, "space": space, "delta": delta,
	})
}

// ColorScaleChannel scales a whole color channel in space.
func (sharedHelpersLib) ColorScaleChannel(channel any, factor []float64, space string) (any, error) {
	return sharedHelpersLib{}.Call("color.scaleChannel", map[string]any{
		"channel": channel, "space": space, "factor": factor,
	})
}

// ColorRandomizeChannel randomizes a whole color channel in space.
func (sharedHelpersLib) ColorRandomizeChannel(channel any, min, max []float64, seed int64, space string) (any, error) {
	return sharedHelpersLib{}.Call("color.randomizeChannel", map[string]any{
		"channel": channel, "space": space, "seed": seed, "min": min, "max": max,
	})
}

// ColorApplyChannel applies a named helper to every keyframe of a color channel.
func (sharedHelpersLib) ColorApplyChannel(channel any, op string, extra map[string]any) (any, error) {
	input := map[string]any{"channel": channel, "op": op}
	for key, value := range extra {
		input[key] = value
	}
	return sharedHelpersLib{}.Call("color.applyChannel", input)
}
