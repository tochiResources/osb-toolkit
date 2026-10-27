//! The shared osu! storyboard layer and sprite-origin types, used by both the 2D
//! and 3D layers.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Sprite anchor origin (osu! `.osb` origin ids 0..8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub enum OsbOrigin {
    TopLeft,
    TopCentre,
    TopRight,
    CentreLeft,
    #[default]
    Centre,
    CentreRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

impl OsbOrigin {
    pub fn osb_id(self) -> u8 {
        match self {
            OsbOrigin::TopLeft => 0,
            OsbOrigin::TopCentre => 1,
            OsbOrigin::TopRight => 2,
            OsbOrigin::CentreLeft => 3,
            OsbOrigin::Centre => 4,
            OsbOrigin::CentreRight => 5,
            OsbOrigin::BottomLeft => 6,
            OsbOrigin::BottomCentre => 7,
            OsbOrigin::BottomRight => 8,
        }
    }

    pub fn from_osb_id(id: u8) -> Option<OsbOrigin> {
        Some(match id {
            0 => OsbOrigin::TopLeft,
            1 => OsbOrigin::TopCentre,
            2 => OsbOrigin::TopRight,
            3 => OsbOrigin::CentreLeft,
            4 => OsbOrigin::Centre,
            5 => OsbOrigin::CentreRight,
            6 => OsbOrigin::BottomLeft,
            7 => OsbOrigin::BottomCentre,
            8 => OsbOrigin::BottomRight,
            _ => return None,
        })
    }

    /// Maps a host popup index (`0..=8`) to an origin, clamping out-of-range values.
    pub fn from_index(index: usize) -> OsbOrigin {
        match index {
            0 => OsbOrigin::TopLeft,
            1 => OsbOrigin::TopCentre,
            2 => OsbOrigin::TopRight,
            3 => OsbOrigin::CentreLeft,
            4 => OsbOrigin::Centre,
            5 => OsbOrigin::CentreRight,
            6 => OsbOrigin::BottomLeft,
            7 => OsbOrigin::BottomCentre,
            _ => OsbOrigin::BottomRight,
        }
    }

    /// Parses an `.osb` origin name (`Centre`, `TopLeft`, ...), case-insensitively.
    pub fn from_name(name: &str) -> Option<OsbOrigin> {
        Some(match name.trim().to_ascii_lowercase().as_str() {
            "topleft" => OsbOrigin::TopLeft,
            "topcentre" | "topcenter" => OsbOrigin::TopCentre,
            "topright" => OsbOrigin::TopRight,
            "centreleft" | "centerleft" => OsbOrigin::CentreLeft,
            "centre" | "center" => OsbOrigin::Centre,
            "centreright" | "centerright" => OsbOrigin::CentreRight,
            "bottomleft" => OsbOrigin::BottomLeft,
            "bottomcentre" | "bottomcenter" => OsbOrigin::BottomCentre,
            "bottomright" => OsbOrigin::BottomRight,
            _ => return None,
        })
    }

    /// The canonical `.osb` origin name.
    pub fn name(self) -> &'static str {
        match self {
            OsbOrigin::TopLeft => "TopLeft",
            OsbOrigin::TopCentre => "TopCentre",
            OsbOrigin::TopRight => "TopRight",
            OsbOrigin::CentreLeft => "CentreLeft",
            OsbOrigin::Centre => "Centre",
            OsbOrigin::CentreRight => "CentreRight",
            OsbOrigin::BottomLeft => "BottomLeft",
            OsbOrigin::BottomCentre => "BottomCentre",
            OsbOrigin::BottomRight => "BottomRight",
        }
    }

    /// Normalized anchor within the sprite bounds (`0.0..=1.0`).
    pub fn anchor(self) -> (f64, f64) {
        let x = match self {
            OsbOrigin::TopLeft | OsbOrigin::CentreLeft | OsbOrigin::BottomLeft => 0.0,
            OsbOrigin::TopCentre | OsbOrigin::Centre | OsbOrigin::BottomCentre => 0.5,
            _ => 1.0,
        };
        let y = match self {
            OsbOrigin::TopLeft | OsbOrigin::TopCentre | OsbOrigin::TopRight => 0.0,
            OsbOrigin::CentreLeft | OsbOrigin::Centre | OsbOrigin::CentreRight => 0.5,
            _ => 1.0,
        };
        (x, y)
    }
}

/// A standard osu! storyboard layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub enum OsbLayer {
    #[default]
    Background,
    Fail,
    Pass,
    Foreground,
    Overlay,
}

impl OsbLayer {
    pub fn osb_id(self) -> u8 {
        match self {
            OsbLayer::Background => 0,
            OsbLayer::Fail => 1,
            OsbLayer::Pass => 2,
            OsbLayer::Foreground => 3,
            OsbLayer::Overlay => 4,
        }
    }

    pub fn all() -> [OsbLayer; 5] {
        [
            OsbLayer::Background,
            OsbLayer::Fail,
            OsbLayer::Pass,
            OsbLayer::Foreground,
            OsbLayer::Overlay,
        ]
    }

    /// The layer identifier for this layer.
    pub fn name(self) -> &'static str {
        match self {
            OsbLayer::Background => "Background",
            OsbLayer::Fail => "Fail",
            OsbLayer::Pass => "Pass",
            OsbLayer::Foreground => "Foreground",
            OsbLayer::Overlay => "Overlay",
        }
    }

    /// Matches a layer identifier, case-insensitively.
    pub fn from_name(name: &str) -> Option<OsbLayer> {
        OsbLayer::all()
            .into_iter()
            .find(|layer| layer.name().eq_ignore_ascii_case(name))
    }

    /// Maps an osu! `.osb` layer id (`0..=4`) to a layer.
    pub fn from_osb_id(id: u8) -> Option<OsbLayer> {
        OsbLayer::all().get(id as usize).copied()
    }

    /// Maps a depth bucket / index onto a layer, clamping out-of-range values
    /// (used when a 3D bake's depth bucket is materialized as an osu! layer).
    pub fn from_index(index: usize) -> OsbLayer {
        OsbLayer::all().get(index).copied().unwrap_or_default()
    }
}
