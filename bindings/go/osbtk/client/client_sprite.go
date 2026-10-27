package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
)

// Sprite is the clean high-level client.
type Sprite struct {
	api *osbtk.SpriteAPIService
}

func NewSprite(api *osbtk.SpriteAPIService) *Sprite {
	return &Sprite{api: api}
}

func IsSprite(value any) bool {
	_, ok := value.(*Sprite)
	return ok
}

func (c *Sprite) Command(ctx context.Context, body osbtk.CommandRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.Command(ctx).CommandRequest(body).Execute()
}

func (c *Sprite) EndGroup(ctx context.Context, body osbtk.HandleRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.EndGroup(ctx).HandleRequest(body).Execute()
}

func (c *Sprite) Info(ctx context.Context, body osbtk.HandleRequest) (*osbtk.SpriteInfo, *http.Response, error) {
	return c.api.Info(ctx).HandleRequest(body).Execute()
}

func (c *Sprite) StartLoopGroup(ctx context.Context, body osbtk.StartLoopGroupRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.StartLoopGroup(ctx).StartLoopGroupRequest(body).Execute()
}

func (c *Sprite) StartTriggerGroup(ctx context.Context, body osbtk.StartTriggerGroupRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.StartTriggerGroup(ctx).StartTriggerGroupRequest(body).Execute()
}

func (c *Sprite) StateAt(ctx context.Context, body osbtk.SpriteTimeRequest) (*osbtk.SpriteState, *http.Response, error) {
	return c.api.StateAt(ctx).SpriteTimeRequest(body).Execute()
}
