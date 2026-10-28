package moe.tochi.osbtk.client.core;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.ParsingAPI;
import moe.tochi.osbtk.model.Beatmap;
import moe.tochi.osbtk.model.ParseOsbResult;
import moe.tochi.osbtk.model.TextRequest;
import moe.tochi.osbtk.client.TextOptions;
import java.util.List;

public final class Parsing {
    private final ParsingAPI api;

    public Parsing(ParsingAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Parsing;
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @return The storyboard summary
     */
    public ParseOsbResult parseOsb(String text) throws ApiException {
        return parseOsb(TextOptions.builder().text(text).build());
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @return The storyboard summary
     */
    public ParseOsbResult parseOsb(TextOptions options) throws ApiException {
        return api.parseOsb(new TextRequest()
            .text(options.text));
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @return The parsed beatmap
     */
    public Beatmap parseOsu(String text) throws ApiException {
        return parseOsu(TextOptions.builder().text(text).build());
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @return The parsed beatmap
     */
    public Beatmap parseOsu(TextOptions options) throws ApiException {
        return api.parseOsu(new TextRequest()
            .text(options.text));
    }
}
