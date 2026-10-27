//! Wire math types, re-exported from the engine (`osb-core::math`, the source of
//! truth) under their contract names (`Vec2` -> `Vector2`, `Vec3` -> `Vector3`,
//! `Vec4` -> `Vector4`). The schema names stay `Vector2/3/4` via `#[schema(as)]`
//! on the engine types.

pub use osb_runtime::math::{
    Box2, Color4, Matrix4, Quaternion, Vec2 as Vector2, Vec3 as Vector3, Vec4 as Vector4,
};
