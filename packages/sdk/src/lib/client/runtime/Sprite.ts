import { SpriteAPI } from '../../api/SpriteAPI';
import type { CommandRequest } from '../../models/CommandRequest';
import type { HandleRequest } from '../../models/HandleRequest';
import type { OkResult } from '../../models/OkResult';
import type { SpriteInfo } from '../../models/SpriteInfo';
import type { SpriteState } from '../../models/SpriteState';
import type { SpriteTimeRequest } from '../../models/SpriteTimeRequest';
import type { StartLoopGroupRequest } from '../../models/StartLoopGroupRequest';
import type { StartTriggerGroupRequest } from '../../models/StartTriggerGroupRequest';

export class Sprite {
    constructor(private readonly api: SpriteAPI) {}

    static instanceOf(value: unknown): value is Sprite {
        return value instanceof Sprite;
    }

    async command(body: CommandRequest): Promise<OkResult> {
        return this.api.command(body);
    }

    async endGroup(body: HandleRequest): Promise<OkResult> {
        return this.api.endGroup(body);
    }

    async info(body: HandleRequest): Promise<SpriteInfo> {
        return this.api.info(body);
    }

    async startLoopGroup(body: StartLoopGroupRequest): Promise<OkResult> {
        return this.api.startLoopGroup(body);
    }

    async startTriggerGroup(body: StartTriggerGroupRequest): Promise<OkResult> {
        return this.api.startTriggerGroup(body);
    }

    async stateAt(body: SpriteTimeRequest): Promise<SpriteState> {
        return this.api.stateAt(body);
    }
}
