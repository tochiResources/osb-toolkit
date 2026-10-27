use super::sprite::{AnimationMeta, OsbSprite};
use super::types::{OsbLayer, OsbLoopType, OsbOrigin};

/// A sound sample attached to a segment.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OsbSample {
    pub path: String,
    pub time: f64,
    pub volume: f64,
}

/// One ordered child of a segment: a sprite, a sample or a nested segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Entry {
    Sprite(usize),
    Sample(usize),
    Segment(usize),
}

/// A storyboard segment: an ordered container of sprites, samples and nested
/// segments, with depth-ordering metadata.
///
/// The five standard layers are top-level segments; `GetLayer`/anonymous
/// `CreateSegment` calls add more. A segment can be nested inside another and is
/// emitted after its parent unless [`reverse_depth`](Self::reverse_depth) is set.
#[derive(Debug, Clone, Default)]
pub struct Layer {
    identifier: String,
    osb_layer: OsbLayer,
    parent: Option<usize>,
    children: Vec<usize>,
    sprites: Vec<OsbSprite>,
    samples: Vec<OsbSample>,
    entries: Vec<Entry>,
    indexes: Vec<f64>,
    lowest_index: f64,
    highest_index: f64,
    reverse_depth: bool,
}

impl Layer {
    /// A named top-level layer (a standard osu! layer or a `GetLayer` name).
    pub fn named(identifier: impl Into<String>, osb_layer: OsbLayer) -> Self {
        Layer {
            identifier: identifier.into(),
            osb_layer,
            ..Layer::default()
        }
    }

    /// An anonymous child segment owned by `parent`.
    pub fn child(parent: usize, osb_layer: OsbLayer) -> Self {
        Layer {
            osb_layer,
            parent: Some(parent),
            ..Layer::default()
        }
    }

    /// The layer/segment identifier (`""` for an anonymous segment).
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// The osu! layer this segment belongs to.
    pub fn osb_layer(&self) -> OsbLayer {
        self.osb_layer
    }

    /// Sets the osu! layer this segment belongs to (`ChangePosition`).
    pub fn set_osb_layer(&mut self, osb_layer: OsbLayer) {
        self.osb_layer = osb_layer;
    }

    /// The parent segment id, or `None` for a top-level layer.
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    /// The ids of this segment's nested child segments, in draw order.
    pub fn children(&self) -> &[usize] {
        &self.children
    }

    pub fn create_sprite(
        &mut self,
        texture_path: impl Into<String>,
        origin: OsbOrigin,
    ) -> &mut OsbSprite {
        self.sprites.push(OsbSprite::new(texture_path, origin));
        let index = self.sprites.len() - 1;
        self.entries.push(Entry::Sprite(index));
        self.sprites.last_mut().unwrap()
    }

    /// Creates a sprite at an initial position.
    pub fn create_sprite_at(
        &mut self,
        texture_path: impl Into<String>,
        origin: OsbOrigin,
        position: crate::math::Vec2,
    ) -> &mut OsbSprite {
        let sprite = self.create_sprite(texture_path, origin);
        sprite.initial_position = position;
        sprite
    }

    /// Creates an animation: a sprite carrying frame-cycling metadata.
    pub fn create_animation(
        &mut self,
        texture_path: impl Into<String>,
        frame_count: i32,
        frame_delay: f64,
        loop_type: OsbLoopType,
        origin: OsbOrigin,
        position: crate::math::Vec2,
    ) -> &mut OsbSprite {
        let sprite = self.create_sprite_at(texture_path, origin, position);
        sprite.animation = Some(AnimationMeta {
            frame_count,
            frame_delay,
            loop_type,
        });
        sprite
    }

    /// Adds a sound sample.
    pub fn create_sample(
        &mut self,
        path: impl Into<String>,
        time: f64,
        volume: f64,
    ) -> &mut OsbSample {
        self.samples.push(OsbSample {
            path: path.into(),
            time,
            volume,
        });
        let index = self.samples.len() - 1;
        self.entries.push(Entry::Sample(index));
        self.samples.last_mut().unwrap()
    }

    pub fn add_sprite(&mut self, sprite: OsbSprite) -> &mut OsbSprite {
        self.sprites.push(sprite);
        let index = self.sprites.len() - 1;
        self.entries.push(Entry::Sprite(index));
        self.sprites.last_mut().unwrap()
    }

    pub fn sprites(&self) -> &[OsbSprite] {
        &self.sprites
    }

    /// Mutable access to the layer's sprites, in draw order.
    pub fn sprites_mut(&mut self) -> &mut [OsbSprite] {
        &mut self.sprites
    }

    /// Mutable access to a sprite by index (`create_sprite` order), used by the
    /// `.osb` importer to add commands without holding a long-lived borrow.
    pub fn sprite_mut(&mut self, index: usize) -> Option<&mut OsbSprite> {
        self.sprites.get_mut(index)
    }

    pub fn samples(&self) -> &[OsbSample] {
        &self.samples
    }

    pub fn len(&self) -> usize {
        self.sprites.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sprites.is_empty()
    }

    /// Whether this segment's children are emitted in reverse order.
    pub fn reverse_depth(&self) -> bool {
        self.reverse_depth
    }

    /// Sets [`reverse_depth`](Self::reverse_depth).
    pub fn set_reverse_depth(&mut self, value: bool) {
        self.reverse_depth = value;
    }

    /// The segment's depth indexes, in insertion order.
    pub fn indexes(&self) -> &[f64] {
        &self.indexes
    }

    /// Records a depth index and updates the lowest/highest bounds.
    pub fn add_index(&mut self, index: f64) {
        self.indexes.push(index);
        self.lowest_index = self.indexes.iter().copied().fold(f64::INFINITY, f64::min);
        self.highest_index = self
            .indexes
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
    }

    pub fn lowest_index(&self) -> f64 {
        self.lowest_index
    }

    pub fn highest_index(&self) -> f64 {
        self.highest_index
    }

    /// Removes zero-duration sprites (`end - start <= 2ms`), returning how many
    /// were dropped. Sprites on a "Delete Beatmap BG" layer are kept.
    pub fn clean(&mut self) -> usize {
        if self.identifier.contains("Delete Beatmap BG") {
            return 0;
        }
        let mut removed = 0;
        let mut remap = vec![None; self.sprites.len()];
        let mut retained = Vec::with_capacity(self.sprites.len());
        for (index, sprite) in self.sprites.drain(..).enumerate() {
            let (start, end) = sprite.time_span();
            if end - start <= 2.0 {
                removed += 1;
            } else {
                remap[index] = Some(retained.len());
                retained.push(sprite);
            }
        }
        self.sprites = retained;
        self.entries = self
            .entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Sprite(index) => remap[*index].map(Entry::Sprite),
                other => Some(*other),
            })
            .collect();
        removed
    }

    /// Sets the excluded flag on a sprite (removing it from output), returning
    /// whether the index was valid.
    pub fn discard_sprite(&mut self, index: usize) -> bool {
        match self.sprites.get_mut(index) {
            Some(sprite) => {
                sprite.exclude = true;
                true
            }
            None => false,
        }
    }

    pub(crate) fn push_child(&mut self, id: usize) {
        self.children.push(id);
        self.entries.push(Entry::Segment(id));
    }

    pub(crate) fn children_mut(&mut self) -> &mut Vec<usize> {
        &mut self.children
    }

    /// This segment's ordered children (`reverse_depth` applied).
    pub(crate) fn ordered_entries(&self) -> impl Iterator<Item = Entry> + '_ {
        let entries: Box<dyn Iterator<Item = &Entry>> = if self.reverse_depth {
            Box::new(self.entries.iter().rev())
        } else {
            Box::new(self.entries.iter())
        };
        entries.copied()
    }
}
