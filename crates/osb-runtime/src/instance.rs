use crate::assets::TextureStore;
use crate::audio::AudioBuffer;
use crate::context::Context;
use crate::mapset::Beatmap;
use crate::render::{render, Concurrency, RenderOptions};
use crate::storyboarding::{Effect, Runtime, Storyboard};

/// Owns an effect's context and built storyboard, and renders frames.
///
/// The host (`PF_Cmd_SEQUENCE_SETUP`/`RENDER` for After Effects and Premiere, or
/// the OFX equivalent) drives this; it holds no host-SDK types, so it can be
/// tested headlessly and shared by every adapter.
///
/// The context is the input and the storyboard the output, kept as two fields so
/// neither is owned by a "runtime" object: `build()` runs the effect once with
/// the context and stores the storyboard it returns.
pub struct EffectInstance {
    effect: Box<dyn Effect + Send + Sync>,
    context: Context,
    storyboard: Storyboard,
    built: bool,
    concurrency: Concurrency,
}

impl EffectInstance {
    pub fn new(effect: Box<dyn Effect + Send + Sync>, width: f32, height: f32) -> Self {
        EffectInstance {
            effect,
            context: Context::new(width, height),
            storyboard: Storyboard::new(),
            built: false,
            concurrency: Concurrency::Auto,
        }
    }

    /// Sets the compositor concurrency (the effect's "Multi-threading" option).
    pub fn set_concurrency(&mut self, concurrency: Concurrency) {
        self.concurrency = concurrency;
    }

    pub fn concurrency(&self) -> Concurrency {
        self.concurrency
    }

    /// The context the effect was (or will be) built with.
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Mutable context access, e.g. to set the beatmap before building.
    pub fn context_mut(&mut self) -> &mut Context {
        self.built = false;
        &mut self.context
    }

    /// The built storyboard (empty until [`build`](Self::build) runs).
    pub fn storyboard(&self) -> &Storyboard {
        &self.storyboard
    }

    /// A mutable authoring view over the storyboard and context; invalidates the
    /// build cache.
    pub fn runtime_mut(&mut self) -> Runtime<'_> {
        self.built = false;
        Runtime::new(&mut self.storyboard, &mut self.context)
    }

    /// Mutable access to the effect, for applying host parameter values before build.
    pub fn effect_mut(&mut self) -> &mut dyn Effect {
        self.built = false;
        self.effect.as_mut()
    }

    pub fn set_beatmap(&mut self, beatmap: Beatmap) {
        self.context.set_beatmap(beatmap);
        self.built = false;
    }

    pub fn set_audio(&mut self, audio: AudioBuffer) {
        self.context.set_audio(audio);
    }

    pub fn parameters(&self) -> Vec<crate::params::ParameterDescriptor> {
        self.effect.parameters()
    }

    /// Builds the storyboard exactly once (the cached `PF_Cmd_SEQUENCE_SETUP` step).
    pub fn ensure_built(&mut self) {
        if self.built {
            return;
        }
        self.storyboard = self.effect.generate(&mut self.context);
        self.built = true;
    }

    /// Builds the storyboard exactly once (the cached `PF_Cmd_SEQUENCE_SETUP` step).
    pub fn build(&mut self) {
        self.ensure_built();
    }

    pub fn is_built(&self) -> bool {
        self.built
    }

    /// Renders a frame from `&self`; call [`build`](Self::build) first.
    ///
    /// Takes `&self` (not `&mut self`) and requires no per-frame mutable state, so
    /// it is safe to call concurrently from multiple threads. That is what lets a
    /// host that supports **multi-frame rendering** (several timeline frames at
    /// once) share one built instance across frames - distinct from within-frame
    /// threaded/tiled rendering, which is controlled by [`Concurrency`].
    pub fn render_frame(
        &self,
        textures: &TextureStore,
        width: u32,
        height: u32,
        time: f64,
    ) -> Vec<u8> {
        debug_assert!(self.built, "render_frame requires build() first");
        render(
            &self.storyboard,
            self.context.resolution,
            textures,
            RenderOptions {
                width,
                height,
                time,
                concurrency: self.concurrency,
            },
        )
    }

    /// Builds (if needed) and renders one frame.
    pub fn render(
        &mut self,
        textures: &TextureStore,
        width: u32,
        height: u32,
        time: f64,
    ) -> Vec<u8> {
        self.ensure_built();
        self.render_frame(textures, width, height, time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::Texture;
    use crate::params::{Control, ParameterDescriptor, ParameterValue};
    use crate::storyboarding::{OsbLayer, OsbOrigin};

    struct SolidEffect;

    impl Effect for SolidEffect {
        fn generate(&mut self, _context: &mut Context) -> Storyboard {
            let mut storyboard = Storyboard::new();
            storyboard
                .layer(OsbLayer::Background)
                .create_sprite("sb/white.png", OsbOrigin::TopLeft)
                .fade(0.0, 1000.0, 1.0, 1.0, crate::animations::Easing::None);
            storyboard
        }

        fn parameters(&self) -> Vec<ParameterDescriptor> {
            vec![ParameterDescriptor::new(
                "BarCount",
                Control::Slider {
                    min: 0.0,
                    max: 64.0,
                },
                ParameterValue::Number(0.0),
            )]
        }
    }

    #[test]
    fn builds_once_and_renders() {
        let mut instance = EffectInstance::new(Box::new(SolidEffect), 2.0, 2.0);
        let mut textures = TextureStore::new();
        textures.insert("sb/white.png", Texture::new(1, 1, vec![255, 0, 0, 255]));
        let frame = instance.render(&textures, 2, 2, 0.0);
        assert_eq!(&frame[0..4], &[255, 0, 0, 255]);
        assert_eq!(instance.parameters().len(), 1);
    }

    #[test]
    fn concurrent_frame_render_is_safe() {
        use rayon::prelude::*;
        let mut instance = EffectInstance::new(Box::new(SolidEffect), 2.0, 2.0);
        let mut textures = TextureStore::new();
        textures.insert("sb/white.png", Texture::new(1, 1, vec![255, 0, 0, 255]));
        instance.build();
        // Multiple frames rendered concurrently share the one built instance.
        let frames: Vec<Vec<u8>> = (0..16)
            .into_par_iter()
            .map(|i| instance.render_frame(&textures, 2, 2, i as f64))
            .collect();
        assert!(frames.iter().all(|frame| frame[0..4] == [255, 0, 0, 255]));
    }
}
