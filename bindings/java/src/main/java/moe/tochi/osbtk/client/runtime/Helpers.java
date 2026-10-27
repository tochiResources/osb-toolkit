package moe.tochi.osbtk.client.runtime;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.HelpersAPI;
import moe.tochi.osbtk.model.HelperRequest;
import moe.tochi.osbtk.client.HelperOptions;
import java.util.List;

public final class Helpers {
    private final HelpersAPI api;

    public Helpers(HelpersAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Helpers;
    }

    public Object call(HelperOptions options) throws ApiException {
        return api.call(new HelperRequest()
            .input(options.input)
            .name(options.name)).getValue();
    }

    public Object call(String name) throws ApiException {
        return call(HelperOptions.builder().name(name).build());
    }

    public Object call(String name, String input) throws ApiException {
        return call(HelperOptions.builder().name(name).input(input).build());
    }

    public Object commands(HelperOptions options) throws ApiException {
        return api.commands(new HelperRequest()
            .input(options.input)
            .name(options.name)).getValue();
    }

    public Object commands(String name) throws ApiException {
        return commands(HelperOptions.builder().name(name).build());
    }

    public Object commands(String name, String input) throws ApiException {
        return commands(HelperOptions.builder().name(name).input(input).build());
    }

    public Object keyframes(HelperOptions options) throws ApiException {
        return api.keyframes(new HelperRequest()
            .input(options.input)
            .name(options.name)).getValue();
    }

    public Object keyframes(String name) throws ApiException {
        return keyframes(HelperOptions.builder().name(name).build());
    }

    public Object keyframes(String name, String input) throws ApiException {
        return keyframes(HelperOptions.builder().name(name).input(input).build());
    }
}
