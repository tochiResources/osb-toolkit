//! Storyboard optimization.
//!
//! Operates on the runtime storyboard model with fine-grained options:
//! **redundant (no-op) command removal**, **de-duplication**, **adjacent
//! merging** (when a command's end matches the next command's start, value and
//! easing) and **command splitting** (into `split_threshold`-millisecond
//! pieces). Also provides a lossless `.osb` **exporter** so optimized
//! storyboards round-trip back to `.osb` text.

use std::cmp::Ordering;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::animations::Easing;
use crate::storyboarding::{
    Command, CommandType, CommandValue, ExportItem, Loop, OsbSprite, ParameterTypeValue,
    Storyboard, Trigger,
};

/// The knobs controlling an optimization pass.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OptimizeOptions {
    /// Remove zero-duration commands whose value does not change.
    pub remove_redundant: bool,
    /// Remove commands identical to an earlier command of the same type.
    pub deduplicate: bool,
    /// Merge a command with the next when they are continuous.
    pub merge_adjacent: bool,
    /// Split commands longer than this many milliseconds (into linear pieces).
    pub split_threshold: Option<f64>,
}

/// The effect of an optimization pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeStats {
    /// The command count before optimizing.
    pub commands_before: usize,
    /// The command count after optimizing.
    pub commands_after: usize,
    /// Commands removed as redundant.
    pub removed: usize,
    /// Commands removed as duplicates.
    pub deduplicated: usize,
    /// Commands merged into a neighbour.
    pub merged: usize,
    /// Extra commands introduced by splitting.
    pub split: usize,
}

/// Optimizes every command timeline in `storyboard`, returning the stats.
pub fn optimize_storyboard(
    storyboard: &mut Storyboard,
    options: &OptimizeOptions,
) -> OptimizeStats {
    let mut stats = OptimizeStats::default();
    storyboard.for_each_layer_mut(|_, layer| {
        for sprite in layer.sprites_mut() {
            optimize_map(sprite.commands_mut(), options, &mut stats);
            for loop_group in sprite.loops_mut() {
                optimize_map(loop_group.commands_mut(), options, &mut stats);
            }
            for trigger in sprite.triggers_mut() {
                optimize_map(trigger.commands_mut(), options, &mut stats);
            }
        }
    });
    stats
}

fn optimize_map(
    map: &mut HashMap<CommandType, Vec<Command>>,
    options: &OptimizeOptions,
    stats: &mut OptimizeStats,
) {
    for commands in map.values_mut() {
        stats.commands_before += commands.len();
        commands.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap_or(Ordering::Equal)
        });

        let mut result: Vec<Command> = Vec::with_capacity(commands.len());
        for command in commands.drain(..) {
            if options.remove_redundant && is_redundant(&command) {
                stats.removed += 1;
                continue;
            }
            if options.deduplicate && result.contains(&command) {
                stats.deduplicated += 1;
                continue;
            }
            if options.merge_adjacent {
                if let Some(last) = result.last_mut() {
                    if can_merge(last, &command) {
                        last.end_time = command.end_time;
                        last.end_value = command.end_value;
                        stats.merged += 1;
                        continue;
                    }
                }
            }
            result.push(command);
        }

        if let Some(threshold) = options.split_threshold {
            let mut split: Vec<Command> = Vec::with_capacity(result.len());
            for command in result {
                let pieces = split_command(&command, threshold);
                if pieces.len() > 1 {
                    stats.split += pieces.len() - 1;
                }
                split.extend(pieces);
            }
            result = split;
        }

        stats.commands_after += result.len();
        *commands = result;
    }
}

fn is_redundant(command: &Command) -> bool {
    command.start_time == command.end_time && command.start_value == command.end_value
}

fn can_merge(first: &Command, second: &Command) -> bool {
    first.easing == second.easing
        && first.end_time == second.start_time
        && first.end_value == second.start_value
        && first.end_time >= first.start_time
        && second.end_time >= second.start_time
}

fn split_command(command: &Command, threshold: f64) -> Vec<Command> {
    let duration = command.end_time - command.start_time;
    if threshold <= 0.0 || duration <= threshold {
        return vec![*command];
    }
    let pieces = (duration / threshold).ceil() as usize;
    let step = duration / pieces as f64;
    (0..pieces)
        .map(|index| {
            let start = command.start_time + step * index as f64;
            let end = command.start_time + step * (index + 1) as f64;
            Command {
                start_time: start,
                end_time: end,
                easing: Easing::None,
                start_value: command.value_at(start),
                end_value: command.value_at(end),
            }
        })
        .collect()
}

/// The order command types are emitted in when exporting.
const EXPORT_ORDER: [CommandType; 10] = [
    CommandType::Move,
    CommandType::MoveX,
    CommandType::MoveY,
    CommandType::Scale,
    CommandType::ScaleVec,
    CommandType::Rotate,
    CommandType::Fade,
    CommandType::Color,
    CommandType::ColorHsb,
    CommandType::Parameter,
];

/// Exports a storyboard to `.osb` text (a lossless round-trip of the model).
pub fn export_osb(storyboard: &Storyboard) -> String {
    let mut out = String::from("osu file format v14\n\n[Events]\n");
    for item in storyboard.export_items() {
        match item {
            ExportItem::Sprite { layer, sprite } => export_sprite(&mut out, layer.name(), sprite),
            ExportItem::Sample { layer, sample } => out.push_str(&format!(
                "Sample,{},{},\"{}\",{}\n",
                number(sample.time),
                layer.name(),
                sample.path,
                number(sample.volume)
            )),
        }
    }
    out
}

fn export_sprite(out: &mut String, layer: &str, sprite: &OsbSprite) {
    let position = sprite.initial_position;
    match sprite.animation {
        Some(animation) => out.push_str(&format!(
            "Animation,{},{},\"{}\",{},{},{},{},{}\n",
            layer,
            sprite.origin.name(),
            sprite.texture_path,
            number(position.x),
            number(position.y),
            animation.frame_count,
            number(animation.frame_delay),
            animation.loop_type.name(),
        )),
        None => out.push_str(&format!(
            "Sprite,{},{},\"{}\",{},{}\n",
            layer,
            sprite.origin.name(),
            sprite.texture_path,
            number(position.x),
            number(position.y),
        )),
    }
    export_commands(out, sprite.commands(), 1);
    for loop_group in sprite.loops() {
        export_loop(out, loop_group);
    }
    for trigger in sprite.triggers() {
        export_trigger(out, trigger);
    }
}

fn export_loop(out: &mut String, loop_group: &Loop) {
    out.push_str(&format!(
        " L,{}, {}\n",
        number(loop_group.start_time),
        loop_group.count
    ));
    export_commands(out, loop_group.commands(), 2);
}

fn export_trigger(out: &mut String, trigger: &Trigger) {
    out.push_str(&format!(
        " T,{},{},{},{}\n",
        trigger.trigger_name,
        number(trigger.start_time),
        number(trigger.end_time),
        trigger.group
    ));
    export_commands(out, trigger.commands(), 2);
}

fn export_commands(out: &mut String, commands: &HashMap<CommandType, Vec<Command>>, indent: usize) {
    let pad = " ".repeat(indent);
    for kind in EXPORT_ORDER {
        let Some(list) = commands.get(&kind) else {
            continue;
        };
        for command in list {
            if let Some(line) = command_line(kind, command) {
                out.push_str(&pad);
                out.push_str(&line);
                out.push('\n');
            }
        }
    }
}

fn command_line(kind: CommandType, command: &Command) -> Option<String> {
    let easing = command.easing.id();
    let start = number(command.start_time);
    let end = number(command.end_time);
    let start_scalar = number(command.start_value.as_scalar());
    let end_scalar = number(command.end_value.as_scalar());
    let start_vec = command.start_value.as_vec2();
    let end_vec = command.end_value.as_vec2();
    let start_color = command.start_value.as_color();
    let end_color = command.end_value.as_color();
    let line = match kind {
        CommandType::Move => format!(
            "M,{easing},{start},{end},{},{},{},{}",
            number(start_vec.x),
            number(start_vec.y),
            number(end_vec.x),
            number(end_vec.y)
        ),
        CommandType::MoveX => format!("MX,{easing},{start},{end},{start_scalar},{end_scalar}"),
        CommandType::MoveY => format!("MY,{easing},{start},{end},{start_scalar},{end_scalar}"),
        CommandType::Scale => format!("S,{easing},{start},{end},{start_scalar},{end_scalar}"),
        CommandType::ScaleVec => format!(
            "V,{easing},{start},{end},{},{},{},{}",
            number(start_vec.x),
            number(start_vec.y),
            number(end_vec.x),
            number(end_vec.y)
        ),
        CommandType::Rotate => format!("R,{easing},{start},{end},{start_scalar},{end_scalar}"),
        CommandType::Fade => format!("F,{easing},{start},{end},{start_scalar},{end_scalar}"),
        CommandType::Color | CommandType::ColorHsb => format!(
            "C,{easing},{start},{end},{},{},{},{},{},{}",
            channel(start_color.r),
            channel(start_color.g),
            channel(start_color.b),
            channel(end_color.r),
            channel(end_color.g),
            channel(end_color.b)
        ),
        CommandType::Parameter => {
            let letter = parameter_letter(&command.start_value)?;
            format!("P,{easing},{start},{end},{letter}")
        }
        // The anchor channel is a runtime-only extension; it has no `.osb` form.
        CommandType::Anchor | CommandType::Loop | CommandType::Trigger => return None,
    };
    Some(line)
}

fn parameter_letter(value: &CommandValue) -> Option<&'static str> {
    match value {
        CommandValue::Parameter(ParameterTypeValue::Additive(_)) => Some("A"),
        CommandValue::Parameter(ParameterTypeValue::FlipHorizontal(_)) => Some("H"),
        CommandValue::Parameter(ParameterTypeValue::FlipVertical(_)) => Some("V"),
        _ => None,
    }
}

fn channel(value: f64) -> String {
    number((value * 255.0).round())
}

/// Formats a float without a trailing `.0` (matching the `.osb` style).
fn number(value: f64) -> String {
    if value == value.trunc() && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;
    use crate::storyboarding::{OsbOrigin, Storyboard};

    fn storyboard() -> Storyboard {
        let mut storyboard = Storyboard::new();
        let sprite = storyboard
            .layer("Foreground")
            .create_sprite("sb/dot.png", OsbOrigin::Centre);
        sprite.fade(0.0, 100.0, 0.0, 1.0, Easing::None);
        sprite.fade(0.0, 100.0, 0.0, 1.0, Easing::None); // duplicate
        sprite.fade(200.0, 200.0, 1.0, 1.0, Easing::None); // redundant no-op
        sprite.move_(
            100.0,
            200.0,
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Easing::None,
        );
        sprite.move_(
            200.0,
            300.0,
            Vec2::new(10.0, 0.0),
            Vec2::new(20.0, 0.0),
            Easing::None,
        );
        storyboard
    }

    #[test]
    fn removes_duplicates_and_redundant_commands() {
        let mut storyboard = storyboard();
        let stats = optimize_storyboard(
            &mut storyboard,
            &OptimizeOptions {
                remove_redundant: true,
                deduplicate: true,
                merge_adjacent: true,
                split_threshold: None,
            },
        );
        assert_eq!(stats.removed, 1);
        assert_eq!(stats.deduplicated, 1);
        assert_eq!(stats.merged, 1); // the two moves are continuous
        assert_eq!(stats.commands_after, 2);
    }

    #[test]
    fn splits_long_commands() {
        let mut storyboard = Storyboard::new();
        let sprite = storyboard
            .layer("Foreground")
            .create_sprite("sb/dot.png", OsbOrigin::Centre);
        sprite.move_(
            0.0,
            1000.0,
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Easing::None,
        );
        let stats = optimize_storyboard(
            &mut storyboard,
            &OptimizeOptions {
                split_threshold: Some(250.0),
                ..Default::default()
            },
        );
        assert_eq!(stats.split, 3); // 1000ms -> 4 x 250ms
        assert_eq!(stats.commands_after, 4);
    }

    #[test]
    fn osb_export_round_trips() {
        let storyboard = storyboard();
        let text = export_osb(&storyboard);
        let mut reloaded = Storyboard::new();
        crate::storyboarding::import_osb(&mut reloaded, &text);

        let original = storyboard.sprite_count();
        assert_eq!(original, reloaded.sprite_count());
        let before = storyboard
            .layers()
            .flat_map(|layer| layer.sprites())
            .map(|sprite| sprite.commands().values().map(Vec::len).sum::<usize>())
            .sum::<usize>();
        let after = reloaded
            .layers()
            .flat_map(|layer| layer.sprites())
            .map(|sprite| sprite.commands().values().map(Vec::len).sum::<usize>())
            .sum::<usize>();
        assert_eq!(before, after);
    }
}
