using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Generator
{
    private readonly GeneratorAPI api;

    public Generator(GeneratorAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Generator;

    public Beatmap Beatmap() => api.Beatmap();

    public ControlPoint ControlPointAt(TimeRequest body) => api.ControlPointAt(body);

    public ImportOsbResult ImportOsb(TextRequest body) => api.ImportOsb(body);

    public LayerHandle LayerHandle(LayerHandleRequest body) => api.LayerHandle(body);

    public LoadBeatmapResult LoadBeatmap(TextRequest body) => api.LoadBeatmap(body);

    public RandomResult Random(RandomRequest body) => api.Random(body);

    public SpriteCountResult SpriteCount() => api.SpriteCount();

    public Storyboard Storyboard() => api.Storyboard();

    public ControlPoint TimingPointAt(TimeRequest body) => api.TimingPointAt(body);
}
