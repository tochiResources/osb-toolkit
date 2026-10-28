#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Runtime;

/// <summary>
/// The clean `Generator` client.
/// </summary>
public sealed class Generator
{
    private readonly GeneratorAPI api;

    public Generator(GeneratorAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Generator;

    /// <summary>
    /// Calls the Generator.beatmap bridge operation.
    /// </summary>
    /// <returns>The loaded beatmap, or null</returns>
    public Beatmap Beatmap() => api.Beatmap();

    /// <summary>
    /// A time query.
    /// </summary>
    /// <param name="Time">The time in milliseconds.</param>
    /// <returns>The active control point, or null</returns>
    public ControlPoint ControlPointAt(double Time) =>
        ControlPointAt(new TimeOptions { Time = Time });

    /// <summary>
    /// A time query.
    /// </summary>
    /// <param name="options">A time query.</param>
    /// <returns>The active control point, or null</returns>
    public ControlPoint ControlPointAt(TimeOptions options) => api.ControlPointAt(new TimeRequest(options.Time));

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="Text">The text payload.</param>
    /// <returns>The sprite count after import</returns>
    public int ImportOsb(string Text) =>
        ImportOsb(new TextOptions { Text = Text });

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="options">A request carrying a block of text (an effect script, `.osu` or `.osb`).</param>
    /// <returns>The sprite count after import</returns>
    public int ImportOsb(TextOptions options) => api.ImportOsb(new TextRequest(options.Text)).SpriteCount;

    /// <summary>
    /// Resolve (or create) a layer by name.
    /// </summary>
    /// <param name="Name">The layer name.</param>
    /// <returns>The layer handle</returns>
    public int LayerHandle(string Name) =>
        LayerHandle(new LayerHandleOptions { Name = Name });

    /// <summary>
    /// Resolve (or create) a layer by name.
    /// </summary>
    /// <param name="options">Resolve (or create) a layer by name.</param>
    /// <returns>The layer handle</returns>
    public int LayerHandle(LayerHandleOptions options) => api.LayerHandle(new LayerHandleRequest(options.Name)).Layer;

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="Text">The text payload.</param>
    /// <returns>A summary of the loaded beatmap</returns>
    public LoadBeatmapResult LoadBeatmap(string Text) =>
        LoadBeatmap(new TextOptions { Text = Text });

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="options">A request carrying a block of text (an effect script, `.osu` or `.osb`).</param>
    /// <returns>A summary of the loaded beatmap</returns>
    public LoadBeatmapResult LoadBeatmap(TextOptions options) => api.LoadBeatmap(new TextRequest(options.Text));

    /// <summary>
    /// A deterministic random request.
    /// </summary>
    /// <param name="Max">The exclusive upper bound.</param>
    /// <param name="Min">The inclusive lower bound.</param>
    /// <returns>A deterministic random value</returns>
    public double Random(double Max, double Min) =>
        Random(new RandomOptions { Max = Max, Min = Min });

    /// <summary>
    /// A deterministic random request.
    /// </summary>
    /// <param name="options">A deterministic random request.</param>
    /// <returns>A deterministic random value</returns>
    public double Random(RandomOptions options) => api.Random(new RandomRequest(options.Max, options.Min)).Value;

    /// <summary>
    /// Calls the Generator.spriteCount bridge operation.
    /// </summary>
    /// <returns>The storyboard sprite count</returns>
    public int SpriteCount() => api.SpriteCount().SpriteCount;

    /// <summary>
    /// Calls the Generator.storyboard bridge operation.
    /// </summary>
    /// <returns>The built storyboard model</returns>
    public Storyboard Storyboard() => api.Storyboard();

    /// <summary>
    /// A time query.
    /// </summary>
    /// <param name="Time">The time in milliseconds.</param>
    /// <returns>The active red timing point</returns>
    public ControlPoint TimingPointAt(double Time) =>
        TimingPointAt(new TimeOptions { Time = Time });

    /// <summary>
    /// A time query.
    /// </summary>
    /// <param name="options">A time query.</param>
    /// <returns>The active red timing point</returns>
    public ControlPoint TimingPointAt(TimeOptions options) => api.TimingPointAt(new TimeRequest(options.Time));
}
