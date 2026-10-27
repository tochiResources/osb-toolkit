// The storyboarding library facade: `.osu`/`.osb` parsing, the stateless
// origin/screen-bound queries and the 2D command utilities (M38).

package osbtk

// commandsLib is the 2D storyboard command utility facade (M38): pure
// transforms over a `[]StoryboardCommand`, canonical in
// `osb_runtime::storyboarding::helpers`.
type commandsLib struct{}

// Call calls any helper by name with a raw input object.
func (commandsLib) Call(name string, input map[string]any) (any, error) {
	return helperValue("Helpers.commands", name, input)
}

func (commandsLib) run(name string, input map[string]any) ([]StoryboardCommand, error) {
	value, err := commandsLib{}.Call(name, input)
	if err != nil {
		return nil, err
	}
	return decodeCommands(value)
}

// Select selects a timeframe (and/or kind) of commands without mutating the source.
func (c commandsLib) Select(commands []StoryboardCommand, kind *StoryboardCommandKind, start, end *float64) ([]StoryboardCommand, error) {
	return c.run("selectCommands", map[string]any{
		"commands": commands, "kind": kind, "start": start, "end": end,
	})
}

// Shift moves a command set in time by `delta` ms.
func (c commandsLib) Shift(commands []StoryboardCommand, delta float64) ([]StoryboardCommand, error) {
	return c.run("shiftCommands", map[string]any{"commands": commands, "delta": delta})
}

// Scale stretches a command set in time about `anchor` by `factor`.
func (c commandsLib) Scale(commands []StoryboardCommand, factor, anchor float64) ([]StoryboardCommand, error) {
	return c.run("scaleCommands", map[string]any{"commands": commands, "factor": factor, "anchor": anchor})
}

// Reverse reverses a command set in time about `[start, end]`.
func (c commandsLib) Reverse(commands []StoryboardCommand, start, end float64) ([]StoryboardCommand, error) {
	return c.run("reverseCommands", map[string]any{"commands": commands, "start": start, "end": end})
}

// Merge merges adjacent commands that are continuous in time and value.
func (c commandsLib) Merge(commands []StoryboardCommand, tolerance float64) ([]StoryboardCommand, error) {
	return c.run("mergeCommands", map[string]any{"commands": commands, "tolerance": tolerance})
}

// Split splits commands spanning `at` into two linear pieces.
func (c commandsLib) Split(commands []StoryboardCommand, at float64) ([]StoryboardCommand, error) {
	return c.run("splitCommands", map[string]any{"commands": commands, "at": at})
}

// Flatten collapses a command channel to a constant.
func (c commandsLib) Flatten(commands []StoryboardCommand, kind *StoryboardCommandKind, mode string) ([]StoryboardCommand, error) {
	return c.run("flattenCommands", map[string]any{"commands": commands, "kind": kind, "mode": mode})
}

// Randomize applies a seeded jitter to command values.
func (c commandsLib) Randomize(commands []StoryboardCommand, min, max float64, seed uint64) ([]StoryboardCommand, error) {
	return c.run("randomizeCommands", map[string]any{"commands": commands, "min": min, "max": max, "seed": seed})
}

// Filter keeps the commands matching a kind/time selector.
func (c commandsLib) Filter(commands []StoryboardCommand, kind *StoryboardCommandKind, start, end *float64) ([]StoryboardCommand, error) {
	return c.run("filterCommands", map[string]any{"commands": commands, "kind": kind, "start": start, "end": end})
}

// Remove removes the commands matching a kind/time selector.
func (c commandsLib) Remove(commands []StoryboardCommand, kind *StoryboardCommandKind, start, end *float64) ([]StoryboardCommand, error) {
	return c.run("removeCommands", map[string]any{"commands": commands, "kind": kind, "start": start, "end": end})
}

// Map applies a scalar value transform (`value * scale + offset`) to every command.
func (c commandsLib) Map(commands []StoryboardCommand, valueScale, valueOffset float64) ([]StoryboardCommand, error) {
	return c.run("mapCommands", map[string]any{"commands": commands, "valueScale": valueScale, "valueOffset": valueOffset})
}

// Duplicate clones a command set `count` times, each copy shifted by `offset` ms.
func (c commandsLib) Duplicate(commands []StoryboardCommand, offset float64, count int) ([]StoryboardCommand, error) {
	return c.run("duplicateCommands", map[string]any{"commands": commands, "offset": offset, "count": count})
}

// Align snaps command times to a beat grid (and values to `valueStep`).
func (c commandsLib) Align(commands []StoryboardCommand, beat, offset float64, subdivision int, valueStep float64) ([]StoryboardCommand, error) {
	return c.run("alignCommands", map[string]any{
		"commands": commands, "beat": beat, "offset": offset, "subdivision": subdivision, "valueStep": valueStep,
	})
}

// Quantize rounds command times/values to a step.
func (c commandsLib) Quantize(commands []StoryboardCommand, timeStep, valueStep float64) ([]StoryboardCommand, error) {
	return c.run("quantizeCommands", map[string]any{"commands": commands, "timeStep": timeStep, "valueStep": valueStep})
}

func decodeCommands(value any) ([]StoryboardCommand, error) {
	return remarshal[[]StoryboardCommand](value)
}

// storyboardingLib is the storyboarding library facade.
type storyboardingLib struct {
	Commands commandsLib
}

// Storyboarding is the storyboarding library: `.osu`/`.osb` parsing and the
// stateless origin/screen-bound queries.
var Storyboarding = storyboardingLib{}

// ParseOsu parses a `.osu` beatmap into the typed model.
func (storyboardingLib) ParseOsu(text string) (*Beatmap, error) {
	return decodeBridge[*Beatmap](map[string]any{"op": "Parsing.parseOsu", "text": text})
}

// ParseOsb parses a `.osb` storyboard into a summary.
func (storyboardingLib) ParseOsb(text string) (*ParseOsbResult, error) {
	return decodeBridge[*ParseOsbResult](map[string]any{"op": "Parsing.parseOsb", "text": text})
}

// GetOriginVector returns the anchor offset for an origin.
func (storyboardingLib) GetOriginVector(origin OsbOrigin, width, height float64) (*Vector2, error) {
	return decodeBridge[*Vector2](map[string]any{
		"op": "Origin.getOriginVector", "origin": origin, "width": width, "height": height,
	})
}

// InScreenBounds reports whether a rotated sprite's bounds intersect the screen.
func (storyboardingLib) InScreenBounds(position, size Vector2, rotation float64, origin Vector2) (bool, error) {
	result, err := decodeBridge[BoolResult](map[string]any{
		"op": "Origin.inScreenBounds", "position": position, "size": size, "rotation": rotation, "origin": origin,
	})
	if err != nil {
		return false, err
	}
	return result.Value, nil
}
