package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
)

// Diagnostics is the clean high-level client.
type Diagnostics struct {
	api *osbtk.DiagnosticsAPIService
}

func NewDiagnostics(api *osbtk.DiagnosticsAPIService) *Diagnostics {
	return &Diagnostics{api: api}
}

func IsDiagnostics(value any) bool {
	_, ok := value.(*Diagnostics)
	return ok
}

func (c *Diagnostics) DiagnoseOsb(ctx context.Context, body osbtk.TextRequest) (*osbtk.DiagnosticReport, *http.Response, error) {
	return c.api.DiagnoseOsb(ctx).TextRequest(body).Execute()
}

func (c *Diagnostics) DiagnoseOsu(ctx context.Context, body osbtk.TextRequest) (*osbtk.DiagnosticReport, *http.Response, error) {
	return c.api.DiagnoseOsu(ctx).TextRequest(body).Execute()
}
