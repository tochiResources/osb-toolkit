package libraries

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Imagegen is the clean high-level client.
type Imagegen struct {
	api *osbtk.ImagegenAPIService
}

func NewImagegen(api *osbtk.ImagegenAPIService) *Imagegen {
	return &Imagegen{api: api}
}

func IsImagegen(value any) bool {
	_, ok := value.(*Imagegen)
	return ok
}

// RenderShapes: A shape-rendering request: a canvas, a background and a shape list.
//
// ctx: The request context.
// options: A shape-rendering request: a canvas, a background and a shape list.
//
// returns: The rendered image's metadata
func (c *Imagegen) RenderShapes(ctx context.Context, options clean.RenderShapesOptions) (*osbtk.ImageInfo, *http.Response, error) {
	body := osbtk.NewRenderShapesRequest(options.Height, options.Output, options.Shapes, options.Width)
	if options.Background != nil {
		body.SetBackground(*options.Background)
	}
	return c.api.RenderShapes(ctx).RenderShapesRequest(*body).Execute()
}
