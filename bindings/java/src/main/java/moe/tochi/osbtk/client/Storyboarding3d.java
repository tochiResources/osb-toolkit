package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.Storyboarding3dAPI;
import moe.tochi.osbtk.model.Bake3dRequest;
import moe.tochi.osbtk.model.BakedStoryboard;
import java.util.List;

public final class Storyboarding3d {
    private final Storyboarding3dAPI api;

    public Storyboarding3d(Storyboarding3dAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Storyboarding3d;
    }

    public BakedStoryboard bake3d(Bake3dOptions options) throws ApiException {
        return api.bake3d(new Bake3dRequest()
            .input(options.input));
    }
}
