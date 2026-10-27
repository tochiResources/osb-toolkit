import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import {
    Core,
    CurveKind,
    Easing,
    Generator,
    OsbOrigin,
    osb,
    Runtime,
    type StoryboardCommand,
    type Timeline,
    type TimelineClip,
} from '../src/index';
import { Layer as CleanLayer, Sprite as CleanSprite } from '../src/lib/client';

const OSU =
    'osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n[TimingPoints]\n0,500,4,2,1,100,1,0\n[HitObjects]\n256,192,1000,1,0\n';

describe('@tochii/osb-sdk-js', () => {
    it('exposes the native version', () => {
        expect(osb.version()).toBeTruthy();
    });

    it('parses a beatmap into the typed model', async () => {
        const beatmap = await Core.parsing.parseOsu({ text: OSU });
        expect(beatmap.audioFilename).toBe('song.mp3');
        expect(beatmap.hitObjects[0]?.kind).toBe('Circle');
    });

    it('exposes the non-runtime Core subpath', async () => {
        const beatmap = await Core.parsing.parseOsu({ text: OSU });
        expect(beatmap.audioFilename).toBe('song.mp3');
    });

    it('computes origin vectors and screen bounds', async () => {
        const origin = await Core.origin.getOriginVector({
            origin: OsbOrigin.Centre,
            width: 100,
            height: 100,
        });
        expect(origin.x).toBeCloseTo(50, 6);
        const inBounds = await Core.origin.inScreenBounds({
            position: { x: 427, y: 240 },
            size: { x: 100, y: 100 },
            rotation: 0,
            origin: { x: 50, y: 50 },
        });
        expect(inBounds.value).toBe(true);
    });

    it('applies easing', async () => {
        expect(
            (await Core.animations.applyEasing({ easing: Easing.InQuad, t: 0.5 })).value
        ).toBeCloseTo(0.25, 6);
        expect(
            (
                await Core.animations.easingValue({
                    cubicBezier: [0.42, 0.0, 0.58, 1.0],
                    t: 1.0,
                })
            ).value
        ).toBeCloseTo(1.0, 6);
    });

    it('drives the storyboard through the generated client', async () => {
        const runtime = new Runtime();
        try {
            const handle = await runtime.layer.createSprite({
                layer: 'Foreground',
                path: 'sb/dot.png',
                origin: OsbOrigin.Centre,
            });
            await runtime.sprite.command({
                kind: 'move',
                layer: handle.layer,
                sprite: handle.sprite,
                startTime: 0,
                endTime: 1000,
                easing: Easing.None,
                start: { x: 0, y: 0 },
                end: { x: 100, y: 0 },
            });
            const state = await runtime.sprite.stateAt({
                layer: handle.layer,
                sprite: handle.sprite,
                time: 500,
            });
            expect(state.position.x).toBeCloseTo(50, 4);
        } finally {
            runtime.free();
        }
    });

    it('drives the handle-bound clean client without passing handles', async () => {
        const runtime = new Runtime();
        try {
            const handle = await runtime.generator.layerHandle({ name: 'Foreground' });
            const layer = new CleanLayer(runtime.layer, runtime.sprite, handle.layer);
            expect(CleanLayer.instanceOf(layer)).toBe(true);

            const sprite = await layer.createSprite({
                path: 'sb/dot.png',
                origin: OsbOrigin.Centre,
            });
            expect(CleanSprite.instanceOf(sprite)).toBe(true);

            await sprite.command({
                kind: 'move',
                startTime: 0,
                endTime: 1000,
                easing: Easing.None,
                start: { x: 0, y: 0 },
                end: { x: 100, y: 0 },
            });
            const state = await sprite.stateAt({ time: 500 });
            expect(state.position.x).toBeCloseTo(50, 4);
        } finally {
            runtime.free();
        }
    });

    it('surfaces bridge errors', async () => {
        await expect(osb.audio.fft({ path: 'does-not-exist.wav' })).rejects.toThrow();
    });

    it('drives the curve library', async () => {
        const curve = {
            kind: CurveKind.Linear,
            points: [
                { x: 0, y: 0 },
                { x: 100, y: 0 },
            ],
        };
        expect((await Core.curve.length({ curve })).value).toBeCloseTo(100, 6);
        expect((await Core.curve.point({ curve, t: 0.5 })).x).toBeCloseTo(50, 6);
        expect((await Core.curve.sample({ curve, count: 3 })).points).toHaveLength(3);
        expect((await Core.curve.boundingBox({ curve })).max.x).toBeCloseTo(100, 6);
        const simplified = await Core.curve.simplify({
            points: [
                { x: 0, y: 0 },
                { x: 1, y: 0.05 },
                { x: 2, y: 0 },
                { x: 3, y: 0 },
            ],
            tolerance: 0.1,
        });
        expect(simplified.points).toHaveLength(2);
        expect((await Core.curve.normal({ curve, t: 0.5 })).x).toBeCloseTo(0, 6);
        expect((await Core.curve.curvature({ curve, t: 0.5 })).value).toBeCloseTo(0, 6);
        expect((await Core.curve.distanceTo({ curve, point: { x: 50, y: 10 } })).value).toBeCloseTo(
            10,
            6
        );
        expect((await Core.curve.offset({ curve, distance: 5 })).points).toHaveLength(2);
        expect(
            (
                await Core.curve.arc({
                    center: { x: 0, y: 0 },
                    radius: 10,
                    startAngle: 0,
                    sweepAngle: 180,
                    count: 3,
                })
            ).points
        ).toHaveLength(3);
        expect(
            (await Core.curve.ellipse({ center: { x: 0, y: 0 }, radiusX: 4, radiusY: 2, count: 4 }))
                .points
        ).toHaveLength(4);
        const hit = await Core.curve.lineIntersect({
            a1: { x: 0, y: 0 },
            a2: { x: 10, y: 10 },
            b1: { x: 0, y: 10 },
            b2: { x: 10, y: 0 },
        });
        expect(hit.point?.x).toBeCloseTo(5, 6);
    });

    it('diagnoses storyboards and mapsets', async () => {
        const report = await osb.diagnostics.diagnoseOsu({
            text: 'osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n',
        });
        expect(report.errorCount).toBeGreaterThanOrEqual(1);
    });

    it('optimizes storyboards', async () => {
        const result = await osb.optimize.optimizeOsb({
            text: '[Events]\nSprite,Foreground,Centre,"sb/dot.png",320,240\n F,0,0,100,0,1\n F,0,0,100,0,1\n',
            options: { deduplicate: true },
        });
        expect(result.stats.deduplicated).toBe(1);
        expect(result.osb).toContain('Sprite');
    });

    it('surfaces image errors', async () => {
        await expect(osb.image.info({ path: 'does-not-exist.png' })).rejects.toThrow();
    });

    it('surfaces media errors', async () => {
        await expect(osb.media.info({ path: 'does-not-exist.mp4' })).rejects.toThrow();
    });

    it('runs a generator-backed effect', async () => {
        class Pulse extends Generator {
            async generate(): Promise<void> {
                await this.createSprite('Foreground', 'sb/dot.png', OsbOrigin.Centre);
            }
        }
        const effect = new Pulse();
        try {
            await effect.generate();
            expect(await effect.spriteCount()).toBe(1);
        } finally {
            effect.free();
        }
    });

    it('reads back the built storyboard', async () => {
        class Effect extends Generator {
            async generate(): Promise<void> {
                const layer = await this.getLayer('Foreground');
                const sprite = await layer.createSprite('sb/dot.png', OsbOrigin.Centre);
                await sprite.fade(0, 1000, 0, 1);
            }
        }
        const effect = new Effect();
        try {
            await effect.generate();
            const storyboard = await effect.storyboard();
            const foreground = storyboard.layers.find(layer => layer.identifier === 'Foreground');
            expect(foreground?.sprites[0]?.texturePath).toBe('sb/dot.png');
            expect(foreground?.sprites[0]?.commands[0]?.kind).toBe('fade');
        } finally {
            effect.free();
        }
    });

    it('authors a sprite through the layer/sprite facade', async () => {
        class Effect extends Generator {
            async generate(): Promise<void> {
                const layer = await this.getLayer('Foreground');
                const sprite = await layer.createSprite('sb/dot.png', OsbOrigin.Centre);
                await sprite.move(Easing.None, 0, 1000, { x: 0, y: 0 }, { x: 100, y: 0 });
                await sprite.startLoopGroup(1000, 2);
                await sprite.fade(0, 100, 0, 1);
                await sprite.fade(150, 1);
                await sprite.endGroup();
                expect((await sprite.positionAt(500)).x).toBeCloseTo(50, 4);
                expect(await this.spriteCount()).toBe(1);
            }
        }
        const effect = new Effect();
        try {
            await effect.generate();
        } finally {
            effect.free();
        }
    });

    it('animates and samples a sprite anchor', async () => {
        class Effect extends Generator {
            async generate(): Promise<void> {
                const layer = await this.getLayer('Foreground');
                const sprite = await layer.createSprite('sb/dot.png', OsbOrigin.Centre);
                await sprite.anchor(Easing.None, 0, 1000, { x: 0, y: 0 }, { x: 100, y: 50 });
                const mid = await sprite.anchorAt(500);
                expect(mid.x).toBeCloseTo(50, 4);
                expect(mid.y).toBeCloseTo(25, 4);
            }
        }
        const effect = new Effect();
        try {
            await effect.generate();
        } finally {
            effect.free();
        }
    });

    it('nests segments through the object facade', async () => {
        class Effect extends Generator {
            async generate(): Promise<void> {
                const group = await this.getLayer('Group');
                await group.createSprite('sb/group.png', OsbOrigin.Centre);
                const child = await group.createSegment();
                await child.createSprite('sb/child.png', OsbOrigin.Centre);
                await child.setReverseDepth(true);
                await child.addIndex(3);
                await child.addIndex(-1);
                const indexes = await child.indexes();
                expect(indexes.highest).toBe(3);
                expect(indexes.lowest).toBe(-1);
                expect(await this.spriteCount()).toBe(2);
                await child.discard();
                expect(await this.spriteCount()).toBe(1);
            }
        }
        const effect = new Effect();
        try {
            await effect.generate();
        } finally {
            effect.free();
        }
    });

    it('runs the 3D motion-graphics keyframe helpers', async () => {
        const scaled = await osb.helpers.keyframe.scaleKeyframes(
            {
                keys: [
                    { time: 0, value: [1, 2] },
                    { time: 100, value: [3, 4] },
                ],
            },
            2
        );
        expect(scaled.keys[1]?.value[0]).toBeCloseTo(6, 6);
        expect(scaled.keys[1]?.value[1]).toBeCloseTo(8, 6);

        const mirrored = await osb.helpers.keyframe.mirror(
            {
                keys: [
                    { time: 0, value: [10] },
                    { time: 10, value: [20] },
                ],
            },
            { pivot: 100 }
        );
        expect(mirrored.keys[0]?.value[0]).toBeCloseTo(190, 6);

        const orbit = await osb.helpers.scene3d.orbit({ radius: 10, count: 5 });
        expect(orbit.keys).toHaveLength(5);

        const points = await osb.helpers.keyframe.scatter({ seed: 1, count: 3 });
        expect(points).toHaveLength(3);

        expect(await osb.helpers.keyframe.stagger(3, 100)).toBe(300);
    });

    it('runs the 2D command helpers', async () => {
        const commands = [
            {
                kind: 'fade',
                startTime: 0,
                endTime: 100,
                easing: Easing.None,
                start: { kind: 'scalar', value: 0 },
                end: { kind: 'scalar', value: 1 },
            },
            {
                kind: 'fade',
                startTime: 200,
                endTime: 300,
                easing: Easing.None,
                start: { kind: 'scalar', value: 1 },
                end: { kind: 'scalar', value: 0 },
            },
        ] as unknown as StoryboardCommand[];

        const shifted = await osb.helpers.command.shiftCommands(commands, 50);
        expect(shifted[0]?.startTime).toBe(50);

        const selected = await osb.helpers.command.selectCommands(commands, {
            start: 150,
        });
        expect(selected).toHaveLength(1);
        expect(selected[0]?.startTime).toBe(200);

        const merged = await osb.helpers.command.mergeCommands(commands);
        expect(merged).toHaveLength(2);

        const duplicated = await osb.helpers.command.duplicateCommands(commands, {
            offset: 1000,
            count: 2,
        });
        expect(duplicated).toHaveLength(4);
    });

    it('shares the motion-graphics helpers between 2D and 3D', async () => {
        // A 2D channel (no Z): the Z axis is optional.
        const scaled = await osb.helpers.keyframe.scaleKeyframes(
            {
                keys: [
                    { time: 0, value: [1, 2] },
                    { time: 100, value: [3, 4] },
                ],
            },
            2
        );
        expect(scaled.keys[1]?.value).toEqual([6, 8]);

        // A 3D channel uses the same helper.
        const moved = await osb.helpers.keyframe.offsetKeyframes(
            { keys: [{ time: 0, value: [1, 2, 3] }] },
            1
        );
        expect(moved.keys[0]?.value).toEqual([2, 3, 4]);

        // The 3D constructor and the shared origin/anchor helper.
        const orbit = await osb.helpers.scene3d.orbit({ radius: 10, count: 5 });
        expect(orbit.keys).toHaveLength(5);
        const resolved = await osb.helpers.origin.resolveAnchor(
            OsbOrigin.Centre,
            { x: 10, y: -5 },
            100,
            100
        );
        expect(resolved.x).toBeCloseTo(60, 6);
        expect(resolved.y).toBeCloseTo(45, 6);

        // The same keyframe helper applied to a 2D command channel.
        const commands = [
            {
                kind: 'fade',
                startTime: 0,
                endTime: 100,
                easing: Easing.None,
                start: { kind: 'scalar', value: 0 },
                end: { kind: 'scalar', value: 1 },
            },
            {
                kind: 'fade',
                startTime: 100,
                endTime: 200,
                easing: Easing.None,
                start: { kind: 'scalar', value: 1 },
                end: { kind: 'scalar', value: 0 },
            },
        ] as unknown as StoryboardCommand[];
        const flattened = await osb.helpers.keyframe.transformCommands(
            commands,
            'fade',
            'keyframe.flatten',
            { mode: 'mean' }
        );
        expect(flattened).toHaveLength(1);
        expect(flattened[0]?.startTime).toBe(0);
        expect(flattened[0]?.endTime).toBe(200);

        // The command group is the same object as the shared helper group.
        expect(typeof osb.helpers.command.shiftCommands).toBe('function');
    });

    it('manipulates easings, timelines and depth anchors', async () => {
        // InQuad at 0.5 is 0.25; amplifying about linear gives 0.5 + (0.25-0.5)*2.
        expect(await osb.helpers.easing.amplify(Easing.InQuad, 0.5, 2)).toBeCloseTo(0, 6);
        expect((await osb.helpers.easing.sample(Easing.OutQuad, 5)).length).toBe(5);

        const clip = (start: number): TimelineClip => ({
            start,
            channel: {
                keys: [
                    { time: 0, value: [0] },
                    { time: 100, value: [1] },
                ],
            },
        });
        const timeline: Timeline = { clips: [clip(0), clip(200)] };
        expect(await osb.helpers.timeline.duration(timeline)).toBeCloseTo(300, 6);
        const flushed = await osb.helpers.timeline.flush(timeline);
        expect(flushed.clips[1]?.channel.keys[0]?.time).toBe(200);
        expect((await osb.helpers.timeline.merge(timeline, timeline)).clips).toHaveLength(4);

        const anchor = await osb.helpers.origin.resolveAnchor3d(
            OsbOrigin.Centre,
            { x: 10, y: -5, z: 42 },
            100,
            100
        );
        expect(anchor.z).toBeCloseTo(42, 6);

        const commands = [
            {
                kind: 'fade',
                startTime: 0,
                endTime: 100,
                easing: Easing.None,
                start: { kind: 'scalar', value: 0 },
                end: { kind: 'scalar', value: 1 },
            },
        ] as unknown as StoryboardCommand[];
        const withEasing = await osb.helpers.command.setEasing(commands, Easing.InQuad);
        expect(withEasing[0]?.easing).toBe(Easing.InQuad);
        expect(
            (await osb.helpers.command.applyEasing(commands, 'fade', Easing.InQuad, 4)).length
        ).toBeGreaterThan(1);
    });

    it('manipulates colors across spaces and gradients', async () => {
        const gray = { r: 0.5, g: 0.5, b: 0.5, a: 1 };
        const lighter = await osb.helpers.color.lighten(gray, 0.25);
        expect(lighter.r).toBeCloseTo(0.75, 6);

        const red = { r: 1, g: 0, b: 0, a: 1 };
        const green = await osb.helpers.color.adjustHue(red, 120);
        expect(green.g).toBeGreaterThan(0.9);
        expect(green.r).toBeLessThan(0.1);

        // Offset in HSB (brightness +0.2).
        const offset = await osb.helpers.color.offset(
            { r: 0.2, g: 0.2, b: 0.2, a: 1 },
            { space: 'hsb', delta: [0, 0, 0.2, 0] }
        );
        expect(offset.r).toBeGreaterThan(0.3);

        // A two-stop gradient sampled halfway is the RGB midpoint.
        const stops = [
            { offset: 0, color: { r: 0, g: 0, b: 0, a: 1 } },
            { offset: 1, color: { r: 1, g: 1, b: 1, a: 1 } },
        ];
        const mid = await osb.helpers.color.gradient(stops, 0.5, { space: 'rgb' });
        expect(mid.r).toBeCloseTo(0.5, 6);

        const gradientChannel = await osb.helpers.color.gradientChannel(stops, {
            count: 5,
            start: 0,
            end: 100,
        });
        expect(gradientChannel.keys).toHaveLength(5);
        expect(gradientChannel.keys[0]?.value).toHaveLength(4);

        // A color command channel flattens through the shared keyframe helper.
        const colorCommands = [
            {
                kind: 'color',
                startTime: 0,
                endTime: 100,
                easing: Easing.None,
                start: { kind: 'color', value: { r: 0, g: 0, b: 0, a: 1 } },
                end: { kind: 'color', value: { r: 1, g: 1, b: 1, a: 1 } },
            },
        ] as unknown as StoryboardCommand[];
        const flattened = await osb.helpers.keyframe.transformCommands(
            colorCommands,
            'color',
            'keyframe.flatten',
            { mode: 'mean' }
        );
        expect(flattened).toHaveLength(1);
    });

    it('resolves a 2D scene and controls it from the top', async () => {
        const scene = {
            root: {
                name: 'root',
                children: [
                    {
                        name: 'leaf',
                        sprite: {
                            texturePath: 'sb/dot.png',
                            origin: OsbOrigin.Centre,
                            transform: {
                                position: {
                                    keys: [
                                        { time: 0, value: [0, 0] },
                                        { time: 100, value: [100, 0] },
                                    ],
                                },
                            },
                        },
                    },
                ],
            },
        };
        const resolved = await osb.helpers.scene2d.resolve(scene);
        expect(resolved).toHaveLength(1);
        expect(resolved[0]?.texturePath).toBe('sb/dot.png');

        // Translating the root cascades into the nested leaf.
        const moved = await osb.helpers.scene2d.translate(scene, [10, 20]);
        const after = await osb.helpers.scene2d.resolve(moved);
        const move = after[0]?.commands.find(c => c.kind === 'move');
        expect(move).toBeTruthy();
        if (!move) {
            throw new Error('expected a move command');
        }
        const start = (move.start as { value?: { x: number; y: number } }).value;
        expect(start?.x).toBeCloseTo(10, 6);
        expect(start?.y).toBeCloseTo(20, 6);

        // A hierarchical timeline flushes its children too.
        const clip = (start: number): TimelineClip => ({
            start,
            channel: {
                keys: [
                    { time: 0, value: [0] },
                    { time: 100, value: [1] },
                ],
            },
        });
        const tree: Timeline = { clips: [clip(0)], children: [{ clips: [clip(100)] }] };
        expect(await osb.helpers.timeline.clipCount(tree)).toBe(2);
        const flushed = await osb.helpers.timeline.flush(tree);
        expect(flushed.clips).toHaveLength(2);
        expect(flushed.children ?? []).toHaveLength(0);

        // Baking a 2D scene yields a full storyboard with the sprite on its layer.
        const baked = await osb.helpers.scene2d.bake(scene);
        const total = baked.layers.reduce((sum, layer) => sum + layer.sprites.length, 0);
        expect(total).toBe(1);

        // A unified 2D + 3D bake composes into one storyboard.
        const combined = await osb.helpers.scene3d.bakeScene({
            worlds2d: [scene],
            worlds3d: [
                {
                    nodes: [{ sprite: 'sb/3d.png' }],
                    camera: { position: [{ time: 0, value: [0, 0, 5] }] },
                },
            ],
            options: { endTime: 0 },
            options2d: { startTime: 0, endTime: 100 },
        });
        const combinedTotal = combined.layers.reduce((sum, layer) => sum + layer.sprites.length, 0);
        expect(combinedTotal).toBe(2);
    });

    it('exposes the file-operations library', () => {
        expect(osb.io.environment()).toBe('node');

        const path = join(tmpdir(), `osbtk-io-${Date.now()}.txt`);
        try {
            osb.io.writeText(path, 'hello');
            expect(osb.io.exists(path)).toBe(true);
            expect(osb.io.isFile(path)).toBe(true);
            expect(osb.io.isDir(path)).toBe(false);
            expect(osb.io.readText(path)).toBe('hello');
            expect(osb.io.fileSize(path)).toBe(5);
        } finally {
            osb.io.removeFile(path);
        }
        expect(osb.io.exists(path)).toBe(false);
    });
});
