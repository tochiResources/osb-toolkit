//! OpenTK/osuTK-compatible math types, shared by the engine libraries
//! (`osb-curve`, `osb-runtime`, ...). Re-exported as `osb_runtime::math` so the
//! existing paths keep working.

mod box2;
mod color4;
mod functions;
mod math_helper;
mod matrix4;
mod quaternion;
mod vec2;
mod vec3;
mod vec4;

pub use box2::Box2;
pub use color4::Color4;
pub use functions::*;
pub use math_helper::MathHelper;
pub use matrix4::Matrix4;
pub use quaternion::Quaternion;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;
