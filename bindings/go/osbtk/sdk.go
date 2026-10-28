// Package osbtk is the typed Go SDK over the osbTK native engine. The models and services in this package are generated from the
// OpenAPI contract in `crates/osb-api`; this file wraps the native C ABI with
// `purego` (no cgo) and holds the entry point, the runtime/generator and the
// layer/sprite facade. The per-library facades live in `library_*.go`; the
// composed clean-client surface is the `clientkit` subpackage.
package osbtk

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"runtime"
)

// Client builds the generated bridge client, wired to the native bridge.
func Client() (*ClientAPI, error) {
	library, err := loadNative()
	if err != nil {
		return nil, err
	}
	configuration := NewConfiguration()
	configuration.HTTPClient = &http.Client{Transport: bridgeTransport{exec: library.callExec}}
	return NewClientAPI(configuration), nil
}

// Runtime is a live storyboard runtime bound to a native handle.
type Runtime struct {
	library *nativeLibrary
	handle  uintptr
	client  *ClientAPI
}

// NewRuntime creates a runtime of the given storyboard resolution.
func NewRuntime(width, height float64) (*Runtime, error) {
	library, err := loadNative()
	if err != nil {
		return nil, err
	}
	instance := &Runtime{library: library, handle: library.runtimeNew(width, height)}
	configuration := NewConfiguration()
	configuration.HTTPClient = &http.Client{Transport: bridgeTransport{exec: instance.callExec}}
	instance.client = NewClientAPI(configuration)
	runtime.SetFinalizer(instance, func(r *Runtime) { r.Close() })
	return instance, nil
}

// Generator is the generated generator client bound to this runtime.
func (instance *Runtime) Generator() *GeneratorAPIService {
	return instance.client.GeneratorAPI
}

// Layer is the generated layer client bound to this runtime.
func (instance *Runtime) Layer() *LayerAPIService {
	return instance.client.LayerAPI
}

// Sprite is the generated sprite client bound to this runtime.
func (instance *Runtime) Sprite() *SpriteAPIService {
	return instance.client.SpriteAPI
}

// Audio is the generated audio client bound to this runtime.
func (instance *Runtime) Audio() *AudioAPIService {
	return instance.client.AudioAPI
}

func (instance *Runtime) callExec(request string) (string, error) {
	buffer := cString(request)
	result := instance.library.runtimeExec(instance.handle, &buffer[0])
	runtime.KeepAlive(buffer)
	if result == nil {
		return "", fmt.Errorf("osbtk: null response")
	}
	text := goString(result)
	instance.library.stringFree(result)
	return text, nil
}

// Close frees the underlying native runtime.
func (instance *Runtime) Close() {
	if instance.handle != 0 {
		runtime.SetFinalizer(instance, nil)
		instance.library.runtimeFree(instance.handle)
		instance.handle = 0
	}
}

// Generator is a storyboard generator (the StoryboardObjectGenerator
// equivalent). Set Generate and call it, or use the helpers.
type Generator struct {
	runtime *Runtime
	bridge  bridgeFunc
	// Generate builds the storyboard (`Generate`).
	Generate func(*Generator)
}

// NewGenerator creates a generator for a storyboard of the given resolution.
func NewGenerator(width, height float64) (*Generator, error) {
	instance, err := NewRuntime(width, height)
	if err != nil {
		return nil, err
	}
	return &Generator{runtime: instance, bridge: runtimeBridge(instance)}, nil
}

// GetLayer returns a layer that creates sprites/animations/samples (`GetLayer`).
func (g *Generator) GetLayer(name string) (*Layer, error) {
	response, err := g.bridge(map[string]any{"op": "Generator.layerHandle", "name": name})
	if err != nil {
		return nil, err
	}
	return &Layer{bridge: g.bridge, name: name, handle: asInt(response["layer"])}, nil
}

// CreateSprite creates a sprite on a layer (`CreateSprite`).
func (g *Generator) CreateSprite(layer, path string, origin OsbOrigin) (*Handle, error) {
	request := CreateSpriteRequest{Path: path, Origin: origin}
	request.SetLayer(layer)
	handle, _, err := g.runtime.Layer().CreateSprite(context.Background()).
		CreateSpriteRequest(request).
		Execute()
	return handle, err
}

// CreateAnimation creates an animation sprite (`CreateAnimation`).
func (g *Generator) CreateAnimation(request CreateAnimationRequest) (*Handle, error) {
	handle, _, err := g.runtime.Layer().CreateAnimation(context.Background()).
		CreateAnimationRequest(request).
		Execute()
	return handle, err
}

// CreateSample adds a sound sample to a layer (`CreateSample`).
func (g *Generator) CreateSample(layer, path string, time, volume float64) error {
	request := CreateSampleRequest{
		Path:   path,
		Time:   time,
		Volume: *NewNullableFloat64(&volume),
	}
	request.SetLayer(layer)
	_, _, err := g.runtime.Layer().CreateSample(context.Background()).
		CreateSampleRequest(request).
		Execute()
	return err
}

// GetBeatmap returns the loaded beatmap (`GetBeatmap`).
func (g *Generator) GetBeatmap() (*Beatmap, error) {
	beatmap, _, err := g.runtime.Generator().Beatmap(context.Background()).Execute()
	return beatmap, err
}

// GetFft returns the FFT magnitudes of the loaded audio (`GetFft`).
func (g *Generator) GetFft(time float64, magnitudes int32) (*FftResult, error) {
	result, _, err := g.runtime.Audio().GetFft(context.Background()).
		GetFftRequest(GetFftRequest{Time: time, Magnitudes: magnitudes}).
		Execute()
	return result, err
}

// TimingPointAt returns the active red timing point (`GetTimingPointAt`).
func (g *Generator) TimingPointAt(time float64) (*ControlPoint, error) {
	point, _, err := g.runtime.Generator().TimingPointAt(context.Background()).
		TimeRequest(TimeRequest{Time: time}).
		Execute()
	return point, err
}

// SpriteCount returns the number of sprites on the storyboard.
func (g *Generator) SpriteCount() (int32, error) {
	result, _, err := g.runtime.Generator().SpriteCount(context.Background()).Execute()
	if err != nil {
		return 0, err
	}
	return result.SpriteCount, nil
}

// Storyboard returns the built storyboard model (`storyboard`).
func (g *Generator) Storyboard() (*Storyboard, error) {
	storyboard, _, err := g.runtime.Generator().Storyboard(context.Background()).Execute()
	return storyboard, err
}

// Close frees the underlying runtime.
func (g *Generator) Close() {
	g.runtime.Close()
}

// RunGenerator runs a generator's Generate (effect entry point).
func RunGenerator(g *Generator) {
	if g.Generate != nil {
		g.Generate(g)
	}
}

// bridgeFunc is a JSON-dict bridge over a runtime.
type bridgeFunc func(request map[string]any) (map[string]any, error)

func runtimeBridge(instance *Runtime) bridgeFunc {
	return func(request map[string]any) (map[string]any, error) {
		text, err := json.Marshal(request)
		if err != nil {
			return nil, err
		}
		response, err := instance.callExec(string(text))
		if err != nil {
			return nil, err
		}
		var result map[string]any
		if err := json.Unmarshal([]byte(response), &result); err != nil {
			return nil, err
		}
		if message, ok := result["error"].(string); ok {
			return nil, fmt.Errorf("osbtk: %s", message)
		}
		return result, nil
	}
}

func asInt(value any) int {
	if number, ok := value.(float64); ok {
		return int(number)
	}
	return 0
}

// Layer is a storyboard segment (`StoryboardSegment`). The handle is implicit.
type Layer struct {
	bridge bridgeFunc
	name   string
	handle int
}

// CreateSprite creates a sprite on the segment (`CreateSprite`).
func (l *Layer) CreateSprite(texturePath string, origin OsbOrigin) (*Sprite, error) {
	response, err := l.bridge(map[string]any{
		"op": "Layer.createSprite", "layerHandle": l.handle, "path": texturePath, "origin": origin,
	})
	if err != nil {
		return nil, err
	}
	return &Sprite{
		bridge: l.bridge,
		layer:  asInt(response["layer"]),
		sprite: asInt(response["sprite"]),
	}, nil
}

// CreateSample adds a sound sample (`CreateSample`).
func (l *Layer) CreateSample(samplePath string, time, volume float64) (*Layer, error) {
	if _, err := l.bridge(map[string]any{
		"op": "Layer.createSample", "layerHandle": l.handle, "path": samplePath, "time": time, "volume": volume,
	}); err != nil {
		return nil, err
	}
	return l, nil
}

// CreateSegment creates a segment nested in this one (`CreateSegment`).
func (l *Layer) CreateSegment() (*Layer, error) {
	response, err := l.bridge(map[string]any{"op": "Layer.createSegment", "layer": l.handle})
	if err != nil {
		return nil, err
	}
	return &Layer{bridge: l.bridge, handle: asInt(response["layer"])}, nil
}

// SetReverseDepth sets the reverse-depth flag (`ReverseDepth`).
func (l *Layer) SetReverseDepth(value bool) (*Layer, error) {
	if _, err := l.bridge(map[string]any{
		"op": "Layer.reverseDepth", "layer": l.handle, "value": value,
	}); err != nil {
		return nil, err
	}
	return l, nil
}

// AddIndex records a depth index (`AddIndex`).
func (l *Layer) AddIndex(index float64) (*Layer, error) {
	if _, err := l.bridge(map[string]any{
		"op": "Layer.addIndex", "layer": l.handle, "index": index,
	}); err != nil {
		return nil, err
	}
	return l, nil
}

// Indexes returns the depth indexes and bounds.
func (l *Layer) Indexes() (map[string]any, error) {
	return l.bridge(map[string]any{"op": "Layer.indexes", "layer": l.handle})
}

// ChangePosition moves the segment to another osu! layer (`ChangePosition`).
func (l *Layer) ChangePosition(osbLayer OsbLayer) (*Layer, error) {
	if _, err := l.bridge(map[string]any{
		"op": "Layer.changePosition", "layer": l.handle, "osbLayer": osbLayer,
	}); err != nil {
		return nil, err
	}
	return l, nil
}

// MoveToTop moves the segment on top of its siblings (`MoveToTop`).
func (l *Layer) MoveToTop() (*Layer, error) {
	if _, err := l.bridge(map[string]any{"op": "Layer.moveToTop", "layer": l.handle}); err != nil {
		return nil, err
	}
	return l, nil
}

// MoveToBottom moves the segment below its siblings (`MoveToBottom`).
func (l *Layer) MoveToBottom() (*Layer, error) {
	if _, err := l.bridge(map[string]any{"op": "Layer.moveToBottom", "layer": l.handle}); err != nil {
		return nil, err
	}
	return l, nil
}

// Clean removes zero-duration sprites (`Clean`).
func (l *Layer) Clean() (map[string]any, error) {
	return l.bridge(map[string]any{"op": "Layer.clean", "layer": l.handle})
}

// Discard removes the segment (`Discard`).
func (l *Layer) Discard() (map[string]any, error) {
	return l.bridge(map[string]any{"op": "Layer.discard", "layer": l.handle})
}

// DiscardSprite removes a sprite from output (`DiscardSprite`).
func (l *Layer) DiscardSprite(sprite *Sprite) (*Layer, error) {
	if _, err := l.bridge(map[string]any{
		"op": "Layer.discardSprite", "layer": l.handle, "sprite": sprite.sprite,
	}); err != nil {
		return nil, err
	}
	return l, nil
}

// Sprite is a storyboard sprite (OsbSprite).
type Sprite struct {
	bridge bridgeFunc
	layer  int
	sprite int
}

// Info returns the sprite's initial state and metadata.
func (s *Sprite) Info() (map[string]any, error) {
	return s.bridge(map[string]any{"op": "Sprite.info", "layer": s.layer, "sprite": s.sprite})
}

// Move adds a move command (`Move`).
func (s *Sprite) Move(startTime, endTime float64, start, end Vector2, easing Easing) (*Sprite, error) {
	return s.add("move", startTime, endTime, easing, start, end, "")
}

// Scale adds a uniform scale command (`Scale`).
func (s *Sprite) Scale(startTime, endTime, start, end float64, easing Easing) (*Sprite, error) {
	return s.add("scale", startTime, endTime, easing, start, end, "")
}

// Rotate adds a rotation command (`Rotate`).
func (s *Sprite) Rotate(startTime, endTime, start, end float64, easing Easing) (*Sprite, error) {
	return s.add("rotate", startTime, endTime, easing, start, end, "")
}

// Fade adds a fade command (`Fade`).
func (s *Sprite) Fade(startTime, endTime, start, end float64, easing Easing) (*Sprite, error) {
	return s.add("fade", startTime, endTime, easing, start, end, "")
}

// Color adds a color command (`Color`).
func (s *Sprite) Color(startTime, endTime float64, start, end Color4, easing Easing) (*Sprite, error) {
	return s.add("color", startTime, endTime, easing, start, end, "")
}

// MoveX adds a horizontal move command (`MoveX`).
func (s *Sprite) MoveX(startTime, endTime, start, end float64, easing Easing) (*Sprite, error) {
	return s.add("moveX", startTime, endTime, easing, start, end, "")
}

// MoveY adds a vertical move command (`MoveY`).
func (s *Sprite) MoveY(startTime, endTime, start, end float64, easing Easing) (*Sprite, error) {
	return s.add("moveY", startTime, endTime, easing, start, end, "")
}

// ScaleVec adds a per-axis scale command (`ScaleVec`).
func (s *Sprite) ScaleVec(startTime, endTime float64, start, end Vector2, easing Easing) (*Sprite, error) {
	return s.add("scaleVec", startTime, endTime, easing, start, end, "")
}

// ColorHsb adds a hue/saturation/brightness color command (`ColorHsb`).
func (s *Sprite) ColorHsb(startTime, endTime float64, start, end Color4, easing Easing) (*Sprite, error) {
	return s.add("colorHsb", startTime, endTime, easing, start, end, "")
}

// Parameter adds a parameter command (`Parameter`).
func (s *Sprite) Parameter(parameter ParameterType, startTime, endTime, start, end float64, easing Easing) (*Sprite, error) {
	return s.add("parameter", startTime, endTime, easing, start, end, parameter)
}

// FlipH adds a horizontal-flip command (`FlipH`).
func (s *Sprite) FlipH(startTime, endTime float64) (*Sprite, error) {
	return s.add("flipH", startTime, endTime, EASING_NONE, 0, 1, "")
}

// FlipV adds a vertical-flip command (`FlipV`).
func (s *Sprite) FlipV(startTime, endTime float64) (*Sprite, error) {
	return s.add("flipV", startTime, endTime, EASING_NONE, 0, 1, "")
}

// Additive adds an additive-blend command (`Additive`).
func (s *Sprite) Additive(startTime, endTime float64) (*Sprite, error) {
	return s.add("additive", startTime, endTime, EASING_NONE, 0, 1, "")
}

// Anchor adds an animatable origin-offset command (`Anchor`). The offset is in
// texture pixels and is added to the sprite's enum origin.
func (s *Sprite) Anchor(startTime, endTime float64, start, end Vector2, easing Easing) (*Sprite, error) {
	return s.add("anchor", startTime, endTime, easing, start, end, "")
}

// SetAnchor sets a constant origin offset (a zero-length `Anchor`).
func (s *Sprite) SetAnchor(point Vector2) (*Sprite, error) {
	return s.add("anchor", 0, 0, EASING_NONE, point, point, "")
}

// MoveTo moves the sprite to a position at a time (instant `Move`).
func (s *Sprite) MoveTo(time float64, position Vector2) (*Sprite, error) {
	return s.add("move", time, time, EASING_NONE, position, position, "")
}

// FadeTo sets the opacity at a time (instant `Fade`).
func (s *Sprite) FadeTo(time, opacity float64) (*Sprite, error) {
	return s.add("fade", time, time, EASING_NONE, opacity, opacity, "")
}

// ScaleTo sets the uniform scale at a time (instant `Scale`).
func (s *Sprite) ScaleTo(time, scale float64) (*Sprite, error) {
	return s.add("scale", time, time, EASING_NONE, scale, scale, "")
}

// RotateTo sets the rotation at a time (instant `Rotate`).
func (s *Sprite) RotateTo(time, rotation float64) (*Sprite, error) {
	return s.add("rotate", time, time, EASING_NONE, rotation, rotation, "")
}

// StartLoopGroup opens a loop group (`StartLoopGroup`).
func (s *Sprite) StartLoopGroup(startTime, count float64) (*Sprite, error) {
	if _, err := s.bridge(map[string]any{
		"op": "Sprite.startLoopGroup", "layer": s.layer, "sprite": s.sprite, "startTime": startTime, "count": count,
	}); err != nil {
		return nil, err
	}
	return s, nil
}

// EndGroup closes the active loop/trigger group (`EndGroup`).
func (s *Sprite) EndGroup() (*Sprite, error) {
	if _, err := s.bridge(map[string]any{
		"op": "Sprite.endGroup", "layer": s.layer, "sprite": s.sprite,
	}); err != nil {
		return nil, err
	}
	return s, nil
}

// PositionAt samples the position at a time (`PositionAt`).
func (s *Sprite) PositionAt(time float64) (Vector2, error) {
	state, err := s.stateAt(time)
	if err != nil {
		return Vector2{}, err
	}
	return decodeVector(state["position"]), nil
}

// OpacityAt samples the opacity at a time (`OpacityAt`).
func (s *Sprite) OpacityAt(time float64) (float64, error) {
	state, err := s.stateAt(time)
	if err != nil {
		return 0, err
	}
	if value, ok := state["opacity"].(float64); ok {
		return value, nil
	}
	return 0, nil
}

// AnchorAt samples the origin offset at a time (`AnchorAt`).
func (s *Sprite) AnchorAt(time float64) (Vector2, error) {
	state, err := s.stateAt(time)
	if err != nil {
		return Vector2{}, err
	}
	return decodeVector(state["anchor"]), nil
}

func (s *Sprite) stateAt(time float64) (map[string]any, error) {
	return s.bridge(map[string]any{
		"op": "Sprite.stateAt", "layer": s.layer, "sprite": s.sprite, "time": time,
	})
}

func decodeVector(value any) Vector2 {
	if object, ok := value.(map[string]any); ok {
		return Vector2{X: asFloat(object["x"]), Y: asFloat(object["y"])}
	}
	return Vector2{}
}

func asFloat(value any) float64 {
	if number, ok := value.(float64); ok {
		return number
	}
	return 0
}

func (s *Sprite) add(
	kind string,
	startTime, endTime float64,
	easing Easing,
	start, end any,
	parameter ParameterType,
) (*Sprite, error) {
	request := map[string]any{
		"op":        "Sprite.command",
		"kind":      kind,
		"layer":     s.layer,
		"sprite":    s.sprite,
		"startTime": startTime,
		"endTime":   endTime,
		"easing":    easing,
		"start":     start,
		"end":       end,
	}
	if parameter != "" {
		request["parameter"] = parameter
	}
	if _, err := s.bridge(request); err != nil {
		return nil, err
	}
	return s, nil
}
