//! Imports `.osb` storyboard text into the storyboard model.
//!
//! This is the runtime equivalent of the `.osb` importer: it parses
//! `[Variables]` substitutions and `[Events]` lines (`Sprite`, `Animation`,
//! `Sample`, `T`/`L` command groups, and the `F/S/V/R/M/MX/MY/C/P` command
//! letters) and replays them onto the sprite/command model. Command groups are
//! recognised by indentation, matching the `.osb` format.

use crate::animations::Easing;
use crate::math::{Color4, Vec2};

use super::runtime::Storyboard;
use super::sprite::OsbSprite;
use super::types::{OsbLoopType, OsbOrigin, ParameterType};

/// Replays `.osb` storyboard `text` into `storyboard`.
pub fn import_osb(storyboard: &mut Storyboard, text: &str) {
    let mut variables: Vec<(String, String)> = Vec::new();
    let mut section = String::new();
    let mut current: Option<(String, usize)> = None;
    let mut in_command_group = false;

    for raw in text.lines() {
        let line = raw.trim_end_matches('\r');
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            if in_command_group {
                close_group(storyboard, &current);
                in_command_group = false;
            }
            section = trimmed[1..trimmed.len() - 1].to_string();
            continue;
        }

        match section.as_str() {
            "Variables" => {
                if let Some((key, value)) = trimmed.split_once('=') {
                    variables.push((key.trim().to_string(), value.trim().to_string()));
                }
            }
            "Events" => {
                if trimmed.starts_with("//") {
                    continue;
                }
                let depth = line.len() - line.trim_start_matches(' ').len();
                let resolved = apply_variables(trimmed, &variables);
                let values: Vec<&str> = resolved.split(',').map(str::trim).collect();
                if values.is_empty() || values[0].is_empty() {
                    continue;
                }
                if in_command_group && depth < 2 {
                    close_group(storyboard, &current);
                    in_command_group = false;
                }
                match values[0] {
                    "Sprite" => {
                        let layer = values.get(1).copied().unwrap_or("").to_string();
                        let origin = parse_origin(values.get(2).copied());
                        let path = remove_quotes(values.get(3).copied().unwrap_or(""));
                        let position = Vec2::new(
                            parse_f64(values.get(4).copied()),
                            parse_f64(values.get(5).copied()),
                        );
                        let entry = storyboard.layer(layer.clone());
                        let index = entry.len();
                        entry.create_sprite_at(path, origin, position);
                        current = Some((layer, index));
                    }
                    "Animation" => {
                        let layer = values.get(1).copied().unwrap_or("").to_string();
                        let origin = parse_origin(values.get(2).copied());
                        let path = remove_quotes(values.get(3).copied().unwrap_or(""));
                        let position = Vec2::new(
                            parse_f64(values.get(4).copied()),
                            parse_f64(values.get(5).copied()),
                        );
                        let frame_count = parse_i32(values.get(6).copied()).unwrap_or(1);
                        let frame_delay = parse_f64(values.get(7).copied());
                        let loop_type = values
                            .get(8)
                            .copied()
                            .and_then(OsbLoopType::from_name)
                            .unwrap_or(OsbLoopType::LoopForever);
                        let entry = storyboard.layer(layer.clone());
                        let index = entry.len();
                        entry.create_animation(
                            path,
                            frame_count,
                            frame_delay,
                            loop_type,
                            origin,
                            position,
                        );
                        current = Some((layer, index));
                    }
                    "Sample" => {
                        let time = parse_f64(values.get(1).copied());
                        let layer = values.get(2).copied().unwrap_or("");
                        let path = remove_quotes(values.get(3).copied().unwrap_or(""));
                        let volume = parse_f64(values.get(4).copied());
                        storyboard.layer(layer).create_sample(path, time, volume);
                    }
                    "T" => {
                        if let Some(sprite) = current_sprite(storyboard, &current) {
                            sprite.start_trigger_group(
                                values.get(1).copied().unwrap_or("").to_string(),
                                parse_f64(values.get(2).copied()),
                                parse_f64(values.get(3).copied()),
                                parse_i32(values.get(4).copied()).unwrap_or(0),
                            );
                            in_command_group = true;
                        }
                    }
                    "L" => {
                        if let Some(sprite) = current_sprite(storyboard, &current) {
                            sprite.start_loop_group(
                                parse_f64(values.get(1).copied()),
                                parse_f64(values.get(2).copied()),
                            );
                            in_command_group = true;
                        }
                    }
                    _ => apply_command(storyboard, &current, &values),
                }
            }
            _ => {}
        }
    }

    if in_command_group {
        close_group(storyboard, &current);
    }
}

/// Applies a command line (`F/S/V/R/M/MX/MY/C/P`) to the current sprite.
fn apply_command(storyboard: &mut Storyboard, current: &Option<(String, usize)>, values: &[&str]) {
    let Some(sprite) = current_sprite(storyboard, current) else {
        return;
    };
    let mut values: Vec<&str> = values.to_vec();
    // A missing end time means the command is instantaneous at its start time.
    if values.get(3).map(|v| v.is_empty()).unwrap_or(false) {
        if let Some(start) = values.get(2).copied() {
            values[3] = start;
        }
    }
    let easing = values
        .get(1)
        .and_then(|v| v.parse::<u8>().ok())
        .and_then(Easing::from_id)
        .unwrap_or(Easing::None);
    let start = parse_f64(values.get(2).copied());
    let end = parse_f64(values.get(3).copied());

    match values[0] {
        "F" => {
            let from = parse_f64(values.get(4).copied());
            let to = value_at(&values, 5, from);
            sprite.fade(start, end, from, to, easing);
        }
        "S" => {
            let from = parse_f64(values.get(4).copied());
            let to = value_at(&values, 5, from);
            sprite.scale(start, end, from, to, easing);
        }
        "V" => {
            let sx = parse_f64(values.get(4).copied());
            let sy = parse_f64(values.get(5).copied());
            let ex = value_at(&values, 6, sx);
            let ey = value_at(&values, 7, sy);
            sprite.scale_vec(start, end, Vec2::new(sx, sy), Vec2::new(ex, ey), easing);
        }
        "R" => {
            let from = parse_f64(values.get(4).copied());
            let to = value_at(&values, 5, from);
            sprite.rotate(start, end, from, to, easing);
        }
        "M" => {
            let sx = parse_f64(values.get(4).copied());
            let sy = parse_f64(values.get(5).copied());
            let ex = value_at(&values, 6, sx);
            let ey = value_at(&values, 7, sy);
            sprite.move_(start, end, Vec2::new(sx, sy), Vec2::new(ex, ey), easing);
        }
        "MX" => {
            let from = parse_f64(values.get(4).copied());
            let to = value_at(&values, 5, from);
            sprite.move_x(start, end, from, to, easing);
        }
        "MY" => {
            let from = parse_f64(values.get(4).copied());
            let to = value_at(&values, 5, from);
            sprite.move_y(start, end, from, to, easing);
        }
        "C" => {
            let sx = parse_f64(values.get(4).copied());
            let sy = parse_f64(values.get(5).copied());
            let sz = parse_f64(values.get(6).copied());
            let ex = value_at(&values, 7, sx);
            let ey = value_at(&values, 8, sy);
            let ez = value_at(&values, 9, sz);
            // `.osb` colour components are 0..255; the runtime uses 0..1.
            let from = Color4::from_rgb(sx / 255.0, sy / 255.0, sz / 255.0);
            let to = Color4::from_rgb(ex / 255.0, ey / 255.0, ez / 255.0);
            sprite.color(start, end, from, to, easing);
        }
        "P" => {
            let parameter = match values.get(4).map(|v| v.trim()) {
                Some("A") => ParameterType::Additive,
                Some("H") => ParameterType::FlipHorizontal,
                Some("V") => ParameterType::FlipVertical,
                _ => return,
            };
            sprite.parameter(start, end, parameter, 0.0, 1.0, Easing::None);
        }
        _ => {}
    }
}

fn close_group(storyboard: &mut Storyboard, current: &Option<(String, usize)>) {
    if let Some(sprite) = current_sprite(storyboard, current) {
        sprite.end_group();
    }
}

/// Borrows the current sprite. Returning a short-lived reference (rather than
/// holding one across lines) keeps the storyboard borrow checker-friendly.
fn current_sprite<'a>(
    storyboard: &'a mut Storyboard,
    current: &Option<(String, usize)>,
) -> Option<&'a mut OsbSprite> {
    let (layer, index) = current.as_ref()?;
    storyboard.layer(layer.clone()).sprite_mut(*index)
}

fn apply_variables(line: &str, variables: &[(String, String)]) -> String {
    if !line.contains('$') {
        return line.to_string();
    }
    let mut result = line.to_string();
    for (key, value) in variables {
        if !key.is_empty() {
            result = result.replace(key, value);
        }
    }
    result
}

fn remove_quotes(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

fn parse_origin(value: Option<&str>) -> OsbOrigin {
    value
        .and_then(OsbOrigin::from_name)
        .unwrap_or(OsbOrigin::Centre)
}

fn parse_f64(value: Option<&str>) -> f64 {
    value
        .map(str::trim)
        .and_then(|v| {
            if v.is_empty() {
                None
            } else {
                v.parse::<f64>().ok()
            }
        })
        .unwrap_or(0.0)
}

fn parse_i32(value: Option<&str>) -> Option<i32> {
    value.map(str::trim).and_then(|v| v.parse::<i32>().ok())
}

/// Reads the numeric value at `index`, falling back to `default` when absent/empty.
fn value_at(values: &[&str], index: usize, default: f64) -> f64 {
    values
        .get(index)
        .copied()
        .map(str::trim)
        .and_then(|v| {
            if v.is_empty() {
                None
            } else {
                v.parse::<f64>().ok()
            }
        })
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storyboarding::OsbLayer;

    const SAMPLE: &str = r#"osu file format v14

[Variables]
$base=sb/ring.png

[Events]
// comment
Sprite,Pass,Centre,"$base",320,240
 F,0,0,1000,0,1
 M,0,0,1000,320,240,400,240
 L,1000,3
  F,0,0,500,1,0
Sprite,Foreground,TopLeft,"sb/dot.png",100,100
 C,0,0,0,255,0,0,0,255,0
 T,hit,0,2000
  S,0,0,500,1,2
Animation,Pass,Centre,"sb/anim.png",320,240,10,100,LoopForever
Sample,1500,Pass,"audio/hit.wav",80
"#;

    #[test]
    fn imports_sprites_commands_loops_triggers_and_variables() {
        let mut storyboard = Storyboard::new();
        import_osb(&mut storyboard, SAMPLE);

        let pass = storyboard.layer(OsbLayer::Pass);
        assert_eq!(pass.len(), 2);
        let ring = &pass.sprites()[0];
        // Variable substitution and quoted-path stripping.
        assert_eq!(ring.texture_path, "sb/ring.png");
        assert_eq!(ring.initial_position, Vec2::new(320.0, 240.0));
        // The `L` group collected its fade command.
        assert_eq!(ring.loops().len(), 1);
        assert_eq!(ring.loops()[0].count, 3);

        // Animation metadata.
        let anim = &pass.sprites()[1];
        assert_eq!(anim.animation.map(|a| a.frame_count), Some(10));
        assert_eq!(
            anim.animation.map(|a| a.loop_type),
            Some(OsbLoopType::LoopForever)
        );

        // Samples.
        assert_eq!(pass.samples().len(), 1);
        assert_eq!(pass.samples()[0].time, 1500.0);

        let foreground = storyboard.layer(OsbLayer::Foreground);
        let dot = &foreground.sprites()[0];
        assert_eq!(dot.origin, OsbOrigin::TopLeft);
        // The `T` group collected its scale command.
        assert_eq!(dot.triggers().len(), 1);
        assert_eq!(dot.triggers()[0].trigger_name, "hit");
        assert_eq!(dot.triggers()[0].group, 0);
        assert!(!dot.triggers()[0].commands().is_empty());
    }
}
