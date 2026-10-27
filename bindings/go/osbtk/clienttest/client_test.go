package clienttest

import (
	"context"
	"testing"

	osbtk "github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	options "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
	cleanruntime "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/runtime"
)

func TestHandleBoundCleanClient(t *testing.T) {
	runtime, err := osbtk.NewRuntime(640, 480)
	if err != nil {
		t.Fatal(err)
	}
	defer runtime.Close()

	handle, _, err := runtime.Generator().
		LayerHandle(context.Background()).
		LayerHandleRequest(osbtk.LayerHandleRequest{Name: "Foreground"}).
		Execute()
	if err != nil {
		t.Fatal(err)
	}

	layer := cleanruntime.NewLayer(runtime.Layer(), runtime.Sprite(), handle.Layer)
	if !cleanruntime.IsLayer(layer) {
		t.Fatal("not a layer")
	}

	sprite, _, err := layer.CreateSprite(
		context.Background(),
		options.CreateSpriteOptions{Path: "sb/dot.png", Origin: osbtk.OSBORIGIN_CENTRE},
	)
	if err != nil {
		t.Fatal(err)
	}
	if !cleanruntime.IsSprite(sprite) {
		t.Fatal("not a sprite")
	}

	_, err = sprite.Command(
		context.Background(),
		options.CommandOptions{
			Kind:      "move",
			StartTime: 0,
			EndTime:   1000,
			Easing:    osbtk.EASING_NONE,
			Start:     map[string]any{"x": 0.0, "y": 0.0},
			End:       map[string]any{"x": 100.0, "y": 0.0},
		},
	)
	if err != nil {
		t.Fatal(err)
	}

	if _, err := sprite.EndGroup(context.Background()); err != nil {
		t.Fatal(err)
	}
	if _, err := layer.AddIndex(
		context.Background(),
		options.SegmentAddIndexOptions{Index: 1},
	); err != nil {
		t.Fatal(err)
	}
	if _, _, err := layer.Clean(context.Background()); err != nil {
		t.Fatal(err)
	}

	state, _, err := sprite.StateAt(context.Background(), options.SpriteTimeOptions{Time: 500})
	if err != nil {
		t.Fatal(err)
	}
	if state.Position.X < 49.9 || state.Position.X > 50.1 {
		t.Fatalf("state x: %v", state.Position.X)
	}
}
