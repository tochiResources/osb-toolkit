import { OriginAPI } from '../../api/OriginAPI';
import type { BoolResult } from '../../models/BoolResult';
import type { Vector2 } from '../../models/Vector2';
import type { GetOriginVectorOptions, InScreenBoundsOptions } from '../options';

export class Origin {
    constructor(private readonly api: OriginAPI) {}

    static instanceOf(value: unknown): value is Origin {
        return value instanceof Origin;
    }

    async getOriginVector(options: GetOriginVectorOptions): Promise<Vector2> {
        return this.api.getOriginVector(options);
    }

    async inScreenBounds(options: InScreenBoundsOptions): Promise<BoolResult> {
        return this.api.inScreenBounds(options);
    }
}
