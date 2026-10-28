import { ImagegenAPI } from '../../api/ImagegenAPI';
import type { ImageInfo } from '../../models/ImageInfo';
import type { RgbaColor } from '../../models/RgbaColor';
import type { Shape } from '../../models/Shape';
import type { RenderShapesOptions } from '../options';
import { isRenderShapesOptions } from '../options';

/**
 * The clean `Imagegen` client.
 */
export class Imagegen {
    constructor(private readonly api: ImagegenAPI) {}

    static instanceOf(value: unknown): value is Imagegen {
        return value instanceof Imagegen;
    }

    /**
     * A shape-rendering request: a canvas, a background and a shape list.
     *
     * @param width The canvas width in pixels.
     * @param height The canvas height in pixels.
     * @param shapes The shapes to draw, in order.
     * @param output The output file path (its extension selects the format).
     *
     * @returns The rendered image's metadata
     */
    async renderShapes(width: number, height: number, shapes: Shape[], output: string): Promise<ImageInfo>;
    /**
     * A shape-rendering request: a canvas, a background and a shape list.
     *
     * @param width The canvas width in pixels.
     * @param height The canvas height in pixels.
     * @param shapes The shapes to draw, in order.
     * @param output The output file path (its extension selects the format).
     * @param background The background colour (default transparent).
     *
     * @returns The rendered image's metadata
     */
    async renderShapes(width: number, height: number, shapes: Shape[], output: string, background: RgbaColor | null): Promise<ImageInfo>;
    /**
     * A shape-rendering request: a canvas, a background and a shape list.
     *
     * @param options A shape-rendering request: a canvas, a background and a shape list.
     *
     * @returns The rendered image's metadata
     */
    async renderShapes(options: RenderShapesOptions): Promise<ImageInfo>;
    async renderShapes(optionsOrWidth?: RenderShapesOptions | number, height?: number, shapes?: Shape[], output?: string, background?: RgbaColor | null): Promise<ImageInfo> {
        const options: RenderShapesOptions = isRenderShapesOptions(optionsOrWidth)
            ? optionsOrWidth
            : ({ width: optionsOrWidth, height: height, shapes: shapes, output: output, background: background } as RenderShapesOptions);
        return this.api.renderShapes(options);
    }
}
