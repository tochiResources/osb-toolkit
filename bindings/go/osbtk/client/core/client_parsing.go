package core

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Parsing is the clean high-level client.
type Parsing struct {
	api *osbtk.ParsingAPIService
}

func NewParsing(api *osbtk.ParsingAPIService) *Parsing {
	return &Parsing{api: api}
}

func IsParsing(value any) bool {
	_, ok := value.(*Parsing)
	return ok
}

// ParseOsb: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// ctx: The request context.
// options: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// returns: The storyboard summary
func (c *Parsing) ParseOsb(ctx context.Context, options clean.TextOptions) (*osbtk.ParseOsbResult, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	return c.api.ParseOsb(ctx).TextRequest(*body).Execute()
}

// ParseOsu: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// ctx: The request context.
// options: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// returns: The parsed beatmap
func (c *Parsing) ParseOsu(ctx context.Context, options clean.TextOptions) (*osbtk.Beatmap, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	return c.api.ParseOsu(ctx).TextRequest(*body).Execute()
}
