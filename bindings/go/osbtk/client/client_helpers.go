package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
)

// Helpers is the clean high-level client.
type Helpers struct {
	api *osbtk.HelpersAPIService
}

func NewHelpers(api *osbtk.HelpersAPIService) *Helpers {
	return &Helpers{api: api}
}

func IsHelpers(value any) bool {
	_, ok := value.(*Helpers)
	return ok
}

func (c *Helpers) Call(ctx context.Context, body osbtk.HelperRequest) (*osbtk.HelperResult, *http.Response, error) {
	return c.api.Call(ctx).HelperRequest(body).Execute()
}

func (c *Helpers) Commands(ctx context.Context, body osbtk.HelperRequest) (*osbtk.HelperResult, *http.Response, error) {
	return c.api.Commands(ctx).HelperRequest(body).Execute()
}

func (c *Helpers) Keyframes(ctx context.Context, body osbtk.HelperRequest) (*osbtk.HelperResult, *http.Response, error) {
	return c.api.Keyframes(ctx).HelperRequest(body).Execute()
}
