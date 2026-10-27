package moe.tochi.osbtk.client.runtime;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.GeneratorAPI;
import moe.tochi.osbtk.model.Beatmap;
import moe.tochi.osbtk.model.ControlPoint;
import moe.tochi.osbtk.model.LayerHandleRequest;
import moe.tochi.osbtk.model.LoadBeatmapResult;
import moe.tochi.osbtk.model.RandomRequest;
import moe.tochi.osbtk.model.Storyboard;
import moe.tochi.osbtk.model.TextRequest;
import moe.tochi.osbtk.model.TimeRequest;
import moe.tochi.osbtk.client.LayerHandleOptions;
import moe.tochi.osbtk.client.RandomOptions;
import moe.tochi.osbtk.client.TextOptions;
import moe.tochi.osbtk.client.TimeOptions;
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

    public ControlPoint controlPointAt(Double time) throws ApiException {
        return controlPointAt(TimeOptions.builder().time(time).build());
    }

    public ControlPoint controlPointAt(TimeOptions options) throws ApiException {
        return api.controlPointAt(new TimeRequest()
            .time(options.time));
    }

    public int importOsb(String text) throws ApiException {
        return importOsb(TextOptions.builder().text(text).build());
    }

    public int importOsb(TextOptions options) throws ApiException {
        return api.importOsb(new TextRequest()
            .text(options.text)).getSpriteCount();
    }

    public int layerHandle(String name) throws ApiException {
        return layerHandle(LayerHandleOptions.builder().name(name).build());
    }

    public int layerHandle(LayerHandleOptions options) throws ApiException {
        return api.layerHandle(new LayerHandleRequest()
            .name(options.name)).getLayer();
    }

    public LoadBeatmapResult loadBeatmap(String text) throws ApiException {
        return loadBeatmap(TextOptions.builder().text(text).build());
    }

    public LoadBeatmapResult loadBeatmap(TextOptions options) throws ApiException {
        return api.loadBeatmap(new TextRequest()
            .text(options.text));
    }

    public double random(Double max, Double min) throws ApiException {
        return random(RandomOptions.builder().max(max).min(min).build());
    }

    public double random(RandomOptions options) throws ApiException {
        return api.random(new RandomRequest()
            .max(options.max)
            .min(options.min)).getValue();
    }

    public int spriteCount() throws ApiException {
        return api.spriteCount().getSpriteCount();
    }

    public Storyboard storyboard() throws ApiException {
        return api.storyboard();
    }

    public ControlPoint timingPointAt(Double time) throws ApiException {
        return timingPointAt(TimeOptions.builder().time(time).build());
    }

    public ControlPoint timingPointAt(TimeOptions options) throws ApiException {
        return api.timingPointAt(new TimeRequest()
            .time(options.time));
    }
}
