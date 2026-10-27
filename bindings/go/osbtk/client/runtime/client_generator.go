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

func (c *Generator) Beatmap(ctx context.Context) (*osbtk.Beatmap, *http.Response, error) {
	return c.api.Beatmap(ctx).Execute()
}

func (c *Generator) ControlPointAt(ctx context.Context, options clean.TimeOptions) (*osbtk.ControlPoint, *http.Response, error) {
	body := osbtk.NewTimeRequest(options.Time)
	return c.api.ControlPointAt(ctx).TimeRequest(*body).Execute()
}

func (c *Generator) ImportOsb(ctx context.Context, options clean.TextOptions) (int32, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	value, resp, err := c.api.ImportOsb(ctx).TextRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.SpriteCount, resp, nil
}

func (c *Generator) LayerHandle(ctx context.Context, options clean.LayerHandleOptions) (int32, *http.Response, error) {
	body := osbtk.NewLayerHandleRequest(options.Name)
	value, resp, err := c.api.LayerHandle(ctx).LayerHandleRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Layer, resp, nil
}

func (c *Generator) LoadBeatmap(ctx context.Context, options clean.TextOptions) (*osbtk.LoadBeatmapResult, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	return c.api.LoadBeatmap(ctx).TextRequest(*body).Execute()
}

func (c *Generator) Random(ctx context.Context, options clean.RandomOptions) (float64, *http.Response, error) {
	body := osbtk.NewRandomRequest(options.Max, options.Min)
	value, resp, err := c.api.Random(ctx).RandomRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Value, resp, nil
}

func (c *Generator) SpriteCount(ctx context.Context) (int32, *http.Response, error) {
	value, resp, err := c.api.SpriteCount(ctx).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.SpriteCount, resp, nil
}

func (c *Generator) Storyboard(ctx context.Context) (*osbtk.Storyboard, *http.Response, error) {
	return c.api.Storyboard(ctx).Execute()
}

func (c *Generator) TimingPointAt(ctx context.Context, options clean.TimeOptions) (*osbtk.ControlPoint, *http.Response, error) {
	body := osbtk.NewTimeRequest(options.Time)
	return c.api.TimingPointAt(ctx).TimeRequest(*body).Execute()
}
