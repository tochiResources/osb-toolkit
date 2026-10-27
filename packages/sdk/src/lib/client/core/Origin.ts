import { OriginAPI } from '../../api/OriginAPI';
import type { BoolResult } from '../../models/BoolResult';
import type { GetOriginVectorRequest } from '../../models/GetOriginVectorRequest';
import type { InScreenBoundsRequest } from '../../models/InScreenBoundsRequest';
import type { Vector2 } from '../../models/Vector2';

export class Origin {
    constructor(private readonly api: OriginAPI) {}

    static instanceOf(value: unknown): value is Origin {
        return value instanceof Origin;
    }

    async getOriginVector(body: GetOriginVectorRequest): Promise<Vector2> {
        return this.api.getOriginVector(body);
    }

    async inScreenBounds(body: InScreenBoundsRequest): Promise<BoolResult> {
        return this.api.inScreenBounds(body);
    }
}
