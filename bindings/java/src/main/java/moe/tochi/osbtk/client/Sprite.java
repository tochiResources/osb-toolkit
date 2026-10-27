package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.SpriteAPI;
import moe.tochi.osbtk.model.CommandRequest;
import moe.tochi.osbtk.model.HandleRequest;
import moe.tochi.osbtk.model.OkResult;
import moe.tochi.osbtk.model.SpriteInfo;
import moe.tochi.osbtk.model.SpriteState;
import moe.tochi.osbtk.model.SpriteTimeRequest;
import moe.tochi.osbtk.model.StartLoopGroupRequest;
import moe.tochi.osbtk.model.StartTriggerGroupRequest;
import java.util.List;

public final class Sprite {
    private final SpriteAPI api;

    public Sprite(SpriteAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Sprite;
    }

    public OkResult command(CommandRequest body) throws ApiException {
        return api.command(body);
    }

    public OkResult endGroup(HandleRequest body) throws ApiException {
        return api.endGroup(body);
    }

    public SpriteInfo info(HandleRequest body) throws ApiException {
        return api.info(body);
    }

    public OkResult startLoopGroup(StartLoopGroupRequest body) throws ApiException {
        return api.startLoopGroup(body);
    }

    public OkResult startTriggerGroup(StartTriggerGroupRequest body) throws ApiException {
        return api.startTriggerGroup(body);
    }

    public SpriteState stateAt(SpriteTimeRequest body) throws ApiException {
        return api.stateAt(body);
    }
}
