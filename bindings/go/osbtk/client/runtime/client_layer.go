package runtime

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Layer is the handle-bound clean layer client.
type Layer struct {
	api     *osbtk.LayerAPIService
	sprites *osbtk.SpriteAPIService
	handle  int32
}

func NewLayer(api *osbtk.LayerAPIService, sprites *osbtk.SpriteAPIService, handle int32) *Layer {
	return &Layer{api: api, sprites: sprites, handle: handle}
}

func IsLayer(value any) bool {
	_, ok := value.(*Layer)
	return ok
}

func (c *Layer) AddIndex(ctx context.Context, options clean.SegmentAddIndexOptions) (*http.Response, error) {
	body := osbtk.NewSegmentAddIndexRequest(options.Index, c.handle)
	_, resp, err := c.api.AddIndex(ctx).SegmentAddIndexRequest(*body).Execute()
	return resp, err
}

func (c *Layer) ChangePosition(ctx context.Context, options clean.SegmentChangePositionOptions) (*http.Response, error) {
	body := osbtk.NewSegmentChangePositionRequest(c.handle, options.OsbLayer)
	_, resp, err := c.api.ChangePosition(ctx).SegmentChangePositionRequest(*body).Execute()
	return resp, err
}

func (c *Layer) Clean(ctx context.Context) (int32, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	value, resp, err := c.api.Clean(ctx).SegmentRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Removed, resp, nil
}

func (c *Layer) CreateAnimation(ctx context.Context, options clean.CreateAnimationOptions) (*Sprite, *http.Response, error) {
	body := osbtk.NewCreateAnimationRequest(options.FrameDelay, options.Frames, options.LoopType, options.Origin, options.Path, options.Position)
	if options.Layer != nil {
		body.SetLayer(*options.Layer)
	}
	body.SetLayerHandle(c.handle)
	handle, resp, err := c.api.CreateAnimation(ctx).CreateAnimationRequest(*body).Execute()
	if err != nil {
		return nil, resp, err
	}
	return NewSprite(c.sprites, handle.Layer, handle.Sprite), resp, nil
}

func (c *Layer) CreateSample(ctx context.Context, options clean.CreateSampleOptions) (*http.Response, error) {
	body := osbtk.NewCreateSampleRequest(options.Path, options.Time)
	if options.Layer != nil {
		body.SetLayer(*options.Layer)
	}
	body.SetLayerHandle(c.handle)
	if options.Volume != nil {
		body.SetVolume(*options.Volume)
	}
	_, resp, err := c.api.CreateSample(ctx).CreateSampleRequest(*body).Execute()
	return resp, err
}

func (c *Layer) CreateSegment(ctx context.Context) (*Layer, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	handle, resp, err := c.api.CreateSegment(ctx).SegmentRequest(*body).Execute()
	if err != nil {
		return nil, resp, err
	}
	return NewLayer(c.api, c.sprites, handle.Layer), resp, nil
}

func (c *Layer) CreateSprite(ctx context.Context, options clean.CreateSpriteOptions) (*Sprite, *http.Response, error) {
	body := osbtk.NewCreateSpriteRequest(options.Origin, options.Path)
	if options.Layer != nil {
		body.SetLayer(*options.Layer)
	}
	body.SetLayerHandle(c.handle)
	if options.Position != nil {
		body.SetPosition(*options.Position)
	}
	handle, resp, err := c.api.CreateSprite(ctx).CreateSpriteRequest(*body).Execute()
	if err != nil {
		return nil, resp, err
	}
	return NewSprite(c.sprites, handle.Layer, handle.Sprite), resp, nil
}

func (c *Layer) Discard(ctx context.Context) (int32, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	value, resp, err := c.api.Discard(ctx).SegmentRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.LayerCount, resp, nil
}

func (c *Layer) DiscardSprite(ctx context.Context, sprite int32) (bool, *http.Response, error) {
	body := osbtk.NewHandleRequest(c.handle, sprite)
	value, resp, err := c.api.DiscardSprite(ctx).HandleRequest(*body).Execute()
	if err != nil {
		return false, resp, err
	}
	return value.Value, resp, nil
}

func (c *Layer) Indexes(ctx context.Context) (*osbtk.SegmentIndexesResult, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	return c.api.Indexes(ctx).SegmentRequest(*body).Execute()
}

func (c *Layer) MoveToBottom(ctx context.Context) (*http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	_, resp, err := c.api.MoveToBottom(ctx).SegmentRequest(*body).Execute()
	return resp, err
}

func (c *Layer) MoveToTop(ctx context.Context) (*http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	_, resp, err := c.api.MoveToTop(ctx).SegmentRequest(*body).Execute()
	return resp, err
}

func (c *Layer) ReverseDepth(ctx context.Context, options clean.SegmentReverseDepthOptions) (*http.Response, error) {
	body := osbtk.NewSegmentReverseDepthRequest(c.handle, options.Value)
	_, resp, err := c.api.ReverseDepth(ctx).SegmentReverseDepthRequest(*body).Execute()
	return resp, err
}
