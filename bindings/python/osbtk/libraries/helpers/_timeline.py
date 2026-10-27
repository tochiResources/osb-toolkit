"""The timeline sandbox (M39/M40), including the child-timeline hierarchy."""

from __future__ import annotations

from typing import Any, Optional

from osbtk.libraries.helpers._base import call


class _Timeline:
    """The timeline sandbox."""

    @staticmethod
    def timeline_duration(timeline: Any) -> float:
        """The timeline's total duration."""
        return call("timeline.durationOf", {"timeline": timeline})

    @staticmethod
    def flush_timeline(timeline: Any) -> Any:
        """Resolves a timeline tree to flat absolute-time clips."""
        return call("timeline.flushTimeline", {"timeline": timeline})

    @staticmethod
    def merge_timelines(a: Any, b: Any) -> Any:
        """Overlays two timelines' clips and children."""
        return call("timeline.mergeTimelines", {"timeline": a, "other": b})

    @staticmethod
    def group_timeline(timeline: Any, child: Optional[Any] = None) -> Any:
        """Adds a nested child timeline (a group)."""
        return call("timeline.addGroup", {"timeline": timeline, "child": child or {"clips": []}})

    @staticmethod
    def timeline_all_clips(timeline: Any) -> Any:
        """Every clip in the tree, in the shared time coordinate."""
        return call("timeline.allClips", {"timeline": timeline})

    @staticmethod
    def timeline_clip_count(timeline: Any) -> int:
        """The total clip count across the tree."""
        return call("timeline.clipCount", {"timeline": timeline})
