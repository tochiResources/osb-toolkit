import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

import { describe, expect, it } from 'vitest';

import { bake3d, parseOsb, parseOsu, Runtime, version } from '../src/index.js';

/** The corpus is also exercised by the Rust, TS, C#, C++ and Python bindings. */
const corpusPath = fileURLToPath(
    new URL('../../../crates/osb-runtime/tests/golden/storyboarding3d/basic.json', import.meta.url)
);

interface Expect {
    spriteCount?: number;
    firstSpritePath?: string;
}

interface Case {
    input: unknown;
    expect: Expect;
}

describe('@tochii/osb-toolkit-native (wasm)', () => {
    it('reports the version', () => {
        expect(version()).toBeTruthy();
    });

    it('bakes the shared golden corpus', () => {
        const corpus = JSON.parse(readFileSync(corpusPath, 'utf8')) as { cases: Case[] };
        expect(corpus.cases.length).toBeGreaterThan(0);
        for (const testCase of corpus.cases) {
            const baked = bake3d(testCase.input) as { sprites: { spritePath: string }[] };
            if (testCase.expect.spriteCount !== undefined) {
                expect(baked.sprites).toHaveLength(testCase.expect.spriteCount);
            }
            if (testCase.expect.firstSpritePath !== undefined) {
                expect(baked.sprites[0]?.spritePath).toBe(testCase.expect.firstSpritePath);
            }
        }
    });

    it('parses a beatmap summary', () => {
        const beatmap = parseOsu(
            'osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n' +
                '[TimingPoints]\n0,500,4,2,1,100,1,0\n' +
                '[HitObjects]\n256,192,1000,1,0\n'
        ) as { audioFilename: string; hitObjects: { kind: string }[] };
        expect(beatmap.audioFilename).toBe('song.mp3');
        expect(beatmap.hitObjects[0]?.kind).toBe('Circle');
    });

    it('parses an osb summary', () => {
        const summary = parseOsb('[Events]\nSprite,Foreground,Centre,"sb/dot.png",320,240\n') as {
            spriteCount: number;
        };
        expect(summary.spriteCount).toBe(1);
    });

    it('drives the storyboard runtime', () => {
        const runtime = new Runtime();
        const handle = runtime.createSprite('Foreground', 'sb/dot.png');
        runtime.exec({
            op: 'Sprite.command',
            kind: 'move',
            layer: handle.layer,
            sprite: handle.sprite,
            startTime: 0,
            endTime: 1000,
            start: { x: 0, y: 0 },
            end: { x: 100, y: 0 },
        });
        const state = runtime.exec({
            op: 'Sprite.stateAt',
            layer: handle.layer,
            sprite: handle.sprite,
            time: 500,
        }) as { position: { x: number } };
        expect(state.position.x).toBe(50);
        runtime.free();
    });
});
