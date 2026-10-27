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

    public ParseOsbResult ParseOsb(TextRequest body) => api.ParseOsb(body);

    public Beatmap ParseOsu(TextRequest body) => api.ParseOsu(body);
}
