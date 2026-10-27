// The timeline sandbox, including the child-timeline hierarchy.

package osbtk

// TimelineDuration returns the timeline's total duration.
func (sharedHelpersLib) TimelineDuration(timeline any) (float64, error) {
	value, err := sharedHelpersLib{}.Call("timeline.durationOf", map[string]any{
		"timeline": timeline,
	})
	if err != nil {
		return 0, err
	}
	return remarshal[float64](value)
}

// FlushTimeline resolves a timeline tree to flat absolute-time clips.
func (sharedHelpersLib) FlushTimeline(timeline any) (any, error) {
	return sharedHelpersLib{}.Call("timeline.flushTimeline", map[string]any{
		"timeline": timeline,
	})
}

// MergeTimelines overlays two timelines' clips and children.
func (sharedHelpersLib) MergeTimelines(a, b any) (any, error) {
	return sharedHelpersLib{}.Call("timeline.mergeTimelines", map[string]any{
		"timeline": a, "other": b,
	})
}

// TimelineClipCount returns the total clip count across the tree.
func (sharedHelpersLib) TimelineClipCount(timeline any) (int, error) {
	value, err := sharedHelpersLib{}.Call("timeline.clipCount", map[string]any{
		"timeline": timeline,
	})
	if err != nil {
		return 0, err
	}
	return remarshal[int](value)
}

// GroupTimeline adds a nested child timeline (a group).
func (sharedHelpersLib) GroupTimeline(timeline, child any) (any, error) {
	return sharedHelpersLib{}.Call("timeline.addGroup", map[string]any{
		"timeline": timeline, "child": child,
	})
}

// TimelineAllClips returns every clip in the tree, in the shared time coordinate.
func (sharedHelpersLib) TimelineAllClips(timeline any) (any, error) {
	return sharedHelpersLib{}.Call("timeline.allClips", map[string]any{"timeline": timeline})
}
