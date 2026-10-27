//! A timeline sandbox with a parent/child hierarchy.
//!
//! A [`Timeline`] is an ordered set of [`TimelineClip`]s, each a keyframe
//! [`Channel`] placed at a start time, plus nested child timelines. Motion is
//! authored as a sequence of clips - grouped into sub-timelines where useful -
//! and then [`Timeline::flush`]ed to resolved, absolute-time channels. The
//! operations (`shift`/`scale`/`reverse`/`repeat`/`slice`/`merge`/`stagger`)
//! cascade from a parent into its children, so a whole set of tracks can be
//! controlled from the top of the hierarchy and resolved in one pass.
//!
//! Children share their parent's time coordinate: nesting is structural, so
//! `shift`/`scale`/`reverse` on a parent move its children with it and `flush`
//! flattens the whole tree.

use crate::common::animations::keyframes::Channel;

/// A keyframe channel placed at `start` on a timeline.
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineClip {
    /// The clip's start time in milliseconds.
    pub start: f64,
    /// The clip's keyframe channel (its keyframe times are clip-relative).
    pub channel: Channel,
}

impl TimelineClip {
    /// Builds a clip.
    pub fn new(channel: Channel, start: f64) -> Self {
        TimelineClip { start, channel }
    }

    /// The clip's end time.
    pub fn end(&self) -> f64 {
        self.start + self.channel.duration()
    }
}

/// An ordered set of keyframe clips and nested child timelines (a motion
/// sandbox). A child shares the parent's time coordinate; operations cascade.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Timeline {
    /// The clips, in insertion order.
    pub clips: Vec<TimelineClip>,
    /// Nested sub-timelines (groups), in insertion order.
    pub children: Vec<Timeline>,
}

impl Timeline {
    /// An empty timeline.
    pub fn new() -> Self {
        Timeline {
            clips: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Adds a clip.
    pub fn add(&mut self, channel: Channel, start: f64) -> &mut Self {
        self.clips.push(TimelineClip::new(channel, start));
        self
    }

    /// Adds a nested child timeline (a group).
    pub fn add_child(&mut self, child: Timeline) -> &mut Self {
        self.children.push(child);
        self
    }

    /// Appends a new empty child timeline and returns it for authoring.
    pub fn group(&mut self) -> &mut Timeline {
        self.children.push(Timeline::new());
        self.children.last_mut().expect("just pushed")
    }

    /// The nested child timelines.
    pub fn children(&self) -> &[Timeline] {
        &self.children
    }

    /// Mutable access to the nested child timelines.
    pub fn children_mut(&mut self) -> &mut [Timeline] {
        &mut self.children
    }

    /// A nested child by index.
    pub fn child(&self, index: usize) -> Option<&Timeline> {
        self.children.get(index)
    }

    /// Mutable access to a nested child by index.
    pub fn child_mut(&mut self, index: usize) -> Option<&mut Timeline> {
        self.children.get_mut(index)
    }

    /// The number of direct children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Every clip in the tree, in this timeline's (shared) time coordinate.
    pub fn all_clips(&self) -> Vec<TimelineClip> {
        let mut out = self.clips.clone();
        for child in &self.children {
            out.extend(child.all_clips());
        }
        out
    }

    /// The total clip count across the whole tree.
    pub fn clip_count(&self) -> usize {
        self.clips.len()
            + self
                .children
                .iter()
                .map(Timeline::clip_count)
                .sum::<usize>()
    }

    /// The timeline's total duration (the latest clip end across the tree).
    pub fn duration(&self) -> f64 {
        self.all_clips()
            .iter()
            .map(TimelineClip::end)
            .fold(0.0, f64::max)
    }

    /// The timeline's occupied `[start, end]` range across the tree, or `None`
    /// when empty.
    pub fn time_range(&self) -> Option<(f64, f64)> {
        let clips = self.all_clips();
        if clips.is_empty() {
            return None;
        }
        let start = clips.iter().map(|c| c.start).fold(f64::MAX, f64::min);
        let end = clips.iter().map(TimelineClip::end).fold(f64::MIN, f64::max);
        Some((start, end))
    }

    /// The active flushed channel at `time`: the latest clip (by absolute start)
    /// whose absolute-time range covers `time`, already shifted to absolute time.
    /// `None` when no clip in the tree covers `time`. This is how a timeline is
    /// sampled as a keyframe source (e.g. a node's transform timeline).
    pub fn channel_at(&self, time: f64) -> Option<Channel> {
        let mut best: Option<(f64, Channel)> = None;
        for clip in self.all_clips() {
            let absolute = clip.channel.shift(clip.start);
            let Some((start, end)) = absolute.time_range() else {
                continue;
            };
            if time < start || time > end {
                continue;
            }
            if best
                .as_ref()
                .is_none_or(|(best_start, _)| start >= *best_start)
            {
                best = Some((start, absolute));
            }
        }
        best.map(|(_, channel)| channel)
    }

    /// Moves the whole timeline (and its children) in time.
    pub fn shift(&mut self, delta: f64) -> &mut Self {
        for clip in &mut self.clips {
            clip.start += delta;
        }
        for child in &mut self.children {
            child.shift(delta);
        }
        self
    }

    /// Scales the whole timeline (and its children) in time about `anchor`.
    pub fn scale(&mut self, factor: f64, anchor: f64) -> &mut Self {
        for clip in &mut self.clips {
            clip.start = anchor + (clip.start - anchor) * factor;
            clip.channel = clip.channel.scale_time(factor, anchor);
        }
        for child in &mut self.children {
            child.scale(factor, anchor);
        }
        self
    }

    /// Reverses the whole timeline (and its children) in time about `[start, end]`.
    pub fn reverse(&mut self, start: f64, end: f64) -> &mut Self {
        for clip in &mut self.clips {
            clip.channel = clip.channel.reverse(start, end);
            clip.start = start + end - clip.end();
        }
        for child in &mut self.children {
            child.reverse(start, end);
        }
        self
    }

    /// Tiles the timeline `count` times, optionally ping-ponging alternate copies.
    /// The tree is flattened first, so a group repeats as one unit.
    pub fn repeat(&self, count: usize, ping_pong: bool) -> Timeline {
        let mut out = Timeline::new();
        let span = self.duration();
        for copy in 0..count.max(1) {
            let reversed = ping_pong && copy % 2 == 1;
            for clip in &self.all_clips() {
                let channel = if reversed {
                    clip.channel.reverse(clip.start, clip.start + span)
                } else {
                    clip.channel.clone()
                };
                let start = if reversed {
                    copy as f64 * span + (span - clip.start - clip.channel.duration())
                } else {
                    clip.start + copy as f64 * span
                };
                out.add(channel, start);
            }
        }
        out
    }

    /// Offsets each clip by `index * step` (a stagger). The tree is flattened
    /// first, so the stagger applies per clip across the whole hierarchy.
    pub fn stagger(&self, step: f64) -> Timeline {
        let mut out = Timeline::new();
        for (index, clip) in self.all_clips().iter().enumerate() {
            out.add(clip.channel.clone(), clip.start + index as f64 * step);
        }
        out
    }

    /// A sub-timeline over `[start, end]`, rebased to `0`. The tree is flattened.
    pub fn slice(&self, start: f64, end: f64) -> Timeline {
        let mut out = Timeline::new();
        for clip in &self.all_clips() {
            if clip.end() < start || clip.start > end {
                continue;
            }
            let rebased = clip.channel.shift(clip.start - start);
            out.add(rebased, 0.0);
        }
        out
    }

    /// Overlays another timeline's clips and children (a merge).
    pub fn merge(&self, other: &Timeline) -> Timeline {
        let mut out = self.clone();
        out.clips.extend(other.clips.iter().cloned());
        out.children.extend(other.children.iter().cloned());
        out
    }

    /// Resolves the whole tree to absolute time (shifting each clip's channel by
    /// its start) and sorts the clips by start. This is the "flush" that turns a
    /// sandbox into resolved channels; the result is flat (no children).
    pub fn flush(&self) -> Timeline {
        let mut clips: Vec<TimelineClip> = self
            .all_clips()
            .into_iter()
            .map(|clip| TimelineClip {
                start: 0.0,
                channel: clip.channel.shift(clip.start),
            })
            .collect();
        clips.sort_by(|a, b| {
            a.channel
                .time_range()
                .map(|(a0, _)| a0)
                .unwrap_or(0.0)
                .total_cmp(&b.channel.time_range().map(|(b0, _)| b0).unwrap_or(0.0))
        });
        Timeline {
            clips,
            children: Vec::new(),
        }
    }
}

/// Merges two timelines (see [`Timeline::merge`]).
pub fn merge_timelines(a: &Timeline, b: &Timeline) -> Timeline {
    a.merge(b)
}

/// Flushes a timeline to resolved absolute-time channels.
pub fn flush_timeline(timeline: &Timeline) -> Timeline {
    timeline.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::animations::KeyframedValue;

    fn ramp() -> Channel {
        let mut kv = KeyframedValue::new();
        kv.add(0.0, 0.0);
        kv.add(100.0, 1.0);
        Channel::Scalar(kv)
    }

    #[test]
    fn duration_and_flush() {
        let mut timeline = Timeline::new();
        timeline.add(ramp(), 50.0);
        timeline.add(ramp(), 250.0);
        assert_eq!(timeline.duration(), 350.0);
        let flushed = timeline.flush();
        assert_eq!(flushed.clips[0].start, 0.0);
        assert_eq!(flushed.clips[0].channel.time_range(), Some((50.0, 150.0)));
        assert_eq!(flushed.clips[1].channel.time_range(), Some((250.0, 350.0)));
    }

    #[test]
    fn shift_scale_stagger() {
        let mut timeline = Timeline::new();
        timeline.add(ramp(), 0.0).add(ramp(), 200.0);
        let staggered = timeline.stagger(100.0);
        assert_eq!(staggered.clips[1].start, 300.0);
        timeline.shift(10.0);
        assert_eq!(timeline.clips[0].start, 10.0);
        timeline.scale(2.0, 0.0);
        assert_eq!(timeline.clips[1].start, 420.0);
    }

    #[test]
    fn repeat_and_merge() {
        let mut a = Timeline::new();
        a.add(ramp(), 0.0);
        let repeated = a.repeat(3, false);
        assert_eq!(repeated.clips.len(), 3);
        let b = Timeline::new();
        let merged = a.merge(&b);
        assert_eq!(merged.clips.len(), 1);
        assert_eq!(merge_timelines(&a, &a).clips.len(), 2);
        assert_eq!(flush_timeline(&a).clips.len(), 1);
    }

    #[test]
    fn children_cascade_and_flush() {
        let mut timeline = Timeline::new();
        timeline.add(ramp(), 0.0);
        {
            let child = timeline.group();
            child.add(ramp(), 100.0);
            let grandchild = child.group();
            grandchild.add(ramp(), 50.0);
        }
        assert_eq!(timeline.child_count(), 1);
        assert_eq!(timeline.clip_count(), 3);
        // The tree spans 0..200 (own clip 0..100, child 100..200, grandchild 50..150).
        assert_eq!(timeline.duration(), 200.0);

        // Shifting the parent moves every descendant.
        timeline.shift(1000.0);
        let clips = timeline.all_clips();
        assert_eq!(clips[0].start, 1000.0);
        assert_eq!(clips[1].start, 1100.0);
        assert_eq!(clips[2].start, 1050.0);

        // Flushing flattens the tree to absolute-time clips, sorted.
        let flushed = timeline.flush();
        assert!(flushed.children.is_empty());
        assert_eq!(flushed.clips.len(), 3);
        let starts: Vec<f64> = flushed
            .clips
            .iter()
            .map(|clip| clip.channel.time_range().unwrap().0)
            .collect();
        assert_eq!(starts, vec![1000.0, 1050.0, 1100.0]);
    }

    #[test]
    fn parent_scale_cascades_into_children() {
        let mut timeline = Timeline::new();
        {
            let child = timeline.group();
            child.add(ramp(), 100.0);
        }
        timeline.scale(2.0, 0.0);
        assert_eq!(timeline.children[0].clips[0].start, 200.0);
    }
}
