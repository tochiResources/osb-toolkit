// The 3D world and unified storyboard bake.

package osbtk

// BakeWorld bakes a 3D world (`{ nodes, camera, options }`) into a storyboard model.
func (sharedHelpersLib) BakeWorld(world, options2d any) (Storyboard, error) {
	value, err := sharedHelpersLib{}.Call("scene3d.bakeWorld", map[string]any{
		"world": world, "options2d": options2d,
	})
	if err != nil {
		return Storyboard{}, err
	}
	return remarshal[Storyboard](value)
}

// BakeScene bakes a set of 2D and 3D worlds (`{ worlds2d, worlds3d, ... }`) into
// one storyboard model.
func (sharedHelpersLib) BakeScene(input map[string]any) (Storyboard, error) {
	value, err := sharedHelpersLib{}.Call("scene3d.bakeScene", input)
	if err != nil {
		return Storyboard{}, err
	}
	return remarshal[Storyboard](value)
}
