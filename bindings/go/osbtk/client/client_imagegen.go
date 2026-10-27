package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
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

func (c *Imagegen) RenderShapes(ctx context.Context, body osbtk.RenderShapesRequest) (*osbtk.ImageInfo, *http.Response, error) {
	return c.api.RenderShapes(ctx).RenderShapesRequest(body).Execute()
}
