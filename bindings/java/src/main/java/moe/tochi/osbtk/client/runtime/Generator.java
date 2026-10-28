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

    /**
     * Calls the Generator.beatmap bridge operation.
     *
     * @return The loaded beatmap, or null
     */
    public Beatmap beatmap() throws ApiException {
        return api.beatmap();
    }

    /**
     * A time query.
     *
     * @param time The time in milliseconds.
     *
     * @return The active control point, or null
     */
    public ControlPoint controlPointAt(Double time) throws ApiException {
        return controlPointAt(TimeOptions.builder().time(time).build());
    }

    /**
     * A time query.
     *
     * @param options A time query.
     *
     * @return The active control point, or null
     */
    public ControlPoint controlPointAt(TimeOptions options) throws ApiException {
        return api.controlPointAt(new TimeRequest()
            .time(options.time));
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @return The sprite count after import
     */
    public int importOsb(String text) throws ApiException {
        return importOsb(TextOptions.builder().text(text).build());
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @return The sprite count after import
     */
    public int importOsb(TextOptions options) throws ApiException {
        return api.importOsb(new TextRequest()
            .text(options.text)).getSpriteCount();
    }

    /**
     * Resolve (or create) a layer by name.
     *
     * @param name The layer name.
     *
     * @return The layer handle
     */
    public int layerHandle(String name) throws ApiException {
        return layerHandle(LayerHandleOptions.builder().name(name).build());
    }

    /**
     * Resolve (or create) a layer by name.
     *
     * @param options Resolve (or create) a layer by name.
     *
     * @return The layer handle
     */
    public int layerHandle(LayerHandleOptions options) throws ApiException {
        return api.layerHandle(new LayerHandleRequest()
            .name(options.name)).getLayer();
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @return A summary of the loaded beatmap
     */
    public LoadBeatmapResult loadBeatmap(String text) throws ApiException {
        return loadBeatmap(TextOptions.builder().text(text).build());
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @return A summary of the loaded beatmap
     */
    public LoadBeatmapResult loadBeatmap(TextOptions options) throws ApiException {
        return api.loadBeatmap(new TextRequest()
            .text(options.text));
    }

    /**
     * A deterministic random request.
     *
     * @param max The exclusive upper bound.
     * @param min The inclusive lower bound.
     *
     * @return A deterministic random value
     */
    public double random(Double max, Double min) throws ApiException {
        return random(RandomOptions.builder().max(max).min(min).build());
    }

    /**
     * A deterministic random request.
     *
     * @param options A deterministic random request.
     *
     * @return A deterministic random value
     */
    public double random(RandomOptions options) throws ApiException {
        return api.random(new RandomRequest()
            .max(options.max)
            .min(options.min)).getValue();
    }

    /**
     * Calls the Generator.spriteCount bridge operation.
     *
     * @return The storyboard sprite count
     */
    public int spriteCount() throws ApiException {
        return api.spriteCount().getSpriteCount();
    }

    /**
     * Calls the Generator.storyboard bridge operation.
     *
     * @return The built storyboard model
     */
    public Storyboard storyboard() throws ApiException {
        return api.storyboard();
    }

    /**
     * A time query.
     *
     * @param time The time in milliseconds.
     *
     * @return The active red timing point
     */
    public ControlPoint timingPointAt(Double time) throws ApiException {
        return timingPointAt(TimeOptions.builder().time(time).build());
    }

    /**
     * A time query.
     *
     * @param options A time query.
     *
     * @return The active red timing point
     */
    public ControlPoint timingPointAt(TimeOptions options) throws ApiException {
        return api.timingPointAt(new TimeRequest()
            .time(options.time));
    }
}
