#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Core;

public sealed class Parsing
{
    private readonly ParsingAPI api;

    public Parsing(ParsingAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Parsing;

    public ParseOsbResult ParseOsb(TextOptions options) => api.ParseOsb(new TextRequest(options.Text));

    public ParseOsbResult ParseOsb(string Text) =>
        ParseOsb(new TextOptions { Text = Text });

    public Beatmap ParseOsu(TextOptions options) => api.ParseOsu(new TextRequest(options.Text));

    public Beatmap ParseOsu(string Text) =>
        ParseOsu(new TextOptions { Text = Text });
}
