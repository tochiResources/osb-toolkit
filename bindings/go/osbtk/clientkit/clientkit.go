// Package clientkit composes the generated clean clients over the native
// bridge. The root osbtk package cannot import its own client subpackages (they
// import the generated models from the root), so this subpackage is the
// composed clean surface: one Client for the stateless libraries and one
// Runtime that resolves handle-bound Layer/Sprite values.
//
// The clean clients are options-only by design; Go has no overloading, so the
// positional convenience stays on the root osbtk facades.
package clientkit

import (
	"context"
	"errors"

	"github.com/tochiResources/osb-toolkit/bindings/go/osbtk"
	core "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/core"
	libraries "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/libraries"
	clean "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/options"
	cleanruntime "github.com/tochiResources/osb-toolkit/bindings/go/osbtk/client/runtime"
)

// Client is the stateless SDK surface: the core and library clean clients.
type Client struct {
	API *osbtk.ClientAPI

	Core *core.Core

	Audio           *cleanruntime.Audio
	Helpers         *cleanruntime.Helpers
	Storyboarding3d *cleanruntime.Storyboarding3d

	Diagnostics *libraries.Diagnostics
	Optimize    *libraries.Optimize
	Image       *libraries.Image
	ImageGen    *libraries.Imagegen
	Media       *libraries.Media
}

// New builds the stateless client over the native bridge.
func New() (*Client, error) {
	api, err := osbtk.Client()
	if err != nil {
		return nil, err
	}
	return &Client{
		API:             api,
		Core:            core.NewCore(api.CoreAPI),
		Audio:           cleanruntime.NewAudio(api.AudioAPI),
		Helpers:         cleanruntime.NewHelpers(api.HelpersAPI),
		Storyboarding3d: cleanruntime.NewStoryboarding3d(api.Storyboarding3dAPI),
		Diagnostics:     libraries.NewDiagnostics(api.DiagnosticsAPI),
		Optimize:        libraries.NewOptimize(api.OptimizeAPI),
		Image:           libraries.NewImage(api.ImageAPI),
		ImageGen:        libraries.NewImagegen(api.ImagegenAPI),
		Media:           libraries.NewMedia(api.MediaAPI),
	}, nil
}

// Runtime is a live storyboard runtime with the clean generator/audio clients
// and helpers that bind Layer/Sprite handles.
type Runtime struct {
	*osbtk.Runtime

	Generator *cleanruntime.Generator
	Audio     *cleanruntime.Audio

	layers  *osbtk.LayerAPIService
	sprites *osbtk.SpriteAPIService
}

// NewRuntime creates a runtime of the given storyboard resolution.
func NewRuntime(width, height float64) (*Runtime, error) {
	instance, err := osbtk.NewRuntime(width, height)
	if err != nil {
		return nil, err
	}
	return &Runtime{
		Runtime:   instance,
		Generator: cleanruntime.NewGenerator(instance.Generator()),
		Audio:     cleanruntime.NewAudio(instance.Audio()),
		layers:    instance.Layer(),
		sprites:   instance.Sprite(),
	}, nil
}

// Layer resolves (creating on first use) a layer by name and binds its handle.
func (r *Runtime) Layer(name string) (*cleanruntime.Layer, error) {
	if name == "" {
		return nil, errors.New("osbtk: layer name is required")
	}
	handle, _, err := r.Generator.LayerHandle(
		context.Background(),
		clean.LayerHandleOptions{Name: name},
	)
	if err != nil {
		return nil, err
	}
	return cleanruntime.NewLayer(r.layers, r.sprites, handle), nil
}

// Sprite binds an existing layer/sprite handle pair.
func (r *Runtime) Sprite(layer, sprite int32) *cleanruntime.Sprite {
	return cleanruntime.NewSprite(r.sprites, layer, sprite)
}
