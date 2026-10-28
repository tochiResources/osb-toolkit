package runtime

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Generator is the clean high-level client.
type Generator struct {
	api *osbtk.GeneratorAPIService
}

func NewGenerator(api *osbtk.GeneratorAPIService) *Generator {
	return &Generator{api: api}
}

func IsGenerator(value any) bool {
	_, ok := value.(*Generator)
	return ok
}

// Beatmap: Calls the Generator.beatmap bridge operation.
//
// ctx: The request context.
//
// returns: The loaded beatmap, or null
func (c *Generator) Beatmap(ctx context.Context) (*osbtk.Beatmap, *http.Response, error) {
	return c.api.Beatmap(ctx).Execute()
}

// ControlPointAt: A time query.
//
// ctx: The request context.
// options: A time query.
//
// returns: The active control point, or null
func (c *Generator) ControlPointAt(ctx context.Context, options clean.TimeOptions) (*osbtk.ControlPoint, *http.Response, error) {
	body := osbtk.NewTimeRequest(options.Time)
	return c.api.ControlPointAt(ctx).TimeRequest(*body).Execute()
}

// ImportOsb: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// ctx: The request context.
// options: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// returns: The sprite count after import
func (c *Generator) ImportOsb(ctx context.Context, options clean.TextOptions) (int32, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	value, resp, err := c.api.ImportOsb(ctx).TextRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.SpriteCount, resp, nil
}

// LayerHandle: Resolve (or create) a layer by name.
//
// ctx: The request context.
// options: Resolve (or create) a layer by name.
//
// returns: The layer handle
func (c *Generator) LayerHandle(ctx context.Context, options clean.LayerHandleOptions) (int32, *http.Response, error) {
	body := osbtk.NewLayerHandleRequest(options.Name)
	value, resp, err := c.api.LayerHandle(ctx).LayerHandleRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Layer, resp, nil
}

// LoadBeatmap: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// ctx: The request context.
// options: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// returns: A summary of the loaded beatmap
func (c *Generator) LoadBeatmap(ctx context.Context, options clean.TextOptions) (*osbtk.LoadBeatmapResult, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	return c.api.LoadBeatmap(ctx).TextRequest(*body).Execute()
}

// Random: A deterministic random request.
//
// ctx: The request context.
// options: A deterministic random request.
//
// returns: A deterministic random value
func (c *Generator) Random(ctx context.Context, options clean.RandomOptions) (float64, *http.Response, error) {
	body := osbtk.NewRandomRequest(options.Max, options.Min)
	value, resp, err := c.api.Random(ctx).RandomRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Value, resp, nil
}

// SpriteCount: Calls the Generator.spriteCount bridge operation.
//
// ctx: The request context.
//
// returns: The storyboard sprite count
func (c *Generator) SpriteCount(ctx context.Context) (int32, *http.Response, error) {
	value, resp, err := c.api.SpriteCount(ctx).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.SpriteCount, resp, nil
}

// Storyboard: Calls the Generator.storyboard bridge operation.
//
// ctx: The request context.
//
// returns: The built storyboard model
func (c *Generator) Storyboard(ctx context.Context) (*osbtk.Storyboard, *http.Response, error) {
	return c.api.Storyboard(ctx).Execute()
}

// TimingPointAt: A time query.
//
// ctx: The request context.
// options: A time query.
//
// returns: The active red timing point
func (c *Generator) TimingPointAt(ctx context.Context, options clean.TimeOptions) (*osbtk.ControlPoint, *http.Response, error) {
	body := osbtk.NewTimeRequest(options.Time)
	return c.api.TimingPointAt(ctx).TimeRequest(*body).Execute()
}
