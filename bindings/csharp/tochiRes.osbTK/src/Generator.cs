using tochiRes.osbTK.Model;

namespace tochiRes.osbTK;

/// <summary>
/// A storyboard generator (the <c>StoryboardObjectGenerator</c> equivalent).
/// Subclass it, override <see cref="Generate"/> and drive the storyboard through
/// the effect-script API exposed by <see cref="osb.Storyboarding.Runtime"/>.
/// </summary>
public abstract class Generator
{
    /// <summary>The live storyboard runtime.</summary>
    public osb.Storyboarding.Runtime Runtime { get; }

    /// <summary>Creates a generator for a storyboard of the given resolution.</summary>
    protected Generator(double width = 640, double height = 480) =>
        Runtime = new osb.Storyboarding.Runtime(width, height);

    /// <summary>Returns a layer by name.</summary>
    public Layer GetLayer(string name) => Runtime.Layer(name);

    /// <summary>Returns a standard layer.</summary>
    public Layer GetLayer(OsbLayer layer) => Runtime.GetLayer(layer);

    /// <summary>The loaded beatmap.</summary>
    public Beatmap? Beatmap => Runtime.Beatmap;

    /// <summary>The FFT magnitudes of the loaded audio.</summary>
    public FftResult GetFft(double time, int magnitudes) => Runtime.GetFft(time, magnitudes);

    /// <summary>The active timing point at a time.</summary>
    public ControlPoint TimingPointAt(double time) => Runtime.TimingPointAt(time);

    /// <summary>The number of sprites on the storyboard.</summary>
    public int SpriteCount => Runtime.SpriteCount;

    /// <summary>The built storyboard model.</summary>
    public Storyboard Storyboard => Runtime.Storyboard;

    /// <summary>Builds the storyboard (the effect entry point).</summary>
    public abstract void Generate();
}
