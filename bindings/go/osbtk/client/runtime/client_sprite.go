package runtime

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Sprite is the handle-bound clean sprite client.
type Sprite struct {
	api    *osbtk.SpriteAPIService
	layer  int32
	sprite int32
}

func NewSprite(api *osbtk.SpriteAPIService, layer int32, sprite int32) *Sprite {
	return &Sprite{api: api, layer: layer, sprite: sprite}
}

func IsSprite(value any) bool {
	_, ok := value.(*Sprite)
	return ok
}

func (c *Sprite) Command(ctx context.Context, options clean.CommandOptions) (*http.Response, error) {
	body := osbtk.NewCommandRequest(options.Easing, options.EndTime, options.Kind, c.layer, c.sprite, options.StartTime)
	if options.End != nil {
		body.SetEnd(options.End)
	}
	if options.Parameter != nil {
		body.SetParameter(*options.Parameter)
	}
	if options.Start != nil {
		body.SetStart(options.Start)
	}
	_, resp, err := c.api.Command(ctx).CommandRequest(*body).Execute()
	return resp, err
}

func (c *Sprite) EndGroup(ctx context.Context) (*http.Response, error) {
	body := osbtk.NewHandleRequest(c.layer, c.sprite)
	_, resp, err := c.api.EndGroup(ctx).HandleRequest(*body).Execute()
	return resp, err
}

func (c *Sprite) Info(ctx context.Context) (*osbtk.SpriteInfo, *http.Response, error) {
	body := osbtk.NewHandleRequest(c.layer, c.sprite)
	return c.api.Info(ctx).HandleRequest(*body).Execute()
}

func (c *Sprite) StartLoopGroup(ctx context.Context, options clean.StartLoopGroupOptions) (*http.Response, error) {
	body := osbtk.NewStartLoopGroupRequest(options.Count, c.layer, c.sprite, options.StartTime)
	_, resp, err := c.api.StartLoopGroup(ctx).StartLoopGroupRequest(*body).Execute()
	return resp, err
}

func (c *Sprite) StartTriggerGroup(ctx context.Context, options clean.StartTriggerGroupOptions) (*http.Response, error) {
	body := osbtk.NewStartTriggerGroupRequest(options.EndTime, options.Group, c.layer, options.Name, c.sprite, options.StartTime)
	_, resp, err := c.api.StartTriggerGroup(ctx).StartTriggerGroupRequest(*body).Execute()
	return resp, err
}

func (c *Sprite) StateAt(ctx context.Context, options clean.SpriteTimeOptions) (*osbtk.SpriteState, *http.Response, error) {
	body := osbtk.NewSpriteTimeRequest(c.layer, c.sprite, options.Time)
	return c.api.StateAt(ctx).SpriteTimeRequest(*body).Execute()
}
