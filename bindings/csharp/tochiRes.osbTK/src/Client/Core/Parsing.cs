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

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="Text">The text payload.</param>
    /// <returns>The storyboard summary</returns>
    public ParseOsbResult ParseOsb(string Text) =>
        ParseOsb(new TextOptions { Text = Text });

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="options">A request carrying a block of text (an effect script, `.osu` or `.osb`).</param>
    /// <returns>The storyboard summary</returns>
    public ParseOsbResult ParseOsb(TextOptions options) => api.ParseOsb(new TextRequest(options.Text));

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="Text">The text payload.</param>
    /// <returns>The parsed beatmap</returns>
    public Beatmap ParseOsu(string Text) =>
        ParseOsu(new TextOptions { Text = Text });

    /// <summary>
    /// A request carrying a block of text (an effect script, `.osu` or `.osb`).
    /// </summary>
    /// <param name="options">A request carrying a block of text (an effect script, `.osu` or `.osb`).</param>
    /// <returns>The parsed beatmap</returns>
    public Beatmap ParseOsu(TextOptions options) => api.ParseOsu(new TextRequest(options.Text));
}
