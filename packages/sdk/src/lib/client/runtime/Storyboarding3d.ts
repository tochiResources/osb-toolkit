import { Storyboarding3dAPI } from '../../api/Storyboarding3dAPI';
import type { Bake3dRequest } from '../../models/Bake3dRequest';
import type { BakedStoryboard } from '../../models/BakedStoryboard';

export class Storyboarding3d {
    constructor(private readonly api: Storyboarding3dAPI) {}

    static instanceOf(value: unknown): value is Storyboarding3d {
        return value instanceof Storyboarding3d;
    }

    async bake3d(body: Bake3dRequest): Promise<BakedStoryboard> {
        return this.api.bake3d(body);
    }
}
