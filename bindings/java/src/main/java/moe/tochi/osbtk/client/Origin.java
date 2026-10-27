package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.OriginAPI;
import moe.tochi.osbtk.model.BoolResult;
import moe.tochi.osbtk.model.GetOriginVectorRequest;
import moe.tochi.osbtk.model.InScreenBoundsRequest;
import moe.tochi.osbtk.model.Vector2;
import java.util.List;

public final class Origin {
    private final OriginAPI api;

    public Origin(OriginAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Origin;
    }

    public Vector2 getOriginVector(GetOriginVectorOptions options) throws ApiException {
        return api.getOriginVector(new GetOriginVectorRequest()
            .height(options.height)
            .origin(options.origin)
            .width(options.width));
    }

    public BoolResult inScreenBounds(InScreenBoundsOptions options) throws ApiException {
        return api.inScreenBounds(new InScreenBoundsRequest()
            .origin(options.origin)
            .position(options.position)
            .rotation(options.rotation)
            .size(options.size));
    }
}
