// The shared motion-graphics helpers: one `helper` op namespaced by
// library (`keyframe.*`, `scene3d.*`, `command.*`, `origin.*`, `easing.*`,
// `timeline.*`, `color.*`, `scene2d.*`). The surface is code-split by namespace
// across `library_helpers_*.go`; this file holds the entry point.

package osbtk

// sharedHelpersLib is the shared motion-graphics helper surface.
type sharedHelpersLib struct{}

// Helpers is the shared motion-graphics helper surface.
var Helpers = sharedHelpersLib{}

// Call calls any shared helper by name with a raw input object.
func (sharedHelpersLib) Call(name string, input map[string]any) (any, error) {
	return helperValue("Helpers.call", name, input)
}

// TransformCommands applies any keyframe transform (e.g. `keyframe.flatten`) to
// a 2D command channel.
func (sharedHelpersLib) TransformCommands(
	commands []StoryboardCommand,
	kind StoryboardCommandKind,
	name string,
	options map[string]any,
) ([]StoryboardCommand, error) {
	input := map[string]any{}
	for key, value := range options {
		input[key] = value
	}
	input["commands"] = commands
	input["kind"] = kind
	value, err := sharedHelpersLib{}.Call(name, input)
	if err != nil {
		return nil, err
	}
	return remarshal[[]StoryboardCommand](value)
}
