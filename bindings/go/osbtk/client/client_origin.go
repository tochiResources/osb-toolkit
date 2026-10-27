package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
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

func (c *Origin) GetOriginVector(ctx context.Context, body osbtk.GetOriginVectorRequest) (*osbtk.Vector2, *http.Response, error) {
	return c.api.GetOriginVector(ctx).GetOriginVectorRequest(body).Execute()
}

func (c *Origin) InScreenBounds(ctx context.Context, body osbtk.InScreenBoundsRequest) (*osbtk.BoolResult, *http.Response, error) {
	return c.api.InScreenBounds(ctx).InScreenBoundsRequest(body).Execute()
}
