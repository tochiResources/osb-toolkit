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

func (c *Layer) AddIndex(ctx context.Context, options clean.SegmentAddIndexOptions) (*osbtk.OkResult, *http.Response, error) {
	body := osbtk.NewSegmentAddIndexRequest(options.Index, c.handle)
	return c.api.AddIndex(ctx).SegmentAddIndexRequest(*body).Execute()
}

func (c *Layer) ChangePosition(ctx context.Context, options clean.SegmentChangePositionOptions) (*osbtk.OkResult, *http.Response, error) {
	body := osbtk.NewSegmentChangePositionRequest(c.handle, options.OsbLayer)
	return c.api.ChangePosition(ctx).SegmentChangePositionRequest(*body).Execute()
}

func (c *Layer) Clean(ctx context.Context) (*osbtk.SegmentCleanResult, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	return c.api.Clean(ctx).SegmentRequest(*body).Execute()
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

func (c *Layer) CreateSample(ctx context.Context, options clean.CreateSampleOptions) (*osbtk.OkResult, *http.Response, error) {
	body := osbtk.NewCreateSampleRequest(options.Path, options.Time)
	if options.Layer != nil {
		body.SetLayer(*options.Layer)
	}
	body.SetLayerHandle(c.handle)
	if options.Volume != nil {
		body.SetVolume(*options.Volume)
	}
	return c.api.CreateSample(ctx).CreateSampleRequest(*body).Execute()
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

func (c *Layer) Discard(ctx context.Context) (*osbtk.SegmentDiscardResult, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	return c.api.Discard(ctx).SegmentRequest(*body).Execute()
}

func (c *Layer) DiscardSprite(ctx context.Context, sprite int32) (*osbtk.BoolResult, *http.Response, error) {
	body := osbtk.NewHandleRequest(c.handle, sprite)
	return c.api.DiscardSprite(ctx).HandleRequest(*body).Execute()
}

func (c *Layer) Indexes(ctx context.Context) (*osbtk.SegmentIndexesResult, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	return c.api.Indexes(ctx).SegmentRequest(*body).Execute()
}

func (c *Layer) MoveToBottom(ctx context.Context) (*osbtk.OkResult, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	return c.api.MoveToBottom(ctx).SegmentRequest(*body).Execute()
}

func (c *Layer) MoveToTop(ctx context.Context) (*osbtk.OkResult, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	return c.api.MoveToTop(ctx).SegmentRequest(*body).Execute()
}

func (c *Layer) ReverseDepth(ctx context.Context, options clean.SegmentReverseDepthOptions) (*osbtk.OkResult, *http.Response, error) {
	body := osbtk.NewSegmentReverseDepthRequest(c.handle, options.Value)
	return c.api.ReverseDepth(ctx).SegmentReverseDepthRequest(*body).Execute()
}
