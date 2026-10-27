use std::collections::HashMap;

use crate::animations::Easing;
use crate::math::{Color4, Vec2};

use super::command::{Command, CommandType, CommandValue, ParameterTypeValue};
use super::types::{OsbLoopType, OsbOrigin, ParameterType};

/// A group of commands repeated over a time range.
#[derive(Debug, Clone, Default)]
pub struct Loop {
    pub start_time: f64,
    pub end_time: f64,
    pub loop_type: OsbLoopType,
    pub count: i32,
    commands: HashMap<CommandType, Vec<Command>>,
}

/// A command group fired by a matching hit object.
#[derive(Debug, Clone, Default)]
pub struct Trigger {
    pub trigger_name: String,
    pub start_time: f64,
    pub end_time: f64,
    pub group: i32,
    commands: HashMap<CommandType, Vec<Command>>,
}

impl Trigger {
    /// The nested command timelines, keyed by command type.
    pub fn commands(&self) -> &HashMap<CommandType, Vec<Command>> {
        &self.commands
    }

    /// Mutable access to the nested command timelines.
    pub fn commands_mut(&mut self) -> &mut HashMap<CommandType, Vec<Command>> {
        &mut self.commands
    }
}

/// A command group a sprite is currently authoring into (`StartLoopGroup` /
/// `StartTriggerGroup`), closed by [`OsbSprite::end_group`].
#[derive(Debug, Clone, Copy)]
enum ActiveGroup {
    Loop(usize),
    Trigger(usize),
}

/// Frame-cycling metadata for an animation (`CreateAnimation` / `.osb` `Animation`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnimationMeta {
    pub frame_count: i32,
    pub frame_delay: f64,
    pub loop_type: OsbLoopType,
}

/// A storyboard sprite with its initial state and command timelines.
#[derive(Debug, Clone)]
pub struct OsbSprite {
    pub texture_path: String,
    pub origin: OsbOrigin,
    /// An animatable origin offset in texture pixels, added to the enum
    /// [`origin`](Self::origin) anchor. Defaults to zero.
    pub anchor_point: Vec2,
    pub initial_position: Vec2,
    pub initial_scale: Vec2,
    pub initial_rotation: f64,
    pub initial_opacity: f64,
    pub initial_color: Color4,
    pub additive: bool,
    pub flip_h: bool,
    pub flip_v: bool,
    /// Whether the sprite is excluded from output (`DiscardSprite`).
    pub exclude: bool,
    /// The command split threshold (used only for `.osb` export splitting).
    pub command_split_threshold: f64,
    /// Animation frame-cycling metadata, or `None` for a plain sprite.
    pub animation: Option<AnimationMeta>,
    commands: HashMap<CommandType, Vec<Command>>,
    loops: Vec<Loop>,
    triggers: Vec<Trigger>,
    /// The loop/trigger currently receiving commands.
    active_group: Option<ActiveGroup>,
}

impl OsbSprite {
    pub fn new(texture_path: impl Into<String>, origin: OsbOrigin) -> Self {
        OsbSprite {
            texture_path: texture_path.into(),
            origin,
            anchor_point: Vec2::ZERO,
            initial_position: Vec2::ZERO,
            initial_scale: Vec2::ONE,
            initial_rotation: 0.0,
            initial_opacity: 1.0,
            initial_color: Color4::WHITE,
            additive: false,
            flip_h: false,
            flip_v: false,
            exclude: false,
            command_split_threshold: 0.0,
            animation: None,
            commands: HashMap::new(),
            loops: Vec::new(),
            triggers: Vec::new(),
            active_group: None,
        }
    }

    pub fn loops(&self) -> &[Loop] {
        &self.loops
    }

    pub fn triggers(&self) -> &[Trigger] {
        &self.triggers
    }

    /// Mutable access to the sprite's loop groups.
    pub fn loops_mut(&mut self) -> &mut [Loop] {
        &mut self.loops
    }

    /// Mutable access to the sprite's trigger groups.
    pub fn triggers_mut(&mut self) -> &mut [Trigger] {
        &mut self.triggers
    }

    /// The sprite's top-level command timelines, keyed by command type.
    pub fn commands(&self) -> &HashMap<CommandType, Vec<Command>> {
        &self.commands
    }

    /// Mutable access to the sprite's top-level command timelines.
    pub fn commands_mut(&mut self) -> &mut HashMap<CommandType, Vec<Command>> {
        &mut self.commands
    }

    fn push(&mut self, kind: CommandType, command: Command) {
        match self.active_group {
            Some(ActiveGroup::Loop(index)) => {
                self.loops[index]
                    .commands
                    .entry(kind)
                    .or_default()
                    .push(command);
            }
            Some(ActiveGroup::Trigger(index)) => {
                self.triggers[index]
                    .commands
                    .entry(kind)
                    .or_default()
                    .push(command);
            }
            None => {
                self.commands.entry(kind).or_default().push(command);
            }
        }
    }

    /// `StartLoopGroup(startTime, loopCount)`: subsequent commands are
    /// added to the loop until [`end_group`](Self::end_group).
    pub fn start_loop_group(
        &mut self,
        start_time: impl Into<f64>,
        count: impl Into<f64>,
    ) -> &mut Self {
        self.loops.push(Loop {
            start_time: start_time.into(),
            end_time: f64::INFINITY,
            loop_type: OsbLoopType::LoopOnce,
            count: count.into() as i32,
            commands: HashMap::new(),
        });
        self.active_group = Some(ActiveGroup::Loop(self.loops.len() - 1));
        self
    }

    /// `StartTriggerGroup(name, startTime, endTime, group)`: subsequent
    /// commands are added to the trigger until [`end_group`](Self::end_group).
    pub fn start_trigger_group(
        &mut self,
        name: impl Into<String>,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        group: i32,
    ) -> &mut Self {
        self.triggers.push(Trigger {
            trigger_name: name.into(),
            start_time: start_time.into(),
            end_time: end_time.into(),
            group,
            commands: HashMap::new(),
        });
        self.active_group = Some(ActiveGroup::Trigger(self.triggers.len() - 1));
        self
    }

    /// `EndGroup`: closes the active loop or trigger. A loop fixes its
    /// active window from its commands and repeat count.
    pub fn end_group(&mut self) -> &mut Self {
        if let Some(ActiveGroup::Loop(index)) = self.active_group.take() {
            let span = self.loops[index].iteration_span();
            let count = self.loops[index].count.max(1) as f64;
            self.loops[index].end_time = self.loops[index].start_time + span * count;
        }
        self
    }

    /// `OsbSprite.GetOriginVector`.
    pub fn get_origin_vector(origin: OsbOrigin, width: f64, height: f64) -> Vec2 {
        let (x, y) = origin.anchor();
        Vec2::new(x * width, y * height)
    }

    /// `OsbSprite.InScreenBounds`: whether the rotated sprite's
    /// bounding box intersects the widescreen storyboard area.
    pub fn in_screen_bounds(position: Vec2, size: Vec2, rotation: f64, origin: Vec2) -> bool {
        let (sin, cos) = rotation.sin_cos();
        let corners = [
            Vec2::new(-origin.x, -origin.y),
            Vec2::new(size.x - origin.x, -origin.y),
            Vec2::new(-origin.x, size.y - origin.y),
            Vec2::new(size.x - origin.x, size.y - origin.y),
        ];
        let mut min = Vec2::new(f64::MAX, f64::MAX);
        let mut max = Vec2::new(f64::MIN, f64::MIN);
        for corner in corners {
            let rotated = Vec2::new(
                corner.x * cos - corner.y * sin,
                corner.x * sin + corner.y * cos,
            );
            let point = position + rotated;
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
        }
        let bounds_min = Vec2::new(-107.0, 0.0);
        let bounds_max = Vec2::new(747.0, 480.0);
        min.x <= bounds_max.x
            && max.x >= bounds_min.x
            && min.y <= bounds_max.y
            && max.y >= bounds_min.y
    }

    pub fn move_(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Vec2,
        end: Vec2,
        easing: Easing,
    ) -> &mut Self {
        self.push(
            CommandType::Move,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Vector2(start),
                CommandValue::Vector2(end),
            ),
        );
        self
    }

    pub fn move_x(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: impl Into<f64>,
        end: impl Into<f64>,
        easing: Easing,
    ) -> &mut Self {
        self.push(
            CommandType::MoveX,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Scalar(start.into()),
                CommandValue::Scalar(end.into()),
            ),
        );
        self
    }

    pub fn move_y(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: impl Into<f64>,
        end: impl Into<f64>,
        easing: Easing,
    ) -> &mut Self {
        self.push(
            CommandType::MoveY,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Scalar(start.into()),
                CommandValue::Scalar(end.into()),
            ),
        );
        self
    }

    pub fn scale(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: impl Into<f64>,
        end: impl Into<f64>,
        easing: Easing,
    ) -> &mut Self {
        self.push(
            CommandType::Scale,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Scalar(start.into()),
                CommandValue::Scalar(end.into()),
            ),
        );
        self
    }

    pub fn scale_vec(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Vec2,
        end: Vec2,
        easing: Easing,
    ) -> &mut Self {
        self.push(
            CommandType::ScaleVec,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Vector2(start),
                CommandValue::Vector2(end),
            ),
        );
        self
    }

    pub fn rotate(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: impl Into<f64>,
        end: impl Into<f64>,
        easing: Easing,
    ) -> &mut Self {
        self.push(
            CommandType::Rotate,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Scalar(start.into()),
                CommandValue::Scalar(end.into()),
            ),
        );
        self
    }

    pub fn fade(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: impl Into<f64>,
        end: impl Into<f64>,
        easing: Easing,
    ) -> &mut Self {
        self.push(
            CommandType::Fade,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Scalar(start.into()),
                CommandValue::Scalar(end.into()),
            ),
        );
        self
    }

    pub fn color(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Color4,
        end: Color4,
        easing: Easing,
    ) -> &mut Self {
        self.push(
            CommandType::Color,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Color(start),
                CommandValue::Color(end),
            ),
        );
        self
    }

    pub fn color_hsb(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Color4,
        end: Color4,
        easing: Easing,
    ) -> &mut Self {
        self.push(
            CommandType::ColorHsb,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Color(start),
                CommandValue::Color(end),
            ),
        );
        self
    }

    pub fn parameter(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        parameter: ParameterType,
        start: f64,
        end: f64,
        easing: Easing,
    ) -> &mut Self {
        let wrap = |value: f64| match parameter {
            ParameterType::FlipHorizontal => ParameterTypeValue::FlipHorizontal(value),
            ParameterType::FlipVertical => ParameterTypeValue::FlipVertical(value),
            ParameterType::Additive => ParameterTypeValue::Additive(value),
        };
        self.push(
            CommandType::Parameter,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Parameter(wrap(start)),
                CommandValue::Parameter(wrap(end)),
            ),
        );
        self
    }

    /// Sets the sprite's initial origin offset in texture pixels.
    pub fn set_anchor(&mut self, point: Vec2) -> &mut Self {
        self.anchor_point = point;
        self
    }

    /// `Anchor(startTime, endTime, start, end, easing)`: animates the origin
    /// offset over a time range.
    pub fn anchor(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Vec2,
        end: Vec2,
        easing: Easing,
    ) -> &mut Self {
        self.push(
            CommandType::Anchor,
            Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Vector2(start),
                CommandValue::Vector2(end),
            ),
        );
        self
    }

    /// Samples the origin offset at `time`, falling
    /// back to the initial [`anchor_point`](Self::anchor_point).
    pub fn anchor_at(&self, time: f64) -> Vec2 {
        self.sample(CommandType::Anchor, time)
            .map(|value| value.as_vec2())
            .unwrap_or(self.anchor_point)
    }

    /// Starts a loop group and returns it for nested command authoring.
    pub fn loop_(
        &mut self,
        start_time: f64,
        end_time: f64,
        count: i32,
        loop_type: OsbLoopType,
    ) -> &mut Loop {
        self.loops.push(Loop {
            start_time,
            end_time,
            loop_type,
            count,
            commands: HashMap::new(),
        });
        self.loops.last_mut().unwrap()
    }

    pub fn start_trigger(&mut self, name: impl Into<String>, group: i32) -> &mut Trigger {
        self.triggers.push(Trigger {
            trigger_name: name.into(),
            start_time: 0.0,
            end_time: f64::INFINITY,
            group,
            commands: HashMap::new(),
        });
        self.triggers.last_mut().unwrap()
    }

    /// Samples a command timeline for a value at `time`.
    fn sample(&self, kind: CommandType, time: f64) -> Option<CommandValue> {
        if let Some(commands) = self.commands.get(&kind) {
            if let Some(value) = sample_timeline(commands, time) {
                return Some(value);
            }
        }
        for loop_group in &self.loops {
            if time >= loop_group.start_time && time <= loop_group.end_time {
                let iteration = loop_group.iteration_span();
                if iteration > 0.0 {
                    let local = loop_group.start_time + (time - loop_group.start_time) % iteration;
                    if let Some(commands) = loop_group.commands.get(&kind) {
                        if let Some(value) = sample_timeline(commands, local) {
                            return Some(value);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn position_at(&self, time: f64) -> Vec2 {
        let mut position = self.initial_position;
        if let Some(value) = self.sample(CommandType::Move, time) {
            position = value.as_vec2();
        } else {
            if let Some(value) = self.sample(CommandType::MoveX, time) {
                position.x = value.as_scalar();
            }
            if let Some(value) = self.sample(CommandType::MoveY, time) {
                position.y = value.as_scalar();
            }
        }
        position
    }

    pub fn scale_at(&self, time: f64) -> Vec2 {
        if let Some(value) = self.sample(CommandType::ScaleVec, time) {
            return value.as_vec2();
        }
        if let Some(value) = self.sample(CommandType::Scale, time) {
            let scalar = value.as_scalar();
            return Vec2::new(scalar, scalar);
        }
        self.initial_scale
    }

    pub fn rotation_at(&self, time: f64) -> f64 {
        self.sample(CommandType::Rotate, time)
            .map(|value| value.as_scalar())
            .unwrap_or(self.initial_rotation)
    }

    pub fn opacity_at(&self, time: f64) -> f64 {
        self.sample(CommandType::Fade, time)
            .map(|value| value.as_scalar())
            .unwrap_or(self.initial_opacity)
    }

    pub fn color_at(&self, time: f64) -> Color4 {
        self.sample(CommandType::Color, time)
            .or_else(|| self.sample(CommandType::ColorHsb, time))
            .map(|value| value.as_color())
            .unwrap_or(self.initial_color)
    }

    pub fn additive_at(&self, time: f64) -> bool {
        if let Some(CommandValue::Parameter(ParameterTypeValue::Additive(amount))) =
            self.sample(CommandType::Parameter, time)
        {
            return amount >= 0.5;
        }
        self.additive
    }

    /// The overall time span of the sprite's commands (`(start, end)`), or
    /// `(0.0, 0.0)` when it has no commands.
    pub fn time_span(&self) -> (f64, f64) {
        let mut start = f64::INFINITY;
        let mut end = f64::NEG_INFINITY;
        for command in self.commands.values().flatten() {
            start = start.min(command.start_time);
            end = end.max(command.end_time);
        }
        for loop_group in &self.loops {
            start = start.min(loop_group.start_time);
            end = end.max(loop_group.end_time);
            for command in loop_group.commands().values().flatten() {
                start = start.min(command.start_time);
                end = end.max(command.end_time);
            }
        }
        for trigger in &self.triggers {
            start = start.min(trigger.start_time);
            end = end.max(trigger.end_time);
            for command in trigger.commands().values().flatten() {
                start = start.min(command.start_time);
                end = end.max(command.end_time);
            }
        }
        if start.is_finite() && end.is_finite() {
            (start, end)
        } else {
            (0.0, 0.0)
        }
    }
}

impl Loop {
    /// One iteration's length: the latest end time among the loop's commands.
    pub fn iteration_span(&self) -> f64 {
        self.commands
            .values()
            .flatten()
            .map(|command| command.end_time)
            .fold(0.0f64, f64::max)
    }

    /// The loop's nested command timelines, keyed by command type.
    pub fn commands(&self) -> &HashMap<CommandType, Vec<Command>> {
        &self.commands
    }

    /// Mutable access to the loop's nested command timelines.
    pub fn commands_mut(&mut self) -> &mut HashMap<CommandType, Vec<Command>> {
        &mut self.commands
    }

    pub fn move_(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Vec2,
        end: Vec2,
        easing: Easing,
    ) -> &mut Self {
        self.commands
            .entry(CommandType::Move)
            .or_default()
            .push(Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Vector2(start),
                CommandValue::Vector2(end),
            ));
        self
    }

    pub fn fade(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: impl Into<f64>,
        end: impl Into<f64>,
        easing: Easing,
    ) -> &mut Self {
        self.commands
            .entry(CommandType::Fade)
            .or_default()
            .push(Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Scalar(start.into()),
                CommandValue::Scalar(end.into()),
            ));
        self
    }

    pub fn rotate(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: impl Into<f64>,
        end: impl Into<f64>,
        easing: Easing,
    ) -> &mut Self {
        self.commands
            .entry(CommandType::Rotate)
            .or_default()
            .push(Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Scalar(start.into()),
                CommandValue::Scalar(end.into()),
            ));
        self
    }

    pub fn scale(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: impl Into<f64>,
        end: impl Into<f64>,
        easing: Easing,
    ) -> &mut Self {
        self.commands
            .entry(CommandType::Scale)
            .or_default()
            .push(Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Scalar(start.into()),
                CommandValue::Scalar(end.into()),
            ));
        self
    }

    /// `Anchor(startTime, endTime, start, end, easing)` inside a loop group.
    pub fn anchor(
        &mut self,
        start_time: impl Into<f64>,
        end_time: impl Into<f64>,
        start: Vec2,
        end: Vec2,
        easing: Easing,
    ) -> &mut Self {
        self.commands
            .entry(CommandType::Anchor)
            .or_default()
            .push(Command::new(
                start_time.into(),
                end_time.into(),
                easing,
                CommandValue::Vector2(start),
                CommandValue::Vector2(end),
            ));
        self
    }
}

fn sample_timeline(commands: &[Command], time: f64) -> Option<CommandValue> {
    if commands.is_empty() {
        return None;
    }
    if time < commands[0].start_time {
        return Some(commands[0].start_value);
    }
    for command in commands {
        if time >= command.start_time && time <= command.end_time {
            return Some(command.value_at(time));
        }
    }
    commands.last().map(|command| command.end_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_move_and_fade() {
        let mut sprite = OsbSprite::new("sb/bar.png", OsbOrigin::Centre);
        sprite.move_(
            0.0,
            100.0,
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Easing::None,
        );
        sprite.fade(0.0, 100.0, 0.0, 1.0, Easing::None);
        assert_eq!(sprite.position_at(0.0), Vec2::new(0.0, 0.0));
        assert!((sprite.position_at(50.0).x - 50.0).abs() < 1e-4);
        assert!((sprite.opacity_at(50.0) - 0.5).abs() < 1e-4);
        assert_eq!(sprite.opacity_at(500.0), 1.0);
    }

    #[test]
    fn loop_repeats_local_time() {
        let mut sprite = OsbSprite::new("sb/bar.png", OsbOrigin::Centre);
        {
            let loop_group = sprite.loop_(0.0, 200.0, 0, OsbLoopType::LoopForever);
            loop_group.move_(
                0.0,
                100.0,
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Easing::None,
            );
        }
        assert!((sprite.position_at(50.0).x - 5.0).abs() < 1e-4);
        assert!((sprite.position_at(150.0).x - 5.0).abs() < 1e-4);
    }

    #[test]
    fn samples_an_animated_anchor() {
        let mut sprite = OsbSprite::new("sb/bar.png", OsbOrigin::Centre);
        // The initial anchor offset applies until the first command starts.
        sprite.set_anchor(Vec2::new(1.0, 2.0));
        assert_eq!(sprite.anchor_at(0.0), Vec2::new(1.0, 2.0));
        sprite.anchor(
            0.0,
            100.0,
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 20.0),
            Easing::None,
        );
        assert_eq!(sprite.anchor_at(0.0), Vec2::new(0.0, 0.0));
        assert!((sprite.anchor_at(50.0).x - 5.0).abs() < 1e-4);
        assert!((sprite.anchor_at(50.0).y - 10.0).abs() < 1e-4);
        assert_eq!(sprite.anchor_at(500.0), Vec2::new(10.0, 20.0));
    }

    #[test]
    fn loop_animates_the_anchor() {
        let mut sprite = OsbSprite::new("sb/bar.png", OsbOrigin::Centre);
        {
            let loop_group = sprite.loop_(0.0, 200.0, 0, OsbLoopType::LoopForever);
            loop_group.anchor(
                0.0,
                100.0,
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Easing::None,
            );
        }
        assert!((sprite.anchor_at(50.0).x - 5.0).abs() < 1e-4);
        assert!((sprite.anchor_at(150.0).x - 5.0).abs() < 1e-4);
    }
}
