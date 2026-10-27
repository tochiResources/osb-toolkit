package osbtk

import (
	"context"
	"encoding/json"
	"testing"
)

const osuText = "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n" +
	"[TimingPoints]\n0,500,4,2,1,100,1,0\n" +
	"[HitObjects]\n256,192,1000,1,0\n"

func TestVersion(t *testing.T) {
	version, err := Version()
	if err != nil {
		t.Fatal(err)
	}
	if version == "" {
		t.Fatal("empty version")
	}
}

func TestParseBeatmap(t *testing.T) {
	beatmap, err := Storyboarding.ParseOsu(osuText)
	if err != nil {
		t.Fatal(err)
	}
	if beatmap.AudioFilename != "song.mp3" {
		t.Fatalf("audio filename: %s", beatmap.AudioFilename)
	}
	if len(beatmap.HitObjects) == 0 || beatmap.HitObjects[0].Kind != HITOBJECTKIND_CIRCLE {
		t.Fatal("hit object kind")
	}
	if len(beatmap.TimingPoints) == 0 || beatmap.TimingPoints[0].Bpm < 119.9 {
		t.Fatal("timing point bpm")
	}
}

func TestApplyEasing(t *testing.T) {
	value, err := Animations.ApplyEasing(EASING_IN_QUAD, 0.5)
	if err != nil {
		t.Fatal(err)
	}
	if value < 0.249 || value > 0.251 {
		t.Fatalf("easing: %v", value)
	}

	cubic, err := Animations.ApplyCubicBezier(0.42, 0.0, 0.58, 1.0, 1.0)
	if err != nil {
		t.Fatal(err)
	}
	if cubic < 0.999 || cubic > 1.001 {
		t.Fatalf("cubic bezier: %v", cubic)
	}
}

func TestRuntime(t *testing.T) {
	instance, err := NewRuntime(640, 480)
	if err != nil {
		t.Fatal(err)
	}
	defer instance.Close()

	layerClient := instance.Layer()
	spriteClient := instance.Sprite()
	request := CreateSpriteRequest{Path: "sb/dot.png", Origin: OSBORIGIN_CENTRE}
	request.SetLayer("Foreground")
	handle, _, err := layerClient.CreateSprite(context.Background()).
		CreateSpriteRequest(request).
		Execute()
	if err != nil {
		t.Fatal(err)
	}
	_, _, err = spriteClient.Command(context.Background()).
		CommandRequest(CommandRequest{
			Kind:      "move",
			Layer:     handle.Layer,
			Sprite:    handle.Sprite,
			StartTime: 0,
			EndTime:   1000,
			Easing:    EASING_NONE,
			Start:     map[string]any{"x": 0.0, "y": 0.0},
			End:       map[string]any{"x": 100.0, "y": 0.0},
		}).
		Execute()
	if err != nil {
		t.Fatal(err)
	}
	state, _, err := spriteClient.StateAt(context.Background()).
		SpriteTimeRequest(SpriteTimeRequest{Layer: handle.Layer, Sprite: handle.Sprite, Time: 500}).
		Execute()
	if err != nil {
		t.Fatal(err)
	}
	if state.Position.X < 49.9 || state.Position.X > 50.1 {
		t.Fatalf("state x: %v", state.Position.X)
	}
}

func TestSegments(t *testing.T) {
	generator, err := NewGenerator(640, 480)
	if err != nil {
		t.Fatal(err)
	}
	defer generator.Close()

	group, err := generator.GetLayer("Group")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := group.CreateSprite("sb/group.png", OSBORIGIN_CENTRE); err != nil {
		t.Fatal(err)
	}
	child, err := group.CreateSegment()
	if err != nil {
		t.Fatal(err)
	}
	if _, err := child.CreateSprite("sb/child.png", OSBORIGIN_CENTRE); err != nil {
		t.Fatal(err)
	}
	if _, err := child.SetReverseDepth(true); err != nil {
		t.Fatal(err)
	}
	if _, err := child.AddIndex(3); err != nil {
		t.Fatal(err)
	}
	if _, err := child.AddIndex(-1); err != nil {
		t.Fatal(err)
	}
	indexes, err := child.Indexes()
	if err != nil {
		t.Fatal(err)
	}
	if asFloat(indexes["highest"]) != 3 || asFloat(indexes["lowest"]) != -1 {
		t.Fatalf("indexes: %v", indexes)
	}
	if count, _ := generator.SpriteCount(); count != 2 {
		t.Fatalf("count: %v", count)
	}
	if _, err := child.Discard(); err != nil {
		t.Fatal(err)
	}
	if count, _ := generator.SpriteCount(); count != 1 {
		t.Fatalf("count after discard: %v", count)
	}
}

func TestStoryboarding3dReusesSurface(t *testing.T) {
	beatmap, err := Storyboarding3d.ParseOsu(osuText)
	if err != nil {
		t.Fatal(err)
	}
	if beatmap.AudioFilename != "song.mp3" {
		t.Fatalf("audio filename: %s", beatmap.AudioFilename)
	}
}

func TestCurve(t *testing.T) {
	curve := CurveSpec{
		Kind:   CURVEKIND_LINEAR,
		Points: []Vector2{{X: 0, Y: 0}, {X: 100, Y: 0}},
	}
	length, err := Curve.Length(curve)
	if err != nil {
		t.Fatal(err)
	}
	if length < 99.9 || length > 100.1 {
		t.Fatalf("length: %v", length)
	}
	point, err := Curve.Point(curve, 0.5, false)
	if err != nil {
		t.Fatal(err)
	}
	if point.X < 49.9 || point.X > 50.1 {
		t.Fatalf("point x: %v", point.X)
	}
	sampled, err := Curve.Sample(curve, 3, false)
	if err != nil {
		t.Fatal(err)
	}
	if len(sampled.Points) != 3 {
		t.Fatalf("sample points: %d", len(sampled.Points))
	}
	bounds, err := Curve.BoundingBox(curve)
	if err != nil {
		t.Fatal(err)
	}
	if bounds.Max.X < 99.9 || bounds.Max.X > 100.1 {
		t.Fatalf("bounds max x: %v", bounds.Max.X)
	}
	simplified, err := Curve.Simplify([]Vector2{
		{X: 0, Y: 0},
		{X: 1, Y: 0.05},
		{X: 2, Y: 0},
		{X: 3, Y: 0},
	}, 0.1)
	if err != nil {
		t.Fatal(err)
	}
	if len(simplified.Points) != 2 {
		t.Fatalf("simplified points: %d", len(simplified.Points))
	}
	normal, err := Curve.Normal(curve, 0.5)
	if err != nil {
		t.Fatal(err)
	}
	if normal.X > 1e-6 || normal.X < -1e-6 {
		t.Fatalf("normal x: %v", normal.X)
	}
	curvature, err := Curve.Curvature(curve, 0.5)
	if err != nil {
		t.Fatal(err)
	}
	if curvature > 1e-6 || curvature < -1e-6 {
		t.Fatalf("curvature: %v", curvature)
	}
	distance, err := Curve.DistanceTo(curve, Vector2{X: 50, Y: 10})
	if err != nil {
		t.Fatal(err)
	}
	if distance < 9.999 || distance > 10.001 {
		t.Fatalf("distance: %v", distance)
	}
	offset, err := Curve.Offset(curve, 5)
	if err != nil {
		t.Fatal(err)
	}
	if len(offset.Points) != 2 {
		t.Fatalf("offset points: %d", len(offset.Points))
	}
	arc, err := Curve.Arc(Vector2{X: 0, Y: 0}, 10, 0, 180, 3)
	if err != nil {
		t.Fatal(err)
	}
	if len(arc.Points) != 3 {
		t.Fatalf("arc points: %d", len(arc.Points))
	}
	ellipse, err := Curve.Ellipse(Vector2{X: 0, Y: 0}, 4, 2, 4)
	if err != nil {
		t.Fatal(err)
	}
	if len(ellipse.Points) != 4 {
		t.Fatalf("ellipse points: %d", len(ellipse.Points))
	}
	hit, err := Curve.LineIntersect(
		Vector2{X: 0, Y: 0}, Vector2{X: 10, Y: 10},
		Vector2{X: 0, Y: 10}, Vector2{X: 10, Y: 0})
	if err != nil {
		t.Fatal(err)
	}
	if point := hit.Point.Get(); point == nil || point.X < 4.999 || point.X > 5.001 {
		t.Fatalf("intersection: %+v", point)
	}
}

func TestGenerator(t *testing.T) {
	generator, err := NewGenerator(640, 480)
	if err != nil {
		t.Fatal(err)
	}
	defer generator.Close()

	generator.Generate = func(g *Generator) {
		if _, err := g.CreateSprite("Foreground", "sb/dot.png", OSBORIGIN_CENTRE); err != nil {
			t.Fatal(err)
		}
	}
	RunGenerator(generator)

	count, err := generator.SpriteCount()
	if err != nil {
		t.Fatal(err)
	}
	if count != 1 {
		t.Fatalf("sprite count: %d", count)
	}
}

func TestReadsBackStoryboard(t *testing.T) {
	generator, err := NewGenerator(640, 480)
	if err != nil {
		t.Fatal(err)
	}
	defer generator.Close()

	layer, err := generator.GetLayer("Foreground")
	if err != nil {
		t.Fatal(err)
	}
	sprite, err := layer.CreateSprite("sb/dot.png", OSBORIGIN_CENTRE)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := sprite.Fade(0, 1000, 0, 1, EASING_NONE); err != nil {
		t.Fatal(err)
	}
	storyboard, err := generator.Storyboard()
	if err != nil {
		t.Fatal(err)
	}
	var foreground *StoryboardLayer
	for i := range storyboard.Layers {
		if storyboard.Layers[i].Identifier == "Foreground" {
			foreground = &storyboard.Layers[i]
		}
	}
	if foreground == nil || len(foreground.Sprites) == 0 {
		t.Fatalf("foreground layer missing: %+v", storyboard.Layers)
	}
	if foreground.Sprites[0].TexturePath != "sb/dot.png" {
		t.Fatalf("texture path: %s", foreground.Sprites[0].TexturePath)
	}
	if len(foreground.Sprites[0].Commands) == 0 ||
		foreground.Sprites[0].Commands[0].Kind != STORYBOARDCOMMANDKIND_FADE {
		t.Fatalf("commands: %+v", foreground.Sprites[0].Commands)
	}
}

func TestOriginVectorAndScreenBounds(t *testing.T) {
	origin, err := Storyboarding.GetOriginVector(OSBORIGIN_CENTRE, 100, 100)
	if err != nil {
		t.Fatal(err)
	}
	if origin.X < 49.999 || origin.X > 50.001 {
		t.Fatalf("origin x: %v", origin.X)
	}
	inBounds, err := Storyboarding.InScreenBounds(
		Vector2{X: 427, Y: 240}, Vector2{X: 100, Y: 100}, 0, Vector2{X: 50, Y: 50})
	if err != nil {
		t.Fatal(err)
	}
	if !inBounds {
		t.Fatal("expected in screen bounds")
	}
}

func TestSpriteFacade(t *testing.T) {
	generator, err := NewGenerator(640, 480)
	if err != nil {
		t.Fatal(err)
	}
	defer generator.Close()

	layer, err := generator.GetLayer("Foreground")
	if err != nil {
		t.Fatal(err)
	}
	sprite, err := layer.CreateSprite("sb/dot.png", OSBORIGIN_CENTRE)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := sprite.Move(0, 1000, Vector2{X: 0, Y: 0}, Vector2{X: 100, Y: 0}, EASING_NONE); err != nil {
		t.Fatal(err)
	}
	if _, err := sprite.StartLoopGroup(1000, 2); err != nil {
		t.Fatal(err)
	}
	if _, err := sprite.Fade(0, 100, 0, 1, EASING_NONE); err != nil {
		t.Fatal(err)
	}
	if _, err := sprite.FadeTo(150, 1); err != nil {
		t.Fatal(err)
	}
	if _, err := sprite.EndGroup(); err != nil {
		t.Fatal(err)
	}
	position, err := sprite.PositionAt(500)
	if err != nil {
		t.Fatal(err)
	}
	if position.X < 49.9 || position.X > 50.1 {
		t.Fatalf("position x: %v", position.X)
	}
}

func TestMediaError(t *testing.T) {
	if _, err := Media.Info("does-not-exist.mp4"); err == nil {
		t.Fatal("expected a media error")
	}
}

func TestRenderShapes(t *testing.T) {
	info, err := ImageGen.RenderShapes(RenderShapesRequest{
		Width:  32,
		Height: 32,
		Shapes: []Shape{{
			Kind:   SHAPEKIND_RECT,
			Center: Vector2{X: 16, Y: 16},
			Size:   Vector2{X: 20, Y: 20},
			Fill:   *NewNullableRgbaColor(&RgbaColor{R: 10, G: 20, B: 30, A: 255}),
		}},
		Output: "osbtk-go-render.png",
	})
	if err != nil {
		t.Fatal(err)
	}
	if info.Width != 32 {
		t.Fatalf("width: %d", info.Width)
	}
}

func TestImageError(t *testing.T) {
	if _, err := Image.Info("does-not-exist.png"); err == nil {
		t.Fatal("expected an image error")
	}
}

func TestOptimize(t *testing.T) {
	deduplicate := true
	result, err := Optimize.OptimizeOsb(
		"[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n F,0,0,100,0,1\n F,0,0,100,0,1\n",
		OptimizeOptionsInput{Deduplicate: *NewNullableBool(&deduplicate)})
	if err != nil {
		t.Fatal(err)
	}
	if result.Stats.Deduplicated != 1 {
		t.Fatalf("deduplicated: %d", result.Stats.Deduplicated)
	}
}

func TestDiagnostics(t *testing.T) {
	report, err := Diagnostics.DiagnoseOsu(
		"osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n")
	if err != nil {
		t.Fatal(err)
	}
	if report.ErrorCount < 1 {
		t.Fatalf("expected at least one error, got %d", report.ErrorCount)
	}
}

func TestBake3d(t *testing.T) {
	five := 5.0
	scene := Scene3dInput{
		Nodes: []Node3dInput{{Sprite: "sb/dot.png"}},
		Camera: &Camera3dInput{
			Position: []Keyframe3{{Time: 0, Value: []float64{0, 0, five}}},
		},
	}
	baked, err := Storyboarding3d.Bake3d(scene)
	if err != nil {
		t.Fatal(err)
	}
	if len(baked.Sprites) != 1 || baked.Sprites[0].SpritePath != "sb/dot.png" {
		t.Fatalf("baked sprites: %+v", baked.Sprites)
	}
}

func TestMotionGraphicsHelpers(t *testing.T) {
	channel := Channel{Keys: []ChannelKey{
		{Time: 0, Value: []float64{1, 2}},
		{Time: 100, Value: []float64{3, 4}},
	}}
	scaled, err := Storyboarding3d.Helpers.ScaleKeyframes(channel, 2)
	if err != nil {
		t.Fatal(err)
	}
	if scaled.Keys[1].Value[0] != 6 || scaled.Keys[1].Value[1] != 8 {
		t.Fatalf("scaled: %+v", scaled)
	}

	orbit, err := Storyboarding3d.Helpers.Orbit(nil, 10, 10, 0, 6.283185, 5)
	if err != nil {
		t.Fatal(err)
	}
	if len(orbit.Keys) != 5 {
		t.Fatalf("orbit keys: %d", len(orbit.Keys))
	}

	points, err := Storyboarding3d.Helpers.Scatter(1, 3, nil, nil, 0)
	if err != nil {
		t.Fatal(err)
	}
	if len(points) != 3 {
		t.Fatalf("scatter: %d", len(points))
	}

	stagger, err := Storyboarding3d.Helpers.Stagger(3, 100)
	if err != nil {
		t.Fatal(err)
	}
	if stagger != 300 {
		t.Fatalf("stagger: %v", stagger)
	}
}

func TestCommandHelpers(t *testing.T) {
	var commands []StoryboardCommand
	raw := `[
		{"kind":"fade","startTime":0,"endTime":100,"easing":"None",
		 "start":{"kind":"scalar","value":0},"end":{"kind":"scalar","value":1}},
		{"kind":"fade","startTime":200,"endTime":300,"easing":"None",
		 "start":{"kind":"scalar","value":1},"end":{"kind":"scalar","value":0}}
	]`
	if err := json.Unmarshal([]byte(raw), &commands); err != nil {
		t.Fatal(err)
	}

	shifted, err := Storyboarding.Commands.Shift(commands, 50)
	if err != nil {
		t.Fatal(err)
	}
	if shifted[0].StartTime != 50 {
		t.Fatalf("shifted: %v", shifted[0].StartTime)
	}

	start := 150.0
	selected, err := Storyboarding.Commands.Select(commands, nil, &start, nil)
	if err != nil {
		t.Fatal(err)
	}
	if len(selected) != 1 || selected[0].StartTime != 200 {
		t.Fatalf("selected: %+v", selected)
	}

	merged, err := Storyboarding.Commands.Merge(commands, 0)
	if err != nil {
		t.Fatal(err)
	}
	if len(merged) != 2 {
		t.Fatalf("merged: %d", len(merged))
	}

	duplicated, err := Storyboarding.Commands.Duplicate(commands, 1000, 2)
	if err != nil {
		t.Fatal(err)
	}
	if len(duplicated) != 4 {
		t.Fatalf("duplicated: %d", len(duplicated))
	}
}

func TestSharedHelpers(t *testing.T) {
	// A 2D channel (no Z): the Z axis is optional.
	scaled, err := Helpers.Call("keyframe.scaleKeyframes", map[string]any{
		"channel": map[string]any{"keys": []any{
			map[string]any{"time": 0.0, "value": []float64{1, 2}},
			map[string]any{"time": 100.0, "value": []float64{3, 4}},
		}},
		"factor": 2.0,
	})
	if err != nil {
		t.Fatal(err)
	}
	channel := scaled.(map[string]any)
	keys := channel["keys"].([]any)
	values := keys[1].(map[string]any)["value"].([]any)
	if values[0].(float64) != 6 || values[1].(float64) != 8 {
		t.Fatalf("scaled: %v", values)
	}

	// The shared origin/anchor helper.
	resolved, err := Helpers.ResolveAnchor(OSBORIGIN_CENTRE, Vector2{X: 10, Y: -5}, 100, 100)
	if err != nil {
		t.Fatal(err)
	}
	if resolved.X != 60 || resolved.Y != 45 {
		t.Fatalf("resolved: %+v", resolved)
	}

	// The same keyframe helper applied to a 2D command channel.
	var commands []StoryboardCommand
	if err := json.Unmarshal([]byte(`[
		{"kind":"fade","startTime":0,"endTime":100,"easing":"None",
		 "start":{"kind":"scalar","value":0},"end":{"kind":"scalar","value":1}},
		{"kind":"fade","startTime":100,"endTime":200,"easing":"None",
		 "start":{"kind":"scalar","value":1},"end":{"kind":"scalar","value":0}}
	]`), &commands); err != nil {
		t.Fatal(err)
	}
	flattened, err := Helpers.TransformCommands(
		commands, STORYBOARDCOMMANDKIND_FADE, "keyframe.flatten", map[string]any{"mode": "mean"})
	if err != nil {
		t.Fatal(err)
	}
	if len(flattened) != 1 || flattened[0].StartTime != 0 || flattened[0].EndTime != 200 {
		t.Fatalf("flattened: %+v", flattened)
	}
}

func TestSharedHelpersExtended(t *testing.T) {
	amplified, err := Helpers.Call(
		"easing.amplifyEasing",
		map[string]any{"easing": EASING_IN_QUAD, "t": 0.5, "amount": 2.0})
	if err != nil {
		t.Fatal(err)
	}
	if amplified.(float64) > 1e-6 || amplified.(float64) < -1e-6 {
		t.Fatalf("amplify: %v", amplified)
	}

	timeline := map[string]any{"clips": []any{
		map[string]any{"start": 0.0, "channel": map[string]any{"keys": []any{
			map[string]any{"time": 0.0, "value": []float64{0}},
			map[string]any{"time": 100.0, "value": []float64{1}}}}},
		map[string]any{"start": 200.0, "channel": map[string]any{"keys": []any{
			map[string]any{"time": 0.0, "value": []float64{0}},
			map[string]any{"time": 100.0, "value": []float64{1}}}}},
	}}
	duration, err := Helpers.Call("timeline.durationOf", map[string]any{"timeline": timeline})
	if err != nil {
		t.Fatal(err)
	}
	if duration.(float64) != 300 {
		t.Fatalf("duration: %v", duration)
	}

	anchor, err := Helpers.Call("origin.resolveAnchor3d", map[string]any{
		"origin": OSBORIGIN_CENTRE, "anchor": Vector3{X: 10, Y: -5, Z: 42}, "width": 100.0, "height": 100.0})
	if err != nil {
		t.Fatal(err)
	}
	if anchor.(map[string]any)["z"].(float64) != 42 {
		t.Fatalf("anchor: %v", anchor)
	}
}

