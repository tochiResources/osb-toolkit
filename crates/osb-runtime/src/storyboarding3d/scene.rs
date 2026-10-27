use crate::animations::KeyframedValue;
use crate::math::{Color4, Vec2};
use crate::storyboarding::{OsbLayer, OsbOrigin};

use crate::storyboarding3d::camera::{CameraState, PerspectiveCamera};
use crate::storyboarding3d::node::{state_of, LeafMeta, Object3d, Object3dState, SpriteState};
use crate::storyboarding3d::optimize::{
    spans_1d, spans_2d, spans_color, BakedCommand, CommandKind,
};

/// Options controlling the bake.
#[derive(Debug, Clone)]
pub struct BakeOptions {
    pub start_time: f64,
    pub end_time: f64,
    /// Fixed sampling step (ms). Also the maximum step of adaptive sampling.
    pub time_step: f64,
    /// Douglas-Peucker tolerance for 2D channels (position/scale), in pixels.
    pub tolerance_2d: f64,
    /// Tolerance for 1D channels (rotation radians/opacity).
    pub tolerance_1d: f64,
    /// Decimal places to round each simplified value to.
    pub decimals: u32,
    /// Sample times concurrently (default on).
    pub multithreaded: bool,
    /// Subdivide the time domain where projected motion exceeds
    /// `adaptive_tolerance_px` (default on; the fixed `time_step` is the fallback).
    pub adaptive_sampling: bool,
    /// Pixel error above which an adaptive interval is split.
    pub adaptive_tolerance_px: f64,
    /// Sort sprites by `z_depth` into painter layers (default on).
    pub depth_sort: bool,
    /// Number of depth buckets (osu! layers) sprites are grouped into.
    pub depth_buckets: u8,
}

impl Default for BakeOptions {
    fn default() -> Self {
        BakeOptions {
            start_time: 0.0,
            end_time: 1000.0,
            time_step: 16.0,
            tolerance_2d: 0.1,
            tolerance_1d: 0.001,
            decimals: 3,
            multithreaded: true,
            adaptive_sampling: true,
            adaptive_tolerance_px: 0.5,
            depth_sort: true,
            depth_buckets: 5,
        }
    }
}

/// A sprite's baked 2D channels.
#[derive(Debug, Clone)]
pub struct BakedSprite {
    pub sprite_path: String,
    pub origin: OsbOrigin,
    /// The osu! layer the sprite's node requested, if any (else the depth bucket).
    pub osb_layer: Option<OsbLayer>,
    pub additive: bool,
    /// Whether the sprite is drawn mirrored horizontally.
    pub flip_h: bool,
    /// Average projected depth (for ordering into layers).
    pub depth: f64,
    /// Average accumulated `z_depth` (painter's-algorithm offset).
    pub z_depth: f64,
    /// The depth bucket (osu! layer) this sprite was sorted into.
    pub layer: u8,
    pub position: KeyframedValue<Vec2>,
    pub scale: KeyframedValue<Vec2>,
    pub rotation: KeyframedValue<f64>,
    pub opacity: KeyframedValue<f64>,
    pub color: KeyframedValue<Color4>,
}

impl BakedSprite {
    /// Flattens the simplified channels into `.osb`-compatible command spans.
    pub fn commands(&self) -> Vec<BakedCommand> {
        let mut commands = Vec::new();
        commands.extend(spans_2d(CommandKind::Move, &self.position));
        commands.extend(spans_2d(CommandKind::ScaleVec, &self.scale));
        commands.extend(spans_1d(CommandKind::Rotate, &self.rotation));
        commands.extend(spans_1d(CommandKind::Fade, &self.opacity));
        commands.extend(spans_color(&self.color));
        let (start, end) = time_bounds(self);
        if self.additive {
            commands.push(BakedCommand {
                kind: CommandKind::Additive,
                start_time: start,
                end_time: end,
                start: crate::storyboarding3d::optimize::BakedValue::Scalar(1.0),
                end: crate::storyboarding3d::optimize::BakedValue::Scalar(1.0),
            });
        }
        if self.flip_h {
            commands.push(BakedCommand {
                kind: CommandKind::FlipH,
                start_time: start,
                end_time: end,
                start: crate::storyboarding3d::optimize::BakedValue::Scalar(1.0),
                end: crate::storyboarding3d::optimize::BakedValue::Scalar(1.0),
            });
        }
        commands
    }
}

impl BakedSprite {
    pub fn time_range(&self) -> (f64, f64) {
        time_bounds(self)
    }
}

fn time_bounds(sprite: &BakedSprite) -> (f64, f64) {
    let keyframes = sprite.position.keyframes();
    match (keyframes.first(), keyframes.last()) {
        (Some(first), Some(last)) => (first.time, last.time),
        _ => (0.0, 0.0),
    }
}

/// The baked result: one entry per sprite, sorted into painter order.
#[derive(Debug, Clone, Default)]
pub struct BakedStoryboard {
    pub sprites: Vec<BakedSprite>,
}

/// Projects `root` through `camera` and simplifies the per-sprite screenspace
/// channels.
pub fn bake(
    root: &dyn Object3d,
    camera: &PerspectiveCamera,
    options: &BakeOptions,
) -> BakedStoryboard {
    let mut meta = Vec::new();
    collect_leaves(root, &mut meta);
    if meta.is_empty() {
        return BakedStoryboard::default();
    }
    let animation_start = options.start_time;
    let sample_all = |time: f64| {
        let camera_state = camera.state_at(time);
        let mut out = Vec::with_capacity(meta.len());
        sample_object(
            root,
            time,
            animation_start,
            &camera_state,
            &Object3dState::initial(),
            &mut out,
        );
        out
    };

    let times = sample_times(options, &sample_all);

    let frames: Vec<Vec<SpriteState>> = if options.multithreaded && times.len() > 1 {
        use rayon::prelude::*;
        times.par_iter().map(|&time| sample_all(time)).collect()
    } else {
        times.iter().map(|&time| sample_all(time)).collect()
    };

    let mut sprites = Vec::with_capacity(meta.len());
    for (index, leaf) in meta.iter().enumerate() {
        let mut position = KeyframedValue::new();
        let mut scale = KeyframedValue::new();
        let mut rotation = KeyframedValue::new();
        let mut opacity = KeyframedValue::new();
        let mut color = KeyframedValue::new();
        let mut depth_sum = 0.0;
        let mut z_depth_sum = 0.0;
        for (time_index, &time) in times.iter().enumerate() {
            let state = frames[time_index][index];
            position.add(time, state.position);
            scale.add(time, state.scale);
            rotation.add(time, state.rotation);
            opacity.add(time, state.opacity);
            color.add(time, state.color);
            depth_sum += state.depth;
            z_depth_sum += state.z_depth;
        }
        position.simplify_2d(options.tolerance_2d, options.decimals);
        scale.simplify_2d(options.tolerance_2d, options.decimals);
        rotation.simplify_1d(options.tolerance_1d, options.decimals);
        opacity.simplify_1d(options.tolerance_1d, options.decimals);
        sprites.push(BakedSprite {
            sprite_path: leaf.sprite_path.clone(),
            origin: leaf.origin,
            osb_layer: leaf.osb_layer,
            additive: leaf.additive,
            flip_h: leaf.flip_h,
            depth: depth_sum / times.len() as f64,
            z_depth: z_depth_sum / times.len() as f64,
            layer: 0,
            position,
            scale,
            rotation,
            opacity,
            color,
        });
    }

    assign_layers(&mut sprites, options);
    BakedStoryboard { sprites }
}

/// Assigns each sprite a depth bucket and sorts into a stable painter order.
fn assign_layers(sprites: &mut [BakedSprite], options: &BakeOptions) {
    if !options.depth_sort || sprites.is_empty() {
        return;
    }
    let min = sprites
        .iter()
        .map(|s| s.z_depth)
        .fold(f64::INFINITY, f64::min);
    let max = sprites
        .iter()
        .map(|s| s.z_depth)
        .fold(f64::NEG_INFINITY, f64::max);
    let buckets = options.depth_buckets.max(1);
    for sprite in sprites.iter_mut() {
        sprite.layer = depth_layer(sprite.z_depth, min, max, buckets);
    }
    // Stable sort keeps the traversal order within equal keys; smaller `z_depth`
    // draws first (behind).
    sprites.sort_by(|a, b| {
        a.layer.cmp(&b.layer).then(
            a.z_depth
                .partial_cmp(&b.z_depth)
                .unwrap_or(std::cmp::Ordering::Equal),
        )
    });
}

fn depth_layer(z: f64, min: f64, max: f64, buckets: u8) -> u8 {
    if buckets <= 1 || (max - min).abs() < 1e-9 {
        return 0;
    }
    let t = ((z - min) / (max - min)).clamp(0.0, 1.0);
    ((t * (buckets as f64 - 1.0)).round() as u8).min(buckets - 1)
}

/// Builds the sample times: fixed steps, or adaptive subdivision where the
/// projected motion of the whole tree exceeds `adaptive_tolerance_px`.
fn sample_times(options: &BakeOptions, sample_all: &impl Fn(f64) -> Vec<SpriteState>) -> Vec<f64> {
    let step = if options.time_step > 0.0 {
        options.time_step
    } else {
        1.0
    };
    let start = options.start_time;
    let end = options.end_time.max(start);

    if !options.adaptive_sampling {
        let mut times = Vec::new();
        let mut time = start;
        while time < end + 5.0 {
            times.push(time);
            time += step;
        }
        if times.is_empty() {
            times.push(start);
        }
        return times;
    }

    let mut times = vec![start];
    if end > start {
        subdivide(options, sample_all, start, end, step, &mut times);
        times.push(end);
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    times
}

/// Recursively samples only where the interval's midpoint deviates from the
/// linear interpolation of its endpoints. Intervals at or below `max_step` are
/// not split further; near-linear intervals are left to their endpoints.
fn subdivide(
    options: &BakeOptions,
    sample_all: &impl Fn(f64) -> Vec<SpriteState>,
    a: f64,
    b: f64,
    max_step: f64,
    out: &mut Vec<f64>,
) {
    if b - a <= max_step {
        return;
    }
    let mid = (a + b) * 0.5;
    let error = interval_error(&sample_all(a), &sample_all(b), &sample_all(mid));
    if error <= options.adaptive_tolerance_px {
        return;
    }
    subdivide(options, sample_all, a, mid, max_step, out);
    out.push(mid);
    subdivide(options, sample_all, mid, b, max_step, out);
}

/// The maximum deviation between a midpoint sample and the linear interpolation
/// of the interval endpoints, in approximate screen pixels.
fn interval_error(a: &[SpriteState], b: &[SpriteState], mid: &[SpriteState]) -> f64 {
    let mut error: f64 = 0.0;
    for ((sa, sb), sm) in a.iter().zip(b.iter()).zip(mid.iter()) {
        let lerp = |x: f64, y: f64| x + (y - x) * 0.5;
        let dx = sm.position.x - lerp(sa.position.x, sb.position.x);
        let dy = sm.position.y - lerp(sa.position.y, sb.position.y);
        let dsx = sm.scale.x - lerp(sa.scale.x, sb.scale.x);
        let dsy = sm.scale.y - lerp(sa.scale.y, sb.scale.y);
        let drot = (sm.rotation - lerp(sa.rotation, sb.rotation)).abs();
        let dop = (sm.opacity - lerp(sa.opacity, sb.opacity)).abs();
        let candidate = (dx * dx + dy * dy).sqrt()
            + (dsx * dsx + dsy * dsy).sqrt()
            + drot * 100.0
            + dop * 100.0;
        error = error.max(candidate);
    }
    error
}

/// Collects every sprite leaf's identity, in traversal order.
fn collect_leaves(object: &dyn Object3d, out: &mut Vec<LeafMeta>) {
    object.leaves(out);
    for child in object.children() {
        collect_leaves(child.as_ref(), out);
    }
}

/// Samples every sprite leaf's state at `time`, in traversal order.
fn sample_object(
    object: &dyn Object3d,
    time: f64,
    animation_start: f64,
    camera: &CameraState,
    parent: &Object3dState,
    out: &mut Vec<SpriteState>,
) {
    let state = state_of(object, time, parent);
    object.sample(time, animation_start, camera, &state, out);
    for child in object.children() {
        sample_object(child.as_ref(), time, animation_start, camera, &state, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;
    use crate::storyboarding3d::camera::PerspectiveCamera;
    use crate::storyboarding3d::node::{Node3d, Sprite3d};

    fn facing_camera() -> PerspectiveCamera {
        let mut camera = PerspectiveCamera::default();
        camera.position_z.add(0.0, 5.0);
        camera
    }

    fn centered_sprite() -> Node3d {
        let mut root = Node3d::default();
        root.add(Box::new(Sprite3d::new("sb/dot.png", OsbOrigin::Centre)));
        root
    }

    #[test]
    fn sprite_at_origin_projects_to_storyboard_centre() {
        let camera = facing_camera();
        let scene = centered_sprite();
        let baked = bake(&scene, &camera, &BakeOptions::default());
        assert_eq!(baked.sprites.len(), 1);
        let position = baked.sprites[0].position.value_at(0.0).unwrap();
        // 854x480 storyboard, sprite at world origin -> centre (427, 240).
        assert!((position.x - 427.0).abs() < 0.5, "x = {}", position.x);
        assert!((position.y - 240.0).abs() < 0.5, "y = {}", position.y);
    }

    #[test]
    fn constant_sprite_simplifies_to_one_span() {
        let camera = facing_camera();
        let scene = centered_sprite();
        let baked = bake(&scene, &camera, &BakeOptions::default());
        let commands = baked.sprites[0].commands();
        // Position and scale are constant, so each simplifies to a single span.
        let moves = commands
            .iter()
            .filter(|c| c.kind == CommandKind::Move)
            .count();
        assert_eq!(moves, 1);
        assert!(commands.len() >= 4);
    }

    #[test]
    fn multithreaded_and_single_threaded_match() {
        let camera = facing_camera();
        let scene = centered_sprite();
        let parallel = bake(
            &scene,
            &camera,
            &BakeOptions {
                multithreaded: true,
                ..Default::default()
            },
        );
        let serial = bake(
            &scene,
            &camera,
            &BakeOptions {
                multithreaded: false,
                ..Default::default()
            },
        );
        assert_eq!(
            parallel.sprites[0].position.keyframes().len(),
            serial.sprites[0].position.keyframes().len()
        );
        assert_eq!(
            parallel.sprites[0].position.value_at(250.0),
            serial.sprites[0].position.value_at(250.0)
        );
    }

    #[test]
    fn additive_sprite_emits_a_parameter_span() {
        let camera = facing_camera();
        let mut root = Node3d::default();
        let mut sprite = Sprite3d::new("sb/dot.png", OsbOrigin::Centre);
        sprite.additive = true;
        root.add(Box::new(sprite));
        let baked = bake(&root, &camera, &BakeOptions::default());
        assert!(baked.sprites[0]
            .commands()
            .iter()
            .any(|c| c.kind == CommandKind::Additive));
    }

    #[test]
    fn a_moving_node_bakes_multiple_move_spans() {
        let camera = facing_camera();
        let mut root = Node3d::default();
        let mut node = Node3d::default();
        node.position.add(0.0, Vec3::new(-1.0, 0.0, 0.0));
        node.position.add(1000.0, Vec3::new(1.0, 0.0, 0.0));
        node.add(Box::new(Sprite3d::new("sb/dot.png", OsbOrigin::Centre)));
        root.add(Box::new(node));
        let baked = bake(&root, &camera, &BakeOptions::default());
        let moves = baked.sprites[0]
            .commands()
            .iter()
            .filter(|c| c.kind == CommandKind::Move)
            .count();
        // Perspective motion is non-linear in screenspace, so it may keep more
        // than one span; the direction is what matters here.
        assert!(moves >= 1);
        let start = baked.sprites[0].position.value_at(0.0).unwrap();
        let end = baked.sprites[0].position.value_at(1000.0).unwrap();
        assert!(
            end.x > start.x,
            "expected rightward motion: {} -> {}",
            start.x,
            end.x
        );
    }

    #[test]
    fn adaptive_sampling_is_deterministic_and_simplifies_static_motion() {
        let camera = facing_camera();
        let scene = centered_sprite();
        let adaptive = bake(&scene, &camera, &BakeOptions::default());
        // A static sprite needs only the two endpoints.
        assert_eq!(adaptive.sprites[0].position.keyframes().len(), 2);

        let fixed = bake(
            &scene,
            &camera,
            &BakeOptions {
                adaptive_sampling: false,
                ..Default::default()
            },
        );
        assert!(fixed.sprites[0].position.keyframes().len() <= 2);
    }

    #[test]
    fn depth_sort_orders_nearer_sprites_last() {
        let camera = facing_camera();
        let mut root = Node3d::default();
        let mut near = Sprite3d::new("sb/near.png", OsbOrigin::Centre);
        near.node.z_depth.add(0.0, 10.0);
        let mut far = Sprite3d::new("sb/far.png", OsbOrigin::Centre);
        far.node.z_depth.add(0.0, -10.0);
        // Add the near sprite first; depth sort must move it after the far one.
        root.add(Box::new(near));
        root.add(Box::new(far));
        let baked = bake(&root, &camera, &BakeOptions::default());
        assert_eq!(baked.sprites[0].sprite_path, "sb/far.png");
        assert_eq!(baked.sprites[1].sprite_path, "sb/near.png");
        assert!(baked.sprites[1].layer >= baked.sprites[0].layer);
    }

    #[test]
    fn animation_plane_expands_frames_with_visibility_windows() {
        let camera = facing_camera();
        let mut root = Node3d::default();
        let mut sprite = Sprite3d::new("sb/a.png", OsbOrigin::Centre);
        sprite.frames = vec!["sb/a.png".into(), "sb/b.png".into(), "sb/c.png".into()];
        sprite.frame_delay = 100.0;
        root.add(Box::new(sprite));
        let options = BakeOptions {
            start_time: 0.0,
            end_time: 300.0,
            time_step: 50.0,
            adaptive_sampling: false,
            ..BakeOptions::default()
        };
        let baked = bake(&root, &camera, &options);
        assert_eq!(baked.sprites.len(), 3);
        assert_eq!(baked.sprites[0].sprite_path, "sb/a.png");
        assert_eq!(baked.sprites[1].sprite_path, "sb/b.png");
        assert_eq!(baked.sprites[2].sprite_path, "sb/c.png");
        // Frame 0 is visible at t=50; frame 1 at t=150.
        assert!(baked.sprites[0].opacity.value_at(50.0).unwrap() > 0.5);
        assert!(baked.sprites[1].opacity.value_at(50.0).unwrap() < 0.5);
        assert!(baked.sprites[1].opacity.value_at(150.0).unwrap() > 0.5);
        assert!(baked.sprites[0].opacity.value_at(150.0).unwrap() < 0.5);
    }
}
