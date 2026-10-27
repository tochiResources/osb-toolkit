using System.Text.Json;

using Xunit;

namespace tochiRes.osbTK.Native.Tests;

/// <summary>
/// Golden-vector parity for the C# binding: the facade must drive the same
/// `osb-runtime` engine as Rust, TypeScript, C++ and Python.
/// </summary>
public class BindingTests
{
    [Fact]
    public void ExposesTheNativeVersion()
    {
        Assert.False(string.IsNullOrEmpty(osb.Version()));
    }

    [Fact]
    public void BakesTheSharedGoldenCorpus()
    {
        using JsonDocument corpus = JsonDocument.Parse(File.ReadAllText(GoldenCorpusPath()));
        int cases = 0;
        foreach (JsonElement testCase in corpus.RootElement.GetProperty("cases").EnumerateArray())
        {
            string input = testCase.GetProperty("input").GetRawText();
            using JsonDocument result = JsonDocument.Parse(osb.Bake3d(input));
            JsonElement sprites = result.RootElement.GetProperty("sprites");
            JsonElement expect = testCase.GetProperty("expect");

            if (expect.TryGetProperty("spriteCount", out JsonElement count))
            {
                Assert.Equal(count.GetInt32(), sprites.GetArrayLength());
            }

            if (expect.TryGetProperty("firstSpritePath", out JsonElement first))
            {
                Assert.Equal(first.GetString(), sprites[0].GetProperty("spritePath").GetString());
            }

            cases++;
        }

        Assert.True(cases > 0, "the golden corpus must have cases");
    }

    [Fact]
    public void RunsTheJsonBridge()
    {
        string request = JsonSerializer.Serialize(
            new
            {
                op = "Parsing.parseOsu",
                text = "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
                    + "[TimingPoints]\n0,500,4,2,1,100,1,0\n"
                    + "[HitObjects]\n256,192,1000,1,0\n",
            }
        );
        using JsonDocument result = JsonDocument.Parse(osb.Exec(request));
        Assert.Equal("song.mp3", result.RootElement.GetProperty("audioFilename").GetString());
        Assert.Equal("Circle", result.RootElement.GetProperty("hitObjects")[0].GetProperty("kind").GetString());
    }

    [Fact]
    public void ReportsAnErrorForInvalidInput()
    {
        using JsonDocument result = JsonDocument.Parse(osb.Exec("not json"));
        Assert.True(result.RootElement.TryGetProperty("error", out _));
    }

    [Fact]
    public void DrivesTheStoryboardRuntime()
    {
        using Runtime runtime = new();
        using JsonDocument handle = JsonDocument.Parse(
            runtime.Exec(JsonSerializer.Serialize(new { op = "Layer.createSprite", layer = "Foreground", path = "sb/dot.png" }))
        );
        long layer = handle.RootElement.GetProperty("layer").GetInt64();
        long sprite = handle.RootElement.GetProperty("sprite").GetInt64();

        runtime.Exec(
            JsonSerializer.Serialize(
                new
                {
                    op = "Sprite.command",
                    kind = "move",
                    layer,
                    sprite,
                    startTime = 0,
                    endTime = 1000,
                    start = new { x = 0, y = 0 },
                    end = new { x = 100, y = 0 },
                }
            )
        );

        using JsonDocument state = JsonDocument.Parse(
            runtime.Exec(JsonSerializer.Serialize(new { op = "Sprite.stateAt", layer, sprite, time = 500 }))
        );
        Assert.Equal(50.0, state.RootElement.GetProperty("position").GetProperty("x").GetDouble());
    }

    private static string GoldenCorpusPath()
    {
        string? directory = AppContext.BaseDirectory;
        for (int depth = 0; depth < 8 && directory is not null; depth++)
        {
            string candidate = Path.Combine(
                directory,
                "crates",
                "osb-runtime",
                "tests",
                "golden",
                "storyboarding3d",
                "basic.json"
            );
            if (File.Exists(candidate))
            {
                return candidate;
            }

            directory = Path.GetDirectoryName(directory);
        }

        throw new FileNotFoundException("golden corpus not found");
    }
}
