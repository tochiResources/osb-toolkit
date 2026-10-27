package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.GeneratorAPI;
import moe.tochi.osbtk.model.Beatmap;
import moe.tochi.osbtk.model.ControlPoint;
import moe.tochi.osbtk.model.ImportOsbResult;
import moe.tochi.osbtk.model.LayerHandle;
import moe.tochi.osbtk.model.LayerHandleRequest;
import moe.tochi.osbtk.model.LoadBeatmapResult;
import moe.tochi.osbtk.model.RandomRequest;
import moe.tochi.osbtk.model.RandomResult;
import moe.tochi.osbtk.model.SpriteCountResult;
import moe.tochi.osbtk.model.Storyboard;
import moe.tochi.osbtk.model.TextRequest;
import moe.tochi.osbtk.model.TimeRequest;
import java.util.List;

public final class Generator {
    private final GeneratorAPI api;

    public Generator(GeneratorAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Generator;
    }

    public Beatmap beatmap() throws ApiException {
        return api.beatmap();
    }

    public ControlPoint controlPointAt(TimeOptions options) throws ApiException {
        return api.controlPointAt(new TimeRequest()
            .time(options.time));
    }

    public ImportOsbResult importOsb(TextOptions options) throws ApiException {
        return api.importOsb(new TextRequest()
            .text(options.text));
    }

    public LayerHandle layerHandle(LayerHandleOptions options) throws ApiException {
        return api.layerHandle(new LayerHandleRequest()
            .name(options.name));
    }

    public LoadBeatmapResult loadBeatmap(TextOptions options) throws ApiException {
        return api.loadBeatmap(new TextRequest()
            .text(options.text));
    }

    public RandomResult random(RandomOptions options) throws ApiException {
        return api.random(new RandomRequest()
            .max(options.max)
            .min(options.min));
    }

    public SpriteCountResult spriteCount() throws ApiException {
        return api.spriteCount();
    }

    public Storyboard storyboard() throws ApiException {
        return api.storyboard();
    }

    public ControlPoint timingPointAt(TimeOptions options) throws ApiException {
        return api.timingPointAt(new TimeRequest()
            .time(options.time));
    }
}
