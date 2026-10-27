import { ImagegenAPI } from '../../api/ImagegenAPI';
import type { ImageInfo } from '../../models/ImageInfo';
import type { RgbaColor } from '../../models/RgbaColor';
import type { Shape } from '../../models/Shape';
import type { RenderShapesOptions } from '../options';
import { isRenderShapesOptions } from '../options';

export class Imagegen {
    constructor(private readonly api: ImagegenAPI) {}

    static instanceOf(value: unknown): value is Imagegen {
        return value instanceof Imagegen;
    }

    async renderShapes(width: number, height: number, shapes: Shape[], output: string): Promise<ImageInfo>;
    async renderShapes(width: number, height: number, shapes: Shape[], output: string, background: RgbaColor | null): Promise<ImageInfo>;
    async renderShapes(options: RenderShapesOptions): Promise<ImageInfo>;
    async renderShapes(optionsOrWidth?: RenderShapesOptions | number, height?: number, shapes?: Shape[], output?: string, background?: RgbaColor | null): Promise<ImageInfo> {
        const options: RenderShapesOptions = isRenderShapesOptions(optionsOrWidth)
            ? optionsOrWidth
            : ({ width: optionsOrWidth, height: height, shapes: shapes, output: output, background: background } as RenderShapesOptions);
        return this.api.renderShapes(options);
    }
}
