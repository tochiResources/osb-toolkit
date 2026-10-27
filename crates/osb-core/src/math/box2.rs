use super::Vec2;

// An axis-aligned bounding box, matching OpenTK's `Box2`.
/// An axis-aligned 2D bounding box.
#[derive(
    Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
pub struct Box2 {
    /// The minimum corner.
    pub min: Vec2,
    /// The maximum corner.
    pub max: Vec2,
}

impl Box2 {
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Box2 { min, max }
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half = size * 0.5;
        Box2::new(center - half, center + half)
    }

    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}
