package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.ParsingAPI;
import moe.tochi.osbtk.model.Beatmap;
import moe.tochi.osbtk.model.ParseOsbResult;
import moe.tochi.osbtk.model.TextRequest;
import java.util.List;

public final class Parsing {
    private final ParsingAPI api;

    public Parsing(ParsingAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Parsing;
    }

    public ParseOsbResult parseOsb(TextOptions options) throws ApiException {
        return api.parseOsb(new TextRequest()
            .text(options.text));
    }

    public Beatmap parseOsu(TextOptions options) throws ApiException {
        return api.parseOsu(new TextRequest()
            .text(options.text));
    }
}
