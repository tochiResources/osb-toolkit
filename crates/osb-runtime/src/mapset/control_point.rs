/// A timing or difficulty control point.
///
/// Red lines (`timing_change == true`) carry the beat duration; green (inherited)
/// lines carry a speed multiplier applied to the current beat duration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlPoint {
    pub time: f64,
    /// Beat duration in milliseconds. Only meaningful on red lines.
    pub beat_duration: f64,
    /// Slider velocity multiplier (`1.0` on red lines).
    pub speed_multiplier: f64,
    pub timing_change: bool,
    pub omit_first_bar_line: bool,
    pub effect_flags: i32,
}

impl ControlPoint {
    pub fn new(time: f64, beat_duration: f64, timing_change: bool) -> Self {
        ControlPoint {
            time,
            beat_duration,
            speed_multiplier: 1.0,
            timing_change,
            omit_first_bar_line: false,
            effect_flags: 0,
        }
    }

    pub fn bpm(&self) -> f64 {
        if self.beat_duration == 0.0 {
            0.0
        } else {
            60000.0 / self.beat_duration
        }
    }

    /// True when this is a green (inherited) line.
    pub fn is_inherited(&self) -> bool {
        !self.timing_change
    }
}

impl PartialOrd for ControlPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ControlPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.partial_cmp(&other.time).unwrap()
    }
}

impl Eq for ControlPoint {}
