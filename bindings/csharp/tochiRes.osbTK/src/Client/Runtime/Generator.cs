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

    public ControlPoint ControlPointAt(TimeOptions options) => api.ControlPointAt(new TimeRequest(options.Time));

    public ControlPoint ControlPointAt(double Time) =>
        ControlPointAt(new TimeOptions { Time = Time });

    public ImportOsbResult ImportOsb(TextOptions options) => api.ImportOsb(new TextRequest(options.Text));

    public ImportOsbResult ImportOsb(string Text) =>
        ImportOsb(new TextOptions { Text = Text });

    public LayerHandle LayerHandle(LayerHandleOptions options) => api.LayerHandle(new LayerHandleRequest(options.Name));

    public LayerHandle LayerHandle(string Name) =>
        LayerHandle(new LayerHandleOptions { Name = Name });

    public LoadBeatmapResult LoadBeatmap(TextOptions options) => api.LoadBeatmap(new TextRequest(options.Text));

    public LoadBeatmapResult LoadBeatmap(string Text) =>
        LoadBeatmap(new TextOptions { Text = Text });

    public RandomResult Random(RandomOptions options) => api.Random(new RandomRequest(options.Max, options.Min));

    public RandomResult Random(double Max, double Min) =>
        Random(new RandomOptions { Max = Max, Min = Min });

    public SpriteCountResult SpriteCount() => api.SpriteCount();

    public Storyboard Storyboard() => api.Storyboard();

    public ControlPoint TimingPointAt(TimeOptions options) => api.TimingPointAt(new TimeRequest(options.Time));

    public ControlPoint TimingPointAt(double Time) =>
        TimingPointAt(new TimeOptions { Time = Time });
}
