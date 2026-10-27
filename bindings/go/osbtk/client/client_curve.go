package client

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
)

// Curve is the clean high-level client.
type Curve struct {
	api *osbtk.CurveAPIService
}

func NewCurve(api *osbtk.CurveAPIService) *Curve {
	return &Curve{api: api}
}

func IsCurve(value any) bool {
	_, ok := value.(*Curve)
	return ok
}

func (c *Curve) Arc(ctx context.Context, body osbtk.ArcRequest) (*osbtk.PointListResult, *http.Response, error) {
	return c.api.Arc(ctx).ArcRequest(body).Execute()
}

func (c *Curve) BoundingBox(ctx context.Context, body osbtk.CurveRequest) (*osbtk.Box2, *http.Response, error) {
	return c.api.BoundingBox(ctx).CurveRequest(body).Execute()
}

func (c *Curve) Curvature(ctx context.Context, body osbtk.CurvePointRequest) (*osbtk.ScalarResult, *http.Response, error) {
	return c.api.Curvature(ctx).CurvePointRequest(body).Execute()
}

func (c *Curve) DistanceTo(ctx context.Context, body osbtk.CurveDistanceRequest) (*osbtk.ScalarResult, *http.Response, error) {
	return c.api.DistanceTo(ctx).CurveDistanceRequest(body).Execute()
}

func (c *Curve) Ellipse(ctx context.Context, body osbtk.EllipseRequest) (*osbtk.PointListResult, *http.Response, error) {
	return c.api.Ellipse(ctx).EllipseRequest(body).Execute()
}

func (c *Curve) Length(ctx context.Context, body osbtk.CurveRequest) (*osbtk.ScalarResult, *http.Response, error) {
	return c.api.Length(ctx).CurveRequest(body).Execute()
}

func (c *Curve) LineIntersect(ctx context.Context, body osbtk.LineIntersectRequest) (*osbtk.OptionalPointResult, *http.Response, error) {
	return c.api.LineIntersect(ctx).LineIntersectRequest(body).Execute()
}

func (c *Curve) Normal(ctx context.Context, body osbtk.CurvePointRequest) (*osbtk.Vector2, *http.Response, error) {
	return c.api.Normal(ctx).CurvePointRequest(body).Execute()
}

func (c *Curve) Offset(ctx context.Context, body osbtk.CurveOffsetRequest) (*osbtk.PointListResult, *http.Response, error) {
	return c.api.Offset(ctx).CurveOffsetRequest(body).Execute()
}

func (c *Curve) Point(ctx context.Context, body osbtk.CurvePointRequest) (*osbtk.Vector2, *http.Response, error) {
	return c.api.Point(ctx).CurvePointRequest(body).Execute()
}

func (c *Curve) Sample(ctx context.Context, body osbtk.CurveSampleRequest) (*osbtk.PointListResult, *http.Response, error) {
	return c.api.Sample(ctx).CurveSampleRequest(body).Execute()
}

func (c *Curve) Simplify(ctx context.Context, body osbtk.SimplifyRequest) (*osbtk.PointListResult, *http.Response, error) {
	return c.api.Simplify(ctx).SimplifyRequest(body).Execute()
}

func (c *Curve) Tangent(ctx context.Context, body osbtk.CurvePointRequest) (*osbtk.Vector2, *http.Response, error) {
	return c.api.Tangent(ctx).CurvePointRequest(body).Execute()
}
