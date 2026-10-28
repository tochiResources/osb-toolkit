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

// AddIndex: Record a depth index on a segment.
//
// ctx: The request context.
// options: Record a depth index on a segment.
//
// returns: Acknowledged
func (c *Layer) AddIndex(ctx context.Context, options clean.SegmentAddIndexOptions) (*http.Response, error) {
	body := osbtk.NewSegmentAddIndexRequest(options.Index, c.handle)
	_, resp, err := c.api.AddIndex(ctx).SegmentAddIndexRequest(*body).Execute()
	return resp, err
}

// ChangePosition: Move a segment to another osu! layer (`ChangePosition`).
//
// ctx: The request context.
// options: Move a segment to another osu! layer (`ChangePosition`).
//
// returns: Acknowledged
func (c *Layer) ChangePosition(ctx context.Context, options clean.SegmentChangePositionOptions) (*http.Response, error) {
	body := osbtk.NewSegmentChangePositionRequest(c.handle, options.OsbLayer)
	_, resp, err := c.api.ChangePosition(ctx).SegmentChangePositionRequest(*body).Execute()
	return resp, err
}

// Clean: A segment handle request.
//
// ctx: The request context.
//
// returns: The number of sprites removed
func (c *Layer) Clean(ctx context.Context) (int32, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	value, resp, err := c.api.Clean(ctx).SegmentRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Removed, resp, nil
}

// CreateAnimation: Create an animation sprite.
//
// ctx: The request context.
// options: Create an animation sprite.
//
// returns: The new animation handle
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

// CreateSample: Create a sound sample on a layer.
//
// ctx: The request context.
// options: Create a sound sample on a layer.
//
// returns: Acknowledged
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

// CreateSegment: A segment handle request.
//
// ctx: The request context.
//
// returns: The new segment handle
func (c *Layer) CreateSegment(ctx context.Context) (*Layer, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	handle, resp, err := c.api.CreateSegment(ctx).SegmentRequest(*body).Execute()
	if err != nil {
		return nil, resp, err
	}
	return NewLayer(c.api, c.sprites, handle.Layer), resp, nil
}

// CreateSprite: Create a sprite.
//
// ctx: The request context.
// options: Create a sprite.
//
// returns: The new sprite handle
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

// Discard: A segment handle request.
//
// ctx: The request context.
//
// returns: The remaining top-level layer count
func (c *Layer) Discard(ctx context.Context) (int32, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	value, resp, err := c.api.Discard(ctx).SegmentRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.LayerCount, resp, nil
}

// DiscardSprite: A sprite handle pair.
//
// ctx: The request context.
// sprite: The sprite handle.
//
// returns: Whether the sprite was discarded
func (c *Layer) DiscardSprite(ctx context.Context, sprite int32) (bool, *http.Response, error) {
	body := osbtk.NewHandleRequest(c.handle, sprite)
	value, resp, err := c.api.DiscardSprite(ctx).HandleRequest(*body).Execute()
	if err != nil {
		return false, resp, err
	}
	return value.Value, resp, nil
}

// Indexes: A segment handle request.
//
// ctx: The request context.
//
// returns: The segment's indexes and bounds
func (c *Layer) Indexes(ctx context.Context) (*osbtk.SegmentIndexesResult, *http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	return c.api.Indexes(ctx).SegmentRequest(*body).Execute()
}

// MoveToBottom: A segment handle request.
//
// ctx: The request context.
//
// returns: Acknowledged
func (c *Layer) MoveToBottom(ctx context.Context) (*http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	_, resp, err := c.api.MoveToBottom(ctx).SegmentRequest(*body).Execute()
	return resp, err
}

// MoveToTop: A segment handle request.
//
// ctx: The request context.
//
// returns: Acknowledged
func (c *Layer) MoveToTop(ctx context.Context) (*http.Response, error) {
	body := osbtk.NewSegmentRequest(c.handle)
	_, resp, err := c.api.MoveToTop(ctx).SegmentRequest(*body).Execute()
	return resp, err
}

// ReverseDepth: Set a segment's reverse-depth flag.
//
// ctx: The request context.
// options: Set a segment's reverse-depth flag.
//
// returns: Acknowledged
func (c *Layer) ReverseDepth(ctx context.Context, options clean.SegmentReverseDepthOptions) (*http.Response, error) {
	body := osbtk.NewSegmentReverseDepthRequest(c.handle, options.Value)
	_, resp, err := c.api.ReverseDepth(ctx).SegmentReverseDepthRequest(*body).Execute()
	return resp, err
}
