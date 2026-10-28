package core

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
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

// Arc: A circular-arc sampling request.
//
// ctx: The request context.
// options: A circular-arc sampling request.
//
// returns: Points along a circular arc
func (c *Curve) Arc(ctx context.Context, options clean.ArcOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewArcRequest(options.Center, options.Radius, options.StartAngle, options.SweepAngle)
	if options.Count != nil {
		body.SetCount(*options.Count)
	}
	return c.api.Arc(ctx).ArcRequest(*body).Execute()
}

// BoundingBox: A curve query.
//
// ctx: The request context.
// options: A curve query.
//
// returns: The curve's axis-aligned bounding box
func (c *Curve) BoundingBox(ctx context.Context, options clean.CurveOptions) (*osbtk.Box2, *http.Response, error) {
	body := osbtk.NewCurveRequest(options.Curve)
	return c.api.BoundingBox(ctx).CurveRequest(*body).Execute()
}

// Curvature: A curve query at a parametric or arc-length position.
//
// ctx: The request context.
// options: A curve query at a parametric or arc-length position.
//
// returns: The curvature at `t`
func (c *Curve) Curvature(ctx context.Context, options clean.CurvePointOptions) (float64, *http.Response, error) {
	body := osbtk.NewCurvePointRequest(options.Curve, options.T)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	value, resp, err := c.api.Curvature(ctx).CurvePointRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Value, resp, nil
}

// DistanceTo: A curve hit-test request.
//
// ctx: The request context.
// options: A curve hit-test request.
//
// returns: The distance from a point to the curve
func (c *Curve) DistanceTo(ctx context.Context, options clean.CurveDistanceOptions) (float64, *http.Response, error) {
	body := osbtk.NewCurveDistanceRequest(options.Curve, options.Point)
	value, resp, err := c.api.DistanceTo(ctx).CurveDistanceRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Value, resp, nil
}

// Ellipse: An ellipse sampling request.
//
// ctx: The request context.
// options: An ellipse sampling request.
//
// returns: Points around an ellipse
func (c *Curve) Ellipse(ctx context.Context, options clean.EllipseOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewEllipseRequest(options.Center, options.RadiusX, options.RadiusY)
	if options.Count != nil {
		body.SetCount(*options.Count)
	}
	return c.api.Ellipse(ctx).EllipseRequest(*body).Execute()
}

// Length: A curve query.
//
// ctx: The request context.
// options: A curve query.
//
// returns: The curve's arc length
func (c *Curve) Length(ctx context.Context, options clean.CurveOptions) (float64, *http.Response, error) {
	body := osbtk.NewCurveRequest(options.Curve)
	value, resp, err := c.api.Length(ctx).CurveRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Value, resp, nil
}

// LineIntersect: A line-segment intersection request.
//
// ctx: The request context.
// options: A line-segment intersection request.
//
// returns: The intersection point, or null
func (c *Curve) LineIntersect(ctx context.Context, options clean.LineIntersectOptions) (*osbtk.Vector2, *http.Response, error) {
	body := osbtk.NewLineIntersectRequest(options.A1, options.A2, options.B1, options.B2)
	value, resp, err := c.api.LineIntersect(ctx).LineIntersectRequest(*body).Execute()
	if err != nil {
		return nil, resp, err
	}
	return value.Point.Get(), resp, nil
}

// Normal: A curve query at a parametric or arc-length position.
//
// ctx: The request context.
// options: A curve query at a parametric or arc-length position.
//
// returns: The unit normal at `t`
func (c *Curve) Normal(ctx context.Context, options clean.CurvePointOptions) (*osbtk.Vector2, *http.Response, error) {
	body := osbtk.NewCurvePointRequest(options.Curve, options.T)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	return c.api.Normal(ctx).CurvePointRequest(*body).Execute()
}

// Offset: A curve offset request.
//
// ctx: The request context.
// options: A curve offset request.
//
// returns: The polyline offset along the curve's normal
func (c *Curve) Offset(ctx context.Context, options clean.CurveOffsetOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewCurveOffsetRequest(options.Curve, options.Distance)
	return c.api.Offset(ctx).CurveOffsetRequest(*body).Execute()
}

// Point: A curve query at a parametric or arc-length position.
//
// ctx: The request context.
// options: A curve query at a parametric or arc-length position.
//
// returns: The point at `t`, parametrically or by arc length
func (c *Curve) Point(ctx context.Context, options clean.CurvePointOptions) (*osbtk.Vector2, *http.Response, error) {
	body := osbtk.NewCurvePointRequest(options.Curve, options.T)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	return c.api.Point(ctx).CurvePointRequest(*body).Execute()
}

// Sample: A curve sampling request.
//
// ctx: The request context.
// options: A curve sampling request.
//
// returns: `count` points along the curve
func (c *Curve) Sample(ctx context.Context, options clean.CurveSampleOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewCurveSampleRequest(options.Count, options.Curve)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	return c.api.Sample(ctx).CurveSampleRequest(*body).Execute()
}

// Simplify: A Douglas-Peucker simplification request.
//
// ctx: The request context.
// options: A Douglas-Peucker simplification request.
//
// returns: The simplified polyline
func (c *Curve) Simplify(ctx context.Context, options clean.SimplifyOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewSimplifyRequest(options.Points, options.Tolerance)
	return c.api.Simplify(ctx).SimplifyRequest(*body).Execute()
}

// Tangent: A curve query at a parametric or arc-length position.
//
// ctx: The request context.
// options: A curve query at a parametric or arc-length position.
//
// returns: The unit tangent at `t`
func (c *Curve) Tangent(ctx context.Context, options clean.CurvePointOptions) (*osbtk.Vector2, *http.Response, error) {
	body := osbtk.NewCurvePointRequest(options.Curve, options.T)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	return c.api.Tangent(ctx).CurvePointRequest(*body).Execute()
}
