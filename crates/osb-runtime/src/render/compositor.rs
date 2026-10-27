use rayon::prelude::*;

use crate::assets::TextureStore;
use crate::math::Vec2;
use crate::storyboarding::{OsbOrigin, Storyboard};

/// How the compositor parallelizes a frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Concurrency {
    /// Use all available parallelism (default), clamped to the frame height.
    #[default]
    Auto,
    /// Use exactly this many worker bands (at least 1).
    Fixed(usize),
    /// Render single-threaded.
    Off,
}

/// Parameters for a single composited frame.
#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub width: u32,
    pub height: u32,
    /// Playfield time in milliseconds.
    pub time: f64,
    /// Concurrency policy; defaults to [`Concurrency::Auto`].
    pub concurrency: Concurrency,
}

impl RenderOptions {
    pub fn new(width: u32, height: u32, time: f64) -> Self {
        RenderOptions {
            width,
            height,
            time,
            concurrency: Concurrency::Auto,
        }
    }
}

/// Composites every visible sprite of `storyboard` into an RGBA8 frame at
/// `options.time`, scaling from the storyboard `resolution`.
///
/// The frame is split into horizontal bands that render concurrently (M13b); each
/// band draws every sprite clipped to its rows, matching single-threaded output.
pub fn render(
    storyboard: &Storyboard,
    resolution: Vec2,
    textures: &TextureStore,
    options: RenderOptions,
) -> Vec<u8> {
    let width = options.width as usize;
    let height = options.height as usize;
    let mut frame = vec![0u8; width * height * 4];
    if width == 0 || height == 0 {
        return frame;
    }

    let scale_x = if resolution.x == 0.0 {
        1.0
    } else {
        options.width as f64 / resolution.x
    };
    let scale_y = if resolution.y == 0.0 {
        1.0
    } else {
        options.height as f64 / resolution.y
    };

    let bands = resolve_bands(options.concurrency, height);
    let rows_per_band = height.div_ceil(bands);
    let bytes_per_band = rows_per_band * width * 4;

    frame
        .par_chunks_mut(bytes_per_band)
        .enumerate()
        .for_each(|(index, chunk)| {
            let row_start = index * rows_per_band;
            let band_rows = chunk.len() / (width * 4);
            for layer in storyboard.layers() {
                for sprite in layer.sprites() {
                    if sprite.exclude {
                        continue;
                    }
                    let Some(texture) = textures.get(&sprite.texture_path) else {
                        continue;
                    };
                    draw_sprite(
                        chunk, width, row_start, band_rows, options, sprite, texture, scale_x,
                        scale_y,
                    );
                }
            }
        });

    frame
}

fn resolve_bands(concurrency: Concurrency, height: usize) -> usize {
    let requested = match concurrency {
        Concurrency::Off => 1,
        Concurrency::Fixed(count) => count.max(1),
        Concurrency::Auto => std::thread::available_parallelism()
            .map(|count| count.get())
            .unwrap_or(1),
    };
    requested.clamp(1, height.max(1)).min(64)
}

#[allow(clippy::too_many_arguments)]
fn draw_sprite(
    frame: &mut [u8],
    frame_width: usize,
    row_start: usize,
    band_rows: usize,
    options: RenderOptions,
    sprite: &crate::storyboarding::OsbSprite,
    texture: &crate::assets::Texture,
    scale_x: f64,
    scale_y: f64,
) {
    let opacity = sprite.opacity_at(options.time);
    if opacity <= 0.0 {
        return;
    }
    let position = sprite.position_at(options.time);
    let scale = sprite.scale_at(options.time);
    if scale.x == 0.0 || scale.y == 0.0 {
        return;
    }
    let rotation = sprite.rotation_at(options.time);
    let color = sprite.color_at(options.time);
    let additive = sprite.additive_at(options.time);

    let size = Vec2::new(
        texture.width as f64 * scale.x,
        texture.height as f64 * scale.y,
    );
    // The effective anchor is the enum origin offset plus the animatable anchor
    // channel, both in texture pixels; scaled into the sprite's local space.
    let anchor_vec = crate::storyboarding::OsbSprite::get_origin_vector(
        sprite.origin,
        texture.width as f64,
        texture.height as f64,
    ) + sprite.anchor_at(options.time);
    let anchor = Vec2::new(anchor_vec.x * scale.x, anchor_vec.y * scale.y);
    let (sin, cos) = (-rotation).sin_cos();

    let to_texel = |output_x: f64, output_y: f64| -> Option<Vec2> {
        let osu = Vec2::new(output_x / scale_x, output_y / scale_y);
        let v = osu - position;
        let u = Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos);
        let local = Vec2::new(u.x + anchor.x, u.y + anchor.y);
        let texel = Vec2::new(local.x / scale.x, local.y / scale.y);
        if texel.x < 0.0
            || texel.y < 0.0
            || texel.x >= texture.width as f64
            || texel.y >= texture.height as f64
        {
            None
        } else {
            Some(texel)
        }
    };

    let corners = [
        Vec2::new(-anchor.x, -anchor.y),
        Vec2::new(size.x - anchor.x, -anchor.y),
        Vec2::new(-anchor.x, size.y - anchor.y),
        Vec2::new(size.x - anchor.x, size.y - anchor.y),
    ];
    let (rot_sin, rot_cos) = (rotation).sin_cos();
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;
    for corner in corners {
        let rotated = Vec2::new(
            corner.x * rot_cos - corner.y * rot_sin,
            corner.x * rot_sin + corner.y * rot_cos,
        );
        let output = Vec2::new(
            (position.x + rotated.x) * scale_x,
            (position.y + rotated.y) * scale_y,
        );
        min_x = min_x.min(output.x);
        min_y = min_y.min(output.y);
        max_x = max_x.max(output.x);
        max_y = max_y.max(output.y);
    }

    // Clip the sprite's bounding box to this band's rows.
    let band_top = row_start as f64;
    let band_bottom = (row_start + band_rows) as f64;
    let x0 = min_x.floor().max(0.0) as usize;
    let y0 = min_y.floor().max(band_top).max(0.0) as usize;
    let x1 = max_x.ceil().min(options.width as f64) as usize;
    let y1 = max_y.ceil().min(band_bottom).min(options.height as f64) as usize;
    if y1 <= y0 || x1 <= x0 {
        return;
    }

    for y in y0..y1 {
        for x in x0..x1 {
            let Some(texel) = to_texel(x as f64 + 0.5, y as f64 + 0.5) else {
                continue;
            };
            let [r, g, b, a] = texture.sample(texel.x.floor(), texel.y.floor());
            let alpha = (a as f64 / 255.0) * opacity * color.a;
            if alpha <= 0.0 {
                continue;
            }
            let index = ((y - row_start) * frame_width + x) * 4;
            let src_r = (r as f64 / 255.0) * color.r;
            let src_g = (g as f64 / 255.0) * color.g;
            let src_b = (b as f64 / 255.0) * color.b;
            if additive {
                frame[index] = clamp8(frame[index] as f64 / 255.0 + src_r * alpha);
                frame[index + 1] = clamp8(frame[index + 1] as f64 / 255.0 + src_g * alpha);
                frame[index + 2] = clamp8(frame[index + 2] as f64 / 255.0 + src_b * alpha);
                frame[index + 3] = clamp8(frame[index + 3] as f64 / 255.0 + alpha);
            } else {
                let inv = 1.0 - alpha;
                frame[index] = clamp8(src_r * alpha + (frame[index] as f64 / 255.0) * inv);
                frame[index + 1] = clamp8(src_g * alpha + (frame[index + 1] as f64 / 255.0) * inv);
                frame[index + 2] = clamp8(src_b * alpha + (frame[index + 2] as f64 / 255.0) * inv);
                frame[index + 3] = clamp8(alpha + (frame[index + 3] as f64 / 255.0) * inv);
            }
        }
    }
}

fn clamp8(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// Convenience: the anchor origin of a sprite (used by tests and adapters).
pub fn origin_anchor(origin: OsbOrigin) -> (f64, f64) {
    origin.anchor()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::Texture;
    use crate::storyboarding::{OsbLayer, OsbOrigin, Storyboard};

    fn solid_frame(concurrency: Concurrency) -> Vec<u8> {
        let mut storyboard = Storyboard::new();
        storyboard
            .layer(OsbLayer::Background)
            .create_sprite("sb/white.png", OsbOrigin::TopLeft)
            .fade(0.0, 1.0, 1.0, 1.0, crate::animations::Easing::None);
        let mut textures = TextureStore::new();
        textures.insert("sb/white.png", Texture::new(1, 1, vec![255, 0, 0, 255]));
        render(
            &storyboard,
            Vec2::new(2.0, 2.0),
            &textures,
            RenderOptions {
                width: 2,
                height: 2,
                time: 0.0,
                concurrency,
            },
        )
    }

    #[test]
    fn solid_sprite_fills_expected_region() {
        let frame = solid_frame(Concurrency::Off);
        assert_eq!(&frame[0..4], &[255, 0, 0, 255]);
    }

    #[test]
    fn parallel_matches_single_threaded() {
        assert_eq!(
            solid_frame(Concurrency::Off),
            solid_frame(Concurrency::Fixed(4))
        );
        assert_eq!(
            solid_frame(Concurrency::Off),
            solid_frame(Concurrency::Auto)
        );
    }

    #[test]
    fn missing_texture_is_skipped() {
        let mut storyboard = Storyboard::new();
        storyboard
            .layer(OsbLayer::Background)
            .create_sprite("sb/missing.png", OsbOrigin::Centre);
        let textures = TextureStore::new();
        let frame = render(
            &storyboard,
            Vec2::new(4.0, 4.0),
            &textures,
            RenderOptions::new(4, 4, 0.0),
        );
        assert!(frame.iter().all(|&byte| byte == 0));
    }

    fn top_left_red_frame(anchor: Vec2) -> Vec<u8> {
        let mut storyboard = Storyboard::new();
        let sprite = storyboard
            .layer(OsbLayer::Background)
            .create_sprite("sb/corner.png", OsbOrigin::TopLeft);
        sprite.set_anchor(anchor);
        let mut textures = TextureStore::new();
        // A 2x2 texture with only the top-left texel opaque red.
        textures.insert(
            "sb/corner.png",
            Texture::new(
                2,
                2,
                vec![255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
        );
        render(
            &storyboard,
            Vec2::new(4.0, 4.0),
            &textures,
            RenderOptions::new(4, 4, 0.0),
        )
    }

    #[test]
    fn animated_anchor_offsets_the_sprite() {
        // With no anchor offset the opaque texel lands on output pixel (0, 0).
        assert_eq!(&top_left_red_frame(Vec2::ZERO)[0..4], &[255, 0, 0, 255]);
        // Offsetting the anchor by one texture pixel shifts the sprite content
        // one pixel up/left, so pixel (0, 0) becomes transparent.
        assert_eq!(
            &top_left_red_frame(Vec2::new(1.0, 0.0))[0..4],
            &[0, 0, 0, 0]
        );
    }
}
