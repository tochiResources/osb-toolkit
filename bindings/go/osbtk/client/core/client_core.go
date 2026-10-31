package core

import (
	"context"
	"net/http"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
)

// Core is the clean high-level client.
type Core struct {
	api *osbtk.CoreAPIService
}

func NewCore(api *osbtk.CoreAPIService) *Core {
	return &Core{api: api}
}

func IsCore(value any) bool {
	_, ok := value.(*Core)
	return ok
}

// ApplyEasing: An easing evaluation request.
//
// ctx: The request context.
// options: An easing evaluation request.
//
// returns: The eased value
func (c *Core) ApplyEasing(ctx context.Context, options clean.ApplyEasingOptions) (float64, *http.Response, error) {
	body := osbtk.NewApplyEasingRequest(options.Easing, options.T)
	value, resp, err := c.api.ApplyEasing(ctx).ApplyEasingRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Value, resp, nil
}

// Arc: A circular-arc sampling request.
//
// ctx: The request context.
// options: A circular-arc sampling request.
//
// returns: Points along a circular arc
func (c *Core) Arc(ctx context.Context, options clean.ArcOptions) (*osbtk.PointListResult, *http.Response, error) {
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
func (c *Core) BoundingBox(ctx context.Context, options clean.CurveOptions) (*osbtk.Box2, *http.Response, error) {
	body := osbtk.NewCurveRequest(options.Curve)
	return c.api.BoundingBox(ctx).CurveRequest(*body).Execute()
}

// Curvature: A curve query at a parametric or arc-length position.
//
// ctx: The request context.
// options: A curve query at a parametric or arc-length position.
//
// returns: The curvature at `t`
func (c *Core) Curvature(ctx context.Context, options clean.CurvePointOptions) (float64, *http.Response, error) {
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
func (c *Core) DistanceTo(ctx context.Context, options clean.CurveDistanceOptions) (float64, *http.Response, error) {
	body := osbtk.NewCurveDistanceRequest(options.Curve, options.Point)
	value, resp, err := c.api.DistanceTo(ctx).CurveDistanceRequest(*body).Execute()
	if err != nil {
		return 0, resp, err
	}
	return value.Value, resp, nil
}

// EasingValue: A named-or-custom easing evaluation request.
//
// ctx: The request context.
// options: A named-or-custom easing evaluation request.
//
// returns: The eased value (named easing or cubic-bezier)
func (c *Core) EasingValue(ctx context.Context, options clean.EasingValueOptions) (float64, *http.Response, error) {
	body := osbtk.NewEasingValueRequest(options.T)
	if options.CubicBezier != nil {
		body.SetCubicBezier(options.CubicBezier)
	}
	if options.Easing != nil {
		body.SetEasing(*options.Easing)
	}
	value, resp, err := c.api.EasingValue(ctx).EasingValueRequest(*body).Execute()
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
func (c *Core) Ellipse(ctx context.Context, options clean.EllipseOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewEllipseRequest(options.Center, options.RadiusX, options.RadiusY)
	if options.Count != nil {
		body.SetCount(*options.Count)
	}
	return c.api.Ellipse(ctx).EllipseRequest(*body).Execute()
}

// GetOriginVector: The anchor offset for an origin.
//
// ctx: The request context.
// options: The anchor offset for an origin.
//
// returns: The anchor offset for the origin
func (c *Core) GetOriginVector(ctx context.Context, options clean.GetOriginVectorOptions) (*osbtk.Vector2, *http.Response, error) {
	body := osbtk.NewGetOriginVectorRequest(options.Height, options.Origin, options.Width)
	return c.api.GetOriginVector(ctx).GetOriginVectorRequest(*body).Execute()
}

// InScreenBounds: Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
//
// ctx: The request context.
// options: Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
//
// returns: Whether the sprite's bounds intersect the screen
func (c *Core) InScreenBounds(ctx context.Context, options clean.InScreenBoundsOptions) (bool, *http.Response, error) {
	body := osbtk.NewInScreenBoundsRequest(options.Origin, options.Position, options.Rotation, options.Size)
	value, resp, err := c.api.InScreenBounds(ctx).InScreenBoundsRequest(*body).Execute()
	if err != nil {
		return false, resp, err
	}
	return value.Value, resp, nil
}

// Length: A curve query.
//
// ctx: The request context.
// options: A curve query.
//
// returns: The curve's arc length
func (c *Core) Length(ctx context.Context, options clean.CurveOptions) (float64, *http.Response, error) {
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
func (c *Core) LineIntersect(ctx context.Context, options clean.LineIntersectOptions) (*osbtk.Vector2, *http.Response, error) {
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
func (c *Core) Normal(ctx context.Context, options clean.CurvePointOptions) (*osbtk.Vector2, *http.Response, error) {
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
func (c *Core) Offset(ctx context.Context, options clean.CurveOffsetOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewCurveOffsetRequest(options.Curve, options.Distance)
	return c.api.Offset(ctx).CurveOffsetRequest(*body).Execute()
}

// ParseOsb: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// ctx: The request context.
// options: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// returns: The storyboard summary
func (c *Core) ParseOsb(ctx context.Context, options clean.TextOptions) (*osbtk.ParseOsbResult, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	return c.api.ParseOsb(ctx).TextRequest(*body).Execute()
}

// ParseOsu: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// ctx: The request context.
// options: A request carrying a block of text (an effect script, `.osu` or `.osb`).
//
// returns: The parsed beatmap
func (c *Core) ParseOsu(ctx context.Context, options clean.TextOptions) (*osbtk.Beatmap, *http.Response, error) {
	body := osbtk.NewTextRequest(options.Text)
	return c.api.ParseOsu(ctx).TextRequest(*body).Execute()
}

// Point: A curve query at a parametric or arc-length position.
//
// ctx: The request context.
// options: A curve query at a parametric or arc-length position.
//
// returns: The point at `t`, parametrically or by arc length
func (c *Core) Point(ctx context.Context, options clean.CurvePointOptions) (*osbtk.Vector2, *http.Response, error) {
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
func (c *Core) Sample(ctx context.Context, options clean.CurveSampleOptions) (*osbtk.PointListResult, *http.Response, error) {
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
func (c *Core) Simplify(ctx context.Context, options clean.SimplifyOptions) (*osbtk.PointListResult, *http.Response, error) {
	body := osbtk.NewSimplifyRequest(options.Points, options.Tolerance)
	return c.api.Simplify(ctx).SimplifyRequest(*body).Execute()
}

// Tangent: A curve query at a parametric or arc-length position.
//
// ctx: The request context.
// options: A curve query at a parametric or arc-length position.
//
// returns: The unit tangent at `t`
func (c *Core) Tangent(ctx context.Context, options clean.CurvePointOptions) (*osbtk.Vector2, *http.Response, error) {
	body := osbtk.NewCurvePointRequest(options.Curve, options.T)
	if options.ByArcLength != nil {
		body.SetByArcLength(*options.ByArcLength)
	}
	return c.api.Tangent(ctx).CurvePointRequest(*body).Execute()
}

// Version: Calls the Core.version bridge operation.
//
// ctx: The request context.
//
// returns: The runtime version
func (c *Core) Version(ctx context.Context) (string, *http.Response, error) {
	value, resp, err := c.api.Version(ctx).Execute()
	if err != nil {
		return "", resp, err
	}
	return value.Version, resp, nil
}
