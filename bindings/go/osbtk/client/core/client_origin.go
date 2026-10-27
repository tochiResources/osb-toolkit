package core

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Origin is the clean high-level client.
type Origin struct {
	api *osbtk.OriginAPIService
}

func NewOrigin(api *osbtk.OriginAPIService) *Origin {
	return &Origin{api: api}
}

func IsOrigin(value any) bool {
	_, ok := value.(*Origin)
	return ok
}

func (c *Origin) GetOriginVector(ctx context.Context, options clean.GetOriginVectorOptions) (*osbtk.Vector2, *http.Response, error) {
	body := osbtk.NewGetOriginVectorRequest(options.Height, options.Origin, options.Width)
	return c.api.GetOriginVector(ctx).GetOriginVectorRequest(*body).Execute()
}

func (c *Origin) InScreenBounds(ctx context.Context, options clean.InScreenBoundsOptions) (*osbtk.BoolResult, *http.Response, error) {
	body := osbtk.NewInScreenBoundsRequest(options.Origin, options.Position, options.Rotation, options.Size)
	return c.api.InScreenBounds(ctx).InScreenBoundsRequest(*body).Execute()
}
