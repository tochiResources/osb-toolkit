//! The generic stateless `math` operation (M34): dispatches to the full
//! `osb-math` helper surface (vectors, matrices, quaternions, boxes, colors and
//! scalars) by name, so the math SDK can expose every helper without a wire op
//! each. Curve helpers stay on their own typed ops (`Curve.length`, ...).

use osb_api::MathRequest;
use osb_runtime::math::{
    clamp, degrees_to_radians, lerp, radians_to_degrees, round_to, Box2, Color4, Matrix4,
    Quaternion, Vec2, Vec3, Vec4,
};
use serde_json::{json, Value};

use crate::error_json;

/// Runs a `math` request, returning its JSON result.
pub fn math_op(value: &Value) -> String {
    let request: MathRequest = match serde_json::from_value(value.clone()) {
        Ok(request) => request,
        Err(error) => return error_json(&error.to_string()),
    };
    match dispatch(&request.name, &request.args) {
        Ok(result) => json!({ "value": result }).to_string(),
        Err(message) => error_json(&message),
    }
}

fn number(value: Option<&Value>) -> Result<f64, String> {
    value
        .and_then(Value::as_f64)
        .ok_or_else(|| "expected a number argument".to_string())
}

fn vec2(value: Option<&Value>) -> Result<Vec2, String> {
    let value = value.ok_or_else(|| "expected a Vector2 argument".to_string())?;
    Ok(Vec2::new(number(value.get("x"))?, number(value.get("y"))?))
}

fn vec3(value: Option<&Value>) -> Result<Vec3, String> {
    let value = value.ok_or_else(|| "expected a Vector3 argument".to_string())?;
    Ok(Vec3::new(
        number(value.get("x"))?,
        number(value.get("y"))?,
        number(value.get("z"))?,
    ))
}

fn vec4(value: Option<&Value>) -> Result<Vec4, String> {
    let value = value.ok_or_else(|| "expected a Vector4 argument".to_string())?;
    Ok(Vec4::new(
        number(value.get("x"))?,
        number(value.get("y"))?,
        number(value.get("z"))?,
        number(value.get("w"))?,
    ))
}

fn quaternion(value: Option<&Value>) -> Result<Quaternion, String> {
    let value = value.ok_or_else(|| "expected a Quaternion argument".to_string())?;
    Ok(Quaternion::new(
        number(value.get("x"))?,
        number(value.get("y"))?,
        number(value.get("z"))?,
        number(value.get("w"))?,
    ))
}

fn matrix4(value: Option<&Value>) -> Result<Matrix4, String> {
    let value = value.ok_or_else(|| "expected a Matrix4 argument".to_string())?;
    let list = value
        .get("m")
        .and_then(Value::as_array)
        .ok_or_else(|| "expected a Matrix4 `m` array".to_string())?;
    let mut m = [0.0f64; 16];
    for (index, element) in list.iter().take(16).enumerate() {
        m[index] = element.as_f64().unwrap_or(0.0);
    }
    Ok(Matrix4 { m })
}

fn box2(value: Option<&Value>) -> Result<Box2, String> {
    let value = value.ok_or_else(|| "expected a Box2 argument".to_string())?;
    Ok(Box2::new(vec2(value.get("min"))?, vec2(value.get("max"))?))
}

fn color4(value: Option<&Value>) -> Result<Color4, String> {
    let value = value.ok_or_else(|| "expected a Color4 argument".to_string())?;
    Ok(Color4::new(
        number(value.get("r"))?,
        number(value.get("g"))?,
        number(value.get("b"))?,
        number(value.get("a"))?,
    ))
}

fn out_vec2(v: Vec2) -> Value {
    json!({ "x": v.x, "y": v.y })
}
fn out_vec3(v: Vec3) -> Value {
    json!({ "x": v.x, "y": v.y, "z": v.z })
}
fn out_vec4(v: Vec4) -> Value {
    json!({ "x": v.x, "y": v.y, "z": v.z, "w": v.w })
}
fn out_quaternion(q: Quaternion) -> Value {
    json!({ "x": q.x, "y": q.y, "z": q.z, "w": q.w })
}
fn out_matrix(m: Matrix4) -> Value {
    json!({ "m": m.m.to_vec() })
}
fn out_color(c: Color4) -> Value {
    json!({ "r": c.r, "g": c.g, "b": c.b, "a": c.a })
}

/// Dispatches a math function by name. Names are grouped `vector.*`, `matrix.*`,
/// `quaternion.*`, `box.*`, `color.*` and `scalar.*`.
#[allow(clippy::too_many_lines)]
fn dispatch(name: &str, args: &[Value]) -> Result<Value, String> {
    let arg = |index: usize| args.get(index);
    Ok(match name {
        // -- scalars -------------------------------------------------------
        "scalar.clamp" => json!(clamp(number(arg(0))?, number(arg(1))?, number(arg(2))?)),
        "scalar.clamp01" => json!(clamp(number(arg(0))?, 0.0, 1.0)),
        "scalar.lerp" => json!(lerp(number(arg(0))?, number(arg(1))?, number(arg(2))?)),
        "scalar.mapRange" => {
            let (value, in_min, in_max, out_min, out_max) = (
                number(arg(0))?,
                number(arg(1))?,
                number(arg(2))?,
                number(arg(3))?,
                number(arg(4))?,
            );
            let t = if in_max == in_min {
                0.0
            } else {
                (value - in_min) / (in_max - in_min)
            };
            json!(out_min + t * (out_max - out_min))
        }
        "scalar.smoothstep" => {
            let (edge0, edge1, x) = (number(arg(0))?, number(arg(1))?, number(arg(2))?);
            let t = if edge1 == edge0 {
                0.0
            } else {
                clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0)
            };
            json!(t * t * (3.0 - 2.0 * t))
        }
        "scalar.degrees" => json!(radians_to_degrees(number(arg(0))?)),
        "scalar.radians" => json!(degrees_to_radians(number(arg(0))?)),
        "scalar.roundTo" => json!(round_to(number(arg(0))?, number(arg(1))?)),
        "scalar.sign" => json!(number(arg(0))?.signum()),
        "scalar.hypot" => json!(number(arg(0))?.hypot(number(arg(1))?)),
        "scalar.min" => json!(number(arg(0))?.min(number(arg(1))?)),
        "scalar.max" => json!(number(arg(0))?.max(number(arg(1))?)),
        "scalar.pow" => json!(number(arg(0))?.powf(number(arg(1))?)),
        "scalar.abs" => json!(number(arg(0))?.abs()),
        "scalar.floor" => json!(number(arg(0))?.floor()),
        "scalar.ceil" => json!(number(arg(0))?.ceil()),
        "scalar.round" => json!(number(arg(0))?.round()),
        "scalar.sqrt" => json!(number(arg(0))?.sqrt()),

        // -- vec2 ----------------------------------------------------------
        "vec2.add" => out_vec2(vec2(arg(0))? + vec2(arg(1))?),
        "vec2.subtract" => out_vec2(vec2(arg(0))? - vec2(arg(1))?),
        "vec2.multiply" => {
            let (a, b) = (vec2(arg(0))?, vec2(arg(1))?);
            out_vec2(Vec2::new(a.x * b.x, a.y * b.y))
        }
        "vec2.scale" => out_vec2(vec2(arg(0))? * number(arg(1))?),
        "vec2.dot" => json!(vec2(arg(0))?.dot(vec2(arg(1))?)),
        "vec2.length" => json!(vec2(arg(0))?.length()),
        "vec2.lengthSquared" => json!(vec2(arg(0))?.length_squared()),
        "vec2.normalize" => out_vec2(vec2(arg(0))?.normalized()),
        "vec2.distance" => json!(vec2(arg(0))?.distance(vec2(arg(1))?)),
        "vec2.lerp" => out_vec2(vec2(arg(0))?.lerp(vec2(arg(1))?, number(arg(2))?)),
        "vec2.perpendicular" => out_vec2(vec2(arg(0))?.perpendicular()),
        "vec2.negate" => out_vec2(Vec2::ZERO - vec2(arg(0))?),
        "vec2.min" => {
            let (a, b) = (vec2(arg(0))?, vec2(arg(1))?);
            out_vec2(Vec2::new(a.x.min(b.x), a.y.min(b.y)))
        }
        "vec2.max" => {
            let (a, b) = (vec2(arg(0))?, vec2(arg(1))?);
            out_vec2(Vec2::new(a.x.max(b.x), a.y.max(b.y)))
        }
        "vec2.reflect" => {
            let (vector, normal) = (vec2(arg(0))?, vec2(arg(1))?);
            out_vec2(vector - normal * (2.0 * vector.dot(normal)))
        }

        // -- vec3 ----------------------------------------------------------
        "vec3.add" => out_vec3(vec3(arg(0))? + vec3(arg(1))?),
        "vec3.subtract" => out_vec3(vec3(arg(0))? - vec3(arg(1))?),
        "vec3.multiply" => {
            let (a, b) = (vec3(arg(0))?, vec3(arg(1))?);
            out_vec3(Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z))
        }
        "vec3.scale" => out_vec3(vec3(arg(0))? * number(arg(1))?),
        "vec3.dot" => json!(vec3(arg(0))?.dot(vec3(arg(1))?)),
        "vec3.cross" => out_vec3(vec3(arg(0))?.cross(vec3(arg(1))?)),
        "vec3.length" => json!(vec3(arg(0))?.length()),
        "vec3.normalize" => out_vec3(vec3(arg(0))?.normalized()),
        "vec3.lerp" => out_vec3(vec3(arg(0))?.lerp(vec3(arg(1))?, number(arg(2))?)),
        "vec3.negate" => {
            let a = vec3(arg(0))?;
            out_vec3(Vec3::new(-a.x, -a.y, -a.z))
        }
        "vec3.xy" => out_vec2(vec3(arg(0))?.xy()),

        // -- vec4 ----------------------------------------------------------
        "vec4.add" => out_vec4(vec4(arg(0))? + vec4(arg(1))?),
        "vec4.subtract" => out_vec4(vec4(arg(0))? - vec4(arg(1))?),
        "vec4.scale" => out_vec4(vec4(arg(0))? * number(arg(1))?),
        "vec4.dot" => json!(vec4(arg(0))?.dot(vec4(arg(1))?)),
        "vec4.length" => json!(vec4(arg(0))?.length()),
        "vec4.normalize" => out_vec4(vec4(arg(0))?.normalized()),

        // -- matrices ------------------------------------------------------
        "matrix.identity" => out_matrix(Matrix4::identity()),
        "matrix.multiply" => out_matrix(matrix4(arg(0))?.mul(&matrix4(arg(1))?)),
        "matrix.transpose" => {
            let m = matrix4(arg(0))?.m;
            let mut out = [0.0f64; 16];
            for col in 0..4 {
                for row in 0..4 {
                    out[col * 4 + row] = m[row * 4 + col];
                }
            }
            out_matrix(Matrix4 { m: out })
        }
        "matrix.determinant" => {
            let m = matrix4(arg(0))?.m;
            let det = |a: f64, b: f64, c: f64, d: f64| a * d - b * c;
            // 2x2 minors of the upper-left 3x3 expansion.
            let sub = |r1: usize, r2: usize, c1: usize, c2: usize| {
                det(
                    m[c1 * 4 + r1],
                    m[c2 * 4 + r1],
                    m[c1 * 4 + r2],
                    m[c2 * 4 + r2],
                )
            };
            json!(m[0] * sub(1, 2, 1, 2) - m[4] * sub(1, 2, 0, 2) + m[8] * sub(1, 2, 0, 1))
        }
        "matrix.translation" => out_matrix(Matrix4::translation(vec3(arg(0))?)),
        "matrix.scale" => out_matrix(Matrix4::scale(vec3(arg(0))?)),
        "matrix.rotationX" => out_matrix(Matrix4::rotation_x(number(arg(0))?)),
        "matrix.rotationY" => out_matrix(Matrix4::rotation_y(number(arg(0))?)),
        "matrix.rotationZ" => out_matrix(Matrix4::rotation_z(number(arg(0))?)),
        "matrix.fromEuler" => out_matrix(Matrix4::from_euler_xyz(
            number(arg(0))?,
            number(arg(1))?,
            number(arg(2))?,
        )),
        "matrix.fromQuaternion" => out_matrix(quaternion(arg(0))?.to_matrix()),
        "matrix.transformPoint" => {
            let (matrix, point) = (matrix4(arg(0))?, vec3(arg(1))?);
            let (clip, _) = matrix.project(point);
            out_vec3(clip)
        }
        "matrix.transformVector" => {
            let (matrix, vector) = (matrix4(arg(0))?, vec4(arg(1))?);
            out_vec4(matrix.transform_vec4(vector))
        }
        "matrix.lookAt" => out_matrix(Matrix4::look_at(
            vec3(arg(0))?,
            vec3(arg(1))?,
            vec3(arg(2))?,
        )),
        "matrix.perspective" => out_matrix(Matrix4::perspective(
            number(arg(0))?,
            number(arg(1))?,
            number(arg(2))?,
            number(arg(3))?,
        )),
        "matrix.orthographic" => out_matrix(Matrix4::orthographic(
            number(arg(0))?,
            number(arg(1))?,
            number(arg(2))?,
            number(arg(3))?,
            number(arg(4))?,
            number(arg(5))?,
        )),

        // -- quaternions ---------------------------------------------------
        "quaternion.identity" => out_quaternion(Quaternion::IDENTITY),
        "quaternion.fromAxisAngle" => {
            out_quaternion(Quaternion::from_axis_angle(vec3(arg(0))?, number(arg(1))?))
        }
        "quaternion.fromEuler" => out_quaternion(Quaternion::from_euler_angles(
            number(arg(0))?,
            number(arg(1))?,
            number(arg(2))?,
        )),
        "quaternion.length" => json!(quaternion(arg(0))?.length()),
        "quaternion.normalize" => out_quaternion(quaternion(arg(0))?.normalized()),
        "quaternion.toMatrix" => out_matrix(quaternion(arg(0))?.to_matrix()),
        "quaternion.conjugate" => {
            let q = quaternion(arg(0))?;
            out_quaternion(Quaternion::new(-q.x, -q.y, -q.z, q.w))
        }
        "quaternion.dot" => {
            let (a, b) = (quaternion(arg(0))?, quaternion(arg(1))?);
            json!(a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w)
        }
        "quaternion.multiply" => {
            let (a, b) = (quaternion(arg(0))?, quaternion(arg(1))?);
            out_quaternion(Quaternion::new(
                a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y,
                a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x,
                a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w,
                a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
            ))
        }
        "quaternion.rotateVector" => {
            let (q, v) = (quaternion(arg(0))?, vec3(arg(1))?);
            // v' = v + 2 * cross(q.xyz, cross(q.xyz, v) + q.w * v)
            let u = Vec3::new(q.x, q.y, q.z);
            let t = u.cross(v) * 2.0;
            out_vec3(v + u.cross(t) + t * q.w)
        }
        "quaternion.slerp" => {
            let (a, b) = (
                quaternion(arg(0))?.normalized(),
                quaternion(arg(1))?.normalized(),
            );
            let t = number(arg(2))?;
            let mut dot = a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;
            let b = if dot < 0.0 {
                dot = -dot;
                Quaternion::new(-b.x, -b.y, -b.z, -b.w)
            } else {
                b
            };
            if dot > 0.9995 {
                out_quaternion(
                    Quaternion::new(
                        lerp(a.x, b.x, t),
                        lerp(a.y, b.y, t),
                        lerp(a.z, b.z, t),
                        lerp(a.w, b.w, t),
                    )
                    .normalized(),
                )
            } else {
                let theta = dot.clamp(-1.0, 1.0).acos();
                let sin_theta = theta.sin();
                let wa = ((1.0 - t) * theta).sin() / sin_theta;
                let wb = (t * theta).sin() / sin_theta;
                out_quaternion(Quaternion::new(
                    a.x * wa + b.x * wb,
                    a.y * wa + b.y * wb,
                    a.z * wa + b.z * wb,
                    a.w * wa + b.w * wb,
                ))
            }
        }

        // -- boxes ---------------------------------------------------------
        "box.center" => out_vec2(box2(arg(0))?.center()),
        "box.size" => {
            let b = box2(arg(0))?;
            out_vec2(Vec2::new(b.width(), b.height()))
        }
        "box.contains" => json!(box2(arg(0))?.contains(vec2(arg(1))?)),
        "box.fromCenterSize" => out_box(box2_from_center_size(vec2(arg(0))?, vec2(arg(1))?)),

        // -- colors --------------------------------------------------------
        "color.fromHsl" => out_color(Color4::from_hsl(vec4(arg(0))?)),
        "color.fromHsb" => out_color(Color4::from_hsb(
            number(arg(0))?,
            number(arg(1))?,
            number(arg(2))?,
            number(arg(3))?,
        )),
        "color.toHsl" => out_vec4(color4(arg(0))?.to_hsl()),
        "color.lerp" => out_color(color4(arg(0))?.lerp(color4(arg(1))?, number(arg(2))?)),
        "color.multiplyRgb" => out_color(color4(arg(0))?.multiply_rgb(number(arg(1))?)),
        "color.toRgba8" => json!(color4(arg(0))?.to_rgba8()),
        "color.named" => match arg(0).and_then(Value::as_str).unwrap_or("") {
            "White" => out_color(Color4::WHITE),
            "Black" => out_color(Color4::BLACK),
            "Transparent" => out_color(Color4::TRANSPARENT),
            "Red" => out_color(Color4::RED),
            "Green" => out_color(Color4::GREEN),
            "Blue" => out_color(Color4::BLUE),
            "Yellow" => out_color(Color4::YELLOW),
            "Cyan" => out_color(Color4::CYAN),
            "Magenta" => out_color(Color4::MAGENTA),
            "Orange" => out_color(Color4::ORANGE),
            "Purple" => out_color(Color4::PURPLE),
            "Gray" => out_color(Color4::GRAY),
            "Pink" => out_color(Color4::PINK),
            "Brown" => out_color(Color4::BROWN),
            "Lime" => out_color(Color4::LIME),
            "Teal" => out_color(Color4::TEAL),
            "Navy" => out_color(Color4::NAVY),
            "Maroon" => out_color(Color4::MAROON),
            "Olive" => out_color(Color4::OLIVE),
            "Silver" => out_color(Color4::SILVER),
            "Gold" => out_color(Color4::GOLD),
            "Indigo" => out_color(Color4::INDIGO),
            "Violet" => out_color(Color4::VIOLET),
            "Crimson" => out_color(Color4::CRIMSON),
            "Salmon" => out_color(Color4::SALMON),
            "Turquoise" => out_color(Color4::TURQUOISE),
            "SkyBlue" => out_color(Color4::SKY_BLUE),
            "RoyalBlue" => out_color(Color4::ROYAL_BLUE),
            "SlateGray" => out_color(Color4::SLATE_GRAY),
            "Snow" => out_color(Color4::SNOW),
            other => return Err(format!("unknown color name: {other}")),
        },

        other => return Err(format!("unknown math function: {other}")),
    })
}

fn box2_from_center_size(center: Vec2, size: Vec2) -> Box2 {
    Box2::from_center_size(center, size)
}

fn out_box(b: Box2) -> Value {
    json!({ "min": out_vec2(b.min), "max": out_vec2(b.max) })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(name: &str, args: Value) -> Value {
        let request = json!({ "name": name, "args": args });
        let response: Value = serde_json::from_str(&math_op(&request)).unwrap();
        response
    }

    #[test]
    fn dispatches_vectors_matrices_quaternions_and_scalars() {
        assert_eq!(run("scalar.clamp", json!([2.0, 0.0, 1.0]))["value"], 1.0);
        assert_eq!(
            run(
                "vec3.cross",
                json!([{ "x": 1.0, "y": 0.0, "z": 0.0 }, { "x": 0.0, "y": 1.0, "z": 0.0 }])
            )["value"]["z"],
            1.0
        );
        let identity = run("matrix.identity", json!([]));
        assert_eq!(identity["value"]["m"].as_array().unwrap().len(), 16);
        let quaternion = run("quaternion.fromEuler", json!([0.0, 0.0, 0.0]));
        assert_eq!(quaternion["value"]["w"], 1.0);
        assert_eq!(run("color.named", json!(["Red"]))["value"]["r"], 1.0);
    }

    #[test]
    fn rejects_unknown_functions() {
        let response = run("does.not.exist", json!([]));
        assert!(response.get("error").is_some());
    }
}
