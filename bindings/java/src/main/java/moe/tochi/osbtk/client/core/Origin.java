package moe.tochi.osbtk.client.core;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.OriginAPI;
import moe.tochi.osbtk.model.GetOriginVectorRequest;
import moe.tochi.osbtk.model.InScreenBoundsRequest;
import moe.tochi.osbtk.model.OsbOrigin;
import moe.tochi.osbtk.model.Vector2;
import moe.tochi.osbtk.client.GetOriginVectorOptions;
import moe.tochi.osbtk.client.InScreenBoundsOptions;
import java.util.List;

/**
 * The clean `Origin` client.
 */
public final class Origin {
    private final OriginAPI api;

    public Origin(OriginAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Origin;
    }

    /**
     * The anchor offset for an origin.
     *
     * @param origin The sprite anchor origin.
     * @param width The sprite width.
     * @param height The sprite height.
     *
     * @return The anchor offset for the origin
     */
    public Vector2 getOriginVector(OsbOrigin origin, Double width, Double height) throws ApiException {
        return getOriginVector(GetOriginVectorOptions.builder().origin(origin).width(width).height(height).build());
    }

    /**
     * The anchor offset for an origin.
     *
     * @param options The anchor offset for an origin.
     *
     * @return The anchor offset for the origin
     */
    public Vector2 getOriginVector(GetOriginVectorOptions options) throws ApiException {
        return api.getOriginVector(new GetOriginVectorRequest()
            .height(options.height)
            .origin(options.origin)
            .width(options.width));
    }

    /**
     * Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
     *
     * @param position The sprite centre position.
     * @param size The sprite size.
     * @param rotation The rotation in radians.
     * @param origin The anchor offset (from `GetOriginVector`).
     *
     * @return Whether the sprite's bounds intersect the screen
     */
    public boolean inScreenBounds(Vector2 position, Vector2 size, Double rotation, Vector2 origin) throws ApiException {
        return inScreenBounds(InScreenBoundsOptions.builder().position(position).size(size).rotation(rotation).origin(origin).build());
    }

    /**
     * Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
     *
     * @param options Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
     *
     * @return Whether the sprite's bounds intersect the screen
     */
    public boolean inScreenBounds(InScreenBoundsOptions options) throws ApiException {
        return api.inScreenBounds(new InScreenBoundsRequest()
            .origin(options.origin)
            .position(options.position)
            .rotation(options.rotation)
            .size(options.size)).getValue();
    }
}
