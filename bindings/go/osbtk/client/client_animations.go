package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
)

// Animations is the clean high-level client.
type Animations struct {
	api *osbtk.AnimationsAPIService
}

func NewAnimations(api *osbtk.AnimationsAPIService) *Animations {
	return &Animations{api: api}
}

func IsAnimations(value any) bool {
	_, ok := value.(*Animations)
	return ok
}

func (c *Animations) ApplyEasing(ctx context.Context, options ApplyEasingOptions) (*osbtk.EasingResult, *http.Response, error) {
	body := osbtk.NewApplyEasingRequest(options.Easing, options.T)
	return c.api.ApplyEasing(ctx).ApplyEasingRequest(*body).Execute()
}

func (c *Animations) EasingValue(ctx context.Context, options EasingValueOptions) (*osbtk.EasingResult, *http.Response, error) {
	body := osbtk.NewEasingValueRequest(options.T)
	if options.CubicBezier != nil {
		body.SetCubicBezier(options.CubicBezier)
	}
	if options.Easing != nil {
		body.SetEasing(*options.Easing)
	}
	return c.api.EasingValue(ctx).EasingValueRequest(*body).Execute()
}
