package moe.tochi.osbtk.client.runtime;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.Storyboarding3dAPI;
import moe.tochi.osbtk.model.Bake3dRequest;
import moe.tochi.osbtk.model.BakedStoryboard;
import moe.tochi.osbtk.model.Scene3dInput;
import moe.tochi.osbtk.client.Bake3dOptions;
import java.util.List;

public final class Storyboarding3d {
    private final Storyboarding3dAPI api;

    public Storyboarding3d(Storyboarding3dAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Storyboarding3d;
    }

    public BakedStoryboard bake3d(Scene3dInput input) throws ApiException {
        return bake3d(Bake3dOptions.builder().input(input).build());
    }

    public BakedStoryboard bake3d(Bake3dOptions options) throws ApiException {
        return api.bake3d(new Bake3dRequest()
            .input(options.input));
    }
}
