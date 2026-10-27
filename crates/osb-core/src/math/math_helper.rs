/// Scalar helpers mirroring OpenTK's `MathHelper`.
pub struct MathHelper;

impl MathHelper {
    pub const PI: f64 = std::f64::consts::PI;
    pub const TWO_PI: f64 = std::f64::consts::TAU;
    pub const PI_OVER_2: f64 = std::f64::consts::FRAC_PI_2;
    pub const E: f64 = std::f64::consts::E;
    pub const DEGREES_TO_RADIANS: f64 = std::f64::consts::PI / 180.0;
    pub const RADIANS_TO_DEGREES: f64 = 180.0 / std::f64::consts::PI;

    pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    pub fn clamp01(value: f64) -> f64 {
        MathHelper::clamp(value, 0.0, 1.0)
    }

    pub fn lerp(a: f64, b: f64, amount: f64) -> f64 {
        a + (b - a) * amount
    }

    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * MathHelper::DEGREES_TO_RADIANS
    }

    pub fn radians_to_degrees(radians: f64) -> f64 {
        radians * MathHelper::RADIANS_TO_DEGREES
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_bounds() {
        assert_eq!(MathHelper::clamp(5.0, 0.0, 1.0), 1.0);
        assert_eq!(MathHelper::clamp(-5.0, 0.0, 1.0), 0.0);
        assert_eq!(MathHelper::clamp(0.5, 0.0, 1.0), 0.5);
    }

    #[test]
    fn degrees_radians_roundtrip() {
        let r = MathHelper::degrees_to_radians(180.0);
        assert!((r - MathHelper::PI).abs() < 1e-6);
        assert!((MathHelper::radians_to_degrees(r) - 180.0).abs() < 1e-4);
    }
}
