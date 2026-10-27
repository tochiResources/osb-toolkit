// The image library facade: decode/encode and processing.

package osbtk

// imageLib is the image library facade.
type imageLib struct{}

// Image is the image library.
var Image = imageLib{}

// Info reads image metadata.
func (imageLib) Info(path string) (*ImageInfo, error) {
	return decodeBridge[*ImageInfo](map[string]any{"op": "Image.info", "path": path})
}

// Process reads `path`, applies `options` and writes `output`.
func (imageLib) Process(path, output string, options ImageOptions) (*ImageInfo, error) {
	return decodeBridge[*ImageInfo](map[string]any{
		"op": "Image.process", "path": path, "output": output, "options": options,
	})
}
