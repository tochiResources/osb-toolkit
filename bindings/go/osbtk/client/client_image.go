package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
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

func (c *Image) Info(ctx context.Context, body osbtk.PathRequest) (*osbtk.ImageInfo, *http.Response, error) {
	return c.api.Info(ctx).PathRequest(body).Execute()
}

func (c *Image) Process(ctx context.Context, body osbtk.ImageProcessRequest) (*osbtk.ImageInfo, *http.Response, error) {
	return c.api.Process(ctx).ImageProcessRequest(body).Execute()
}
