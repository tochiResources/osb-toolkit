use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::client::parse;
use crate::error::Error;
use crate::models::{
    AudioDurationResult, Beatmap, BoolResult, Color4, ControlPoint, Easing, FftResult, Handle,
    ImportOsbResult, LayerHandle, LoadBeatmapResult, OkResult, OsbLayer, OsbLoopType, OsbOrigin,
    ParameterType, RandomResult, SegmentCleanResult, SegmentDiscardResult, SegmentIndexesResult,
    SpriteCountResult, SpriteInfo, SpriteState, Vector2,
};

/// A live storyboard runtime bound to the native engine.
pub struct Runtime {
    inner: osb_native::OsbtkRuntime,
}

impl Runtime {
    /// Creates a runtime of the given storyboard resolution.
    pub fn new(width: f32, height: f32) -> Self {
        Runtime {
            inner: osb_native::OsbtkRuntime::new(width, height),
        }
    }

    /// The number of sprites on the storyboard.
    pub fn sprite_count(&mut self) -> Result<i32, Error> {
        Ok(self
            .call::<SpriteCountResult>(json!({ "op": "Generator.spriteCount" }))?
            .sprite_count)
    }

    /// The built storyboard model (`storyboard`).
    pub fn storyboard(&mut self) -> Result<crate::models::Storyboard, Error> {
        self.call(json!({ "op": "Generator.storyboard" }))
    }

    /// The loaded audio duration in milliseconds.
    pub fn audio_duration(&mut self) -> Result<f64, Error> {
        Ok(self
            .call::<AudioDurationResult>(json!({ "op": "Audio.audioDuration" }))?
            .audio_duration)
    }

    /// Imports a `.osb` storyboard (`ImportOsb`).
    pub fn import_osb(&mut self, text: &str) -> Result<ImportOsbResult, Error> {
        self.call(json!({ "op": "Generator.importOsb", "text": text }))
    }

    /// Loads a `.osu` beatmap into the runtime.
    pub fn load_beatmap(&mut self, text: &str) -> Result<LoadBeatmapResult, Error> {
        self.call(json!({ "op": "Generator.loadBeatmap", "text": text }))
    }

    /// The loaded beatmap, or `None`.
    pub fn beatmap(&mut self) -> Result<Option<Beatmap>, Error> {
        let value: Value = self.call(json!({ "op": "Generator.beatmap" }))?;
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(serde_json::from_value(value)?))
        }
    }

    /// The active red timing point at a time.
    pub fn timing_point_at(&mut self, time: f64) -> Result<ControlPoint, Error> {
        self.call(json!({ "op": "Generator.timingPointAt", "time": time }))
    }

    /// The active control point at a time, or `None`.
    pub fn control_point_at(&mut self, time: f64) -> Result<Option<ControlPoint>, Error> {
        let value: Value = self.call(json!({ "op": "Generator.controlPointAt", "time": time }))?;
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(serde_json::from_value(value)?))
        }
    }

    /// Returns (creating on first use) a layer handle by name.
    pub fn layer_handle(&mut self, name: &str) -> Result<LayerHandle, Error> {
        self.call(json!({ "op": "Generator.layerHandle", "name": name }))
    }

    /// Returns a [`Segment`] view for a layer (creating it on first use), with the
    /// handle kept implicit behind the object.
    pub fn layer(&mut self, name: &str) -> Result<Segment<'_>, Error> {
        let handle = self.layer_handle(name)?;
        Ok(Segment {
            runtime: self,
            handle,
        })
    }

    /// A deterministic random value in `[min, max)`.
    pub fn random(&mut self, min: f64, max: f64) -> Result<f64, Error> {
        Ok(self
            .call::<RandomResult>(json!({ "op": "Generator.random", "min": min, "max": max }))?
            .value)
    }

    /// Creates a sprite on a layer.
    pub fn create_sprite(
        &mut self,
        layer: &str,
        path: &str,
        origin: OsbOrigin,
    ) -> Result<Handle, Error> {
        self.call(
            json!({ "op": "Layer.createSprite", "layer": layer, "path": path, "origin": origin }),
        )
    }

    /// Creates a positioned sprite on a layer.
    pub fn create_sprite_at(
        &mut self,
        layer: &str,
        path: &str,
        origin: OsbOrigin,
        position: Vector2,
    ) -> Result<Handle, Error> {
        self.call(json!({
            "op": "Layer.createSprite", "layer": layer, "path": path, "origin": origin, "position": position
        }))
    }

    /// Creates an animation sprite on a layer.
    #[allow(clippy::too_many_arguments)]
    pub fn create_animation(
        &mut self,
        layer: &str,
        path: &str,
        frames: i32,
        frame_delay: f64,
        loop_type: OsbLoopType,
        origin: OsbOrigin,
        position: Vector2,
    ) -> Result<Handle, Error> {
        self.call(json!({
            "op": "Layer.createAnimation", "layer": layer, "path": path, "frames": frames,
            "frameDelay": frame_delay, "loopType": loop_type, "origin": origin, "position": position
        }))
    }

    /// Adds a move command.
    pub fn move_(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: Vector2,
        end: Vector2,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "move",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Adds a uniform scale command.
    pub fn scale(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: f64,
        end: f64,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "scale",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Adds a rotation command (radians).
    pub fn rotate(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: f64,
        end: f64,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "rotate",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Adds a horizontal move command.
    pub fn move_x(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: f64,
        end: f64,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "moveX",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Adds a vertical move command.
    pub fn move_y(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: f64,
        end: f64,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "moveY",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Adds a per-axis scale command.
    pub fn scale_vec(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: Vector2,
        end: Vector2,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "scaleVec",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Adds a fade command.
    pub fn fade(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: f64,
        end: f64,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "fade",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Adds a color command.
    pub fn color(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: Color4,
        end: Color4,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "color",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Adds a hue/saturation/brightness color command.
    pub fn color_hsb(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: Color4,
        end: Color4,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "colorHsb",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Adds a parameter command (`Parameter`).
    #[allow(clippy::too_many_arguments)]
    pub fn parameter(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        parameter: ParameterType,
        start: f64,
        end: f64,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "parameter",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end, "parameter": parameter }),
        )
    }

    /// Adds a horizontal-flip command (`FlipH`).
    pub fn flip_h(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
    ) -> Result<OkResult, Error> {
        self.parameter(
            handle,
            start_time,
            end_time,
            ParameterType::FlipHorizontal,
            0.0,
            1.0,
            Easing::None,
        )
    }

    /// Adds a vertical-flip command (`FlipV`).
    pub fn flip_v(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
    ) -> Result<OkResult, Error> {
        self.parameter(
            handle,
            start_time,
            end_time,
            ParameterType::FlipVertical,
            0.0,
            1.0,
            Easing::None,
        )
    }

    /// Adds an additive-blend parameter command.
    pub fn additive(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: f64,
        end: f64,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "additive",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Animates a sprite's origin offset (`Anchor`).
    pub fn anchor(
        &mut self,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        start: Vector2,
        end: Vector2,
        easing: Easing,
    ) -> Result<OkResult, Error> {
        self.command(
            "anchor",
            handle,
            start_time,
            end_time,
            easing,
            json!({ "start": start, "end": end }),
        )
    }

    /// Sets a constant origin offset (a zero-length `Anchor`).
    pub fn set_anchor(&mut self, handle: &Handle, point: Vector2) -> Result<OkResult, Error> {
        self.anchor(handle, 0.0, 0.0, point.clone(), point, Easing::None)
    }

    /// Samples the sprite's origin offset at a time (`AnchorAt`).
    pub fn anchor_at(&mut self, handle: &Handle, time: f64) -> Result<Vector2, Error> {
        Ok(*self.sprite_state_at(handle, time)?.anchor)
    }

    /// Samples the sprite's full state at a time.
    pub fn sprite_state_at(&mut self, handle: &Handle, time: f64) -> Result<SpriteState, Error> {
        self.call(json!({
            "op": "Sprite.stateAt", "layer": handle.layer, "sprite": handle.sprite, "time": time
        }))
    }

    /// The sprite's initial state and metadata.
    pub fn sprite_info(&mut self, handle: &Handle) -> Result<SpriteInfo, Error> {
        self.call(json!({
            "op": "Sprite.info", "layer": handle.layer, "sprite": handle.sprite
        }))
    }

    /// Adds a sound sample to a layer (`CreateSample`).
    pub fn create_sample(
        &mut self,
        layer: &str,
        path: &str,
        time: f64,
        volume: f64,
    ) -> Result<OkResult, Error> {
        self.call(json!({
            "op": "Layer.createSample", "layer": layer, "path": path, "time": time, "volume": volume
        }))
    }

    /// Creates an anonymous segment nested in `parent` (`CreateSegment`).
    pub fn create_segment(&mut self, parent: &LayerHandle) -> Result<LayerHandle, Error> {
        self.call(json!({ "op": "Layer.createSegment", "layer": parent.layer }))
    }

    /// Creates a sprite on a segment.
    pub fn create_sprite_on(
        &mut self,
        segment: &LayerHandle,
        path: &str,
        origin: OsbOrigin,
    ) -> Result<Handle, Error> {
        self.call(json!({
            "op": "Layer.createSprite", "layerHandle": segment.layer, "path": path, "origin": origin
        }))
    }

    /// Creates a positioned sprite on a segment.
    pub fn create_sprite_at_on(
        &mut self,
        segment: &LayerHandle,
        path: &str,
        origin: OsbOrigin,
        position: Vector2,
    ) -> Result<Handle, Error> {
        self.call(json!({
            "op": "Layer.createSprite", "layerHandle": segment.layer, "path": path, "origin": origin,
            "position": position
        }))
    }

    /// Creates an animation sprite on a segment.
    #[allow(clippy::too_many_arguments)]
    pub fn create_animation_on(
        &mut self,
        segment: &LayerHandle,
        path: &str,
        frames: i32,
        frame_delay: f64,
        loop_type: OsbLoopType,
        origin: OsbOrigin,
        position: Vector2,
    ) -> Result<Handle, Error> {
        self.call(json!({
            "op": "Layer.createAnimation", "layerHandle": segment.layer, "path": path, "frames": frames,
            "frameDelay": frame_delay, "loopType": loop_type, "origin": origin, "position": position
        }))
    }

    /// Adds a sound sample to a segment.
    pub fn create_sample_on(
        &mut self,
        segment: &LayerHandle,
        path: &str,
        time: f64,
        volume: f64,
    ) -> Result<OkResult, Error> {
        self.call(json!({
            "op": "Layer.createSample", "layerHandle": segment.layer, "path": path, "time": time,
            "volume": volume
        }))
    }

    /// Sets a segment's reverse-depth flag (`ReverseDepth`).
    pub fn segment_reverse_depth(
        &mut self,
        segment: &LayerHandle,
        value: bool,
    ) -> Result<OkResult, Error> {
        self.call(json!({ "op": "Layer.reverseDepth", "layer": segment.layer, "value": value }))
    }

    /// Records a depth index on a segment (`AddIndex`).
    pub fn segment_add_index(
        &mut self,
        segment: &LayerHandle,
        index: f64,
    ) -> Result<OkResult, Error> {
        self.call(json!({ "op": "Layer.addIndex", "layer": segment.layer, "index": index }))
    }

    /// The segment's depth indexes and bounds (`Indexes`/`LowestIndex`/`HighestIndex`).
    pub fn segment_indexes(
        &mut self,
        segment: &LayerHandle,
    ) -> Result<SegmentIndexesResult, Error> {
        self.call(json!({ "op": "Layer.indexes", "layer": segment.layer }))
    }

    /// Moves a segment to another osu! layer (`ChangePosition`).
    pub fn segment_change_position(
        &mut self,
        segment: &LayerHandle,
        osb_layer: OsbLayer,
    ) -> Result<OkResult, Error> {
        self.call(json!({
            "op": "Layer.changePosition", "layer": segment.layer, "osbLayer": osb_layer
        }))
    }

    /// Moves a segment on top of its siblings (`MoveToTop`).
    pub fn segment_move_to_top(&mut self, segment: &LayerHandle) -> Result<OkResult, Error> {
        self.call(json!({ "op": "Layer.moveToTop", "layer": segment.layer }))
    }

    /// Moves a segment below its siblings (`MoveToBottom`).
    pub fn segment_move_to_bottom(&mut self, segment: &LayerHandle) -> Result<OkResult, Error> {
        self.call(json!({ "op": "Layer.moveToBottom", "layer": segment.layer }))
    }

    /// Removes zero-duration sprites from a segment (`Clean`).
    pub fn segment_clean(&mut self, segment: &LayerHandle) -> Result<SegmentCleanResult, Error> {
        self.call(json!({ "op": "Layer.clean", "layer": segment.layer }))
    }

    /// Removes a segment (`Discard`).
    pub fn segment_discard(
        &mut self,
        segment: &LayerHandle,
    ) -> Result<SegmentDiscardResult, Error> {
        self.call(json!({ "op": "Layer.discard", "layer": segment.layer }))
    }

    /// Removes a sprite from output (`DiscardSprite`).
    pub fn segment_discard_sprite(&mut self, handle: &Handle) -> Result<BoolResult, Error> {
        self.call(json!({
            "op": "Layer.discardSprite", "layer": handle.layer, "sprite": handle.sprite
        }))
    }

    /// The FFT magnitudes of the loaded audio (`GetFft`).
    pub fn get_fft(&mut self, time: f64, magnitudes: usize) -> Result<FftResult, Error> {
        self.call(json!({ "op": "Audio.getFft", "time": time, "magnitudes": magnitudes }))
    }

    /// Opens a loop group (`StartLoopGroup`).
    pub fn start_loop_group(
        &mut self,
        handle: &Handle,
        start_time: f64,
        count: f64,
    ) -> Result<OkResult, Error> {
        self.call(json!({
            "op": "Sprite.startLoopGroup", "layer": handle.layer, "sprite": handle.sprite,
            "startTime": start_time, "count": count
        }))
    }

    /// Closes the active loop/trigger group (`EndGroup`).
    pub fn end_group(&mut self, handle: &Handle) -> Result<OkResult, Error> {
        self.call(
            json!({ "op": "Sprite.endGroup", "layer": handle.layer, "sprite": handle.sprite }),
        )
    }

    /// Opens a trigger group (`StartTriggerGroup`).
    pub fn start_trigger_group(
        &mut self,
        handle: &Handle,
        name: &str,
        start_time: f64,
        end_time: f64,
    ) -> Result<OkResult, Error> {
        self.call(json!({
            "op": "Sprite.startTriggerGroup", "layer": handle.layer, "sprite": handle.sprite,
            "name": name, "startTime": start_time, "endTime": end_time
        }))
    }

    fn command(
        &mut self,
        kind: &str,
        handle: &Handle,
        start_time: f64,
        end_time: f64,
        easing: Easing,
        values: Value,
    ) -> Result<OkResult, Error> {
        let mut request = json!({
            "op": "Sprite.command", "kind": kind, "layer": handle.layer, "sprite": handle.sprite,
            "startTime": start_time, "endTime": end_time, "easing": easing
        });
        if let (Value::Object(target), Value::Object(source)) = (&mut request, values) {
            target.extend(source);
        }
        self.call(request)
    }

    /// Runs a typed JSON request against this runtime.
    pub fn exec<T: DeserializeOwned>(&mut self, request: Value) -> Result<T, Error> {
        self.call(request)
    }

    fn call<T: DeserializeOwned>(&mut self, request: Value) -> Result<T, Error> {
        parse(&osb_native::runtime_exec(
            &mut self.inner,
            &request.to_string(),
        ))
    }
}

/// A storyboard segment bound to a runtime: the `StoryboardSegment` surface with
/// the handle implicit (the flat `*_on`/`segment_*` methods on [`Runtime`] are the
/// handle-first equivalents).
pub struct Segment<'a> {
    runtime: &'a mut Runtime,
    handle: LayerHandle,
}

impl<'a> Segment<'a> {
    /// The segment's handle.
    pub fn handle(&self) -> LayerHandle {
        self.handle.clone()
    }

    /// Creates an anonymous segment nested in this one (`CreateSegment`).
    pub fn create_segment(&mut self) -> Result<Segment<'_>, Error> {
        let handle = self.runtime.create_segment(&self.handle)?;
        Ok(Segment {
            runtime: &mut *self.runtime,
            handle,
        })
    }

    /// Creates a sprite on this segment.
    pub fn create_sprite(&mut self, path: &str, origin: OsbOrigin) -> Result<Handle, Error> {
        self.runtime.create_sprite_on(&self.handle, path, origin)
    }

    /// Creates a positioned sprite on this segment.
    pub fn create_sprite_at(
        &mut self,
        path: &str,
        origin: OsbOrigin,
        position: Vector2,
    ) -> Result<Handle, Error> {
        self.runtime
            .create_sprite_at_on(&self.handle, path, origin, position)
    }

    /// Creates an animation sprite on this segment.
    #[allow(clippy::too_many_arguments)]
    pub fn create_animation(
        &mut self,
        path: &str,
        frames: i32,
        frame_delay: f64,
        loop_type: OsbLoopType,
        origin: OsbOrigin,
        position: Vector2,
    ) -> Result<Handle, Error> {
        self.runtime.create_animation_on(
            &self.handle,
            path,
            frames,
            frame_delay,
            loop_type,
            origin,
            position,
        )
    }

    /// Adds a sound sample to this segment.
    pub fn create_sample(&mut self, path: &str, time: f64, volume: f64) -> Result<OkResult, Error> {
        self.runtime
            .create_sample_on(&self.handle, path, time, volume)
    }

    /// Sets the reverse-depth flag (`ReverseDepth`).
    pub fn set_reverse_depth(&mut self, value: bool) -> Result<OkResult, Error> {
        self.runtime.segment_reverse_depth(&self.handle, value)
    }

    /// Records a depth index (`AddIndex`).
    pub fn add_index(&mut self, index: f64) -> Result<OkResult, Error> {
        self.runtime.segment_add_index(&self.handle, index)
    }

    /// The depth indexes and bounds.
    pub fn indexes(&mut self) -> Result<SegmentIndexesResult, Error> {
        self.runtime.segment_indexes(&self.handle)
    }

    /// Moves this segment to another osu! layer (`ChangePosition`).
    pub fn change_position(&mut self, osb_layer: OsbLayer) -> Result<OkResult, Error> {
        self.runtime
            .segment_change_position(&self.handle, osb_layer)
    }

    /// Moves this segment on top of its siblings (`MoveToTop`).
    pub fn move_to_top(&mut self) -> Result<OkResult, Error> {
        self.runtime.segment_move_to_top(&self.handle)
    }

    /// Moves this segment below its siblings (`MoveToBottom`).
    pub fn move_to_bottom(&mut self) -> Result<OkResult, Error> {
        self.runtime.segment_move_to_bottom(&self.handle)
    }

    /// Removes zero-duration sprites (`Clean`).
    pub fn clean(&mut self) -> Result<SegmentCleanResult, Error> {
        self.runtime.segment_clean(&self.handle)
    }

    /// Removes this segment (`Discard`).
    pub fn discard(&mut self) -> Result<SegmentDiscardResult, Error> {
        self.runtime.segment_discard(&self.handle)
    }

    /// Removes a sprite from output (`DiscardSprite`).
    pub fn discard_sprite(&mut self, handle: &Handle) -> Result<BoolResult, Error> {
        self.runtime.segment_discard_sprite(handle)
    }

    /// Borrows the underlying runtime (for command authoring).
    pub fn runtime(&mut self) -> &mut Runtime {
        self.runtime
    }
}
