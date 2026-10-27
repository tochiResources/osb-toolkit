// The image-generation library facade: draw primitive shapes.

package osbtk

// imageGenLib is the image-generation library facade.
type imageGenLib struct{}

// ImageGen is the image-generation library.
var ImageGen = imageGenLib{}

// RenderShapes renders primitive shapes to a new image.
func (imageGenLib) RenderShapes(request RenderShapesRequest) (*ImageInfo, error) {
	payload, err := remarshal[map[string]any](request)
	if err != nil {
		return nil, err
	}
	payload["op"] = "Imagegen.renderShapes"
	return decodeBridge[*ImageInfo](payload)
}
