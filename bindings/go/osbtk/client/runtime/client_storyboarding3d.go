package runtime

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Storyboarding3d is the clean high-level client.
type Storyboarding3d struct {
	api *osbtk.Storyboarding3dAPIService
}

func NewStoryboarding3d(api *osbtk.Storyboarding3dAPIService) *Storyboarding3d {
	return &Storyboarding3d{api: api}
}

func IsStoryboarding3d(value any) bool {
	_, ok := value.(*Storyboarding3d)
	return ok
}

func (c *Storyboarding3d) Bake3d(ctx context.Context, options clean.Bake3dOptions) (*osbtk.BakedStoryboard, *http.Response, error) {
	body := osbtk.NewBake3dRequest(options.Input)
	return c.api.Bake3d(ctx).Bake3dRequest(*body).Execute()
}
