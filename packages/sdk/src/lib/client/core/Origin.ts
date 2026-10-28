import { OriginAPI } from '../../api/OriginAPI';
import type { OsbOrigin } from '../../models/OsbOrigin';
import type { Vector2 } from '../../models/Vector2';
import type { GetOriginVectorOptions, InScreenBoundsOptions } from '../options';
import { isGetOriginVectorOptions, isInScreenBoundsOptions } from '../options';

export class Origin {
    constructor(private readonly api: OriginAPI) {}

    static instanceOf(value: unknown): value is Origin {
        return value instanceof Origin;
    }

    /**
     * The anchor offset for an origin.
     *
     * @param origin The sprite anchor origin.
     * @param width The sprite width.
     * @param height The sprite height.
     *
     * @returns The anchor offset for the origin
     */
    async getOriginVector(origin: OsbOrigin, width: number, height: number): Promise<Vector2>;
    /**
     * The anchor offset for an origin.
     *
     * @param options The anchor offset for an origin.
     *
     * @returns The anchor offset for the origin
     */
    async getOriginVector(options: GetOriginVectorOptions): Promise<Vector2>;
    async getOriginVector(optionsOrOrigin?: GetOriginVectorOptions | OsbOrigin, width?: number, height?: number): Promise<Vector2> {
        const options: GetOriginVectorOptions = isGetOriginVectorOptions(optionsOrOrigin)
            ? optionsOrOrigin
            : ({ origin: optionsOrOrigin, width: width, height: height } as GetOriginVectorOptions);
        return this.api.getOriginVector(options);
    }

    /**
     * Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
     *
     * @param position The sprite centre position.
     * @param size The sprite size.
     * @param rotation The rotation in radians.
     * @param origin The anchor offset (from `GetOriginVector`).
     *
     * @returns Whether the sprite's bounds intersect the screen
     */
    async inScreenBounds(position: Vector2, size: Vector2, rotation: number, origin: Vector2): Promise<boolean>;
    /**
     * Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
     *
     * @param options Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
     *
     * @returns Whether the sprite's bounds intersect the screen
     */
    async inScreenBounds(options: InScreenBoundsOptions): Promise<boolean>;
    async inScreenBounds(optionsOrPosition?: InScreenBoundsOptions | Vector2, size?: Vector2, rotation?: number, origin?: Vector2): Promise<boolean> {
        const options: InScreenBoundsOptions = isInScreenBoundsOptions(optionsOrPosition)
            ? optionsOrPosition
            : ({ position: optionsOrPosition, size: size, rotation: rotation, origin: origin } as InScreenBoundsOptions);
        return (await this.api.inScreenBounds(options)).value;
    }
}
