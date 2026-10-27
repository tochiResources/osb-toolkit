// The 2D command-list utilities.

package osbtk

// SetEasing sets an easing on every command (apply an easing to a whole
// group/scene).
func (sharedHelpersLib) SetEasing(commands []StoryboardCommand, easing Easing) ([]StoryboardCommand, error) {
	value, err := sharedHelpersLib{}.Call("command.setEasing", map[string]any{
		"commands": commands, "easing": easing,
	})
	if err != nil {
		return nil, err
	}
	return remarshal[[]StoryboardCommand](value)
}
