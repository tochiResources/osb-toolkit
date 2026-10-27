package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
)

// Layer is the clean high-level client.
type Layer struct {
	api *osbtk.LayerAPIService
}

func NewLayer(api *osbtk.LayerAPIService) *Layer {
	return &Layer{api: api}
}

func IsLayer(value any) bool {
	_, ok := value.(*Layer)
	return ok
}

func (c *Layer) AddIndex(ctx context.Context, body osbtk.SegmentAddIndexRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.AddIndex(ctx).SegmentAddIndexRequest(body).Execute()
}

func (c *Layer) ChangePosition(ctx context.Context, body osbtk.SegmentChangePositionRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.ChangePosition(ctx).SegmentChangePositionRequest(body).Execute()
}

func (c *Layer) Clean(ctx context.Context, body osbtk.SegmentRequest) (*osbtk.SegmentCleanResult, *http.Response, error) {
	return c.api.Clean(ctx).SegmentRequest(body).Execute()
}

func (c *Layer) CreateAnimation(ctx context.Context, body osbtk.CreateAnimationRequest) (*osbtk.Handle, *http.Response, error) {
	return c.api.CreateAnimation(ctx).CreateAnimationRequest(body).Execute()
}

func (c *Layer) CreateSample(ctx context.Context, body osbtk.CreateSampleRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.CreateSample(ctx).CreateSampleRequest(body).Execute()
}

func (c *Layer) CreateSegment(ctx context.Context, body osbtk.SegmentRequest) (*osbtk.LayerHandle, *http.Response, error) {
	return c.api.CreateSegment(ctx).SegmentRequest(body).Execute()
}

func (c *Layer) CreateSprite(ctx context.Context, body osbtk.CreateSpriteRequest) (*osbtk.Handle, *http.Response, error) {
	return c.api.CreateSprite(ctx).CreateSpriteRequest(body).Execute()
}

func (c *Layer) Discard(ctx context.Context, body osbtk.SegmentRequest) (*osbtk.SegmentDiscardResult, *http.Response, error) {
	return c.api.Discard(ctx).SegmentRequest(body).Execute()
}

func (c *Layer) DiscardSprite(ctx context.Context, body osbtk.HandleRequest) (*osbtk.BoolResult, *http.Response, error) {
	return c.api.DiscardSprite(ctx).HandleRequest(body).Execute()
}

func (c *Layer) Indexes(ctx context.Context, body osbtk.SegmentRequest) (*osbtk.SegmentIndexesResult, *http.Response, error) {
	return c.api.Indexes(ctx).SegmentRequest(body).Execute()
}

func (c *Layer) MoveToBottom(ctx context.Context, body osbtk.SegmentRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.MoveToBottom(ctx).SegmentRequest(body).Execute()
}

func (c *Layer) MoveToTop(ctx context.Context, body osbtk.SegmentRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.MoveToTop(ctx).SegmentRequest(body).Execute()
}

func (c *Layer) ReverseDepth(ctx context.Context, body osbtk.SegmentReverseDepthRequest) (*osbtk.OkResult, *http.Response, error) {
	return c.api.ReverseDepth(ctx).SegmentReverseDepthRequest(body).Execute()
}
