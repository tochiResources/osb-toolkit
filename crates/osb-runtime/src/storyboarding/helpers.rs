//! Pure command-list utilities over the 2D storyboard model.
//!
//! These are the canonical, state-free transforms an effect author reaches for
//! when a command list needs selecting, retiming, reversing, merging, splitting,
//! randomizing, aligning or quantizing. Commands live in per-channel lists in
//! the runtime, so a "command set" is a list of `(CommandType, Command)` pairs -
//! the kind travels with the command. The helpers take `&[KindedCommand]` and
//! return new lists (or mutate in place for the timing ops), so they compose and
//! are shared by the wire bridge and every SDK facade.

use crate::animations::keyframes::{Channel, FlattenMode};
use crate::animations::timeline::Timeline;
use crate::animations::{Easing, KeyframedValue};
use crate::context::Rng;
use crate::math::{Color4, Vec2};

use super::{Command, CommandType, CommandValue, ParameterTypeValue};

/// A command together with the channel it belongs to.
pub type KindedCommand = (CommandType, Command);

/// Keeps commands of `kind` (when given) that overlap `[start, end]` (when
/// given). The source is not mutated.
pub fn select_commands(
    commands: &[KindedCommand],
    kind: Option<CommandType>,
    start: Option<f64>,
    end: Option<f64>,
) -> Vec<KindedCommand> {
    commands
        .iter()
        .filter(|(command_kind, command)| {
            if let Some(kind) = kind {
                if *command_kind != kind {
                    return false;
                }
            }
            if let Some(start) = start {
                if command.end_time < start {
                    return false;
                }
            }
            if let Some(end) = end {
                if command.start_time > end {
                    return false;
                }
            }
            true
        })
        .copied()
        .collect()
}

/// Adds `delta` milliseconds to every command's times.
pub fn shift_commands(commands: &mut [KindedCommand], delta: f64) {
    for (_, command) in commands.iter_mut() {
        command.start_time += delta;
        command.end_time += delta;
    }
}

/// Scales command times about `anchor` by `factor`.
pub fn scale_time_commands(commands: &mut [KindedCommand], factor: f64, anchor: f64) {
    for (_, command) in commands.iter_mut() {
        command.start_time = anchor + (command.start_time - anchor) * factor;
        command.end_time = anchor + (command.end_time - anchor) * factor;
    }
}

/// Reverses the command set in time about the `[start, end]` span, swapping each
/// command's endpoint values. Commands outside the span are left alone.
pub fn reverse_commands(commands: &[KindedCommand], start: f64, end: f64) -> Vec<KindedCommand> {
    commands
        .iter()
        .map(|(kind, command)| {
            if command.start_time < start || command.end_time > end {
                return (*kind, *command);
            }
            (
                *kind,
                Command {
                    start_time: start + end - command.end_time,
                    end_time: start + end - command.start_time,
                    easing: command.easing,
                    start_value: command.end_value,
                    end_value: command.start_value,
                },
            )
        })
        .collect()
}

/// Merges adjacent commands of the same kind that are continuous in time and
/// value (within `tolerance`) into one span.
pub fn merge_commands(commands: &[KindedCommand], tolerance: f64) -> Vec<KindedCommand> {
    let mut source: Vec<KindedCommand> = commands.to_vec();
    source.sort_by(|a, b| {
        a.1.start_time
            .total_cmp(&b.1.start_time)
            .then(a.1.end_time.total_cmp(&b.1.end_time))
    });
    let mut out: Vec<KindedCommand> = Vec::new();
    for (kind, command) in source {
        if let Some((last_kind, last)) = out.last_mut() {
            let touching = (last.end_time - command.start_time).abs() <= tolerance;
            let continuous = values_close(&last.end_value, &command.start_value, tolerance);
            if *last_kind == kind && touching && continuous {
                last.end_time = command.end_time;
                last.end_value = command.end_value;
                continue;
            }
        }
        out.push((kind, command));
    }
    out
}

/// Splits any command spanning `at` into two linear pieces at `at`.
pub fn split_commands(commands: &[KindedCommand], at: f64) -> Vec<KindedCommand> {
    let mut out = Vec::new();
    for (kind, command) in commands {
        if command.start_time < at && at < command.end_time {
            let mid = command.value_at(at);
            out.push((
                *kind,
                Command {
                    start_time: command.start_time,
                    end_time: at,
                    easing: Easing::None,
                    start_value: command.start_value,
                    end_value: mid,
                },
            ));
            out.push((
                *kind,
                Command {
                    start_time: at,
                    end_time: command.end_time,
                    easing: Easing::None,
                    start_value: mid,
                    end_value: command.end_value,
                },
            ));
        } else {
            out.push((*kind, *command));
        }
    }
    out
}

/// Collapses commands of `kind` (when given) to a single constant span.
pub fn flatten_commands(
    commands: &[KindedCommand],
    kind: Option<CommandType>,
    mode: FlattenMode,
) -> Vec<KindedCommand> {
    let mut other: Vec<KindedCommand> = Vec::new();
    let mut matching: Vec<&KindedCommand> = Vec::new();
    for entry in commands {
        if kind.map(|k| k == entry.0).unwrap_or(true) {
            matching.push(entry);
        } else {
            other.push(*entry);
        }
    }
    let Some(first) = matching.first() else {
        return other;
    };
    let start = matching
        .iter()
        .map(|(_, command)| command.start_time)
        .fold(f64::MAX, f64::min);
    let end = matching
        .iter()
        .map(|(_, command)| command.end_time)
        .fold(f64::MIN, f64::max);
    let constant_value = match mode {
        FlattenMode::First => first.1.start_value,
        FlattenMode::Last => matching.last().unwrap().1.end_value,
        FlattenMode::Value(value) => scalar_like(&first.1.start_value, value),
        FlattenMode::Mean => {
            let values: Vec<CommandValue> = matching
                .iter()
                .map(|(_, command)| command.end_value)
                .collect();
            mean_value(&values, &first.1.start_value)
        }
    };
    let mut result = other;
    result.push((
        first.0,
        Command {
            start_time: start,
            end_time: end,
            easing: Easing::None,
            start_value: constant_value,
            end_value: constant_value,
        },
    ));
    result
}

/// Adds a seeded jitter in `min..max` to every scalar-valued command endpoint.
pub fn randomize_commands(
    commands: &[KindedCommand],
    min: f64,
    max: f64,
    seed: u64,
) -> Vec<KindedCommand> {
    let mut rng = Rng::new(seed);
    commands
        .iter()
        .map(|(kind, command)| {
            (
                *kind,
                Command {
                    start_value: jitter(&command.start_value, &mut rng, min, max),
                    end_value: jitter(&command.end_value, &mut rng, min, max),
                    ..*command
                },
            )
        })
        .collect()
}

/// Applies `f` to each command, producing a new list.
pub fn map_commands(
    commands: &[KindedCommand],
    f: impl Fn(&KindedCommand) -> KindedCommand,
) -> Vec<KindedCommand> {
    commands.iter().map(f).collect()
}

/// Keeps the commands a predicate accepts.
pub fn filter_commands(
    commands: &[KindedCommand],
    keep: impl Fn(&KindedCommand) -> bool,
) -> Vec<KindedCommand> {
    commands
        .iter()
        .filter(|entry| keep(entry))
        .copied()
        .collect()
}

/// Removes the commands a predicate accepts.
pub fn remove_commands(
    commands: &[KindedCommand],
    remove: impl Fn(&KindedCommand) -> bool,
) -> Vec<KindedCommand> {
    commands
        .iter()
        .filter(|entry| !remove(entry))
        .copied()
        .collect()
}

/// Clones the command set `count` times, each copy shifted by `offset` ms.
pub fn duplicate_commands(
    commands: &[KindedCommand],
    offset: f64,
    count: usize,
) -> Vec<KindedCommand> {
    let mut out = Vec::with_capacity(commands.len() * count.max(1));
    for copy in 0..count.max(1) {
        let shift = offset * copy as f64;
        for (kind, command) in commands {
            out.push((
                *kind,
                Command {
                    start_time: command.start_time + shift,
                    end_time: command.end_time + shift,
                    ..*command
                },
            ));
        }
    }
    out
}

/// Snaps command times to a beat grid and their scalar values to `value_step`
/// (when `value_step > 0`).
pub fn align_commands(
    commands: &[KindedCommand],
    beat: f64,
    offset: f64,
    subdivision: u32,
    value_step: f64,
) -> Vec<KindedCommand> {
    let subdivision = subdivision.max(1);
    let step = beat / subdivision as f64;
    commands
        .iter()
        .map(|(kind, command)| {
            let snap = |time: f64| {
                if step <= 0.0 {
                    time
                } else {
                    offset + ((time - offset) / step).round() * step
                }
            };
            (
                *kind,
                Command {
                    start_time: snap(command.start_time),
                    end_time: snap(command.end_time),
                    start_value: quantize_value(command.start_value, value_step),
                    end_value: quantize_value(command.end_value, value_step),
                    ..*command
                },
            )
        })
        .collect()
}

/// Rounds command times and scalar values to `time_step`/`value_step` multiples.
pub fn quantize_commands(
    commands: &[KindedCommand],
    time_step: f64,
    value_step: f64,
) -> Vec<KindedCommand> {
    let round = |value: f64, step: f64| {
        if step > 0.0 {
            (value / step).round() * step
        } else {
            value
        }
    };
    commands
        .iter()
        .map(|(kind, command)| {
            (
                *kind,
                Command {
                    start_time: round(command.start_time, time_step),
                    end_time: round(command.end_time, time_step),
                    start_value: quantize_value(command.start_value, value_step),
                    end_value: quantize_value(command.end_value, value_step),
                    ..*command
                },
            )
        })
        .collect()
}

/// Applies a scalar value transform to every command endpoint (used by
/// `mapCommands`): `value -> value * scale + offset`.
pub fn transform_command_values(
    commands: &[KindedCommand],
    scale: f64,
    offset: f64,
) -> Vec<KindedCommand> {
    commands
        .iter()
        .map(|(kind, command)| {
            (
                *kind,
                Command {
                    start_value: transform_value(command.start_value, scale, offset),
                    end_value: transform_value(command.end_value, scale, offset),
                    ..*command
                },
            )
        })
        .collect()
}

// --- command <-> keyframe channel bridging --------------------------
//
// The shared keyframe helpers in `crate::animations::keyframes` operate on a
// dimension-agnostic `Channel`. These functions extract a command kind's values
// into such a channel and write a transformed channel back as commands, so a 2D
// command channel can use exactly the same helpers as a 3D node channel.

/// Extracts the keyframe channel for one command kind from a command list.
/// Returns `None` when there are no commands of that kind, or the kind's value is
/// not a supported channel.
///
/// Every command kind is supported because they are all keyframe channels at
/// heart: `Move`/`ScaleVec`/`Anchor` are 2-vectors, `Color`/`ColorHsb` are RGBA
/// colors, and the scalar kinds (`MoveX`/`MoveY`/`Scale`/`Rotate`/`Fade`/
/// `Parameter`) are one-component channels.
pub fn command_channel(commands: &[KindedCommand], kind: CommandType) -> Option<Channel> {
    let mut scalars: Vec<(f64, f64, Easing)> = Vec::new();
    let mut vectors: Vec<(f64, Vec2, Easing)> = Vec::new();
    let mut colors: Vec<(f64, Color4, Easing)> = Vec::new();
    let mut shape = ChannelShape::Scalar;
    let mut any = false;
    for (command_kind, command) in commands {
        if *command_kind != kind {
            continue;
        }
        any = true;
        let easing = command.easing;
        match (&command.start_value, &command.end_value) {
            (CommandValue::Vector2(a), CommandValue::Vector2(b)) => {
                shape = ChannelShape::Vector2;
                vectors.push((command.start_time, *a, easing));
                vectors.push((command.end_time, *b, easing));
            }
            (CommandValue::Color(a), CommandValue::Color(b)) => {
                shape = ChannelShape::Color;
                colors.push((command.start_time, *a, easing));
                colors.push((command.end_time, *b, easing));
            }
            (CommandValue::Parameter(a), CommandValue::Parameter(b)) => {
                // A parameter payload is numeric; its sub-kind (flip/additive)
                // is carried by the command kind, not the value.
                scalars.push((command.start_time, a.value(), easing));
                scalars.push((command.end_time, b.value(), easing));
            }
            (CommandValue::Scalar(a), CommandValue::Scalar(b)) => {
                scalars.push((command.start_time, *a, easing));
                scalars.push((command.end_time, *b, easing));
            }
            _ => return None,
        }
    }
    if !any {
        return None;
    }
    Some(match shape {
        ChannelShape::Color => {
            let mut kv = KeyframedValue::new();
            for (time, value, easing) in colors {
                kv.add_eased(time, value, easing);
            }
            Channel::Color(kv)
        }
        ChannelShape::Vector2 => {
            let mut kv = KeyframedValue::new();
            for (time, value, easing) in vectors {
                kv.add_eased(time, value, easing);
            }
            Channel::Vector2(kv)
        }
        ChannelShape::Scalar => {
            let mut kv = KeyframedValue::new();
            for (time, value, easing) in scalars {
                kv.add_eased(time, value, easing);
            }
            Channel::Scalar(kv)
        }
    })
}

/// The channel shape a command kind's values take.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChannelShape {
    Scalar,
    Vector2,
    Color,
}

/// Writes a transformed channel back as commands of `kind`, replacing the
/// existing commands of that kind (each consecutive keyframe pair becomes a
/// linear command).
pub fn apply_command_channel(
    commands: &[KindedCommand],
    kind: CommandType,
    channel: &Channel,
) -> Vec<KindedCommand> {
    apply_command_channel_with(commands, kind, channel, |_, _| Easing::None)
}

/// Bridges a command channel through a keyframe transform: extract `kind`'s
/// channel, apply `f`, and write it back as commands.
pub fn transform_command_channel(
    commands: &[KindedCommand],
    kind: CommandType,
    f: impl FnOnce(Channel) -> Channel,
) -> Option<Vec<KindedCommand>> {
    let channel = command_channel(commands, kind)?;
    Some(apply_command_channel(commands, kind, &f(channel)))
}

/// Bakes a timeline (a set of keyframe clips, flushed to absolute time) into
/// commands of `kind`, so a timeline is usable wherever a command set is.
pub fn timeline_commands(timeline: &Timeline, kind: CommandType) -> Vec<KindedCommand> {
    let flushed = timeline.flush();
    let mut out = Vec::new();
    for clip in &flushed.clips {
        out.extend(apply_command_channel(&[], kind, &clip.channel));
    }
    out.sort_by(|a, b| a.1.start_time.total_cmp(&b.1.start_time));
    out
}

// --- easing-aware bridging and group/scene easing -------------------

/// The per-command easings of `kind`, in command order.
pub fn command_easings(commands: &[KindedCommand], kind: CommandType) -> Vec<Easing> {
    commands
        .iter()
        .filter(|(command_kind, _)| *command_kind == kind)
        .map(|(_, command)| command.easing)
        .collect()
}

/// Like [`apply_command_channel`], but re-attaches the **original easing** of the
/// command whose interval contains each resolved segment, so a transform returns
/// the same easing shape with resolved commands instead of `Easing::None`.
pub fn apply_command_channel_eased(
    commands: &[KindedCommand],
    kind: CommandType,
    channel: &Channel,
    easings: &[Easing],
) -> Vec<KindedCommand> {
    let matching: Vec<&Command> = commands
        .iter()
        .filter(|(command_kind, _)| *command_kind == kind)
        .map(|(_, command)| command)
        .collect();
    let easing_at = |time: f64| -> Easing {
        matching
            .iter()
            .find(|command| time >= command.start_time && time < command.end_time)
            .or_else(|| matching.last())
            .map(|command| command.easing)
            .unwrap_or(Easing::None)
    };
    let fallback = easings.first().copied().unwrap_or(Easing::None);
    apply_command_channel_with(commands, kind, channel, |time, index| {
        if matching.is_empty() || (index == 0 && channel.len() == 1) {
            fallback
        } else {
            easing_at(time)
        }
    })
}

/// Sets `easing` on every command (apply an easing to a whole group/scene).
pub fn set_easing(commands: &[KindedCommand], easing: Easing) -> Vec<KindedCommand> {
    commands
        .iter()
        .map(|(kind, command)| (*kind, Command { easing, ..*command }))
        .collect()
}

/// Re-eases a command channel with a named easing, resolving it into commands
/// (inserting `samples` intermediate keyframes per original span).
pub fn retime_easing(
    commands: &[KindedCommand],
    kind: CommandType,
    easing: Easing,
    samples: usize,
) -> Option<Vec<KindedCommand>> {
    let easings = command_easings(commands, kind);
    let channel = command_channel(commands, kind)?.ease(easing, samples);
    Some(apply_command_channel_eased(
        commands, kind, &channel, &easings,
    ))
}

/// Re-eases a command channel with a cubic-bezier, resolving it into commands.
#[allow(clippy::too_many_arguments)]
pub fn retime_bezier(
    commands: &[KindedCommand],
    kind: CommandType,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    samples: usize,
) -> Option<Vec<KindedCommand>> {
    let easings = command_easings(commands, kind);
    let channel = command_channel(commands, kind)?.ease_bezier(x1, y1, x2, y2, samples);
    Some(apply_command_channel_eased(
        commands, kind, &channel, &easings,
    ))
}

/// Shared implementation of the command-channel writers: `easing_for(time, index)`
/// picks the easing attached to a resolved segment.
fn apply_command_channel_with(
    commands: &[KindedCommand],
    kind: CommandType,
    channel: &Channel,
    easing_for: impl Fn(f64, usize) -> Easing,
) -> Vec<KindedCommand> {
    let mut range: Option<(f64, f64)> = None;
    for (command_kind, command) in commands {
        if *command_kind != kind {
            continue;
        }
        range = Some(match range {
            Some((start, end)) => (start.min(command.start_time), end.max(command.end_time)),
            None => (command.start_time, command.end_time),
        });
    }
    let mut out: Vec<KindedCommand> = commands
        .iter()
        .filter(|(command_kind, _)| *command_kind != kind)
        .cloned()
        .collect();
    let template = parameter_template(commands, kind);
    let keys = channel_keys(channel, kind, template);
    // A channel key carries the easing of its outgoing segment; when it is linear
    // (`None`), the caller's `easing_for` fallback applies (e.g. a flattened
    // channel re-attaches the original command's easing).
    let pick = |key_easing: Easing, time: f64, index: usize| {
        if key_easing != Easing::None {
            key_easing
        } else {
            easing_for(time, index)
        }
    };
    if keys.len() == 1 {
        if let Some((start, end)) = range {
            let easing = pick(keys[0].2, start, 0);
            out.push((
                kind,
                Command {
                    start_time: start,
                    end_time: end,
                    easing,
                    start_value: keys[0].1,
                    end_value: keys[0].1,
                },
            ));
        }
    } else {
        for (index, pair) in keys.windows(2).enumerate() {
            let easing = pick(pair[0].2, pair[0].0, index);
            out.push((
                kind,
                Command {
                    start_time: pair[0].0,
                    end_time: pair[1].0,
                    easing,
                    start_value: pair[0].1,
                    end_value: pair[1].1,
                },
            ));
        }
    }
    out.sort_by(|a, b| a.1.start_time.total_cmp(&b.1.start_time));
    out
}

fn channel_keys(
    channel: &Channel,
    kind: CommandType,
    template: Option<ParameterTypeValue>,
) -> Vec<(f64, CommandValue, Easing)> {
    match channel {
        Channel::Scalar(kv) => kv
            .keyframes()
            .iter()
            .map(|key| {
                (
                    key.time,
                    coerce_value(kind, CommandValue::Scalar(key.value), template),
                    key.easing,
                )
            })
            .collect(),
        Channel::Vector2(kv) => kv
            .keyframes()
            .iter()
            .map(|key| (key.time, CommandValue::Vector2(key.value), key.easing))
            .collect(),
        Channel::Vector3(kv) => kv
            .keyframes()
            .iter()
            .map(|key| {
                (
                    key.time,
                    CommandValue::Vector2(Vec2::new(key.value.x, key.value.y)),
                    key.easing,
                )
            })
            .collect(),
        Channel::Color(kv) => kv
            .keyframes()
            .iter()
            .map(|key| (key.time, CommandValue::Color(key.value), key.easing))
            .collect(),
    }
}

/// A parameter command's sub-kind, taken from an existing command of that kind so
/// a resolved numeric channel is written back as the right parameter.
fn parameter_template(commands: &[KindedCommand], kind: CommandType) -> Option<ParameterTypeValue> {
    commands.iter().find_map(|(command_kind, command)| {
        if *command_kind != kind {
            return None;
        }
        match command.start_value {
            CommandValue::Parameter(value) => Some(value),
            _ => None,
        }
    })
}

/// Wraps a resolved value into the shape `kind` expects: a numeric channel for a
/// `Parameter` kind becomes a parameter payload with the command's sub-kind.
fn coerce_value(
    kind: CommandType,
    value: CommandValue,
    template: Option<ParameterTypeValue>,
) -> CommandValue {
    if kind != CommandType::Parameter {
        return value;
    }
    let scalar = value.as_scalar();
    CommandValue::Parameter(match template {
        Some(ParameterTypeValue::FlipHorizontal(_)) => ParameterTypeValue::FlipHorizontal(scalar),
        Some(ParameterTypeValue::FlipVertical(_)) => ParameterTypeValue::FlipVertical(scalar),
        _ => ParameterTypeValue::Additive(scalar),
    })
}

/// Builds a `CommandValue` of the same shape as `template` holding `scalar`.
fn scalar_like(template: &CommandValue, scalar: f64) -> CommandValue {
    match template {
        CommandValue::Scalar(_) => CommandValue::Scalar(scalar),
        CommandValue::Vector2(_) => CommandValue::Vector2(Vec2::new(scalar, scalar)),
        CommandValue::Color(_) => CommandValue::Color(Color4::new(scalar, scalar, scalar, 1.0)),
        CommandValue::Parameter(ParameterTypeValue::FlipHorizontal(_)) => {
            CommandValue::Parameter(ParameterTypeValue::FlipHorizontal(scalar))
        }
        CommandValue::Parameter(ParameterTypeValue::FlipVertical(_)) => {
            CommandValue::Parameter(ParameterTypeValue::FlipVertical(scalar))
        }
        CommandValue::Parameter(ParameterTypeValue::Additive(_)) => {
            CommandValue::Parameter(ParameterTypeValue::Additive(scalar))
        }
        CommandValue::None => CommandValue::None,
    }
}

/// The component-wise mean of a set of command values, shaped like `template`.
fn mean_value(values: &[CommandValue], template: &CommandValue) -> CommandValue {
    if values.is_empty() {
        return *template;
    }
    let n = values.len() as f64;
    match template {
        CommandValue::Color(_) => {
            let (mut r, mut g, mut b, mut a) = (0.0, 0.0, 0.0, 0.0);
            for value in values {
                let color = value.as_color();
                r += color.r;
                g += color.g;
                b += color.b;
                a += color.a;
            }
            CommandValue::Color(Color4::new(r / n, g / n, b / n, a / n))
        }
        CommandValue::Vector2(_) => {
            let (mut x, mut y) = (0.0, 0.0);
            for value in values {
                let vector = value.as_vec2();
                x += vector.x;
                y += vector.y;
            }
            CommandValue::Vector2(Vec2::new(x / n, y / n))
        }
        _ => {
            let sum: f64 = values.iter().map(CommandValue::as_scalar).sum();
            scalar_like(template, sum / n)
        }
    }
}

fn values_close(a: &CommandValue, b: &CommandValue, tolerance: f64) -> bool {
    match (a, b) {
        (CommandValue::Scalar(x), CommandValue::Scalar(y)) => (x - y).abs() <= tolerance,
        (CommandValue::Vector2(x), CommandValue::Vector2(y)) => x.distance(*y) <= tolerance,
        (CommandValue::Color(x), CommandValue::Color(y)) => {
            (x.r - y.r)
                .abs()
                .max((x.g - y.g).abs())
                .max((x.b - y.b).abs())
                <= tolerance
        }
        (CommandValue::Parameter(x), CommandValue::Parameter(y)) => {
            (x.value() - y.value()).abs() <= tolerance
        }
        _ => false,
    }
}

fn jitter(value: &CommandValue, rng: &mut Rng, min: f64, max: f64) -> CommandValue {
    let delta = rng.range(min, max);
    transform_value(*value, 1.0, delta)
}

fn transform_value(value: CommandValue, scale: f64, offset: f64) -> CommandValue {
    let apply = |v: f64| v * scale + offset;
    match value {
        CommandValue::Scalar(v) => CommandValue::Scalar(apply(v)),
        CommandValue::Vector2(v) => CommandValue::Vector2(Vec2::new(apply(v.x), apply(v.y))),
        CommandValue::Color(c) => {
            CommandValue::Color(Color4::new(apply(c.r), apply(c.g), apply(c.b), c.a))
        }
        CommandValue::Parameter(ParameterTypeValue::FlipHorizontal(v)) => {
            CommandValue::Parameter(ParameterTypeValue::FlipHorizontal(apply(v)))
        }
        CommandValue::Parameter(ParameterTypeValue::FlipVertical(v)) => {
            CommandValue::Parameter(ParameterTypeValue::FlipVertical(apply(v)))
        }
        CommandValue::Parameter(ParameterTypeValue::Additive(v)) => {
            CommandValue::Parameter(ParameterTypeValue::Additive(apply(v)))
        }
        CommandValue::None => CommandValue::None,
    }
}

fn quantize_value(value: CommandValue, step: f64) -> CommandValue {
    if step <= 0.0 {
        return value;
    }
    let round = |v: f64| (v / step).round() * step;
    match value {
        CommandValue::Scalar(v) => CommandValue::Scalar(round(v)),
        CommandValue::Vector2(v) => CommandValue::Vector2(Vec2::new(round(v.x), round(v.y))),
        CommandValue::Color(c) => {
            CommandValue::Color(Color4::new(round(c.r), round(c.g), round(c.b), round(c.a)))
        }
        CommandValue::Parameter(ParameterTypeValue::FlipHorizontal(v)) => {
            CommandValue::Parameter(ParameterTypeValue::FlipHorizontal(round(v)))
        }
        CommandValue::Parameter(ParameterTypeValue::FlipVertical(v)) => {
            CommandValue::Parameter(ParameterTypeValue::FlipVertical(round(v)))
        }
        CommandValue::Parameter(ParameterTypeValue::Additive(v)) => {
            CommandValue::Parameter(ParameterTypeValue::Additive(round(v)))
        }
        CommandValue::None => CommandValue::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fade(start: f64, end: f64, a: f64, b: f64) -> KindedCommand {
        (
            CommandType::Fade,
            Command::new(
                start,
                end,
                Easing::None,
                CommandValue::Scalar(a),
                CommandValue::Scalar(b),
            ),
        )
    }

    #[test]
    fn select_filters_by_time_and_kind() {
        let commands = vec![
            fade(0.0, 100.0, 0.0, 1.0),
            fade(200.0, 300.0, 1.0, 0.0),
            (
                CommandType::Move,
                Command::new(
                    0.0,
                    100.0,
                    Easing::None,
                    CommandValue::Vector2(Vec2::ZERO),
                    CommandValue::Vector2(Vec2::ONE),
                ),
            ),
        ];
        let selected = select_commands(&commands, Some(CommandType::Fade), Some(150.0), None);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].1.start_time, 200.0);
        let moves = select_commands(&commands, Some(CommandType::Move), None, None);
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn shift_and_scale_time() {
        let mut commands = vec![fade(100.0, 200.0, 0.0, 1.0)];
        shift_commands(&mut commands, 50.0);
        assert_eq!(commands[0].1.start_time, 150.0);
        scale_time_commands(&mut commands, 2.0, 0.0);
        assert_eq!(commands[0].1.start_time, 300.0);
        assert_eq!(commands[0].1.end_time, 500.0);
    }

    #[test]
    fn reverse_mirrors_times_and_swaps_values() {
        let commands = vec![fade(0.0, 100.0, 0.0, 1.0)];
        let reversed = reverse_commands(&commands, 0.0, 100.0);
        assert_eq!(reversed[0].1.start_time, 0.0);
        assert_eq!(reversed[0].1.end_time, 100.0);
        assert_eq!(reversed[0].1.start_value, CommandValue::Scalar(1.0));
    }

    #[test]
    fn merge_joins_continuous_spans() {
        let commands = vec![fade(0.0, 100.0, 0.0, 1.0), fade(100.0, 200.0, 1.0, 0.0)];
        let merged = merge_commands(&commands, 0.0);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].1.end_time, 200.0);
    }

    #[test]
    fn merge_ignores_different_kinds() {
        let commands = vec![
            fade(0.0, 100.0, 0.0, 1.0),
            (
                CommandType::Move,
                Command::new(
                    100.0,
                    200.0,
                    Easing::None,
                    CommandValue::Scalar(1.0),
                    CommandValue::Scalar(0.0),
                ),
            ),
        ];
        let merged = merge_commands(&commands, 0.0);
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn split_breaks_a_span_at_a_time() {
        let commands = vec![fade(0.0, 100.0, 0.0, 1.0)];
        let split = split_commands(&commands, 50.0);
        assert_eq!(split.len(), 2);
        assert_eq!(split[0].1.end_value, CommandValue::Scalar(0.5));
        assert_eq!(split[1].1.start_value, CommandValue::Scalar(0.5));
    }

    #[test]
    fn flatten_collapses_to_a_constant() {
        let commands = vec![fade(0.0, 100.0, 0.0, 1.0), fade(100.0, 200.0, 1.0, 0.0)];
        let flattened = flatten_commands(&commands, Some(CommandType::Fade), FlattenMode::Last);
        assert_eq!(flattened.len(), 1);
        assert_eq!(flattened[0].1.start_value, CommandValue::Scalar(0.0));
        assert_eq!(flattened[0].1.start_time, 0.0);
        assert_eq!(flattened[0].1.end_time, 200.0);
    }

    #[test]
    fn randomize_is_seeded() {
        let commands = vec![fade(0.0, 100.0, 0.0, 0.0)];
        let a = randomize_commands(&commands, -1.0, 1.0, 42);
        let b = randomize_commands(&commands, -1.0, 1.0, 42);
        assert_eq!(a, b);
    }

    #[test]
    fn duplicate_offsets_each_copy() {
        let commands = vec![fade(0.0, 100.0, 0.0, 1.0)];
        let copies = duplicate_commands(&commands, 100.0, 3);
        assert_eq!(copies.len(), 3);
        assert_eq!(copies[2].1.start_time, 200.0);
    }

    #[test]
    fn align_and_quantize() {
        let commands = vec![fade(10.0, 130.0, 0.3, 0.7)];
        let aligned = align_commands(&commands, 100.0, 0.0, 2, 0.0);
        assert_eq!(aligned[0].1.start_time, 0.0);
        assert_eq!(aligned[0].1.end_time, 150.0);
        let quantized = quantize_commands(&commands, 50.0, 0.5);
        assert_eq!(quantized[0].1.start_value, CommandValue::Scalar(0.5));
    }

    #[test]
    fn map_transform_and_filter() {
        let commands = vec![fade(0.0, 100.0, 0.0, 1.0), fade(200.0, 300.0, 1.0, 0.0)];
        let scaled = transform_command_values(&commands, 2.0, 0.5);
        assert_eq!(scaled[0].1.start_value, CommandValue::Scalar(0.5));
        assert_eq!(scaled[0].1.end_value, CommandValue::Scalar(2.5));
        let filtered = filter_commands(&commands, |(_, command)| command.start_time < 100.0);
        assert_eq!(filtered.len(), 1);
        let removed = remove_commands(&commands, |(_, command)| command.start_time < 100.0);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0].1.start_time, 200.0);
        let mapped = map_commands(&commands, |(kind, command)| {
            (
                *kind,
                Command {
                    start_time: command.start_time + 1.0,
                    ..*command
                },
            )
        });
        assert_eq!(mapped[0].1.start_time, 1.0);
    }

    fn color_command(start: f64, end: f64, a: Color4, b: Color4) -> KindedCommand {
        (
            CommandType::Color,
            Command::new(
                start,
                end,
                Easing::None,
                CommandValue::Color(a),
                CommandValue::Color(b),
            ),
        )
    }

    #[test]
    fn color_command_channels_bridge_like_vectors() {
        let commands = vec![color_command(
            0.0,
            100.0,
            Color4::new(0.0, 0.0, 0.0, 1.0),
            Color4::new(1.0, 0.5, 0.25, 1.0),
        )];
        let channel = command_channel(&commands, CommandType::Color).unwrap();
        assert_eq!(channel.component_count(), 4);
        // Scaling the color doubles every component.
        let scaled = channel.scale(2.0);
        let rebuilt = apply_command_channel(&commands, CommandType::Color, &scaled);
        assert_eq!(rebuilt.len(), 1);
        assert_eq!(
            rebuilt[0].1.end_value,
            CommandValue::Color(Color4::new(2.0, 1.0, 0.5, 2.0))
        );
    }

    #[test]
    fn parameter_command_channels_keep_their_subkind() {
        let commands = vec![(
            CommandType::Parameter,
            Command::new(
                0.0,
                100.0,
                Easing::None,
                CommandValue::Parameter(ParameterTypeValue::Additive(0.0)),
                CommandValue::Parameter(ParameterTypeValue::Additive(1.0)),
            ),
        )];
        let channel = command_channel(&commands, CommandType::Parameter).unwrap();
        assert_eq!(channel.component_count(), 1);
        let rebuilt = apply_command_channel(&commands, CommandType::Parameter, &channel.scale(2.0));
        assert_eq!(
            rebuilt[0].1.end_value,
            CommandValue::Parameter(ParameterTypeValue::Additive(2.0))
        );
    }

    #[test]
    fn flatten_mean_is_component_wise_for_colors() {
        let commands = vec![
            color_command(
                0.0,
                100.0,
                Color4::new(0.0, 0.0, 0.0, 1.0),
                Color4::new(0.0, 0.0, 0.0, 1.0),
            ),
            color_command(
                100.0,
                200.0,
                Color4::new(1.0, 0.5, 0.0, 1.0),
                Color4::new(1.0, 0.5, 0.0, 1.0),
            ),
        ];
        let flattened = flatten_commands(&commands, Some(CommandType::Color), FlattenMode::Mean);
        assert_eq!(
            flattened[0].1.start_value,
            CommandValue::Color(Color4::new(0.5, 0.25, 0.0, 1.0))
        );
    }

    #[test]
    fn command_channel_round_trips_per_segment_easing() {
        let commands = vec![(
            CommandType::Fade,
            Command::new(
                0.0,
                100.0,
                Easing::InQuad,
                CommandValue::Scalar(0.0),
                CommandValue::Scalar(1.0),
            ),
        )];
        let channel = command_channel(&commands, CommandType::Fade).unwrap();
        // The channel's first segment carries the command's easing.
        assert_eq!(
            channel
                .component(crate::animations::keyframes::Axis::X)
                .unwrap()
                .easing_at(50.0),
            Easing::InQuad
        );
        // Writing the channel back preserves it.
        let rebuilt = apply_command_channel(&commands, CommandType::Fade, &channel);
        assert_eq!(rebuilt[0].1.easing, Easing::InQuad);

        // A value transform keeps the easing (no `apply_command_channel_eased` needed).
        let scaled = channel.scale(2.0);
        let rebuilt = apply_command_channel(&commands, CommandType::Fade, &scaled);
        assert_eq!(rebuilt[0].1.easing, Easing::InQuad);
        assert_eq!(rebuilt[0].1.end_value, CommandValue::Scalar(2.0));
    }
}
