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

// AudioDuration: Calls the Audio.audioDuration bridge operation.
//
// ctx: The request context.
//
// returns: The loaded audio duration
func (c *Audio) AudioDuration(ctx context.Context) (float64, *http.Response, error) {
	value, resp, err := c.api.AudioDuration(ctx).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.AudioDuration, resp, nil
}

// Fft: An FFT request against a `.wav` file.
//
// ctx: The request context.
// options: An FFT request against a `.wav` file.
//
// returns: The FFT magnitudes
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

// GetFft: An FFT request against the runtime's loaded audio.
//
// ctx: The request context.
// options: An FFT request against the runtime's loaded audio.
//
// returns: The FFT magnitudes
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
