package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
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

func (c *Audio) Fft(ctx context.Context, body osbtk.FftRequest) (*osbtk.FftResult, *http.Response, error) {
	return c.api.Fft(ctx).FftRequest(body).Execute()
}

func (c *Audio) GetFft(ctx context.Context, body osbtk.GetFftRequest) (*osbtk.FftResult, *http.Response, error) {
	return c.api.GetFft(ctx).GetFftRequest(body).Execute()
}
