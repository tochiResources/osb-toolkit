//! Free math functions the generated effect code calls for `Math.*` in C#.

pub fn sin(x: f64) -> f64 {
    x.sin()
}

pub fn cos(x: f64) -> f64 {
    x.cos()
}

pub fn tan(x: f64) -> f64 {
    x.tan()
}

pub fn asin(x: f64) -> f64 {
    x.asin()
}

pub fn acos(x: f64) -> f64 {
    x.acos()
}

pub fn atan(x: f64) -> f64 {
    x.atan()
}

pub fn atan2(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

pub fn abs(x: f64) -> f64 {
    x.abs()
}

pub fn floor(x: f64) -> f64 {
    x.floor()
}

pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

pub fn round(x: f64) -> f64 {
    x.round()
}

/// Rounds to a number of decimal places (C# `Math.Round(value, digits)`).
pub fn round_to(value: f64, digits: impl Into<f64>) -> f64 {
    let factor = 10f64.powi(digits.into() as i32);
    (value * factor).round() / factor
}

pub fn trunc(x: f64) -> f64 {
    x.trunc()
}

pub fn sign(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else {
        x.signum()
    }
}

pub fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}

pub fn exp(x: f64) -> f64 {
    x.exp()
}

pub fn log(x: f64) -> f64 {
    x.ln()
}

pub fn log10(x: f64) -> f64 {
    x.log10()
}

pub fn log2(x: f64) -> f64 {
    x.log2()
}

pub fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

pub fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.clamp(min, max)
}

pub fn lerp(a: f64, b: f64, amount: f64) -> f64 {
    a + (b - a) * amount
}

pub fn hypot(a: f64, b: f64) -> f64 {
    a.hypot(b)
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees.to_radians()
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    radians.to_degrees()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(abs(-3.0), 3.0);
        assert_eq!(clamp(5.0, 0.0, 1.0), 1.0);
        assert!((sin(std::f64::consts::FRAC_PI_2) - 1.0).abs() < 1e-9);
    }
}
