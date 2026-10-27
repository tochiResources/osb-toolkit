import { ImagegenAPI } from '../../api/ImagegenAPI';
import type { ImageInfo } from '../../models/ImageInfo';
import type { RenderShapesRequest } from '../../models/RenderShapesRequest';

export class Imagegen {
    constructor(private readonly api: ImagegenAPI) {}

    static instanceOf(value: unknown): value is Imagegen {
        return value instanceof Imagegen;
    }

    async renderShapes(body: RenderShapesRequest): Promise<ImageInfo> {
        return this.api.renderShapes(body);
    }
}
