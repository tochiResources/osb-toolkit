package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
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

func (c *Generator) Beatmap(ctx context.Context) (*osbtk.Beatmap, *http.Response, error) {
	return c.api.Beatmap(ctx).Execute()
}

func (c *Generator) ControlPointAt(ctx context.Context, options TimeOptions) (*osbtk.ControlPoint, *http.Response, error) {
	body := osbtk.NewTimeRequest(options.Time)
	return c.api.ControlPointAt(ctx).TimeRequest(*body).Execute()
}

func (c *Generator) ImportOsb(ctx context.Context, options TextOptions) (*osbtk.ImportOsbResult, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	return c.api.ImportOsb(ctx).TextRequest(*body).Execute()
}

func (c *Generator) LayerHandle(ctx context.Context, options LayerHandleOptions) (*osbtk.LayerHandle, *http.Response, error) {
	body := osbtk.NewLayerHandleRequest(options.Name)
	return c.api.LayerHandle(ctx).LayerHandleRequest(*body).Execute()
}

func (c *Generator) LoadBeatmap(ctx context.Context, options TextOptions) (*osbtk.LoadBeatmapResult, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	return c.api.LoadBeatmap(ctx).TextRequest(*body).Execute()
}

func (c *Generator) Random(ctx context.Context, options RandomOptions) (*osbtk.RandomResult, *http.Response, error) {
	body := osbtk.NewRandomRequest(options.Max, options.Min)
	return c.api.Random(ctx).RandomRequest(*body).Execute()
}

func (c *Generator) SpriteCount(ctx context.Context) (*osbtk.SpriteCountResult, *http.Response, error) {
	return c.api.SpriteCount(ctx).Execute()
}

func (c *Generator) Storyboard(ctx context.Context) (*osbtk.Storyboard, *http.Response, error) {
	return c.api.Storyboard(ctx).Execute()
}

func (c *Generator) TimingPointAt(ctx context.Context, options TimeOptions) (*osbtk.ControlPoint, *http.Response, error) {
	body := osbtk.NewTimeRequest(options.Time)
	return c.api.TimingPointAt(ctx).TimeRequest(*body).Execute()
}
