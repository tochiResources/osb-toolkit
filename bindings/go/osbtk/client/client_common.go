package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
)

// Common is the clean high-level client.
type Common struct {
	api *osbtk.CommonAPIService
}

func NewCommon(api *osbtk.CommonAPIService) *Common {
	return &Common{api: api}
}

func IsCommon(value any) bool {
	_, ok := value.(*Common)
	return ok
}

func (c *Common) Version(ctx context.Context) (*osbtk.VersionResult, *http.Response, error) {
	return c.api.Version(ctx).Execute()
}
