package libraries

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
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

// DiagnoseOsb: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// ctx: The request context.
// options: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// returns: Diagnostics for an `.osb` storyboard
func (c *Diagnostics) DiagnoseOsb(ctx context.Context, options clean.TextOptions) (*osbtk.DiagnosticReport, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	return c.api.DiagnoseOsb(ctx).TextRequest(*body).Execute()
}

// DiagnoseOsu: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// ctx: The request context.
// options: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// returns: Diagnostics for an `.osu` beatmap
func (c *Diagnostics) DiagnoseOsu(ctx context.Context, options clean.TextOptions) (*osbtk.DiagnosticReport, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	return c.api.DiagnoseOsu(ctx).TextRequest(*body).Execute()
}
