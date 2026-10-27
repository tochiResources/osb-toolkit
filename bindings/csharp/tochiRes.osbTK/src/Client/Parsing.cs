using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client;

public sealed class Parsing
{
    private readonly ParsingAPI api;

    public Parsing(ParsingAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Parsing;

    public ParseOsbResult ParseOsb(TextOptions options) => api.ParseOsb(new TextRequest(options.Text));

    public Beatmap ParseOsu(TextOptions options) => api.ParseOsu(new TextRequest(options.Text));
}
