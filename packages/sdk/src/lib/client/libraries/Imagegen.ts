import { ImagegenAPI } from '../../api/ImagegenAPI';
import type { ImageInfo } from '../../models/ImageInfo';
import type { RenderShapesOptions } from '../options';

export class Imagegen {
    constructor(private readonly api: ImagegenAPI) {}

    static instanceOf(value: unknown): value is Imagegen {
        return value instanceof Imagegen;
    }

    async renderShapes(options: RenderShapesOptions): Promise<ImageInfo> {
        return this.api.renderShapes(options);
    }
}
