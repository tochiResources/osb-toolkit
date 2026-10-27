import { Storyboarding3dAPI } from '../../api/Storyboarding3dAPI';
import type { BakedStoryboard } from '../../models/BakedStoryboard';
import type { Bake3dOptions } from '../options';

export class Storyboarding3d {
    constructor(private readonly api: Storyboarding3dAPI) {}

    static instanceOf(value: unknown): value is Storyboarding3d {
        return value instanceof Storyboarding3d;
    }

    async bake3d(options: Bake3dOptions): Promise<BakedStoryboard> {
        return this.api.bake3d(options);
    }
}
