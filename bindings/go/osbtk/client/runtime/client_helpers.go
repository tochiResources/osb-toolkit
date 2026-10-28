package runtime

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Helpers is the clean high-level client.
type Helpers struct {
	api *osbtk.HelpersAPIService
}

func NewHelpers(api *osbtk.HelpersAPIService) *Helpers {
	return &Helpers{api: api}
}

func IsHelpers(value any) bool {
	_, ok := value.(*Helpers)
	return ok
}

// Call: A motion-graphics helper call: a helper `name` plus its input.
//
// ctx: The request context.
// options: A motion-graphics helper call: a helper `name` plus its input.
//
// returns: The shared motion-graphics helper's result
func (c *Helpers) Call(ctx context.Context, options clean.HelperOptions) (any, *http.Response, error) {
	body := osbtk.NewHelperRequest(options.Name)
	if options.Input != nil {
		body.SetInput(options.Input)
	}
	value, resp, err := c.api.Call(ctx).HelperRequest(*body).Execute()
	if err != nil {
		return nil, resp, err
	}
	return value.Value, resp, nil
}

// Commands: A motion-graphics helper call: a helper `name` plus its input.
//
// ctx: The request context.
// options: A motion-graphics helper call: a helper `name` plus its input.
//
// returns: The helper's result (a command list or scalar)
func (c *Helpers) Commands(ctx context.Context, options clean.HelperOptions) (any, *http.Response, error) {
	body := osbtk.NewHelperRequest(options.Name)
	if options.Input != nil {
		body.SetInput(options.Input)
	}
	value, resp, err := c.api.Commands(ctx).HelperRequest(*body).Execute()
	if err != nil {
		return nil, resp, err
	}
	return value.Value, resp, nil
}

// Keyframes: A motion-graphics helper call: a helper `name` plus its input.
//
// ctx: The request context.
// options: A motion-graphics helper call: a helper `name` plus its input.
//
// returns: The helper's result (a keyframe channel or vectors)
func (c *Helpers) Keyframes(ctx context.Context, options clean.HelperOptions) (any, *http.Response, error) {
	body := osbtk.NewHelperRequest(options.Name)
	if options.Input != nil {
		body.SetInput(options.Input)
	}
	value, resp, err := c.api.Keyframes(ctx).HelperRequest(*body).Execute()
	if err != nil {
		return nil, resp, err
	}
	return value.Value, resp, nil
}
