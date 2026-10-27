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

    public ControlPoint controlPointAt(TimeRequest body) throws ApiException {
        return api.controlPointAt(body);
    }

    public ImportOsbResult importOsb(TextRequest body) throws ApiException {
        return api.importOsb(body);
    }

    public LayerHandle layerHandle(LayerHandleRequest body) throws ApiException {
        return api.layerHandle(body);
    }

    public LoadBeatmapResult loadBeatmap(TextRequest body) throws ApiException {
        return api.loadBeatmap(body);
    }

    public RandomResult random(RandomRequest body) throws ApiException {
        return api.random(body);
    }

    public SpriteCountResult spriteCount() throws ApiException {
        return api.spriteCount();
    }

    public Storyboard storyboard() throws ApiException {
        return api.storyboard();
    }

    public ControlPoint timingPointAt(TimeRequest body) throws ApiException {
        return api.timingPointAt(body);
    }
}
