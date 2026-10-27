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

func (c *Generator) ControlPointAt(ctx context.Context, body osbtk.TimeRequest) (*osbtk.ControlPoint, *http.Response, error) {
	return c.api.ControlPointAt(ctx).TimeRequest(body).Execute()
}

func (c *Generator) ImportOsb(ctx context.Context, body osbtk.TextRequest) (*osbtk.ImportOsbResult, *http.Response, error) {
	return c.api.ImportOsb(ctx).TextRequest(body).Execute()
}

func (c *Generator) LayerHandle(ctx context.Context, body osbtk.LayerHandleRequest) (*osbtk.LayerHandle, *http.Response, error) {
	return c.api.LayerHandle(ctx).LayerHandleRequest(body).Execute()
}

func (c *Generator) LoadBeatmap(ctx context.Context, body osbtk.TextRequest) (*osbtk.LoadBeatmapResult, *http.Response, error) {
	return c.api.LoadBeatmap(ctx).TextRequest(body).Execute()
}

func (c *Generator) Random(ctx context.Context, body osbtk.RandomRequest) (*osbtk.RandomResult, *http.Response, error) {
	return c.api.Random(ctx).RandomRequest(body).Execute()
}

func (c *Generator) SpriteCount(ctx context.Context) (*osbtk.SpriteCountResult, *http.Response, error) {
	return c.api.SpriteCount(ctx).Execute()
}

func (c *Generator) Storyboard(ctx context.Context) (*osbtk.Storyboard, *http.Response, error) {
	return c.api.Storyboard(ctx).Execute()
}

func (c *Generator) TimingPointAt(ctx context.Context, body osbtk.TimeRequest) (*osbtk.ControlPoint, *http.Response, error) {
	return c.api.TimingPointAt(ctx).TimeRequest(body).Execute()
}
