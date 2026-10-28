package moe.tochi.osbtk.client.runtime;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.SpriteAPI;
import moe.tochi.osbtk.model.CommandRequest;
import moe.tochi.osbtk.model.Easing;
import moe.tochi.osbtk.model.HandleRequest;
import moe.tochi.osbtk.model.ParameterType;
import moe.tochi.osbtk.model.SpriteInfo;
import moe.tochi.osbtk.model.SpriteState;
import moe.tochi.osbtk.model.SpriteTimeRequest;
import moe.tochi.osbtk.model.StartLoopGroupRequest;
import moe.tochi.osbtk.model.StartTriggerGroupRequest;
import moe.tochi.osbtk.client.CommandOptions;
import moe.tochi.osbtk.client.SpriteTimeOptions;
import moe.tochi.osbtk.client.StartLoopGroupOptions;
import moe.tochi.osbtk.client.StartTriggerGroupOptions;
import java.util.List;

public final class Sprite {
    private final SpriteAPI api;
    private final int layer;
    private final int sprite;

    public Sprite(SpriteAPI api, int layer, int sprite) {
        this.api = api;
        this.layer = layer;
        this.sprite = sprite;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Sprite;
    }

    /**
     * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @param kind The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
     * @param startTime The command start time in milliseconds.
     * @param endTime The command end time in milliseconds.
     * @param easing The easing function.
     */
    public void command(String kind, Double startTime, Double endTime, Easing easing) throws ApiException {
        command(CommandOptions.builder().kind(kind).startTime(startTime).endTime(endTime).easing(easing).build());
    }

    /**
     * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @param kind The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
     * @param startTime The command start time in milliseconds.
     * @param endTime The command end time in milliseconds.
     * @param easing The easing function.
     * @param start The start value (scalar, point or color).
     */
    public void command(String kind, Double startTime, Double endTime, Easing easing, Object start) throws ApiException {
        command(CommandOptions.builder().kind(kind).startTime(startTime).endTime(endTime).easing(easing).start(start).build());
    }

    /**
     * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @param kind The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
     * @param startTime The command start time in milliseconds.
     * @param endTime The command end time in milliseconds.
     * @param easing The easing function.
     * @param start The start value (scalar, point or color).
     * @param end The end value (scalar, point or color).
     */
    public void command(String kind, Double startTime, Double endTime, Easing easing, Object start, Object end) throws ApiException {
        command(CommandOptions.builder().kind(kind).startTime(startTime).endTime(endTime).easing(easing).start(start).end(end).build());
    }

    /**
     * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @param kind The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`, `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`, `flipV`, `additive`).
     * @param startTime The command start time in milliseconds.
     * @param endTime The command end time in milliseconds.
     * @param easing The easing function.
     * @param start The start value (scalar, point or color).
     * @param end The end value (scalar, point or color).
     * @param parameter The parameter kind for a `parameter` command.
     */
    public void command(String kind, Double startTime, Double endTime, Easing easing, Object start, Object end, ParameterType parameter) throws ApiException {
        command(CommandOptions.builder().kind(kind).startTime(startTime).endTime(endTime).easing(easing).start(start).end(end).parameter(parameter).build());
    }

    /**
     * A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     *
     * @param options A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4` depending on `kind`.
     */
    public void command(CommandOptions options) throws ApiException {
        api.command(new CommandRequest()
            .easing(options.easing)
            .end(options.end)
            .endTime(options.endTime)
            .kind(options.kind)
            .layer(this.layer)
            .parameter(options.parameter)
            .sprite(this.sprite)
            .start(options.start)
            .startTime(options.startTime));
    }

    /**
     * A sprite handle pair.
     */
    public void endGroup() throws ApiException {
        api.endGroup(new HandleRequest()
            .layer(this.layer)
            .sprite(this.sprite));
    }

    /**
     * A sprite handle pair.
     *
     * @return The sprite's initial state
     */
    public SpriteInfo info() throws ApiException {
        return api.info(new HandleRequest()
            .layer(this.layer)
            .sprite(this.sprite));
    }

    /**
     * Start a loop group.
     *
     * @param startTime The loop start time in milliseconds.
     * @param count The number of iterations.
     */
    public void startLoopGroup(Double startTime, Double count) throws ApiException {
        startLoopGroup(StartLoopGroupOptions.builder().startTime(startTime).count(count).build());
    }

    /**
     * Start a loop group.
     *
     * @param options Start a loop group.
     */
    public void startLoopGroup(StartLoopGroupOptions options) throws ApiException {
        api.startLoopGroup(new StartLoopGroupRequest()
            .count(options.count)
            .layer(this.layer)
            .sprite(this.sprite)
            .startTime(options.startTime));
    }

    /**
     * Start a trigger group.
     *
     * @param name The trigger name (matches a hit object).
     * @param startTime The trigger start time in milliseconds.
     * @param endTime The trigger end time in milliseconds.
     * @param group The trigger group index.
     */
    public void startTriggerGroup(String name, Double startTime, Double endTime, Integer group) throws ApiException {
        startTriggerGroup(StartTriggerGroupOptions.builder().name(name).startTime(startTime).endTime(endTime).group(group).build());
    }

    /**
     * Start a trigger group.
     *
     * @param options Start a trigger group.
     */
    public void startTriggerGroup(StartTriggerGroupOptions options) throws ApiException {
        api.startTriggerGroup(new StartTriggerGroupRequest()
            .endTime(options.endTime)
            .group(options.group)
            .layer(this.layer)
            .name(options.name)
            .sprite(this.sprite)
            .startTime(options.startTime));
    }

    /**
     * A sprite handle pair plus a sample time.
     *
     * @param time The sample time in milliseconds.
     *
     * @return The sampled sprite state
     */
    public SpriteState stateAt(Double time) throws ApiException {
        return stateAt(SpriteTimeOptions.builder().time(time).build());
    }

    /**
     * A sprite handle pair plus a sample time.
     *
     * @param options A sprite handle pair plus a sample time.
     *
     * @return The sampled sprite state
     */
    public SpriteState stateAt(SpriteTimeOptions options) throws ApiException {
        return api.stateAt(new SpriteTimeRequest()
            .layer(this.layer)
            .sprite(this.sprite)
            .time(options.time));
    }
}
