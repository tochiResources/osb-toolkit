//! The storyboarding library: `.osu`/`.osb` parsing, the stateless
//! origin/screen-bound queries, and the 2D command utilities (M38).

use crate::error::Error;
use crate::models::{Beatmap, OsbOrigin, ParseOsbResult, Vector2};

/// Parses a `.osu` beatmap into the typed model.
pub fn parse_osu(text: &str) -> Result<Beatmap, Error> {
    crate::Client::parse_beatmap(text)
}

/// Parses a `.osb` storyboard into a summary.
pub fn parse_osb(text: &str) -> Result<ParseOsbResult, Error> {
    crate::Client::parse_osb(text)
}

/// `OsbSprite.GetOriginVector`.
pub fn get_origin_vector(origin: OsbOrigin, width: f64, height: f64) -> Result<Vector2, Error> {
    crate::Client::get_origin_vector(origin, width, height)
}

/// `OsbSprite.InScreenBounds`.
pub fn in_screen_bounds(
    position: Vector2,
    size: Vector2,
    rotation: f64,
    origin: Vector2,
) -> Result<bool, Error> {
    crate::Client::in_screen_bounds(position, size, rotation, origin)
}

/// The 2D storyboard command utilities (M38): pure transforms over a
/// `StoryboardCommand[]`, canonical in `osb_runtime::storyboarding::helpers`.
pub mod commands {
    use serde_json::{json, Value};

    use crate::error::Error;
    use crate::models::{StoryboardCommand, StoryboardCommandKind};

    /// Calls any helper by name with a raw input object.
    pub fn call(name: &str, input: Value) -> Result<Value, Error> {
        crate::helpers::call(&format!("command.{name}"), input)
    }

    /// Bridges any keyframe transform onto a command channel (M38): extract the
    /// `kind` channel, apply the named `keyframe.*` helper with `options`, and
    /// write it back as commands. This is how a 2D command channel uses the same
    /// helpers as a 3D node channel.
    pub fn keyframe_transform(
        commands: Vec<StoryboardCommand>,
        kind: StoryboardCommandKind,
        name: &str,
        options: Value,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        crate::helpers::transform_commands(commands, kind, name, options)
    }

    fn command_call(name: &str, input: Value) -> Result<Vec<StoryboardCommand>, Error> {
        Ok(serde_json::from_value(call(name, input)?)?)
    }

    /// Sets an easing on every command (apply an easing to a whole group/scene).
    pub fn set_easing(
        commands: Vec<StoryboardCommand>,
        easing: crate::models::Easing,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "setEasing",
            json!({ "commands": commands, "easing": easing }),
        )
    }

    /// Re-eases a command channel with a named easing, resolving it into commands.
    pub fn apply_easing(
        commands: Vec<StoryboardCommand>,
        kind: StoryboardCommandKind,
        easing: crate::models::Easing,
        samples: usize,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "applyEasing",
            json!({ "commands": commands, "kind": kind, "easing": easing, "samples": samples }),
        )
    }

    /// Re-eases a command channel with a cubic-bezier, resolving it into commands.
    #[allow(clippy::too_many_arguments)]
    pub fn apply_bezier(
        commands: Vec<StoryboardCommand>,
        kind: StoryboardCommandKind,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        samples: usize,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "applyBezier",
            json!({ "commands": commands, "kind": kind, "x1": x1, "y1": y1, "x2": x2, "y2": y2, "samples": samples }),
        )
    }

    /// Selects a timeframe (and/or kind) of commands without mutating the source.
    pub fn select_commands(
        commands: Vec<StoryboardCommand>,
        kind: Option<StoryboardCommandKind>,
        start: Option<f64>,
        end: Option<f64>,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "selectCommands",
            json!({ "commands": commands, "kind": kind, "start": start, "end": end }),
        )
    }

    /// Moves a command set in time by `delta` ms.
    pub fn shift_commands(
        commands: Vec<StoryboardCommand>,
        delta: f64,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "shiftCommands",
            json!({ "commands": commands, "delta": delta }),
        )
    }

    /// Stretches a command set in time about `anchor` by `factor`.
    pub fn scale_commands(
        commands: Vec<StoryboardCommand>,
        factor: f64,
        anchor: f64,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "scaleCommands",
            json!({ "commands": commands, "factor": factor, "anchor": anchor }),
        )
    }

    /// Reverses a command set in time about `[start, end]`.
    pub fn reverse_commands(
        commands: Vec<StoryboardCommand>,
        start: f64,
        end: f64,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "reverseCommands",
            json!({ "commands": commands, "start": start, "end": end }),
        )
    }

    /// Merges adjacent commands that are continuous in time and value.
    pub fn merge_commands(
        commands: Vec<StoryboardCommand>,
        tolerance: f64,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "mergeCommands",
            json!({ "commands": commands, "tolerance": tolerance }),
        )
    }

    /// Splits commands spanning `at` into two linear pieces.
    pub fn split_commands(
        commands: Vec<StoryboardCommand>,
        at: f64,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call("splitCommands", json!({ "commands": commands, "at": at }))
    }

    /// Collapses a command channel to a constant.
    pub fn flatten_commands(
        commands: Vec<StoryboardCommand>,
        kind: Option<StoryboardCommandKind>,
        mode: &str,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "flattenCommands",
            json!({ "commands": commands, "kind": kind, "mode": mode }),
        )
    }

    /// Seeded jitter of command values.
    pub fn randomize_commands(
        commands: Vec<StoryboardCommand>,
        min: f64,
        max: f64,
        seed: u64,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "randomizeCommands",
            json!({ "commands": commands, "min": min, "max": max, "seed": seed }),
        )
    }

    /// Keeps the commands matching a kind/time selector.
    pub fn filter_commands(
        commands: Vec<StoryboardCommand>,
        kind: Option<StoryboardCommandKind>,
        start: Option<f64>,
        end: Option<f64>,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "filterCommands",
            json!({ "commands": commands, "kind": kind, "start": start, "end": end }),
        )
    }

    /// Removes the commands matching a kind/time selector.
    pub fn remove_commands(
        commands: Vec<StoryboardCommand>,
        kind: Option<StoryboardCommandKind>,
        start: Option<f64>,
        end: Option<f64>,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "removeCommands",
            json!({ "commands": commands, "kind": kind, "start": start, "end": end }),
        )
    }

    /// Applies a scalar value transform (`value * scale + offset`) to every command.
    pub fn map_commands(
        commands: Vec<StoryboardCommand>,
        value_scale: f64,
        value_offset: f64,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "mapCommands",
            json!({ "commands": commands, "valueScale": value_scale, "valueOffset": value_offset }),
        )
    }

    /// Clones a command set `count` times, each copy shifted by `offset` ms.
    pub fn duplicate_commands(
        commands: Vec<StoryboardCommand>,
        offset: f64,
        count: usize,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "duplicateCommands",
            json!({ "commands": commands, "offset": offset, "count": count }),
        )
    }

    /// Snaps command times to a beat grid (and values to `value_step`).
    pub fn align_commands(
        commands: Vec<StoryboardCommand>,
        beat: f64,
        offset: f64,
        subdivision: u32,
        value_step: f64,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "alignCommands",
            json!({ "commands": commands, "beat": beat, "offset": offset,
                    "subdivision": subdivision, "valueStep": value_step }),
        )
    }

    /// Rounds command times/values to a step.
    pub fn quantize_commands(
        commands: Vec<StoryboardCommand>,
        time_step: f64,
        value_step: f64,
    ) -> Result<Vec<StoryboardCommand>, Error> {
        command_call(
            "quantizeCommands",
            json!({ "commands": commands, "timeStep": time_step, "valueStep": value_step }),
        )
    }
}
