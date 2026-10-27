use super::Vec4;

// An RGBA color with single-precision channels, matching OpenTK's `Color4`.
/// A linear RGBA color with channels in `0.0..=1.0`.
#[derive(
    Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
pub struct Color4 {
    /// Red channel.
    pub r: f64,
    /// Green channel.
    pub g: f64,
    /// Blue channel.
    pub b: f64,
    /// Alpha channel.
    pub a: f64,
}

impl Color4 {
    pub const WHITE: Color4 = Color4 {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLACK: Color4 = Color4 {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    /// Matches OpenTK's `Color4.Transparent` (white with zero alpha).
    pub const TRANSPARENT: Color4 = Color4 {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.0,
    };

    pub const ALICE_BLUE: Color4 = rgba8(240, 248, 255, 255);
    pub const ANTIQUE_WHITE: Color4 = rgba8(250, 235, 215, 255);
    pub const AQUA: Color4 = rgba8(0, 255, 255, 255);
    pub const AQUAMARINE: Color4 = rgba8(127, 255, 212, 255);
    pub const AZURE: Color4 = rgba8(240, 255, 255, 255);
    pub const BEIGE: Color4 = rgba8(245, 245, 220, 255);
    pub const BISQUE: Color4 = rgba8(255, 228, 196, 255);
    pub const BLANCHED_ALMOND: Color4 = rgba8(255, 235, 205, 255);
    pub const BLUE: Color4 = rgba8(0, 0, 255, 255);
    pub const BLUE_VIOLET: Color4 = rgba8(138, 43, 226, 255);
    pub const BROWN: Color4 = rgba8(165, 42, 42, 255);
    pub const BURLY_WOOD: Color4 = rgba8(222, 184, 135, 255);
    pub const CADET_BLUE: Color4 = rgba8(95, 158, 160, 255);
    pub const CHARTREUSE: Color4 = rgba8(127, 255, 0, 255);
    pub const CHOCOLATE: Color4 = rgba8(210, 105, 30, 255);
    pub const CORAL: Color4 = rgba8(255, 127, 80, 255);
    pub const CORNFLOWER_BLUE: Color4 = rgba8(100, 149, 237, 255);
    pub const CORNSILK: Color4 = rgba8(255, 248, 220, 255);
    pub const CRIMSON: Color4 = rgba8(220, 20, 60, 255);
    pub const CYAN: Color4 = rgba8(0, 255, 255, 255);
    pub const DARK_BLUE: Color4 = rgba8(0, 0, 139, 255);
    pub const DARK_CYAN: Color4 = rgba8(0, 139, 139, 255);
    pub const DARK_GOLDENROD: Color4 = rgba8(184, 134, 11, 255);
    pub const DARK_GRAY: Color4 = rgba8(169, 169, 169, 255);
    pub const DARK_GREEN: Color4 = rgba8(0, 100, 0, 255);
    pub const DARK_KHAKI: Color4 = rgba8(189, 183, 107, 255);
    pub const DARK_MAGENTA: Color4 = rgba8(139, 0, 139, 255);
    pub const DARK_OLIVE_GREEN: Color4 = rgba8(85, 107, 47, 255);
    pub const DARK_ORANGE: Color4 = rgba8(255, 140, 0, 255);
    pub const DARK_ORCHID: Color4 = rgba8(153, 50, 204, 255);
    pub const DARK_RED: Color4 = rgba8(139, 0, 0, 255);
    pub const DARK_SALMON: Color4 = rgba8(233, 150, 122, 255);
    pub const DARK_SEA_GREEN: Color4 = rgba8(143, 188, 139, 255);
    pub const DARK_SLATE_BLUE: Color4 = rgba8(72, 61, 139, 255);
    pub const DARK_SLATE_GRAY: Color4 = rgba8(47, 79, 79, 255);
    pub const DARK_TURQUOISE: Color4 = rgba8(0, 206, 209, 255);
    pub const DARK_VIOLET: Color4 = rgba8(148, 0, 211, 255);
    pub const DEEP_PINK: Color4 = rgba8(255, 20, 147, 255);
    pub const DEEP_SKY_BLUE: Color4 = rgba8(0, 191, 255, 255);
    pub const DIM_GRAY: Color4 = rgba8(105, 105, 105, 255);
    pub const DODGER_BLUE: Color4 = rgba8(30, 144, 255, 255);
    pub const FIREBRICK: Color4 = rgba8(178, 34, 34, 255);
    pub const FLORAL_WHITE: Color4 = rgba8(255, 250, 240, 255);
    pub const FOREST_GREEN: Color4 = rgba8(34, 139, 34, 255);
    pub const FUCHSIA: Color4 = rgba8(255, 0, 255, 255);
    pub const GAINSBORO: Color4 = rgba8(220, 220, 220, 255);
    pub const GHOST_WHITE: Color4 = rgba8(248, 248, 255, 255);
    pub const GOLD: Color4 = rgba8(255, 215, 0, 255);
    pub const GOLDENROD: Color4 = rgba8(218, 165, 32, 255);
    pub const GRAY: Color4 = rgba8(128, 128, 128, 255);
    pub const GREEN: Color4 = rgba8(0, 128, 0, 255);
    pub const GREEN_YELLOW: Color4 = rgba8(173, 255, 47, 255);
    pub const HONEYDEW: Color4 = rgba8(240, 255, 240, 255);
    pub const HOT_PINK: Color4 = rgba8(255, 105, 180, 255);
    pub const INDIAN_RED: Color4 = rgba8(205, 92, 92, 255);
    pub const INDIGO: Color4 = rgba8(75, 0, 130, 255);
    pub const IVORY: Color4 = rgba8(255, 255, 240, 255);
    pub const KHAKI: Color4 = rgba8(240, 230, 140, 255);
    pub const LAVENDER: Color4 = rgba8(230, 230, 250, 255);
    pub const LAVENDER_BLUSH: Color4 = rgba8(255, 240, 245, 255);
    pub const LAWN_GREEN: Color4 = rgba8(124, 252, 0, 255);
    pub const LEMON_CHIFFON: Color4 = rgba8(255, 250, 205, 255);
    pub const LIGHT_BLUE: Color4 = rgba8(173, 216, 230, 255);
    pub const LIGHT_CORAL: Color4 = rgba8(240, 128, 128, 255);
    pub const LIGHT_CYAN: Color4 = rgba8(224, 255, 255, 255);
    pub const LIGHT_GOLDENROD_YELLOW: Color4 = rgba8(250, 250, 210, 255);
    pub const LIGHT_GRAY: Color4 = rgba8(211, 211, 211, 255);
    pub const LIGHT_GREEN: Color4 = rgba8(144, 238, 144, 255);
    pub const LIGHT_PINK: Color4 = rgba8(255, 182, 193, 255);
    pub const LIGHT_SALMON: Color4 = rgba8(255, 160, 122, 255);
    pub const LIGHT_SEA_GREEN: Color4 = rgba8(32, 178, 170, 255);
    pub const LIGHT_SKY_BLUE: Color4 = rgba8(135, 206, 250, 255);
    pub const LIGHT_SLATE_GRAY: Color4 = rgba8(119, 136, 153, 255);
    pub const LIGHT_STEEL_BLUE: Color4 = rgba8(176, 196, 222, 255);
    pub const LIGHT_YELLOW: Color4 = rgba8(255, 255, 224, 255);
    pub const LIME: Color4 = rgba8(0, 255, 0, 255);
    pub const LIME_GREEN: Color4 = rgba8(50, 205, 50, 255);
    pub const LINEN: Color4 = rgba8(250, 240, 230, 255);
    pub const MAGENTA: Color4 = rgba8(255, 0, 255, 255);
    pub const MAROON: Color4 = rgba8(128, 0, 0, 255);
    pub const MEDIUM_AQUAMARINE: Color4 = rgba8(102, 205, 170, 255);
    pub const MEDIUM_BLUE: Color4 = rgba8(0, 0, 205, 255);
    pub const MEDIUM_ORCHID: Color4 = rgba8(186, 85, 211, 255);
    pub const MEDIUM_PURPLE: Color4 = rgba8(147, 112, 219, 255);
    pub const MEDIUM_SEA_GREEN: Color4 = rgba8(60, 179, 113, 255);
    pub const MEDIUM_SLATE_BLUE: Color4 = rgba8(123, 104, 238, 255);
    pub const MEDIUM_SPRING_GREEN: Color4 = rgba8(0, 250, 154, 255);
    pub const MEDIUM_TURQUOISE: Color4 = rgba8(72, 209, 204, 255);
    pub const MEDIUM_VIOLET_RED: Color4 = rgba8(199, 21, 133, 255);
    pub const MIDNIGHT_BLUE: Color4 = rgba8(25, 25, 112, 255);
    pub const MINT_CREAM: Color4 = rgba8(245, 255, 250, 255);
    pub const MISTY_ROSE: Color4 = rgba8(255, 228, 225, 255);
    pub const MOCCASIN: Color4 = rgba8(255, 228, 181, 255);
    pub const NAVAJO_WHITE: Color4 = rgba8(255, 222, 173, 255);
    pub const NAVY: Color4 = rgba8(0, 0, 128, 255);
    pub const OLD_LACE: Color4 = rgba8(253, 245, 230, 255);
    pub const OLIVE: Color4 = rgba8(128, 128, 0, 255);
    pub const OLIVE_DRAB: Color4 = rgba8(107, 142, 35, 255);
    pub const ORANGE: Color4 = rgba8(255, 165, 0, 255);
    pub const ORANGE_RED: Color4 = rgba8(255, 69, 0, 255);
    pub const ORCHID: Color4 = rgba8(218, 112, 214, 255);
    pub const PALE_GOLDENROD: Color4 = rgba8(238, 232, 170, 255);
    pub const PALE_GREEN: Color4 = rgba8(152, 251, 152, 255);
    pub const PALE_TURQUOISE: Color4 = rgba8(175, 238, 238, 255);
    pub const PALE_VIOLET_RED: Color4 = rgba8(219, 112, 147, 255);
    pub const PAPAYA_WHIP: Color4 = rgba8(255, 239, 213, 255);
    pub const PEACH_PUFF: Color4 = rgba8(255, 218, 185, 255);
    pub const PERU: Color4 = rgba8(205, 133, 63, 255);
    pub const PINK: Color4 = rgba8(255, 192, 203, 255);
    pub const PLUM: Color4 = rgba8(221, 160, 221, 255);
    pub const POWDER_BLUE: Color4 = rgba8(176, 224, 230, 255);
    pub const PURPLE: Color4 = rgba8(128, 0, 128, 255);
    pub const RED: Color4 = rgba8(255, 0, 0, 255);
    pub const ROSY_BROWN: Color4 = rgba8(188, 143, 143, 255);
    pub const ROYAL_BLUE: Color4 = rgba8(65, 105, 225, 255);
    pub const SADDLE_BROWN: Color4 = rgba8(139, 69, 19, 255);
    pub const SALMON: Color4 = rgba8(250, 128, 114, 255);
    pub const SANDY_BROWN: Color4 = rgba8(244, 164, 96, 255);
    pub const SEA_GREEN: Color4 = rgba8(46, 139, 87, 255);
    pub const SEA_SHELL: Color4 = rgba8(255, 245, 238, 255);
    pub const SIENNA: Color4 = rgba8(160, 82, 45, 255);
    pub const SILVER: Color4 = rgba8(192, 192, 192, 255);
    pub const SKY_BLUE: Color4 = rgba8(135, 206, 235, 255);
    pub const SLATE_BLUE: Color4 = rgba8(106, 90, 205, 255);
    pub const SLATE_GRAY: Color4 = rgba8(112, 128, 144, 255);
    pub const SNOW: Color4 = rgba8(255, 250, 250, 255);
    pub const SPRING_GREEN: Color4 = rgba8(0, 255, 127, 255);
    pub const STEEL_BLUE: Color4 = rgba8(70, 130, 180, 255);
    pub const TAN: Color4 = rgba8(210, 180, 140, 255);
    pub const TEAL: Color4 = rgba8(0, 128, 128, 255);
    pub const THISTLE: Color4 = rgba8(216, 191, 216, 255);
    pub const TOMATO: Color4 = rgba8(255, 99, 71, 255);
    pub const TURQUOISE: Color4 = rgba8(64, 224, 208, 255);
    pub const VIOLET: Color4 = rgba8(238, 130, 238, 255);
    pub const WHEAT: Color4 = rgba8(245, 222, 179, 255);
    pub const WHITE_SMOKE: Color4 = rgba8(245, 245, 245, 255);
    pub const YELLOW: Color4 = rgba8(255, 255, 0, 255);
    pub const YELLOW_GREEN: Color4 = rgba8(154, 205, 50, 255);

    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color4 { r, g, b, a }
    }

    pub const fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Color4 { r, g, b, a: 1.0 }
    }

    /// Builds a color from hue/saturation/brightness (all `0.0..=1.0`).
    pub fn from_hsb(hue: f64, saturation: f64, brightness: f64, alpha: f64) -> Self {
        if saturation == 0.0 {
            return Color4::new(brightness, brightness, brightness, alpha);
        }
        let h = (hue.fract() + 1.0).fract() * 6.0;
        let i = h.floor();
        let f = h - i;
        let p = brightness * (1.0 - saturation);
        let q = brightness * (1.0 - saturation * f);
        let t = brightness * (1.0 - saturation * (1.0 - f));
        let (r, g, b) = match i as i32 % 6 {
            0 => (brightness, t, p),
            1 => (q, brightness, p),
            2 => (p, brightness, t),
            3 => (p, q, brightness),
            4 => (t, p, brightness),
            _ => (brightness, p, q),
        };
        Color4::new(r, g, b, alpha)
    }

    /// Builds a color from hue/saturation/lightness (`Vec4`, all `0.0..=1.0`),
    /// matching OpenTK's `Color4.FromHsl`.
    pub fn from_hsl(hsl: Vec4) -> Self {
        let hue = hsl.x * 360.0;
        let saturation = hsl.y;
        let lightness = hsl.z;
        let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let h = hue / 60.0;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
        let (r, g, b) = if (0.0..1.0).contains(&h) {
            (c, x, 0.0)
        } else if (1.0..2.0).contains(&h) {
            (x, c, 0.0)
        } else if (2.0..3.0).contains(&h) {
            (0.0, c, x)
        } else if (3.0..4.0).contains(&h) {
            (0.0, x, c)
        } else if (4.0..5.0).contains(&h) {
            (x, 0.0, c)
        } else if (5.0..=6.0).contains(&h) {
            (c, 0.0, x)
        } else {
            (0.0, 0.0, 0.0)
        };
        let m = (lightness - c / 2.0).max(0.0);
        Color4::new(r + m, g + m, b + m, hsl.w)
    }

    /// Converts to hue/saturation/lightness (`Vec4`), matching OpenTK's `Color4.ToHsl`.
    pub fn to_hsl(self) -> Vec4 {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let diff = max - min;

        let mut h = 0.0;
        if diff != 0.0 {
            if max == self.r {
                h = ((self.g - self.b) / diff) % 6.0;
                if h < 0.0 {
                    h += 6.0;
                }
            } else if max == self.g {
                h = (self.b - self.r) / diff + 2.0;
            } else {
                h = (self.r - self.g) / diff + 4.0;
            }
        }
        let mut hue = h / 6.0;
        if hue < 0.0 {
            hue += 1.0;
        }
        let lightness = (max + min) / 2.0;
        let denom = 1.0 - (2.0 * lightness - 1.0).abs();
        let saturation = if denom != 0.0 { diff / denom } else { 0.0 };
        Vec4::new(hue, saturation, lightness, self.a)
    }

    /// Converts to hue/saturation/brightness (`Vec4`, all `0.0..=1.0`), the
    /// inverse of [`from_hsb`](Self::from_hsb). HSV is the same as HSB.
    pub fn to_hsb(self) -> Vec4 {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let diff = max - min;

        let mut hue = 0.0;
        if diff != 0.0 {
            if max == self.r {
                hue = ((self.g - self.b) / diff) % 6.0;
                if hue < 0.0 {
                    hue += 6.0;
                }
            } else if max == self.g {
                hue = (self.b - self.r) / diff + 2.0;
            } else {
                hue = (self.r - self.g) / diff + 4.0;
            }
            hue /= 6.0;
        }
        let brightness = max;
        let saturation = if max != 0.0 { diff / max } else { 0.0 };
        Vec4::new(hue, saturation, brightness, self.a)
    }

    /// Converts to hue/saturation/value (`Vec4`); value is the same as brightness.
    pub fn to_hsv(self) -> Vec4 {
        self.to_hsb()
    }

    /// Builds a color from hue/saturation/value (`0.0..=1.0`); value is brightness.
    pub fn from_hsv(hue: f64, saturation: f64, value: f64, alpha: f64) -> Self {
        Color4::from_hsb(hue, saturation, value, alpha)
    }

    pub fn lerp(self, other: Color4, amount: f64) -> Color4 {
        Color4::new(
            self.r + (other.r - self.r) * amount,
            self.g + (other.g - self.g) * amount,
            self.b + (other.b - self.b) * amount,
            self.a + (other.a - self.a) * amount,
        )
    }

    /// Multiplies RGB channels (the color command scales the sprite tint).
    pub fn multiply_rgb(self, factor: f64) -> Color4 {
        Color4::new(self.r * factor, self.g * factor, self.b * factor, self.a)
    }

    pub fn to_rgba8(self) -> [u8; 4] {
        [
            (self.r.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.g.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.b.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.a.clamp(0.0, 1.0) * 255.0).round() as u8,
        ]
    }
}

/// Builds a `Color4` from 8-bit channels, as OpenTK's named colors are defined.
const fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Color4 {
    Color4::new(
        r as f64 / 255.0,
        g as f64 / 255.0,
        b as f64 / 255.0,
        a as f64 / 255.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_colors_match_opentk_bytes() {
        assert_eq!(Color4::BEIGE.to_rgba8(), [245, 245, 220, 255]);
        assert_eq!(Color4::CYAN.to_rgba8(), [0, 255, 255, 255]);
        assert_eq!(
            Color4::LIGHT_GOLDENROD_YELLOW.to_rgba8(),
            [250, 250, 210, 255]
        );
        assert_eq!(Color4::TRANSPARENT.to_rgba8(), [255, 255, 255, 0]);
    }

    #[test]
    fn hsb_primary_colors() {
        let red = Color4::from_hsb(0.0, 1.0, 1.0, 1.0);
        assert!((red.r - 1.0).abs() < 1e-5 && red.g.abs() < 1e-5 && red.b.abs() < 1e-5);
        let green = Color4::from_hsb(1.0 / 3.0, 1.0, 1.0, 1.0);
        assert!((green.g - 1.0).abs() < 1e-5);
        let blue = Color4::from_hsb(2.0 / 3.0, 1.0, 1.0, 1.0);
        assert!((blue.b - 1.0).abs() < 1e-5);
    }

    #[test]
    fn grayscale_when_saturation_zero() {
        let c = Color4::from_hsb(0.5, 0.0, 0.4, 1.0);
        assert_eq!(c.r, c.g);
        assert_eq!(c.g, c.b);
    }

    #[test]
    fn hsb_round_trips() {
        let c = Color4::new(0.2, 0.6, 0.9, 0.5);
        let hsb = c.to_hsb();
        let back = Color4::from_hsb(hsb.x, hsb.y, hsb.z, hsb.w);
        assert!((back.r - c.r).abs() < 1e-6);
        assert!((back.g - c.g).abs() < 1e-6);
        assert!((back.b - c.b).abs() < 1e-6);
        assert!((back.a - c.a).abs() < 1e-6);
        // HSV is an alias of HSB.
        assert_eq!(c.to_hsv(), hsb);
    }
}
