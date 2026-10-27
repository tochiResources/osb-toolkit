use std::collections::HashMap;

use crate::animations::Easing;
use crate::context::Context;
use crate::math::{Color4, Vec2};

use super::layer::{Entry, Layer, OsbSample};
use super::sprite::OsbSprite;
use super::types::{LayerHandle, OsbLayer, OsbLoopType, OsbOrigin, ParameterType, SpriteHandle};

/// A layer identifier accepted by [`Storyboard::layer`]: an [`OsbLayer`] or a name.
pub trait LayerKey {
    fn layer_name(&self) -> String;
}

impl LayerKey for OsbLayer {
    fn layer_name(&self) -> String {
        self.name().to_string()
    }
}

impl LayerKey for &str {
    fn layer_name(&self) -> String {
        (*self).to_string()
    }
}

impl LayerKey for String {
    fn layer_name(&self) -> String {
        self.clone()
    }
}

impl LayerKey for &String {
    fn layer_name(&self) -> String {
        (*self).clone()
    }
}

/// One item to write when exporting a storyboard, in draw order.
pub enum ExportItem<'a> {
    /// A sprite on an osu! layer.
    Sprite {
        layer: OsbLayer,
        sprite: &'a OsbSprite,
    },
    /// A sound sample on an osu! layer.
    Sample {
        layer: OsbLayer,
        sample: &'a OsbSample,
    },
}

/// The mutable storyboard an effect builds: a tree of layers/segments and their
/// sprites/samples.
pub struct Storyboard {
    segments: Vec<Layer>,
    roots: Vec<usize>,
    layer_index: HashMap<String, usize>,
}

impl Storyboard {
    pub fn new() -> Self {
        let mut segments = Vec::new();
        let mut roots = Vec::new();
        let mut layer_index = HashMap::new();
        // The five standard layers exist up front so they keep their z-order even
        // when a script only ever retrieves one of them.
        for layer in OsbLayer::all() {
            let id = segments.len();
            let name = layer.name().to_string();
            segments.push(Layer::named(name.clone(), layer));
            layer_index.insert(name, id);
            roots.push(id);
        }
        Storyboard {
            segments,
            roots,
            layer_index,
        }
    }

    /// Returns a layer by [`OsbLayer`] or by name,
    /// creating a named layer on first use.
    pub fn layer<L: LayerKey>(&mut self, key: L) -> &mut Layer {
        let index = self.ensure_layer(key);
        &mut self.segments[index]
    }

    /// Resolves a layer to its index, creating a named layer on first use.
    pub fn ensure_layer<L: LayerKey>(&mut self, key: L) -> usize {
        let name = key.layer_name();
        if let Some(&index) = self.layer_index.get(&name) {
            return index;
        }
        let osb_layer = OsbLayer::from_name(&name).unwrap_or_default();
        let index = self.segments.len();
        self.segments.push(Layer::named(name.clone(), osb_layer));
        self.layer_index.insert(name, index);
        self.roots.push(index);
        index
    }

    /// Returns a non-borrowing [`LayerHandle`] for a layer held in a variable.
    pub fn layer_handle<L: LayerKey>(&mut self, key: L) -> LayerHandle {
        LayerHandle(self.ensure_layer(key))
    }

    // -- segment API (the `StoryboardSegment` surface) ----------------------

    /// Creates an anonymous segment nested in `parent` (`CreateSegment`).
    pub fn create_segment(&mut self, parent: LayerHandle) -> LayerHandle {
        let parent_id = parent.0;
        let osb_layer = self.segments[parent_id].osb_layer();
        let id = self.segments.len();
        self.segments.push(Layer::child(parent_id, osb_layer));
        self.segments[parent_id].push_child(id);
        LayerHandle(id)
    }

    /// Borrows a segment by handle.
    pub fn segment(&self, handle: LayerHandle) -> &Layer {
        &self.segments[handle.0]
    }

    /// Mutably borrows a segment by handle.
    pub fn segment_mut(&mut self, handle: LayerHandle) -> &mut Layer {
        &mut self.segments[handle.0]
    }

    /// Moves a segment to a different osu! layer (`ChangePosition`).
    pub fn change_position(&mut self, handle: LayerHandle, osb_layer: OsbLayer) {
        self.segments[handle.0].set_osb_layer(osb_layer);
    }

    /// Moves a segment last among its siblings so it draws on top (`MoveToTop`).
    pub fn move_to_top(&mut self, handle: LayerHandle) {
        self.reorder(handle.0, true);
    }

    /// Moves a segment first among its siblings so it draws below (`MoveToBottom`).
    pub fn move_to_bottom(&mut self, handle: LayerHandle) {
        self.reorder(handle.0, false);
    }

    fn reorder(&mut self, id: usize, top: bool) {
        let parent = self.segments[id].parent();
        let list = match parent {
            Some(parent) => self.segments[parent].children_mut(),
            None => &mut self.roots,
        };
        if let Some(position) = list.iter().position(|&entry| entry == id) {
            let item = list.remove(position);
            if top {
                list.push(item);
            } else {
                list.insert(0, item);
            }
        }
    }

    /// Removes a segment (and its contents) from the storyboard (`Discard`),
    /// returning the number of remaining top-level layers.
    pub fn discard(&mut self, handle: LayerHandle) -> usize {
        let id = handle.0;
        let parent = self.segments[id].parent();
        let list = match parent {
            Some(parent) => self.segments[parent].children_mut(),
            None => &mut self.roots,
        };
        if let Some(position) = list.iter().position(|&entry| entry == id) {
            list.remove(position);
        }
        self.layer_index.retain(|_, &mut value| value != id);
        self.segments[id] = Layer::default();
        self.roots.len()
    }

    /// Removes a sprite from output (`DiscardSprite`), returning whether it existed.
    pub fn discard_sprite(&mut self, handle: SpriteHandle) -> bool {
        self.segments[handle.layer].discard_sprite(handle.sprite)
    }

    /// Removes zero-duration sprites from a segment (`Clean`), returning the count.
    pub fn clean(&mut self, handle: LayerHandle) -> usize {
        self.segments[handle.0].clean()
    }

    // -- creation forwarders ------------------------------------------------

    /// Creates a sprite on `target` and returns a non-borrowing handle.
    pub fn create_sprite<T: SpriteTarget>(
        &mut self,
        target: T,
        texture_path: impl Into<String>,
        origin: OsbOrigin,
    ) -> SpriteHandle {
        let layer = target.resolve(self);
        let sprite = self.segments[layer].len();
        self.segments[layer].create_sprite(texture_path, origin);
        SpriteHandle { layer, sprite }
    }

    /// Creates a positioned sprite and returns a non-borrowing handle.
    pub fn create_sprite_at<T: SpriteTarget>(
        &mut self,
        target: T,
        texture_path: impl Into<String>,
        origin: OsbOrigin,
        position: crate::math::Vec2,
    ) -> SpriteHandle {
        let layer = target.resolve(self);
        let sprite = self.segments[layer].len();
        self.segments[layer].create_sprite_at(texture_path, origin, position);
        SpriteHandle { layer, sprite }
    }

    /// Creates an animation and returns a non-borrowing handle.
    #[allow(clippy::too_many_arguments)]
    pub fn create_animation<T: SpriteTarget>(
        &mut self,
        target: T,
        texture_path: impl Into<String>,
        frame_count: i32,
        frame_delay: f64,
        loop_type: OsbLoopType,
        origin: OsbOrigin,
        position: crate::math::Vec2,
    ) -> SpriteHandle {
        let layer = target.resolve(self);
        let sprite = self.segments[layer].len();
        self.segments[layer].create_animation(
            texture_path,
            frame_count,
            frame_delay,
            loop_type,
            origin,
            position,
        );
        SpriteHandle { layer, sprite }
    }

    /// Borrows a sprite by handle.
    pub fn sprite(&self, handle: SpriteHandle) -> &OsbSprite {
        &self.segments[handle.layer].sprites()[handle.sprite]
    }

    /// Mutably borrows a sprite by handle.
    pub fn sprite_mut(&mut self, handle: SpriteHandle) -> &mut OsbSprite {
        self.segments[handle.layer]
            .sprite_mut(handle.sprite)
            .expect("sprite handle is valid for the storyboard that produced it")
    }

    // Command forwarders used by generated code for a sprite held as a
    // [`SpriteHandle`]. The receiver is the storyboard itself, so a command whose
    // argument reads the same sprite (`scale(h, sprite(h).scale_at(t).x, ..)`)
    // borrows it once as the receiver (`vec.push(vec.len())` two-phase) instead
    // of through a `&mut OsbSprite` reference.
    pub fn move_(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Vec2,
        end: Vec2,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .move_(start_time, end_time, start, end, easing);
    }

    pub fn move_x(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: f64,
        end: f64,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .move_x(start_time, end_time, start, end, easing);
    }

    pub fn move_y(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: f64,
        end: f64,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .move_y(start_time, end_time, start, end, easing);
    }

    pub fn scale(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: f64,
        end: f64,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .scale(start_time, end_time, start, end, easing);
    }

    pub fn scale_vec(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Vec2,
        end: Vec2,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .scale_vec(start_time, end_time, start, end, easing);
    }

    pub fn rotate(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: f64,
        end: f64,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .rotate(start_time, end_time, start, end, easing);
    }

    pub fn fade(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: f64,
        end: f64,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .fade(start_time, end_time, start, end, easing);
    }

    pub fn color(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Color4,
        end: Color4,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .color(start_time, end_time, start, end, easing);
    }

    pub fn color_hsb(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Color4,
        end: Color4,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .color_hsb(start_time, end_time, start, end, easing);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn parameter(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        parameter: ParameterType,
        start: f64,
        end: f64,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .parameter(start_time, end_time, parameter, start, end, easing);
    }

    /// Animates a sprite's origin offset (`Anchor`).
    pub fn anchor(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Vec2,
        end: Vec2,
        easing: Easing,
    ) {
        self.sprite_mut(handle)
            .anchor(start_time, end_time, start, end, easing);
    }

    pub fn start_loop_group(
        &mut self,
        handle: SpriteHandle,
        start_time: impl Into<f64>,
        count: impl Into<f64>,
    ) {
        self.sprite_mut(handle).start_loop_group(start_time, count);
    }
    pub fn end_group(&mut self, handle: SpriteHandle) {
        self.sprite_mut(handle).end_group();
    }

    pub fn start_trigger_group(
        &mut self,
        handle: SpriteHandle,
        name: impl Into<String>,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        group: i32,
    ) {
        self.sprite_mut(handle)
            .start_trigger_group(name, start_time, end_time, group);
    }

    // -- iteration ----------------------------------------------------------

    /// The segments in draw order (each top-level layer followed by its nested
    /// segments).
    pub fn layers(&self) -> impl Iterator<Item = &Layer> {
        let segments = &self.segments;
        self.draw_order().into_iter().map(move |id| &segments[id])
    }

    /// The storyboard's segments with their osu! layer name, in z-order.
    pub fn named_layers(&self) -> impl Iterator<Item = (&'static str, &Layer)> {
        let segments = &self.segments;
        self.draw_order()
            .into_iter()
            .map(move |id| (segments[id].osb_layer().name(), &segments[id]))
    }

    /// Visits the storyboard's segments with their osu! layer name, in z-order,
    /// mutably.
    pub fn for_each_layer_mut<F: FnMut(&'static str, &mut Layer)>(&mut self, mut visit: F) {
        for id in self.draw_order() {
            let name = self.segments[id].osb_layer().name();
            visit(name, &mut self.segments[id]);
        }
    }

    /// The export units (sprites and samples) in draw order, excluding discarded
    /// sprites.
    pub fn export_items(&self) -> Vec<ExportItem<'_>> {
        let mut out = Vec::new();
        for &root in &self.roots {
            self.collect_export(root, &mut out);
        }
        out
    }

    fn collect_export<'a>(&'a self, id: usize, out: &mut Vec<ExportItem<'a>>) {
        let segment = &self.segments[id];
        for entry in segment.ordered_entries() {
            match entry {
                Entry::Sprite(index) => {
                    let sprite = &segment.sprites()[index];
                    if !sprite.exclude {
                        out.push(ExportItem::Sprite {
                            layer: segment.osb_layer(),
                            sprite,
                        });
                    }
                }
                Entry::Sample(index) => out.push(ExportItem::Sample {
                    layer: segment.osb_layer(),
                    sample: &segment.samples()[index],
                }),
                Entry::Segment(child) => self.collect_export(child, out),
            }
        }
    }

    fn draw_order(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for &root in &self.roots {
            self.collect_order(root, &mut out);
        }
        out
    }

    fn collect_order(&self, id: usize, out: &mut Vec<usize>) {
        out.push(id);
        let segment = &self.segments[id];
        let children: Vec<usize> = if segment.reverse_depth() {
            segment.children().iter().rev().copied().collect()
        } else {
            segment.children().to_vec()
        };
        for child in children {
            self.collect_order(child, out);
        }
    }

    /// The number of sprites on the storyboard (excluding discarded sprites).
    pub fn sprite_count(&self) -> usize {
        self.segments
            .iter()
            .flat_map(Layer::sprites)
            .filter(|sprite| !sprite.exclude)
            .count()
    }

    /// The number of top-level layers.
    pub fn layer_count(&self) -> usize {
        self.roots.len()
    }
}

impl Default for Storyboard {
    fn default() -> Self {
        Storyboard::new()
    }
}

/// A layer receiver for sprite/animation creation: a [`LayerHandle`] or any
/// [`LayerKey`] (a name or [`OsbLayer`]).
pub trait SpriteTarget {
    fn resolve(self, storyboard: &mut Storyboard) -> usize;
}

impl SpriteTarget for LayerHandle {
    fn resolve(self, _storyboard: &mut Storyboard) -> usize {
        self.0
    }
}

impl SpriteTarget for OsbLayer {
    fn resolve(self, storyboard: &mut Storyboard) -> usize {
        storyboard.ensure_layer(self)
    }
}

impl SpriteTarget for &str {
    fn resolve(self, storyboard: &mut Storyboard) -> usize {
        storyboard.ensure_layer(self)
    }
}

impl SpriteTarget for String {
    fn resolve(self, storyboard: &mut Storyboard) -> usize {
        storyboard.ensure_layer(self)
    }
}

impl SpriteTarget for &String {
    fn resolve(self, storyboard: &mut Storyboard) -> usize {
        storyboard.ensure_layer(self)
    }
}

/// A borrowed authoring view over a working [`Storyboard`] and the read-mostly
/// [`Context`] - what an effect's `Generate()` writes into.
///
/// The two halves are separate references, so an effect can hold sprite
/// references (borrowing the storyboard) while it reads the beatmap or audio.
/// The storyboard is owned by the caller (the host), not by this view.
pub struct Runtime<'a> {
    pub storyboard: &'a mut Storyboard,
    pub context: &'a mut Context,
}

impl<'a> Runtime<'a> {
    /// Builds a view over a storyboard and a context.
    pub fn new(storyboard: &'a mut Storyboard, context: &'a mut Context) -> Self {
        Runtime {
            storyboard,
            context,
        }
    }

    pub fn layer<L: LayerKey>(&mut self, key: L) -> &mut Layer {
        self.storyboard.layer(key)
    }

    pub fn layers(&self) -> impl Iterator<Item = &Layer> {
        self.storyboard.layers()
    }

    pub fn sprite_count(&self) -> usize {
        self.storyboard.sprite_count()
    }

    pub fn random(&mut self, min: f64, max: f64) -> f64 {
        self.context.random(min, max)
    }

    pub fn beatmap(&self) -> Option<&crate::mapset::Beatmap> {
        self.context.beatmap()
    }

    pub fn audio_duration(&self) -> f64 {
        self.context.audio_duration()
    }

    /// Imports a `.osb` storyboard into the working storyboard.
    ///
    /// `path_or_text` is treated as a file path relative to the asset root
    /// unless it contains a newline, in which case it is the `.osb` text itself.
    pub fn import_osb(&mut self, path_or_text: impl AsRef<str>) {
        let input = path_or_text.as_ref();
        let text = if input.contains('\n') {
            Some(input.to_string())
        } else {
            self.context.read_text(input)
        };
        if let Some(text) = text {
            super::import::import_osb(&mut *self.storyboard, &text);
        }
    }
}

/// An effect: the compiled form of a `Generate()` override.
///
/// `Generate()`'s only output is the storyboard it authors, so `generate`
/// returns it. It is handed the read-mostly [`Context`] (beatmap, audio, RNG,
/// resolution) and returns a freshly built [`Storyboard`]; use
/// [`Runtime::new`] for a view that builds into a storyboard of your choosing.
pub trait Effect {
    fn generate(&mut self, context: &mut Context) -> Storyboard;

    /// The host-facing parameter declarations for this effect.
    fn parameters(&self) -> Vec<crate::params::ParameterDescriptor> {
        Vec::new()
    }

    /// Applies a host parameter value (by configurable name) before the storyboard
    /// is built. Generated effects override this to set their fields.
    fn set_parameter(&mut self, _name: &str, _value: &crate::params::ParameterValue) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storyboarding::OsbOrigin;

    /// The two halves a host owns (context in, storyboard out); the test body
    /// builds a `Runtime` view over them.
    fn runtime(width: f32, height: f32) -> (Storyboard, Context) {
        (Storyboard::new(), Context::new(width, height))
    }

    #[test]
    fn builds_layers_and_sprites() {
        let (mut storyboard, mut context) = runtime(1920.0, 1080.0);
        let mut rt = Runtime::new(&mut storyboard, &mut context);
        rt.layer(OsbLayer::Background)
            .create_sprite("sb/bg.png", OsbOrigin::Centre);
        rt.layer(OsbLayer::Foreground)
            .create_sprite("sb/fg.png", OsbOrigin::Centre);
        assert_eq!(rt.sprite_count(), 2);
        assert_eq!(rt.layer(OsbLayer::Background).len(), 1);
    }

    #[test]
    fn holds_sprite_while_reading_context() {
        let (mut storyboard, mut context) = runtime(1920.0, 1080.0);
        context.set_beatmap(crate::mapset::Beatmap::new());
        let rt = Runtime::new(&mut storyboard, &mut context);
        // The storyboard and context borrows must be disjoint.
        let sprite = rt
            .storyboard
            .layer("")
            .create_sprite("sb/x.png", OsbOrigin::Centre);
        sprite.fade(0.0, 100.0, 0.0, 1.0, crate::animations::Easing::None);
        let _ = rt.context.beatmap();
        assert_eq!(rt.storyboard.sprite_count(), 1);
    }

    #[test]
    fn imports_inline_osb_text() {
        let (mut storyboard, mut context) = runtime(1920.0, 1080.0);
        let mut rt = Runtime::new(&mut storyboard, &mut context);
        // A newline means the argument is `.osb` text rather than a file path.
        rt.import_osb("[Events]\nSprite,Pass,Centre,\"sb/x.png\",10,20\n F,0,0,1,0,1\n");
        assert_eq!(rt.storyboard.layer(OsbLayer::Pass).len(), 1);
        assert_eq!(
            rt.storyboard.layer(OsbLayer::Pass).sprites()[0].texture_path,
            "sb/x.png"
        );
    }

    #[test]
    fn sprite_handles_do_not_borrow_the_storyboard() {
        let (mut storyboard, mut context) = runtime(1920.0, 1080.0);
        let rt = Runtime::new(&mut storyboard, &mut context);
        let dot = rt.storyboard.create_sprite_at(
            "Dot",
            "sb/x.png",
            OsbOrigin::Centre,
            crate::math::Vec2::new(0.0, 0.0),
        );
        // A command whose argument reads the same sprite borrows the storyboard
        // only as the receiver (the handle decouples the sprite).
        let start = rt.storyboard.sprite(dot).position_at(0.0);
        rt.storyboard.move_(
            dot,
            0.0,
            100.0,
            start,
            crate::math::Vec2::new(10.0, 0.0),
            crate::animations::Easing::None,
        );
        assert!((rt.storyboard.sprite(dot).position_at(50.0).x - 5.0).abs() < 1e-4);
    }

    #[test]
    fn layer_handles_create_sprites() {
        let (mut storyboard, mut context) = runtime(1920.0, 1080.0);
        let rt = Runtime::new(&mut storyboard, &mut context);
        let layer = rt.storyboard.layer_handle("Spectrum");
        let bar = rt.storyboard.create_sprite_at(
            layer,
            "sb/bar.png",
            OsbOrigin::BottomLeft,
            crate::math::Vec2::new(0.0, 0.0),
        );
        assert_eq!(rt.storyboard.sprite(bar).texture_path, "sb/bar.png");
    }

    #[test]
    fn segments_nest_and_reorder() {
        let (mut storyboard, mut context) = runtime(1920.0, 1080.0);
        let rt = Runtime::new(&mut storyboard, &mut context);
        let group = rt.storyboard.layer_handle("Group");
        rt.storyboard
            .create_sprite(group, "sb/group.png", OsbOrigin::Centre);
        let child = rt.storyboard.create_segment(group);
        rt.storyboard
            .create_sprite(child, "sb/child.png", OsbOrigin::Centre);
        assert_eq!(rt.storyboard.sprite_count(), 2);
        // The child is visited right after its parent in draw order.
        let ids: Vec<_> = rt
            .storyboard
            .layers()
            .map(|layer| layer.identifier().to_string())
            .collect();
        assert_eq!(
            &ids[..6],
            &[
                "Background",
                "Fail",
                "Pass",
                "Foreground",
                "Overlay",
                "Group"
            ]
        );
        assert_eq!(ids.len(), 7);

        rt.storyboard.segment_mut(group).set_reverse_depth(true);
        assert!(rt.storyboard.segment(group).reverse_depth());
        rt.storyboard.move_to_top(LayerHandle(0));
        assert_eq!(rt.storyboard.layer_count(), 6);
    }

    #[test]
    fn clean_and_discard() {
        let (mut storyboard, mut context) = runtime(1920.0, 1080.0);
        let rt = Runtime::new(&mut storyboard, &mut context);
        let layer = rt.storyboard.layer_handle("Foreground");
        let keep = rt
            .storyboard
            .create_sprite(layer, "sb/a.png", OsbOrigin::Centre);
        rt.storyboard
            .fade(keep, 0.0, 100.0, 0.0, 1.0, crate::animations::Easing::None);
        rt.storyboard
            .create_sprite(layer, "sb/b.png", OsbOrigin::Centre);
        assert_eq!(rt.storyboard.clean(layer), 1);
        assert_eq!(rt.storyboard.segment(layer).len(), 1);
        assert_eq!(rt.storyboard.sprite_count(), 1);

        let discard = rt
            .storyboard
            .create_sprite(layer, "sb/c.png", OsbOrigin::Centre);
        assert_eq!(rt.storyboard.sprite_count(), 2);
        assert!(rt.storyboard.discard_sprite(discard));
        assert_eq!(rt.storyboard.sprite_count(), 1);

        let child = rt.storyboard.create_segment(layer);
        rt.storyboard
            .create_sprite(child, "sb/d.png", OsbOrigin::Centre);
        assert_eq!(rt.storyboard.sprite_count(), 2);
        rt.storyboard.discard(child);
        assert_eq!(rt.storyboard.sprite_count(), 1);
    }

    #[test]
    fn layers_and_indexes() {
        let (mut storyboard, mut context) = runtime(1920.0, 1080.0);
        let rt = Runtime::new(&mut storyboard, &mut context);
        let group = rt.storyboard.layer_handle("Group");
        rt.storyboard.segment_mut(group).add_index(4.0);
        rt.storyboard.segment_mut(group).add_index(-1.0);
        let segment = rt.storyboard.segment(group);
        assert_eq!(segment.lowest_index(), -1.0);
        assert_eq!(segment.highest_index(), 4.0);
        assert_eq!(segment.indexes(), &[4.0, -1.0][..]);
        rt.storyboard.change_position(group, OsbLayer::Overlay);
        assert_eq!(rt.storyboard.segment(group).osb_layer(), OsbLayer::Overlay);
    }
}
