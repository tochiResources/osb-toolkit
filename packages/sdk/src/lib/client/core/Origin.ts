import { OriginAPI } from '../../api/OriginAPI';
import type { BoolResult } from '../../models/BoolResult';
import type { OsbOrigin } from '../../models/OsbOrigin';
import type { Vector2 } from '../../models/Vector2';
import type { GetOriginVectorOptions, InScreenBoundsOptions } from '../options';
import { isGetOriginVectorOptions, isInScreenBoundsOptions } from '../options';

export class Origin {
    constructor(private readonly api: OriginAPI) {}

    static instanceOf(value: unknown): value is Origin {
        return value instanceof Origin;
    }

    async getOriginVector(options: GetOriginVectorOptions): Promise<Vector2>;
    async getOriginVector(origin: OsbOrigin, width: number, height: number): Promise<Vector2>;
    async getOriginVector(optionsOrOrigin?: GetOriginVectorOptions | OsbOrigin, width?: number, height?: number): Promise<Vector2> {
        const options: GetOriginVectorOptions = isGetOriginVectorOptions(optionsOrOrigin)
            ? optionsOrOrigin
            : ({ origin: optionsOrOrigin, width: width, height: height } as GetOriginVectorOptions);
        return this.api.getOriginVector(options);
    }

    async inScreenBounds(options: InScreenBoundsOptions): Promise<BoolResult>;
    async inScreenBounds(position: Vector2, size: Vector2, rotation: number, origin: Vector2): Promise<BoolResult>;
    async inScreenBounds(optionsOrPosition?: InScreenBoundsOptions | Vector2, size?: Vector2, rotation?: number, origin?: Vector2): Promise<BoolResult> {
        const options: InScreenBoundsOptions = isInScreenBoundsOptions(optionsOrPosition)
            ? optionsOrPosition
            : ({ position: optionsOrPosition, size: size, rotation: rotation, origin: origin } as InScreenBoundsOptions);
        return this.api.inScreenBounds(options);
    }
}
