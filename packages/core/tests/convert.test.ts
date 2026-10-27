import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, it } from 'vitest';

import { convertEffectBundle, convertEffectScript } from '../src/convert.js';

const here = fileURLToPath(new URL('.', import.meta.url));
const examples = join(here, '..', 'examples');

function backgroundSource(): string {
    return readFileSync(join(examples, 'Background.cs'), 'utf8');
}

function spectrumSource(): string {
    return readFileSync(join(examples, 'Spectrum.cs'), 'utf8');
}

/** The M14 reference corpus: every script must convert without diagnostics. */
const REFERENCE_EFFECTS = [
    'Background.cs',
    'HitObjectHighlight.cs',
    'ImportOsb.cs',
    'Particles.cs',
    'Spectrum.cs',
    'RadialSpectrum.cs',
];

describe('convertEffectScript', () => {
    it('generates a Rust plug-in crate for After Effects', () => {
        const result = convertEffectScript(backgroundSource(), {
            target: 'after-effects',
            sourceFile: 'Background.cs',
        });

        const paths = result.files.map(file => file.path);
        expect(paths).toContain('Background/Cargo.toml');
        expect(paths).toContain('Background/src/lib.rs');
        expect(paths).toContain('Background/manifest.json');
        // The host boundary is a Rust crate + a small PF_ C++ entry + PiPL, not CMake.
        expect(paths).toContain('Background/src/ae_entry.cpp');
        expect(paths).toContain('Background/build.rs');
        expect(paths.some(path => path.endsWith('CMakeLists.txt'))).toBe(false);

        const cargo = result.files.find(file => file.path.endsWith('Cargo.toml'));
        expect(cargo?.contents).toContain('osb-runtime');
        expect(cargo?.contents).toContain('osb-plugin-ae');

        const lib = result.files.find(file => file.path.endsWith('src/lib.rs'));
        expect(lib?.contents).toContain('impl Effect for Background');
        expect(lib?.contents).toContain('fn generate');

        const manifest = result.files.find(file => file.path.endsWith('manifest.json'));
        const parsed = JSON.parse(manifest?.contents ?? '{}') as {
            parameters?: { control?: string }[];
        };
        expect(parsed.parameters?.length).toBeGreaterThan(0);
        // Background is a fully supported reference effect: it must convert cleanly.
        expect(result.diagnostics).toHaveLength(0);
    });

    it('generates a Rust crate with the OFX adapter for DaVinci Resolve', () => {
        const result = convertEffectScript(backgroundSource(), {
            target: 'davinci-resolve',
            sourceFile: 'Background.cs',
        });
        const cargo = result.files.find(file => file.path.endsWith('Cargo.toml'));
        expect(cargo?.contents).toContain('osb-plugin-ofx');
    });

    it('generates a Rust crate with the premiere feature for Premiere Pro', () => {
        const result = convertEffectScript(backgroundSource(), {
            target: 'premiere-pro',
            sourceFile: 'Background.cs',
        });
        const cargo = result.files.find(file => file.path.endsWith('Cargo.toml'));
        expect(cargo?.contents).toContain('premiere = ["osb-plugin-ae/premiere"]');
    });

    it.each(REFERENCE_EFFECTS)('converts the %s reference effect cleanly', file => {
        const source = readFileSync(join(examples, file), 'utf8');
        const result = convertEffectScript(source, { target: 'after-effects', sourceFile: file });
        expect(result.diagnostics).toHaveLength(0);

        const lib = result.files.find(candidate => candidate.path.endsWith('src/lib.rs'));
        expect(lib?.contents).toContain('impl Effect for');
        expect(lib?.contents).toContain('fn generate');
    });

    it('converts the ThreeDimRing ternary acceptance target', () => {
        const source = readFileSync(join(examples, 'ThreeDimRing.cs'), 'utf8');
        const result = convertEffectScript(source, {
            target: 'after-effects',
            sourceFile: 'ThreeDimRing.cs',
        });
        const lib = result.files.find(file => file.path.endsWith('src/lib.rs'));
        // `RandomTrailColor ? new Color4(..) : MinColor` becomes a Rust if/else.
        expect(lib?.contents).toContain('if self.random_trail_color.clone() { Color4::new(');
        expect(lib?.contents).toContain('} else { self.min_color.clone() }');
        // A held sprite is a non-borrowing handle, and helpers that use generator
        // APIs receive `rt` (so a helper can call GetLayer/Random).
        expect(lib?.contents).toContain('let mut Dot = rt.storyboard.create_sprite_at("Dot"');
        expect(lib?.contents).toContain('rt.storyboard.move_(Dot,');
        expect(lib?.contents).toContain(
            'fn blur(&mut self, rt: &mut Runtime, Content: SpriteHandle'
        );
        expect(lib?.contents).toContain('self.blur(rt,');
        expect(result.diagnostics.some(d => d.message.includes('ternary'))).toBe(false);
    });

    it('maps the ImportOsb script onto the runtime .osb importer', () => {
        const source = readFileSync(join(examples, 'ImportOsb.cs'), 'utf8');
        const result = convertEffectScript(source, {
            target: 'after-effects',
            sourceFile: 'ImportOsb.cs',
        });
        expect(result.diagnostics).toHaveLength(0);
        const lib = result.files.find(file => file.path.endsWith('src/lib.rs'));
        expect(lib?.contents).toContain('rt.import_osb(self.path.clone());');
        // The hand-written switch/StreamReader parser helpers are not emitted.
        expect(lib?.contents).not.toContain('parse_events_section');
        expect(lib?.contents).not.toContain('switch');
    });

    it('flattens external library methods and maps their calls', () => {
        const library =
            'namespace S { public class MathUtil { public static double Twice(double x) { return x * 2; } } }';
        const effect = `using osbTKCommon.Scripting;
            using osbTKCommon.Storyboarding;
            namespace S { public class UsesLib : StoryboardObjectGenerator {
                [Configurable] public double Amount = 5;
                public override void Generate() {
                    var l = GetLayer("Main");
                    var s = l.CreateSprite("sb/x.png", OsbOrigin.Centre);
                    s.Scale(0, MathUtil.Twice(Amount));
                }
            } }`;
        const result = convertEffectScript(effect, {
            target: 'after-effects',
            libraries: [{ source: library, name: 'MathUtil' }],
        });
        const lib = result.files.find(file => file.path.endsWith('src/lib.rs'));
        expect(lib?.contents).toContain('self.math_util_twice(self.amount.clone())');
        expect(lib?.contents).toContain('fn math_util_twice');
    });
});

describe('convertEffectBundle', () => {
    it('bundles multiple effects into a single plug-in', () => {
        const result = convertEffectBundle({
            target: 'after-effects',
            bundle: true,
            effects: [
                { source: backgroundSource(), name: 'Background' },
                { source: spectrumSource(), name: 'Spectrum' },
            ],
        });
        expect(result.bundle).toBe(true);
        expect(result.plugins).toHaveLength(1);
        const lib = result.plugins[0]?.files.find(file => file.path.endsWith('src/lib.rs'));
        expect(lib?.contents).toContain('pub background_opacity: f64');
        expect(lib?.contents).toContain('pub spectrum_bar_count: f64');

        // Bundled parameters are grouped by source effect by default.
        const manifest = result.plugins[0]?.files.find(file => file.path.endsWith('manifest.json'));
        const parameters = (
            JSON.parse(manifest?.contents ?? '{}') as {
                parameters?: { name: string; group?: string }[];
            }
        ).parameters;
        expect(parameters?.find(p => p.name === 'Background_StartTime')?.group).toBe(
            'Background - Timing'
        );
        expect(parameters?.find(p => p.name === 'Spectrum_Position')?.group).toBe(
            'Spectrum - Bars'
        );
    });

    it('combines bundled parameters by their original [Group] when asked', () => {
        const result = convertEffectBundle({
            target: 'after-effects',
            bundle: true,
            parameterGrouping: 'combined',
            effects: [{ source: backgroundSource() }, { source: spectrumSource() }],
        });
        const manifest = result.plugins[0]?.files.find(file => file.path.endsWith('manifest.json'));
        const parameters = (
            JSON.parse(manifest?.contents ?? '{}') as {
                parameters?: { name: string; group?: string }[];
            }
        ).parameters;
        expect(parameters?.find(p => p.name === 'Background_StartTime')?.group).toBe('Timing');
        expect(parameters?.find(p => p.name === 'Spectrum_Position')?.group).toBe('Bars');
    });

    it('converts multiple effects into individual plug-ins by default', () => {
        const result = convertEffectBundle({
            target: 'after-effects',
            effects: [{ source: backgroundSource() }, { source: spectrumSource() }],
        });
        expect(result.bundle).toBe(false);
        expect(result.plugins.map(plugin => plugin.effectName)).toEqual(['Background', 'Spectrum']);
    });
});
