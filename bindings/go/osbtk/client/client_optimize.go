package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
)

// Optimize is the clean high-level client.
type Optimize struct {
	api *osbtk.OptimizeAPIService
}

func NewOptimize(api *osbtk.OptimizeAPIService) *Optimize {
	return &Optimize{api: api}
}

func IsOptimize(value any) bool {
	_, ok := value.(*Optimize)
	return ok
}

func (c *Optimize) OptimizeOsb(ctx context.Context, body osbtk.OptimizeRequest) (*osbtk.OptimizeResult, *http.Response, error) {
	return c.api.OptimizeOsb(ctx).OptimizeRequest(body).Execute()
}
