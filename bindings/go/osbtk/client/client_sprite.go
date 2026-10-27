package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
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

func (c *Sprite) Command(ctx context.Context, options CommandOptions) (*osbtk.OkResult, *http.Response, error) {
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
	return c.api.Command(ctx).CommandRequest(*body).Execute()
}

func (c *Sprite) EndGroup(ctx context.Context) (*osbtk.OkResult, *http.Response, error) {
	body := osbtk.NewHandleRequest(c.layer, c.sprite)
	return c.api.EndGroup(ctx).HandleRequest(*body).Execute()
}

func (c *Sprite) Info(ctx context.Context) (*osbtk.SpriteInfo, *http.Response, error) {
	body := osbtk.NewHandleRequest(c.layer, c.sprite)
	return c.api.Info(ctx).HandleRequest(*body).Execute()
}

func (c *Sprite) StartLoopGroup(ctx context.Context, options StartLoopGroupOptions) (*osbtk.OkResult, *http.Response, error) {
	body := osbtk.NewStartLoopGroupRequest(options.Count, c.layer, c.sprite, options.StartTime)
	return c.api.StartLoopGroup(ctx).StartLoopGroupRequest(*body).Execute()
}

func (c *Sprite) StartTriggerGroup(ctx context.Context, options StartTriggerGroupOptions) (*osbtk.OkResult, *http.Response, error) {
	body := osbtk.NewStartTriggerGroupRequest(options.EndTime, options.Group, c.layer, options.Name, c.sprite, options.StartTime)
	return c.api.StartTriggerGroup(ctx).StartTriggerGroupRequest(*body).Execute()
}

func (c *Sprite) StateAt(ctx context.Context, options SpriteTimeOptions) (*osbtk.SpriteState, *http.Response, error) {
	body := osbtk.NewSpriteTimeRequest(c.layer, c.sprite, options.Time)
	return c.api.StateAt(ctx).SpriteTimeRequest(*body).Execute()
}
