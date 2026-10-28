package moe.tochi.osbtk.client.runtime;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.HelpersAPI;
import moe.tochi.osbtk.model.HelperRequest;
import moe.tochi.osbtk.client.HelperOptions;
import java.util.List;

/**
 * The clean `Helpers` client.
 */
public final class Helpers {
    private final HelpersAPI api;

    public Helpers(HelpersAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Helpers;
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     *
     * @return The shared motion-graphics helper's result
     */
    public Object call(String name) throws ApiException {
        return call(HelperOptions.builder().name(name).build());
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     * @param input The helper's input (a keyframe channel, a command list, or its parameters).
     *
     * @return The shared motion-graphics helper's result
     */
    public Object call(String name, String input) throws ApiException {
        return call(HelperOptions.builder().name(name).input(input).build());
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param options A motion-graphics helper call: a helper `name` plus its input.
     *
     * @return The shared motion-graphics helper's result
     */
    public Object call(HelperOptions options) throws ApiException {
        return api.call(new HelperRequest()
            .input(options.input)
            .name(options.name)).getValue();
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     *
     * @return The helper's result (a command list or scalar)
     */
    public Object commands(String name) throws ApiException {
        return commands(HelperOptions.builder().name(name).build());
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     * @param input The helper's input (a keyframe channel, a command list, or its parameters).
     *
     * @return The helper's result (a command list or scalar)
     */
    public Object commands(String name, String input) throws ApiException {
        return commands(HelperOptions.builder().name(name).input(input).build());
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param options A motion-graphics helper call: a helper `name` plus its input.
     *
     * @return The helper's result (a command list or scalar)
     */
    public Object commands(HelperOptions options) throws ApiException {
        return api.commands(new HelperRequest()
            .input(options.input)
            .name(options.name)).getValue();
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     *
     * @return The helper's result (a keyframe channel or vectors)
     */
    public Object keyframes(String name) throws ApiException {
        return keyframes(HelperOptions.builder().name(name).build());
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param name The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
     * @param input The helper's input (a keyframe channel, a command list, or its parameters).
     *
     * @return The helper's result (a keyframe channel or vectors)
     */
    public Object keyframes(String name, String input) throws ApiException {
        return keyframes(HelperOptions.builder().name(name).input(input).build());
    }

    /**
     * A motion-graphics helper call: a helper `name` plus its input.
     *
     * @param options A motion-graphics helper call: a helper `name` plus its input.
     *
     * @return The helper's result (a keyframe channel or vectors)
     */
    public Object keyframes(HelperOptions options) throws ApiException {
        return api.keyframes(new HelperRequest()
            .input(options.input)
            .name(options.name)).getValue();
    }
}
