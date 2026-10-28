import { Storyboarding3dAPI } from '../../api/Storyboarding3dAPI';
import type { BakedStoryboard } from '../../models/BakedStoryboard';
import type { Scene3dInput } from '../../models/Scene3dInput';
import type { Bake3dOptions } from '../options';
import { isBake3dOptions } from '../options';

export class Storyboarding3d {
    constructor(private readonly api: Storyboarding3dAPI) {}

    static instanceOf(value: unknown): value is Storyboarding3d {
        return value instanceof Storyboarding3d;
    }

    /**
     * A `bake3d` request: a plain 3D scene.
     *
     * @param input The 3D scene.
     *
     * @returns Baked command spans
     */
    async bake3d(input: Scene3dInput): Promise<BakedStoryboard>;
    /**
     * A `bake3d` request: a plain 3D scene.
     *
     * @param options A `bake3d` request: a plain 3D scene.
     *
     * @returns Baked command spans
     */
    async bake3d(options: Bake3dOptions): Promise<BakedStoryboard>;
    async bake3d(optionsOrInput?: Bake3dOptions | Scene3dInput): Promise<BakedStoryboard> {
        const options: Bake3dOptions = isBake3dOptions(optionsOrInput)
            ? optionsOrInput
            : ({ input: optionsOrInput } as Bake3dOptions);
        return this.api.bake3d(options);
    }
}
