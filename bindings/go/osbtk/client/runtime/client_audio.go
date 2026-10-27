package runtime

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Audio is the clean high-level client.
type Audio struct {
	api *osbtk.AudioAPIService
}

func NewAudio(api *osbtk.AudioAPIService) *Audio {
	return &Audio{api: api}
}

func IsAudio(value any) bool {
	_, ok := value.(*Audio)
	return ok
}

func (c *Audio) AudioDuration(ctx context.Context) (*osbtk.AudioDurationResult, *http.Response, error) {
	return c.api.AudioDuration(ctx).Execute()
}

func (c *Audio) Fft(ctx context.Context, options clean.FftOptions) (*osbtk.FftResult, *http.Response, error) {
	body := osbtk.NewFftRequest(options.Path)
	if options.Magnitudes != nil {
		body.SetMagnitudes(*options.Magnitudes)
	}
	if options.Time != nil {
		body.SetTime(*options.Time)
	}
	return c.api.Fft(ctx).FftRequest(*body).Execute()
}

func (c *Audio) GetFft(ctx context.Context, options clean.GetFftOptions) (*osbtk.FftResult, *http.Response, error) {
	body := osbtk.NewGetFftRequest(options.Magnitudes, options.Time)
	if options.Cutoff != nil {
		body.SetCutoff(*options.Cutoff)
	}
	if options.Easing != nil {
		body.SetEasing(*options.Easing)
	}
	return c.api.GetFft(ctx).GetFftRequest(*body).Execute()
}
