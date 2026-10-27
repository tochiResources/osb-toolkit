package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
)

// Media is the clean high-level client.
type Media struct {
	api *osbtk.MediaAPIService
}

func NewMedia(api *osbtk.MediaAPIService) *Media {
	return &Media{api: api}
}

func IsMedia(value any) bool {
	_, ok := value.(*Media)
	return ok
}

func (c *Media) EncodeVideo(ctx context.Context, body osbtk.EncodeVideoRequest) (*osbtk.MediaInfo, *http.Response, error) {
	return c.api.EncodeVideo(ctx).EncodeVideoRequest(body).Execute()
}

func (c *Media) ExtractAudio(ctx context.Context, body osbtk.ExtractAudioRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.ExtractAudio(ctx).ExtractAudioRequest(body).Execute()
}

func (c *Media) ExtractFrames(ctx context.Context, body osbtk.ExtractFramesRequest) (*osbtk.FrameResult, *http.Response, error) {
	return c.api.ExtractFrames(ctx).ExtractFramesRequest(body).Execute()
}

func (c *Media) Info(ctx context.Context, body osbtk.PathRequest) (*osbtk.MediaInfo, *http.Response, error) {
	return c.api.Info(ctx).PathRequest(body).Execute()
}
