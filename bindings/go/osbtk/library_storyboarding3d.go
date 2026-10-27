// The 3D storyboard library facade: reuses the storyboarding surface and adds
// the 3D -> 2D bake (M28) and the motion-graphics keyframe helpers (M38).

package osbtk

// ChannelKey is a single keyframe: a time and a value of one, two or three components.
type ChannelKey struct {
	Time  float64   `json:"time"`
	Value []float64 `json:"value"`
}

// Channel is a keyframed channel: a list of `(time, value)` samples.
type Channel struct {
	Keys []ChannelKey `json:"keys"`
}

// helpersLib is the 3D motion-graphics keyframe helper facade (M38).
type helpersLib struct{}

// Call calls any helper by name with a raw input object.
func (helpersLib) Call(name string, input map[string]any) (any, error) {
	return helperValue("Helpers.keyframes", name, input)
}

func (helpersLib) channel(name string, input map[string]any) (*Channel, error) {
	value, err := helpersLib{}.Call(name, input)
	if err != nil {
		return nil, err
	}
	channel, err := remarshal[Channel](value)
	if err != nil {
		return nil, err
	}
	return &channel, nil
}

// Flatten collapses an axis (or every axis) to a constant.
func (h helpersLib) Flatten(channel Channel, axis *string, mode string) (*Channel, error) {
	return h.channel("flatten", map[string]any{"channel": channel, "axis": axis, "mode": mode})
}

// ScaleKeyframes multiplies every component by `factor`.
func (h helpersLib) ScaleKeyframes(channel Channel, factor float64) (*Channel, error) {
	return h.channel("scaleKeyframes", map[string]any{"channel": channel, "factor": factor})
}

// OffsetKeyframes adds a constant `delta` to every component.
func (h helpersLib) OffsetKeyframes(channel Channel, delta float64) (*Channel, error) {
	return h.channel("offsetKeyframes", map[string]any{"channel": channel, "delta": delta})
}

// Mirror reflects an axis (or every axis) about `pivot`.
func (h helpersLib) Mirror(channel Channel, axis *string, pivot float64) (*Channel, error) {
	return h.channel("mirror", map[string]any{"channel": channel, "axis": axis, "pivot": pivot})
}

// Randomize applies a seeded jitter in `min..max`, preserving keyframe times.
func (h helpersLib) Randomize(channel Channel, axis *string, min, max float64, seed uint64, step float64) (*Channel, error) {
	return h.channel("randomize", map[string]any{
		"channel": channel, "axis": axis, "min": min, "max": max, "seed": seed, "step": step,
	})
}

// ReverseKeyframes reverses the channel in time about `[start, end]`.
func (h helpersLib) ReverseKeyframes(channel Channel, start, end float64) (*Channel, error) {
	return h.channel("reverseKeyframes", map[string]any{"channel": channel, "start": start, "end": end})
}

// RepeatKeyframes tiles the `[start, end]` span `count` times.
func (h helpersLib) RepeatKeyframes(channel Channel, count int, start, end float64, pingPong bool) (*Channel, error) {
	return h.channel("repeatKeyframes", map[string]any{
		"channel": channel, "count": count, "start": start, "end": end, "pingPong": pingPong,
	})
}

// ResampleKeyframes resamples to a fixed `step` (or to exactly `count` samples when `step <= 0`).
func (h helpersLib) ResampleKeyframes(channel Channel, step float64, count int) (*Channel, error) {
	return h.channel("resampleKeyframes", map[string]any{"channel": channel, "step": step, "count": count})
}

// SmoothKeyframes applies a moving-average smoothing over a window of neighbouring keyframes.
func (h helpersLib) SmoothKeyframes(channel Channel, window int) (*Channel, error) {
	return h.channel("smoothKeyframes", map[string]any{"channel": channel, "window": window})
}

// SimplifyKeyframes runs Douglas-Peucker simplification of each component.
func (h helpersLib) SimplifyKeyframes(channel Channel, tolerance float64, decimals int) (*Channel, error) {
	return h.channel("simplifyKeyframes", map[string]any{"channel": channel, "tolerance": tolerance, "decimals": decimals})
}

// AlignKeyframes snaps keyframe times to a beat grid.
func (h helpersLib) AlignKeyframes(channel Channel, beat, offset float64, subdivision int) (*Channel, error) {
	return h.channel("alignKeyframes", map[string]any{
		"channel": channel, "beat": beat, "offset": offset, "subdivision": subdivision,
	})
}

// EaseKeyframes re-eases each span (named easing).
func (h helpersLib) EaseKeyframes(channel Channel, easing string, samples int) (*Channel, error) {
	return h.channel("easeKeyframes", map[string]any{"channel": channel, "easing": easing, "samples": samples})
}

// MergeKeyframes blends the channel toward `other` by `amount` (`0..=1`).
func (h helpersLib) MergeKeyframes(channel, other Channel, amount float64) (*Channel, error) {
	return h.channel("mergeKeyframes", map[string]any{"channel": channel, "other": other, "amount": amount})
}

// Stagger returns a stagger delay for element `index`: `index * step`.
func (h helpersLib) Stagger(index int, step float64) (float64, error) {
	value, err := helpersLib{}.Call("stagger", map[string]any{"index": index, "step": step})
	if err != nil {
		return 0, err
	}
	return remarshal[float64](value)
}

// Orbit builds a circular orbit position channel.
func (h helpersLib) Orbit(center []float64, radiusX, radiusY, startAngle, sweep float64, count int) (*Channel, error) {
	return h.channel("orbit", map[string]any{
		"center": center, "radiusX": radiusX, "radiusY": radiusY,
		"startAngle": startAngle, "sweep": sweep, "count": count,
	})
}

// Pulse builds a pulse oscillation channel.
func (h helpersLib) Pulse(base, amplitude, frequency, phase, step, duration float64) (*Channel, error) {
	return h.channel("pulse", map[string]any{
		"base": base, "amplitude": amplitude, "frequency": frequency,
		"phase": phase, "step": step, "duration": duration,
	})
}

// Scatter returns seeded random placements in a box.
func (h helpersLib) Scatter(seed uint64, count int, min, max []float64, spread float64) ([]Vector3, error) {
	value, err := helpersLib{}.Call("scatter", map[string]any{
		"seed": seed, "count": count, "min": min, "max": max, "spread": spread,
	})
	if err != nil {
		return nil, err
	}
	return remarshal[[]Vector3](value)
}

// Spin builds a procedural spin about `axis`.
func (h helpersLib) Spin(axis string, turns, duration float64, count int) (*Channel, error) {
	return h.channel("spin", map[string]any{"axis": axis, "turns": turns, "duration": duration, "count": count})
}

// Tumble builds independent spins about X, Y and Z.
func (h helpersLib) Tumble(turns []float64, duration float64, count int) (*Channel, error) {
	return h.channel("tumble", map[string]any{"turns": turns, "duration": duration, "count": count})
}

// PathFollow builds a position channel sampled along a polyline.
func (h helpersLib) PathFollow(points [][]float64, count int, byArcLength bool) (*Channel, error) {
	return h.channel("pathFollow", map[string]any{"points": points, "count": count, "byArcLength": byArcLength})
}

// storyboarding3dLib is the 3D storyboard library facade; it embeds the
// storyboarding surface so the 3D API is exposed alongside it.
type storyboarding3dLib struct {
	storyboardingLib
	Helpers helpersLib
}

// Storyboarding3d is the 3D storyboard library: a sibling that reuses the
// storyboarding surface and adds the bake and the keyframe helpers.
var Storyboarding3d = storyboarding3dLib{storyboardingLib: Storyboarding}

// Bake3d bakes a 3D scene into a 2D storyboard.
func (storyboarding3dLib) Bake3d(scene Scene3dInput) (*BakedStoryboard, error) {
	return decodeBridge[*BakedStoryboard](map[string]any{"op": "Storyboarding3d.bake3d", "input": scene})
}
