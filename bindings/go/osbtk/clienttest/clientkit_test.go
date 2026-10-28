package clienttest

import (
	"context"
	"testing"

	"github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
	"github.com/tochiResources/osb-toolkit/bindings/go/osbtk/clientkit"
)

func TestComposedStatelessClient(t *testing.T) {
	client, err := clientkit.New()
	if err != nil {
		t.Fatal(err)
	}
	curve := osbtk.CurveSpec{
		Kind:   osbtk.CURVEKIND_LINEAR,
		Points: []osbtk.Vector2{{X: 0, Y: 0}, {X: 100, Y: 0}},
	}
	length, _, err := client.Curve.Length(
		context.Background(),
		clean.CurveOptions{Curve: curve},
	)
	if err != nil {
		t.Fatal(err)
	}
	if length < 99.9 || length > 100.1 {
		t.Fatalf("length: %v", length)
	}
}

func TestComposedHelpersAcceptStructuredInput(t *testing.T) {
	client, err := clientkit.New()
	if err != nil {
		t.Fatal(err)
	}
	value, _, err := client.Helpers.Call(
		context.Background(),
		clean.HelperOptions{
			Name: "keyframe.scaleKeyframes",
			Input: map[string]interface{}{
				"channel": map[string]interface{}{
					"keys": []interface{}{
						map[string]interface{}{"time": 0.0, "value": []float64{1, 2}},
						map[string]interface{}{"time": 100.0, "value": []float64{3, 4}},
					},
				},
				"factor": 2.0,
			},
		},
	)
	if err != nil {
		t.Fatal(err)
	}
	channel, ok := value.(map[string]interface{})
	if !ok {
		t.Fatalf("helper result: %T", value)
	}
	keys, ok := channel["keys"].([]interface{})
	if !ok || len(keys) != 2 {
		t.Fatalf("keys: %+v", channel["keys"])
	}
	last, ok := keys[1].(map[string]interface{})
	if !ok {
		t.Fatalf("key: %T", keys[1])
	}
	values, ok := last["value"].([]interface{})
	if !ok || len(values) != 2 {
		t.Fatalf("values: %+v", last["value"])
	}
	if values[0].(float64) != 6 || values[1].(float64) != 8 {
		t.Fatalf("scaled: %+v", values)
	}
}

func TestComposedRuntimeBindsLayers(t *testing.T) {
	runtime, err := clientkit.NewRuntime(640, 480)
	if err != nil {
		t.Fatal(err)
	}
	defer runtime.Close()

	layer, err := runtime.Layer("Foreground")
	if err != nil {
		t.Fatal(err)
	}
	sprite, _, err := layer.CreateSprite(
		context.Background(),
		clean.CreateSpriteOptions{Path: "sb/dot.png", Origin: osbtk.OSBORIGIN_CENTRE},
	)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := sprite.Command(
		context.Background(),
		clean.CommandOptions{
			Kind:      "move",
			StartTime: 0,
			EndTime:   1000,
			Easing:    osbtk.EASING_NONE,
			Start:     map[string]interface{}{"x": 0.0, "y": 0.0},
			End:       map[string]interface{}{"x": 100.0, "y": 0.0},
		},
	); err != nil {
		t.Fatal(err)
	}

	state, _, err := sprite.StateAt(context.Background(), clean.SpriteTimeOptions{Time: 500})
	if err != nil {
		t.Fatal(err)
	}
	if state.Position.X < 49.9 || state.Position.X > 50.1 {
		t.Fatalf("state x: %v", state.Position.X)
	}
}
