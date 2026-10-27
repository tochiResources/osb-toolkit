//! Loop detection and end solving for baked command spans ().
//!
//! A [`LoopItem`] describes a loopable command
//! window, [`solve_loop_end`] resolves where a loop actually ends, and
//! [`detect_loop`] finds the fundamental period of a sampled signal by interval
//! analysis (cheaper and more robust than a per-keyframe replay).

use std::collections::BTreeMap;

/// The half-open margin the original helper adds around a loop's times.
const LOOP_MARGIN: f64 = 2.0;

/// A loopable command window.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopItem {
    pub name: String,
    pub can_loop: bool,
    pub start_time: f64,
    pub end_time: f64,
    pub command_start_time: f64,
    pub command_end_time: f64,
    pub command_duration: f64,
    pub loop_duration: f64,
    pub loop_count: i64,
    pub last_loop_command_time: f64,
}

impl LoopItem {
    /// Builds a loop item, applying the `2ms` margin and the derived durations
    /// and loop count the original helper computes in `initialize`.
    pub fn new(
        name: impl Into<String>,
        can_loop: bool,
        start_time: f64,
        end_time: f64,
        command_start: f64,
        command_end: f64,
    ) -> Self {
        let start_time = start_time - LOOP_MARGIN;
        let end_time = end_time + LOOP_MARGIN;
        let command_start = command_start - LOOP_MARGIN;
        let command_end = command_end + LOOP_MARGIN;

        let start_offset = command_start - start_time;
        let end_offset = end_time - command_end;
        let command_duration = (command_end - command_start).abs();
        let loop_duration = (command_duration + end_offset - start_offset).abs();
        let loop_count_ceil = if command_duration > 0.0 {
            (loop_duration / command_duration).ceil().abs() as i64
        } else {
            0
        };
        let loop_count_round = if command_duration > 0.0 {
            (loop_duration / command_duration).round().abs() as i64
        } else {
            0
        };
        let loop_count = loop_count_ceil.max(loop_count_round);

        LoopItem {
            name: name.into(),
            can_loop,
            start_time,
            end_time,
            command_start_time: command_start,
            command_end_time: command_end,
            command_duration,
            loop_duration,
            loop_count,
            last_loop_command_time: start_time + command_duration * loop_count as f64,
        }
    }
}

/// A named collection of loop items, cached by name.
#[derive(Debug, Clone, Default)]
pub struct LoopHelper {
    items: BTreeMap<String, LoopItem>,
}

impl LoopHelper {
    pub fn new() -> Self {
        LoopHelper::default()
    }

    /// Adds an item, replacing any existing item with the same name.
    pub fn add(&mut self, item: LoopItem) {
        self.items.insert(item.name.clone(), item);
    }

    pub fn get(&self, name: &str) -> Option<&LoopItem> {
        self.items.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &LoopItem> {
        self.items.values()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The cached end solve for a named item, if it exists.
    #[allow(clippy::too_many_arguments)]
    pub fn solve_end<T: Clone>(
        &self,
        name: &str,
        value_at_time: impl Fn(i64) -> T,
        start_time: f64,
        end_time: f64,
        end_value: T,
        offset: f64,
        max_end_time: Option<i64>,
    ) -> Option<(f64, T)> {
        let item = self.items.get(name)?;
        Some(solve_loop_end(
            item,
            value_at_time,
            start_time,
            end_time,
            end_value,
            offset,
            max_end_time,
        ))
    }
}

/// Resolves where a loop ends and what value it ends on: an explicit
/// `max_end_time` clips the window and
/// samples the end value there; otherwise a loopable item stops at its command
/// end, or the window is extended by `offset`.
#[allow(clippy::too_many_arguments)]
pub fn solve_loop_end<T: Clone>(
    loop_item: &LoopItem,
    value_at_time: impl Fn(i64) -> T,
    start_time: f64,
    end_time: f64,
    end_value: T,
    offset: f64,
    max_end_time: Option<i64>,
) -> (f64, T) {
    let mut real_end_value = end_value.clone();
    let mut real_end_time = match max_end_time {
        Some(max) => {
            let max = max as f64;
            if end_time + offset > max && start_time + offset < max {
                max
            } else {
                end_time + offset
            }
        }
        None => end_time + offset,
    };

    match max_end_time {
        Some(max) => {
            let clipped = end_time + offset > max as f64 && start_time + offset < max as f64;
            real_end_value = if clipped {
                value_at_time(max - offset as i64)
            } else {
                end_value
            };
        }
        None => {
            if loop_item.can_loop && end_time > loop_item.command_end_time {
                real_end_time = loop_item.command_end_time;
            } else {
                real_end_time = end_time + offset;
                real_end_value = end_value;
            }
        }
    }

    (real_end_time, real_end_value)
}

/// The detected fundamental period of a sampled signal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoopDetection {
    /// The fundamental period in the samples' time units.
    pub period: f64,
    /// Phase offset (the first sample time).
    pub phase: f64,
    /// The maximum mismatch between `f(t)` and `f(t + period)`.
    pub error: f64,
}

/// Finds the smallest period in `[min_period, max_period]` such that the sampled
/// signal repeats within `tolerance` (interval analysis: candidate periods come
/// from pairs of samples with near-equal values, then each is verified).
pub fn detect_loop(
    samples: &[(f64, f64)],
    min_period: f64,
    max_period: f64,
    tolerance: f64,
) -> Option<LoopDetection> {
    if samples.len() < 3 || max_period <= min_period {
        return None;
    }
    let phase = samples[0].0;

    let mut candidates: Vec<f64> = Vec::new();
    for (i, &(ti, vi)) in samples.iter().enumerate() {
        for &(tj, vj) in samples.iter().skip(i + 1) {
            let period = tj - ti;
            if period < min_period || period > max_period {
                continue;
            }
            if (vi - vj).abs() <= tolerance {
                candidates.push(period);
            }
        }
    }
    candidates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    candidates.dedup_by(|a, b| (*a - *b).abs() < 1e-9);

    for period in candidates {
        if let Some(error) = verify_period(samples, period, tolerance) {
            return Some(LoopDetection {
                period,
                phase,
                error,
            });
        }
    }
    None
}

/// Returns the maximum mismatch for `period`, or `None` when it exceeds
/// `tolerance` (the period is rejected).
fn verify_period(samples: &[(f64, f64)], period: f64, tolerance: f64) -> Option<f64> {
    let mut error: f64 = 0.0;
    let mut matched = 0;
    for &(t, v) in samples {
        if let Some(value) = sample_at(samples, t + period) {
            error = error.max((value - v).abs());
            matched += 1;
        }
    }
    if matched == 0 || error > tolerance {
        None
    } else {
        Some(error)
    }
}

/// Linear interpolation over the sample list; `None` outside its range.
fn sample_at(samples: &[(f64, f64)], time: f64) -> Option<f64> {
    let first = samples.first()?;
    let last = samples.last()?;
    if time < first.0 || time > last.0 {
        return None;
    }
    for window in samples.windows(2) {
        let (t0, v0) = window[0];
        let (t1, v1) = window[1];
        if time <= t1 {
            let span = t1 - t0;
            let t = if span == 0.0 { 0.0 } else { (time - t0) / span };
            return Some(v0 + (v1 - v0) * t);
        }
    }
    Some(last.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_item_applies_margin_and_derives_durations() {
        let item = LoopItem::new("Move", true, 0.0, 1000.0, 0.0, 1000.0);
        assert_eq!(item.start_time, -2.0);
        assert_eq!(item.end_time, 1002.0);
        assert_eq!(item.command_start_time, -2.0);
        assert_eq!(item.command_end_time, 1002.0);
        assert_eq!(item.command_duration, 1004.0);
        assert_eq!(item.loop_duration, 1004.0);
        assert_eq!(item.loop_count, 1);
        assert_eq!(item.last_loop_command_time, -2.0 + 1004.0);
    }

    #[test]
    fn solve_loop_end_clips_to_the_max_time() {
        let item = LoopItem::new("Move", true, 0.0, 1000.0, 0.0, 1000.0);
        let value = |t: i64| t as f64;
        // max_end_time inside the window: clip and sample the value there.
        let (end, value_at) = solve_loop_end(&item, value, 0.0, 1000.0, 0.0, 0.0, Some(500));
        assert_eq!(end, 500.0);
        assert_eq!(value_at, 500.0);

        // No max: a loopable item stops at its command end when the window
        // runs past it.
        let (end, _) = solve_loop_end(&item, value, 0.0, 2000.0, 0.0, 0.0, None);
        assert_eq!(end, item.command_end_time);
    }

    #[test]
    fn solve_loop_end_extends_by_offset_when_not_loopable() {
        let item = LoopItem::new("Move", false, 0.0, 1000.0, 0.0, 1000.0);
        let value = |t: i64| t as f64;
        let (end, end_value) = solve_loop_end(&item, value, 0.0, 1000.0, 42.0, 50.0, None);
        assert_eq!(end, 1050.0);
        assert_eq!(end_value, 42.0);
    }

    #[test]
    fn detect_loop_finds_the_fundamental_period() {
        // A sine of period 100 sampled every 5ms.
        let samples: Vec<(f64, f64)> = (0..=200)
            .map(|i| {
                let t = i as f64 * 5.0;
                (t, (t * std::f64::consts::TAU / 100.0).sin())
            })
            .collect();
        let detection = detect_loop(&samples, 10.0, 500.0, 1e-6).expect("a loop");
        assert!(
            (detection.period - 100.0).abs() < 1e-6,
            "period = {}",
            detection.period
        );
        assert!(detection.error < 1e-6);
    }

    #[test]
    fn detect_loop_rejects_noise() {
        let samples: Vec<(f64, f64)> = (0..50)
            .map(|i| (i as f64, ((i * 7919) % 1013) as f64 / 1013.0))
            .collect();
        assert!(detect_loop(&samples, 2.0, 40.0, 0.001).is_none());
    }

    #[test]
    fn loop_helper_caches_by_name() {
        let mut helper = LoopHelper::new();
        helper.add(LoopItem::new("Fade", true, 0.0, 500.0, 0.0, 500.0));
        assert_eq!(helper.len(), 1);
        assert!(helper.get("Fade").is_some());
        let value = |t: i64| t as f64;
        let solved = helper.solve_end("Fade", value, 0.0, 500.0, 0.0, 0.0, Some(200));
        assert_eq!(solved.map(|(t, _)| t), Some(200.0));
        assert!(helper.get("Missing").is_none());
    }
}
