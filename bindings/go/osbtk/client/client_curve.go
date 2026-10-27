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

func (c *Curve) Arc(ctx context.Context, options ArcOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewArcRequest(options.Center, options.Radius, options.StartAngle, options.SweepAngle)
	if options.Count != nil {
		body.SetCount(*options.Count)
	}
	return c.api.Arc(ctx).ArcRequest(*body).Execute()
}

func (c *Curve) BoundingBox(ctx context.Context, options CurveOptions) (*osbtk.Box2, *http.Response, error) {
	body := osbtk.NewCurveRequest(options.Curve)
	return c.api.BoundingBox(ctx).CurveRequest(*body).Execute()
}

func (c *Curve) Curvature(ctx context.Context, options CurvePointOptions) (*osbtk.ScalarResult, *http.Response, error) {
	body := osbtk.NewCurvePointRequest(options.Curve, options.T)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	return c.api.Curvature(ctx).CurvePointRequest(*body).Execute()
}

func (c *Curve) DistanceTo(ctx context.Context, options CurveDistanceOptions) (*osbtk.ScalarResult, *http.Response, error) {
	body := osbtk.NewCurveDistanceRequest(options.Curve, options.Point)
	return c.api.DistanceTo(ctx).CurveDistanceRequest(*body).Execute()
}

func (c *Curve) Ellipse(ctx context.Context, options EllipseOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewEllipseRequest(options.Center, options.RadiusX, options.RadiusY)
	if options.Count != nil {
		body.SetCount(*options.Count)
	}
	return c.api.Ellipse(ctx).EllipseRequest(*body).Execute()
}

func (c *Curve) Length(ctx context.Context, options CurveOptions) (*osbtk.ScalarResult, *http.Response, error) {
	body := osbtk.NewCurveRequest(options.Curve)
	return c.api.Length(ctx).CurveRequest(*body).Execute()
}

func (c *Curve) LineIntersect(ctx context.Context, options LineIntersectOptions) (*osbtk.OptionalPointResult, *http.Response, error) {
	body := osbtk.NewLineIntersectRequest(options.A1, options.A2, options.B1, options.B2)
	return c.api.LineIntersect(ctx).LineIntersectRequest(*body).Execute()
}

func (c *Curve) Normal(ctx context.Context, options CurvePointOptions) (*osbtk.Vector2, *http.Response, error) {
	body := osbtk.NewCurvePointRequest(options.Curve, options.T)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	return c.api.Normal(ctx).CurvePointRequest(*body).Execute()
}

func (c *Curve) Offset(ctx context.Context, options CurveOffsetOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewCurveOffsetRequest(options.Curve, options.Distance)
	return c.api.Offset(ctx).CurveOffsetRequest(*body).Execute()
}

func (c *Curve) Point(ctx context.Context, options CurvePointOptions) (*osbtk.Vector2, *http.Response, error) {
	body := osbtk.NewCurvePointRequest(options.Curve, options.T)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	return c.api.Point(ctx).CurvePointRequest(*body).Execute()
}

func (c *Curve) Sample(ctx context.Context, options CurveSampleOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewCurveSampleRequest(options.Count, options.Curve)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	return c.api.Sample(ctx).CurveSampleRequest(*body).Execute()
}

func (c *Curve) Simplify(ctx context.Context, options SimplifyOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewSimplifyRequest(options.Points, options.Tolerance)
	return c.api.Simplify(ctx).SimplifyRequest(*body).Execute()
}

func (c *Curve) Tangent(ctx context.Context, options CurvePointOptions) (*osbtk.Vector2, *http.Response, error) {
	body := osbtk.NewCurvePointRequest(options.Curve, options.T)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	return c.api.Tangent(ctx).CurvePointRequest(*body).Execute()
}
