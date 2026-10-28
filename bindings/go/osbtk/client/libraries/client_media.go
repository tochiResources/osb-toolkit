package libraries

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
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

// EncodeVideo: An image-sequence encoding request (frames -> video).
//
// ctx: The request context.
// options: An image-sequence encoding request (frames -> video).
//
// returns: The encoded video's metadata
func (c *Media) EncodeVideo(ctx context.Context, options clean.EncodeVideoOptions) (*osbtk.MediaInfo, *http.Response, error) {
	body := osbtk.NewEncodeVideoRequest(options.Fps, options.InputPattern, options.Output)
	if options.Codec != nil {
		body.SetCodec(*options.Codec)
	}
	if options.Crf != nil {
		body.SetCrf(*options.Crf)
	}
	return c.api.EncodeVideo(ctx).EncodeVideoRequest(*body).Execute()
}

// ExtractAudio: An audio-extraction request (media -> audio file).
//
// ctx: The request context.
// options: An audio-extraction request (media -> audio file).
//
// returns: Acknowledged
func (c *Media) ExtractAudio(ctx context.Context, options clean.ExtractAudioOptions) (*http.Response, error) {
	body := osbtk.NewExtractAudioRequest(options.Input, options.Output)
	_, resp, err := c.api.ExtractAudio(ctx).ExtractAudioRequest(*body).Execute()
	return resp, err
}

// ExtractFrames: A frame-extraction request (video -> image sequence).
//
// ctx: The request context.
// options: A frame-extraction request (video -> image sequence).
//
// returns: The extracted frame count
func (c *Media) ExtractFrames(ctx context.Context, options clean.ExtractFramesOptions) (*osbtk.FrameResult, *http.Response, error) {
	body := osbtk.NewExtractFramesRequest(options.Input, options.OutputDir)
	if options.Count != nil {
		body.SetCount(*options.Count)
	}
	if options.Fps != nil {
		body.SetFps(*options.Fps)
	}
	if options.Height != nil {
		body.SetHeight(*options.Height)
	}
	if options.Start != nil {
		body.SetStart(*options.Start)
	}
	if options.Width != nil {
		body.SetWidth(*options.Width)
	}
	return c.api.ExtractFrames(ctx).ExtractFramesRequest(*body).Execute()
}

// Info: A request carrying a single file path.
//
// ctx: The request context.
// options: A request carrying a single file path.
//
// returns: The media metadata
func (c *Media) Info(ctx context.Context, options clean.PathOptions) (*osbtk.MediaInfo, *http.Response, error) {
	body := osbtk.NewPathRequest(options.Path)
	return c.api.Info(ctx).PathRequest(*body).Execute()
}
