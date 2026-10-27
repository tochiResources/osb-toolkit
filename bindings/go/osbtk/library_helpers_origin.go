// The shared origin/anchor helpers.

package osbtk

// OriginVector returns the anchor offset for an enum origin, in texture pixels.
func (sharedHelpersLib) OriginVector(origin OsbOrigin, width, height float64) (Vector2, error) {
	value, err := sharedHelpersLib{}.Call("origin.getOriginVector", map[string]any{
		"origin": origin, "width": width, "height": height,
	})
	if err != nil {
		return Vector2{}, err
	}
	return remarshal[Vector2](value)
}

// ResolveAnchor returns the effective pivot: the enum origin vector plus an
// explicit anchor offset.
func (sharedHelpersLib) ResolveAnchor(origin OsbOrigin, anchor Vector2, width, height float64) (Vector2, error) {
	value, err := sharedHelpersLib{}.Call("origin.resolveAnchor", map[string]any{
		"origin": origin, "anchor": anchor, "width": width, "height": height,
	})
	if err != nil {
		return Vector2{}, err
	}
	return remarshal[Vector2](value)
}

// ResolveAnchor3d returns the 3D effective pivot: a Vector3 anchor whose z
// anchors the node's depth (z-index).
func (sharedHelpersLib) ResolveAnchor3d(origin OsbOrigin, anchor Vector3, width, height float64) (Vector3, error) {
	value, err := sharedHelpersLib{}.Call("origin.resolveAnchor3d", map[string]any{
		"origin": origin, "anchor": anchor, "width": width, "height": height,
	})
	if err != nil {
		return Vector3{}, err
	}
	return remarshal[Vector3](value)
}

// InScreenBounds reports whether a rotated sprite's bounds intersect the screen.
func (sharedHelpersLib) InScreenBounds(position, size Vector2, rotation float64, origin Vector2) (bool, error) {
	value, err := sharedHelpersLib{}.Call("origin.inScreenBounds", map[string]any{
		"position": position, "size": size, "rotation": rotation, "origin": origin,
	})
	if err != nil {
		return false, err
	}
	return remarshal[bool](value)
}
