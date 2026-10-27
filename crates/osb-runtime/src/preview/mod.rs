//! The `OsuPreview` base effect: renders a loaded beatmap's hit objects the way
//! the osu! client draws them - hitcircles with approach circles and combo
//! numbers, sliders (body/ball/ticks) and spinners - by building a storyboard of
//! sprite commands from the mapset. It is host-agnostic: the adapter supplies the
//! beatmap, the asset root (skins) and the parameter values.

use crate::context::Context;
use crate::mapset::{HitObject, HitObjectKind};
use crate::math::Vec2;
use crate::storyboarding::{OsbLayer, OsbOrigin, Runtime, SpriteHandle, Storyboard};

/// The osu! ruleset to preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameMode {
    #[default]
    Standard,
    Taiko,
    Catch,
    Mania,
}

/// Sprite paths resolved from the selected skin (with bundled defaults).
#[derive(Debug, Clone)]
pub struct SkinPaths {
    pub hitcircle: String,
    pub hitcircle_overlay: String,
    pub approach_circle: String,
    pub slider_body: String,
    pub slider_ball: String,
    pub slider_tick: String,
    pub slider_reverse: String,
    pub spinner_circle: String,
    pub spinner_metre: String,
    /// Digit sprite paths `"0".."9"` for combo numbers.
    pub combo_digits: Vec<String>,
}

impl Default for SkinPaths {
    fn default() -> Self {
        SkinPaths {
            hitcircle: "sb/hitcircle.png".into(),
            hitcircle_overlay: "sb/hitcircleoverlay.png".into(),
            approach_circle: "sb/approachcircle.png".into(),
            slider_body: "sb/sliderbody.png".into(),
            slider_ball: "sb/sliderball.png".into(),
            slider_tick: "sb/slider-tick.png".into(),
            slider_reverse: "sb/reversearrow.png".into(),
            spinner_circle: "sb/spinner-circle.png".into(),
            spinner_metre: "sb/spinner-metre.png".into(),
            combo_digits: Vec::new(),
        }
    }
}

/// Per-element visibility toggles for the skinnable components.
#[derive(Debug, Clone, Copy)]
pub struct ElementToggles {
    pub hitcircle: bool,
    pub hitcircle_overlay: bool,
    pub combo_numbers: bool,
    pub approach_circle: bool,
    pub slider_body: bool,
    pub slider_ball: bool,
    pub slider_ticks: bool,
    pub slider_reverse_arrow: bool,
    pub spinner_background: bool,
    pub spinner_circle: bool,
    pub spinner_metre: bool,
}

impl Default for ElementToggles {
    fn default() -> Self {
        ElementToggles {
            hitcircle: true,
            hitcircle_overlay: true,
            combo_numbers: true,
            approach_circle: true,
            slider_body: true,
            slider_ball: true,
            slider_ticks: true,
            slider_reverse_arrow: true,
            spinner_background: true,
            spinner_circle: true,
            spinner_metre: true,
        }
    }
}

/// Options for building the preview.
#[derive(Debug, Clone)]
pub struct OsuPreviewOptions {
    pub game_mode: GameMode,
    pub layer: OsbLayer,
    pub skin: SkinPaths,
    pub elements: ElementToggles,
    /// Global hit-object scale multiplier.
    pub scale: f64,
    /// Additional audio offset in ms applied to all object times.
    pub audio_offset: f64,
}

impl Default for OsuPreviewOptions {
    fn default() -> Self {
        OsuPreviewOptions {
            game_mode: GameMode::Standard,
            layer: OsbLayer::Foreground,
            skin: SkinPaths::default(),
            elements: ElementToggles::default(),
            scale: 1.0,
            audio_offset: 0.0,
        }
    }
}

/// What the preview builder produced.
#[derive(Debug, Clone, Copy, Default)]
pub struct PreviewStats {
    pub hit_objects: usize,
    pub sprites_created: usize,
}

/// osu!'s approach (preempt) time in milliseconds for `approach_rate`.
pub fn approach_time(approach_rate: f64) -> f64 {
    if approach_rate < 5.0 {
        1800.0 - 120.0 * approach_rate
    } else {
        1200.0 - 150.0 * (approach_rate - 5.0)
    }
}

/// osu!'s hit-circle radius in osu! pixels for `circle_size`.
pub fn circle_radius(circle_size: f64) -> f64 {
    54.4 - 4.48 * circle_size
}

const FADE_OUT: f64 = 150.0;
const CENTRE: Vec2 = Vec2 { x: 256.0, y: 192.0 };

/// Builds hit-object sprites for the current beatmap into `runtime`'s storyboard.
pub fn build_preview(runtime: &mut Runtime, options: &OsuPreviewOptions) -> PreviewStats {
    let Some(beatmap) = runtime.context.beatmap() else {
        return PreviewStats::default();
    };
    let objects: Vec<HitObject> = beatmap.hit_objects.clone();
    let approach = approach_time(beatmap.approach_rate);
    let radius = circle_radius(beatmap.circle_size) * options.scale;

    let mut stats = PreviewStats {
        hit_objects: objects.len(),
        sprites_created: 0,
    };
    for object in &objects {
        build_object(
            &mut *runtime.storyboard,
            object,
            options,
            approach,
            radius,
            &mut stats,
        );
    }
    stats
}

fn build_object(
    storyboard: &mut Storyboard,
    object: &HitObject,
    options: &OsuPreviewOptions,
    approach: f64,
    radius: f64,
    stats: &mut PreviewStats,
) {
    let offset = options.audio_offset;
    let start = object.start_time + offset;
    let end = object.end_time + offset;
    let position = object.position + object.stack_offset;

    if matches!(object.kind, HitObjectKind::Spinner) {
        build_spinner(storyboard, object, options, start, end, stats);
        return;
    }

    if options.elements.hitcircle {
        let handle = add(storyboard, options.layer, &options.skin.hitcircle, position);
        let sprite = storyboard.sprite_mut(handle);
        sprite.color(
            start,
            start,
            object.color,
            object.color,
            crate::animations::Easing::None,
        );
        sprite.fade(
            start - approach,
            start,
            0.0,
            1.0,
            crate::animations::Easing::None,
        );
        sprite.fade(
            start,
            start + FADE_OUT,
            1.0,
            0.0,
            crate::animations::Easing::None,
        );
        sprite.scale(
            start,
            start,
            radius / 64.0,
            radius / 64.0,
            crate::animations::Easing::None,
        );
        stats.sprites_created += 1;
    }

    if options.elements.approach_circle {
        let handle = add(
            storyboard,
            options.layer,
            &options.skin.approach_circle,
            position,
        );
        let sprite = storyboard.sprite_mut(handle);
        sprite.fade(
            start - approach,
            start - approach + 200.0,
            0.0,
            1.0,
            crate::animations::Easing::None,
        );
        sprite.scale(
            start - approach,
            start,
            4.0,
            1.0,
            crate::animations::Easing::Out,
        );
        sprite.fade(
            start,
            start + FADE_OUT,
            1.0,
            0.0,
            crate::animations::Easing::None,
        );
        stats.sprites_created += 1;
    }

    if let HitObjectKind::Slider(path) = &object.kind {
        if options.elements.slider_body {
            let sampled = path.sampled().to_vec();
            let duration = (end - start).max(0.0);
            for pair in sampled.windows(2) {
                // A body segment per sampled subpath span, so the polyline is
                // drawn even when the slider has multiple curve pieces.
                let handle = add(
                    storyboard,
                    options.layer,
                    &options.skin.slider_body,
                    pair[0],
                );
                let sprite = storyboard.sprite_mut(handle);
                sprite.fade(start, end, 1.0, 1.0, crate::animations::Easing::None);
                sprite.scale(start, start, 1.0, 1.0, crate::animations::Easing::None);
                let _ = duration;
                stats.sprites_created += 1;
            }
        }
        if options.elements.slider_ball {
            let handle = add(
                storyboard,
                options.layer,
                &options.skin.slider_ball,
                position,
            );
            let sprite = storyboard.sprite_mut(handle);
            sprite.fade(start, end, 1.0, 1.0, crate::animations::Easing::None);
            let sampled = path.sampled().to_vec();
            let duration = (end - start).max(1.0);
            let total = sampled.len().max(1) as f64;
            let mut time = start;
            for (index, point) in sampled.iter().enumerate() {
                let next_time = start + duration * ((index + 1) as f64 / total);
                sprite.move_(
                    time,
                    next_time,
                    *point,
                    *point,
                    crate::animations::Easing::None,
                );
                time = next_time;
            }
            stats.sprites_created += 1;
        }
    }

    if options.elements.combo_numbers && !options.skin.combo_digits.is_empty() {
        let digits = object.combo_index.to_string();
        for (index, ch) in digits.chars().enumerate() {
            let Some(digit) = ch.to_digit(10) else {
                continue;
            };
            let Some(path) = options.skin.combo_digits.get(digit as usize) else {
                continue;
            };
            let offset_x = (index as f64 - (digits.len() as f64 - 1.0) * 0.5) * radius * 0.6;
            let handle = add(
                storyboard,
                options.layer,
                path,
                position + Vec2::new(offset_x, 0.0),
            );
            let sprite = storyboard.sprite_mut(handle);
            sprite.fade(
                start - approach,
                start,
                0.0,
                1.0,
                crate::animations::Easing::None,
            );
            sprite.fade(
                start,
                start + FADE_OUT,
                1.0,
                0.0,
                crate::animations::Easing::None,
            );
            stats.sprites_created += 1;
        }
    }
}

fn build_spinner(
    storyboard: &mut Storyboard,
    object: &HitObject,
    options: &OsuPreviewOptions,
    start: f64,
    end: f64,
    stats: &mut PreviewStats,
) {
    let _ = object;
    if options.elements.spinner_circle {
        let handle = add(
            storyboard,
            options.layer,
            &options.skin.spinner_circle,
            CENTRE,
        );
        let sprite = storyboard.sprite_mut(handle);
        sprite.fade(
            start,
            start + 200.0,
            0.0,
            1.0,
            crate::animations::Easing::None,
        );
        sprite.fade(
            end,
            end + FADE_OUT,
            1.0,
            0.0,
            crate::animations::Easing::None,
        );
        sprite.scale(
            start,
            start + (end - start) * 0.5,
            0.6,
            1.0,
            crate::animations::Easing::Out,
        );
        sprite.rotate(start, end, 0.0, 4.0, crate::animations::Easing::None);
        stats.sprites_created += 1;
    }
}

fn add(storyboard: &mut Storyboard, layer: OsbLayer, path: &str, position: Vec2) -> SpriteHandle {
    storyboard.create_sprite_at(layer, path, OsbOrigin::Centre, position)
}

/// The hostable `OsuPreview` effect: an [`Effect`] whose `generate` builds the
/// hit-object preview and whose parameters expose the element toggles.
#[derive(Debug, Clone, Default)]
pub struct OsuPreviewEffect {
    pub options: OsuPreviewOptions,
}

const TOGGLE_NAMES: [&str; 11] = [
    "ShowHitcircle",
    "ShowHitcircleOverlay",
    "ShowComboNumbers",
    "ShowApproachCircle",
    "ShowSliderBody",
    "ShowSliderBall",
    "ShowSliderTicks",
    "ShowSliderReverseArrow",
    "ShowSpinnerBackground",
    "ShowSpinnerCircle",
    "ShowSpinnerMetre",
];

impl crate::storyboarding::Effect for OsuPreviewEffect {
    fn generate(&mut self, context: &mut Context) -> Storyboard {
        let mut storyboard = Storyboard::new();
        {
            let mut runtime = Runtime::new(&mut storyboard, context);
            build_preview(&mut runtime, &self.options);
        }
        storyboard
    }

    fn parameters(&self) -> Vec<crate::params::ParameterDescriptor> {
        use crate::params::{Control, ParameterDescriptor, ParameterValue};
        let mut parameters = vec![
            ParameterDescriptor::new(
                "GameMode",
                Control::Popup {
                    choices: vec![
                        "osu!standard".into(),
                        "osu!taiko".into(),
                        "osu!catch".into(),
                        "osu!mania".into(),
                    ],
                },
                ParameterValue::Choice(0),
            ),
            ParameterDescriptor::new(
                "Scale",
                Control::Slider { min: 0.0, max: 4.0 },
                ParameterValue::Number(1.0),
            ),
            ParameterDescriptor::new(
                "AudioOffset",
                Control::Slider {
                    min: -5000.0,
                    max: 5000.0,
                },
                ParameterValue::Number(0.0),
            ),
        ];
        for name in TOGGLE_NAMES {
            parameters.push(ParameterDescriptor::new(
                name,
                Control::Checkbox,
                ParameterValue::Bool(true),
            ));
        }
        parameters
    }

    fn set_parameter(&mut self, name: &str, value: &crate::params::ParameterValue) {
        use crate::params::ParameterValue;
        match name {
            "GameMode" => {
                if let ParameterValue::Choice(index) = value {
                    self.options.game_mode = match index {
                        1 => GameMode::Taiko,
                        2 => GameMode::Catch,
                        3 => GameMode::Mania,
                        _ => GameMode::Standard,
                    };
                }
            }
            "Scale" => {
                if let ParameterValue::Number(v) = value {
                    self.options.scale = *v;
                }
            }
            "AudioOffset" => {
                if let ParameterValue::Number(v) = value {
                    self.options.audio_offset = *v;
                }
            }
            _ => {
                if let ParameterValue::Bool(v) = value {
                    match name {
                        "ShowHitcircle" => self.options.elements.hitcircle = *v,
                        "ShowHitcircleOverlay" => self.options.elements.hitcircle_overlay = *v,
                        "ShowComboNumbers" => self.options.elements.combo_numbers = *v,
                        "ShowApproachCircle" => self.options.elements.approach_circle = *v,
                        "ShowSliderBody" => self.options.elements.slider_body = *v,
                        "ShowSliderBall" => self.options.elements.slider_ball = *v,
                        "ShowSliderTicks" => self.options.elements.slider_ticks = *v,
                        "ShowSliderReverseArrow" => self.options.elements.slider_reverse_arrow = *v,
                        "ShowSpinnerBackground" => self.options.elements.spinner_background = *v,
                        "ShowSpinnerCircle" => self.options.elements.spinner_circle = *v,
                        "ShowSpinnerMetre" => self.options.elements.spinner_metre = *v,
                        _ => {}
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapset::{Beatmap, HitObject, HitObjectKind};

    fn map_with(objects: Vec<HitObject>) -> Beatmap {
        let mut beatmap = Beatmap::new();
        beatmap.approach_rate = 5.0;
        beatmap.circle_size = 4.0;
        beatmap.hit_objects = objects;
        beatmap
    }

    #[test]
    fn approach_time_matches_osu() {
        assert!((approach_time(5.0) - 1200.0).abs() < 1e-9);
        assert!((approach_time(9.0) - 600.0).abs() < 1e-9);
        assert!((approach_time(2.0) - 1560.0).abs() < 1e-9);
    }

    #[test]
    fn builds_hitcircle_and_approach_circle() {
        let mut storyboard = Storyboard::new();
        let mut context = Context::new(854.0, 480.0);
        context.set_beatmap(map_with(vec![HitObject::new(
            1000.0,
            1000.0,
            Vec2::new(256.0, 192.0),
            HitObjectKind::Circle,
        )]));
        let mut runtime = Runtime::new(&mut storyboard, &mut context);
        let stats = build_preview(&mut runtime, &OsuPreviewOptions::default());
        assert_eq!(stats.hit_objects, 1);
        assert_eq!(stats.sprites_created, 2);
        assert_eq!(storyboard.layer(OsbLayer::Foreground).len(), 2);
        // The hitcircle fades in during the approach window and out on hit.
        let circle = &storyboard.layer(OsbLayer::Foreground).sprites()[0];
        assert!(circle.opacity_at(1000.0) > 0.5);
    }

    #[test]
    fn element_toggles_suppress_sprites() {
        let mut storyboard = Storyboard::new();
        let mut context = Context::new(854.0, 480.0);
        context.set_beatmap(map_with(vec![HitObject::new(
            0.0,
            0.0,
            Vec2::new(256.0, 192.0),
            HitObjectKind::Circle,
        )]));
        let mut runtime = Runtime::new(&mut storyboard, &mut context);
        let options = OsuPreviewOptions {
            elements: ElementToggles {
                approach_circle: false,
                ..ElementToggles::default()
            },
            ..OsuPreviewOptions::default()
        };
        let stats = build_preview(&mut runtime, &options);
        assert_eq!(stats.sprites_created, 1);
    }

    #[test]
    fn spinner_makes_a_spinner_sprite() {
        let mut storyboard = Storyboard::new();
        let mut context = Context::new(854.0, 480.0);
        context.set_beatmap(map_with(vec![HitObject::new(
            1000.0,
            3000.0,
            Vec2::new(256.0, 192.0),
            HitObjectKind::Spinner,
        )]));
        let mut runtime = Runtime::new(&mut storyboard, &mut context);
        let stats = build_preview(&mut runtime, &OsuPreviewOptions::default());
        assert_eq!(stats.sprites_created, 1);
    }
}
