package libraries

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Image is the clean high-level client.
type Image struct {
	api *osbtk.ImageAPIService
}

func NewImage(api *osbtk.ImageAPIService) *Image {
	return &Image{api: api}
}

func IsImage(value any) bool {
	_, ok := value.(*Image)
	return ok
}

// Info: A request carrying a single file path.
//
// ctx: The request context.
// options: A request carrying a single file path.
//
// returns: The image's metadata
func (c *Image) Info(ctx context.Context, options clean.PathOptions) (*osbtk.ImageInfo, *http.Response, error) {
	body := osbtk.NewPathRequest(options.Path)
	return c.api.Info(ctx).PathRequest(*body).Execute()
}

// Process: An image processing request: read `path`, apply `options`, write `output`.
//
// ctx: The request context.
// options: An image processing request: read `path`, apply `options`, write `output`.
//
// returns: The processed image's metadata
func (c *Image) Process(ctx context.Context, options clean.ImageProcessOptions) (*osbtk.ImageInfo, *http.Response, error) {
	body := osbtk.NewImageProcessRequest(options.Options, options.Output, options.Path)
	return c.api.Process(ctx).ImageProcessRequest(*body).Execute()
}
