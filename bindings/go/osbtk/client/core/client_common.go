package core

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

func (c *Common) Version(ctx context.Context) (string, *http.Response, error) {
	value, resp, err := c.api.Version(ctx).Execute()
	if err != nil {
		return "", resp, err
	}
	return value.Version, resp, nil
}
