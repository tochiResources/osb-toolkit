//! Image generation (M25).
//!
//! Renders primitive shapes - rectangle, rounded rectangle, ellipse, circle,
//! line and polygon - with fill and stroke onto a new raster canvas, then writes
//! it to a file. Colours blend src-over with 8-bit alpha.

use image::{Rgba, RgbaImage};
use osb_api::{ImageInfo, RenderShapesRequest, RgbaColor, Shape, ShapeKind, Vector2};

/// Renders the requested shapes and writes the image to `output`.
pub fn render(request: &RenderShapesRequest) -> Result<ImageInfo, String> {
    let width = request.width.max(1);
    let height = request.height.max(1);
    let mut canvas = RgbaImage::new(width, height);
    if let Some(background) = request.background {
        for pixel in canvas.pixels_mut() {
            *pixel = to_rgba(background);
        }
    }
    for shape in &request.shapes {
        draw_shape(&mut canvas, shape);
    }
    if let Some(parent) = std::path::Path::new(&request.output).parent() {
        if !parent.as_os_str().is_empty() {
            osb_core::io::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
    }
    canvas
        .save(&request.output)
        .map_err(|error| error.to_string())?;
    Ok(ImageInfo {
        width,
        height,
        format: extension(&request.output),
    })
}

fn draw_shape(canvas: &mut RgbaImage, shape: &Shape) {
    if let Some(fill) = shape.fill {
        let (min_x, min_y, max_x, max_y) = bounds(shape, canvas);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if inside(shape, x as f64 + 0.5, y as f64 + 0.5) {
                    blend(canvas, x, y, fill);
                }
            }
        }
    }
    if let Some(stroke) = shape.stroke {
        let stroke_width = shape.stroke_width.unwrap_or(1.0).max(1.0);
        if let Some((points, closed)) = boundary(shape) {
            let count = points.len();
            if count >= 2 {
                let segments = if closed { count } else { count - 1 };
                for index in 0..segments {
                    draw_thick_line(
                        canvas,
                        points[index],
                        points[(index + 1) % count],
                        stroke_width,
                        stroke,
                    );
                }
            }
        }
    }
}

fn inside(shape: &Shape, x: f64, y: f64) -> bool {
    let (cx, cy) = (shape.center.x, shape.center.y);
    let half_w = (shape.size.x / 2.0).abs();
    let half_h = (shape.size.y / 2.0).abs();
    match shape.kind {
        ShapeKind::Rect => (x - cx).abs() <= half_w && (y - cy).abs() <= half_h,
        ShapeKind::RoundedRect => {
            if (x - cx).abs() > half_w || (y - cy).abs() > half_h {
                return false;
            }
            let radius = shape
                .corner_radius
                .unwrap_or(0.0)
                .clamp(0.0, half_w.min(half_h));
            let dx = (x - cx).abs() - (half_w - radius);
            let dy = (y - cy).abs() - (half_h - radius);
            if dx > 0.0 && dy > 0.0 {
                dx * dx + dy * dy <= radius * radius
            } else {
                true
            }
        }
        ShapeKind::Ellipse => {
            if half_w == 0.0 || half_h == 0.0 {
                return false;
            }
            let nx = (x - cx) / half_w;
            let ny = (y - cy) / half_h;
            nx * nx + ny * ny <= 1.0
        }
        ShapeKind::Circle => {
            let radius = half_w;
            let (dx, dy) = (x - cx, y - cy);
            dx * dx + dy * dy <= radius * radius
        }
        ShapeKind::Polygon => shape
            .points
            .as_ref()
            .map(|points| point_in_polygon(points, x, y))
            .unwrap_or(false),
        ShapeKind::Line => false,
    }
}

fn point_in_polygon(points: &[Vector2], x: f64, y: f64) -> bool {
    let mut inside = false;
    let count = points.len();
    let mut j = count - 1;
    for i in 0..count {
        let (xi, yi) = (points[i].x, points[i].y);
        let (xj, yj) = (points[j].x, points[j].y);
        if (yi > y) != (yj > y) && x < (xj - xi) * (y - yi) / (yj - yi) + xi {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn boundary(shape: &Shape) -> Option<(Vec<Vector2>, bool)> {
    let (cx, cy) = (shape.center.x, shape.center.y);
    let half_w = (shape.size.x / 2.0).abs();
    let half_h = (shape.size.y / 2.0).abs();
    match shape.kind {
        ShapeKind::Rect => Some((
            vec![
                Vector2 {
                    x: cx - half_w,
                    y: cy - half_h,
                },
                Vector2 {
                    x: cx + half_w,
                    y: cy - half_h,
                },
                Vector2 {
                    x: cx + half_w,
                    y: cy + half_h,
                },
                Vector2 {
                    x: cx - half_w,
                    y: cy + half_h,
                },
            ],
            true,
        )),
        ShapeKind::RoundedRect => {
            let radius = shape
                .corner_radius
                .unwrap_or(0.0)
                .clamp(0.0, half_w.min(half_h));
            let corners = [
                (cx - half_w + radius, cy - half_h + radius, 180.0),
                (cx + half_w - radius, cy - half_h + radius, 270.0),
                (cx + half_w - radius, cy + half_h - radius, 0.0),
                (cx - half_w + radius, cy + half_h - radius, 90.0),
            ];
            let mut points = Vec::new();
            for (arc_x, arc_y, start) in corners {
                for step in 0..=8 {
                    let angle = (start + 90.0 * step as f64 / 8.0).to_radians();
                    points.push(Vector2 {
                        x: arc_x + radius * angle.cos(),
                        y: arc_y + radius * angle.sin(),
                    });
                }
            }
            Some((points, true))
        }
        ShapeKind::Ellipse | ShapeKind::Circle => {
            let (radius_x, radius_y) = if shape.kind == ShapeKind::Circle {
                (half_w, half_w)
            } else {
                (half_w, half_h)
            };
            const STEPS: usize = 64;
            let points = (0..STEPS)
                .map(|index| {
                    let angle = std::f64::consts::TAU * index as f64 / STEPS as f64;
                    Vector2 {
                        x: cx + radius_x * angle.cos(),
                        y: cy + radius_y * angle.sin(),
                    }
                })
                .collect();
            Some((points, true))
        }
        ShapeKind::Polygon => shape.points.clone().map(|points| (points, true)),
        ShapeKind::Line => shape
            .points
            .as_ref()
            .and_then(|points| (points.len() >= 2).then(|| (vec![points[0], points[1]], false))),
    }
}

fn bounds(shape: &Shape, canvas: &RgbaImage) -> (u32, u32, u32, u32) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    );
    let mut consider = |x: f64, y: f64| {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    };
    if let Some(points) = &shape.points {
        for point in points {
            consider(point.x, point.y);
        }
    }
    if shape.points.is_none() || shape.kind == ShapeKind::Polygon {
        let half_w = (shape.size.x / 2.0).abs();
        let half_h = (shape.size.y / 2.0).abs();
        consider(shape.center.x - half_w, shape.center.y - half_h);
        consider(shape.center.x + half_w, shape.center.y + half_h);
    }
    let clamp_x = |value: f64| value.floor().clamp(0.0, (canvas.width() - 1) as f64) as u32;
    let clamp_y = |value: f64| value.floor().clamp(0.0, (canvas.height() - 1) as f64) as u32;
    (
        clamp_x(min_x),
        clamp_y(min_y),
        clamp_x(max_x),
        clamp_y(max_y),
    )
}

fn draw_thick_line(canvas: &mut RgbaImage, a: Vector2, b: Vector2, width: f64, color: RgbaColor) {
    let half = width / 2.0;
    let min_x = a.x.min(b.x) - half - 1.0;
    let min_y = a.y.min(b.y) - half - 1.0;
    let max_x = a.x.max(b.x) + half + 1.0;
    let max_y = a.y.max(b.y) + half + 1.0;
    let x0 = min_x.floor().clamp(0.0, (canvas.width() - 1) as f64) as u32;
    let y0 = min_y.floor().clamp(0.0, (canvas.height() - 1) as f64) as u32;
    let x1 = max_x.floor().clamp(0.0, (canvas.width() - 1) as f64) as u32;
    let y1 = max_y.floor().clamp(0.0, (canvas.height() - 1) as f64) as u32;
    for y in y0..=y1 {
        for x in x0..=x1 {
            if distance_to_segment(x as f64 + 0.5, y as f64 + 0.5, a, b) <= half {
                blend(canvas, x, y, color);
            }
        }
    }
}

fn distance_to_segment(px: f64, py: f64, a: Vector2, b: Vector2) -> f64 {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let length_squared = dx * dx + dy * dy;
    if length_squared == 0.0 {
        return ((px - a.x).powi(2) + (py - a.y).powi(2)).sqrt();
    }
    let t = (((px - a.x) * dx + (py - a.y) * dy) / length_squared).clamp(0.0, 1.0);
    let (cx, cy) = (a.x + t * dx, a.y + t * dy);
    ((px - cx).powi(2) + (py - cy).powi(2)).sqrt()
}

fn blend(canvas: &mut RgbaImage, x: u32, y: u32, color: RgbaColor) {
    if color.a == 0 || x >= canvas.width() || y >= canvas.height() {
        return;
    }
    let destination = canvas.get_pixel_mut(x, y);
    if color.a == 255 {
        *destination = Rgba([color.r, color.g, color.b, 255]);
        return;
    }
    let source_alpha = color.a as f64 / 255.0;
    let destination_alpha = destination[3] as f64 / 255.0;
    let out_alpha = source_alpha + destination_alpha * (1.0 - source_alpha);
    if out_alpha == 0.0 {
        *destination = Rgba([0, 0, 0, 0]);
        return;
    }
    let channel = |source: u8, destination: u8| {
        (((source as f64 * source_alpha)
            + (destination as f64 * destination_alpha * (1.0 - source_alpha)))
            / out_alpha)
            .round() as u8
    };
    *destination = Rgba([
        channel(color.r, destination[0]),
        channel(color.g, destination[1]),
        channel(color.b, destination[2]),
        (out_alpha * 255.0).round() as u8,
    ]);
}

fn to_rgba(color: RgbaColor) -> Rgba<u8> {
    Rgba([color.r, color.g, color.b, color.a])
}

fn extension(path: &str) -> String {
    std::path::Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_uppercase())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;

    fn output_path(name: &str) -> std::path::PathBuf {
        let directory = osb_core::io::temp_dir().join("osbtk-image-gen-tests");
        osb_core::io::create_dir_all(&directory).unwrap();
        directory.join(name)
    }

    #[test]
    fn renders_a_filled_circle() {
        let output = output_path("circle.png");
        let request = RenderShapesRequest {
            width: 64,
            height: 64,
            background: Some(RgbaColor {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }),
            shapes: vec![Shape {
                kind: ShapeKind::Circle,
                center: Vector2 { x: 32.0, y: 32.0 },
                size: Vector2 { x: 40.0, y: 40.0 },
                corner_radius: None,
                points: None,
                fill: Some(RgbaColor {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                }),
                stroke: None,
                stroke_width: None,
            }],
            output: output.to_str().unwrap().to_string(),
        };
        let info = render(&request).unwrap();
        assert_eq!(info.width, 64);
        let image = image::open(&output).unwrap();
        let center = image.get_pixel(32, 32);
        assert!(center[0] > 200 && center[1] < 60 && center[2] < 60);
        let corner = image.get_pixel(1, 1);
        assert!(corner[0] < 60);
    }

    #[test]
    fn renders_a_stroked_rect_and_line() {
        let output = output_path("shapes.png");
        let request = RenderShapesRequest {
            width: 48,
            height: 48,
            background: None,
            shapes: vec![
                Shape {
                    kind: ShapeKind::Rect,
                    center: Vector2 { x: 24.0, y: 24.0 },
                    size: Vector2 { x: 40.0, y: 40.0 },
                    corner_radius: None,
                    points: None,
                    fill: None,
                    stroke: Some(RgbaColor {
                        r: 0,
                        g: 255,
                        b: 0,
                        a: 255,
                    }),
                    stroke_width: Some(2.0),
                },
                Shape {
                    kind: ShapeKind::Line,
                    center: Vector2 { x: 0.0, y: 0.0 },
                    size: Vector2 { x: 0.0, y: 0.0 },
                    corner_radius: None,
                    points: Some(vec![
                        Vector2 { x: 4.0, y: 4.0 },
                        Vector2 { x: 44.0, y: 44.0 },
                    ]),
                    fill: None,
                    stroke: Some(RgbaColor {
                        r: 0,
                        g: 0,
                        b: 255,
                        a: 255,
                    }),
                    stroke_width: Some(2.0),
                },
            ],
            output: output.to_str().unwrap().to_string(),
        };
        render(&request).unwrap();
        let image = image::open(&output).unwrap();
        let edge = image.get_pixel(4, 24);
        assert!(edge[1] > 200);
        let middle = image.get_pixel(24, 24);
        assert!(middle[2] > 200);
    }
}
