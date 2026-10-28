package core

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
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

// ApplyEasing: An easing evaluation request.
//
// ctx: The request context.
// options: An easing evaluation request.
//
// returns: The eased value
func (c *Animations) ApplyEasing(ctx context.Context, options clean.ApplyEasingOptions) (float64, *http.Response, error) {
	body := osbtk.NewApplyEasingRequest(options.Easing, options.T)
	value, resp, err := c.api.ApplyEasing(ctx).ApplyEasingRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Value, resp, nil
}

// EasingValue: A named-or-custom easing evaluation request.
//
// ctx: The request context.
// options: A named-or-custom easing evaluation request.
//
// returns: The eased value (named easing or cubic-bezier)
func (c *Animations) EasingValue(ctx context.Context, options clean.EasingValueOptions) (float64, *http.Response, error) {
	body := osbtk.NewEasingValueRequest(options.T)
	if options.CubicBezier != nil {
		body.SetCubicBezier(options.CubicBezier)
	}
	if options.Easing != nil {
		body.SetEasing(*options.Easing)
	}
	value, resp, err := c.api.EasingValue(ctx).EasingValueRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Value, resp, nil
}
