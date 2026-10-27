#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Runtime;

public sealed class Generator
{
    private readonly GeneratorAPI api;

    public Generator(GeneratorAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Generator;

    public Beatmap Beatmap() => api.Beatmap();

    public ControlPoint ControlPointAt(double Time) =>
        ControlPointAt(new TimeOptions { Time = Time });

    public ControlPoint ControlPointAt(TimeOptions options) => api.ControlPointAt(new TimeRequest(options.Time));

    public int ImportOsb(string Text) =>
        ImportOsb(new TextOptions { Text = Text });

    public int ImportOsb(TextOptions options) => api.ImportOsb(new TextRequest(options.Text)).SpriteCount;

    public int LayerHandle(string Name) =>
        LayerHandle(new LayerHandleOptions { Name = Name });

    public int LayerHandle(LayerHandleOptions options) => api.LayerHandle(new LayerHandleRequest(options.Name)).Layer;

    public LoadBeatmapResult LoadBeatmap(string Text) =>
        LoadBeatmap(new TextOptions { Text = Text });

    public LoadBeatmapResult LoadBeatmap(TextOptions options) => api.LoadBeatmap(new TextRequest(options.Text));

    public double Random(double Max, double Min) =>
        Random(new RandomOptions { Max = Max, Min = Min });

    public double Random(RandomOptions options) => api.Random(new RandomRequest(options.Max, options.Min)).Value;

    public int SpriteCount() => api.SpriteCount().SpriteCount;

    public Storyboard Storyboard() => api.Storyboard();

    public ControlPoint TimingPointAt(double Time) =>
        TimingPointAt(new TimeOptions { Time = Time });

    public ControlPoint TimingPointAt(TimeOptions options) => api.TimingPointAt(new TimeRequest(options.Time));
}
