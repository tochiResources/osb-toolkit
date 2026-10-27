import { describe, expect, it } from 'vitest';

import { transpileCSharpToRust } from '../src/csharp/rust/index.js';

function rust(source: string): string {
    return transpileCSharpToRust(source).code;
}

describe('transpileCSharpToRust', () => {
    it('emits let mut for local declarations', () => {
        expect(rust('var i = 0;')).toContain('let mut i = 0.0;');
        expect(rust('int count = 3;')).toContain('let mut count = 3.0;');
    });

    it('rewrites foreach to a for loop over iter()', () => {
        const code = rust('foreach (var hitObject in beatmap.HitObjects) { DoThing(hitObject); }');
        expect(code).toContain('for hitObject in (beatmap.hit_objects).iter() {');
    });

    it('maps Vector constructors to runtime math types', () => {
        expect(rust('var p = new Vector2(1, 2);')).toContain('Vec2::new(1.0, 2.0)');
        expect(rust('var v = new Vector3(1, 2, 3);')).toContain('Vec3::new(1.0, 2.0, 3.0)');
    });

    it('maps Math calls to runtime free functions', () => {
        expect(rust('var s = Math.Sin(x);')).toContain('osb_runtime::math::sin(x)');
        expect(rust('var c = Math.Clamp(v, 0, 1);')).toContain(
            'osb_runtime::math::clamp(v, 0.0, 1.0)'
        );
        expect(rust('var pi = Math.PI;')).toContain('std::f64::consts::PI');
    });

    it('rewrites casts to Rust as-expressions', () => {
        expect(rust('int n = (int)value;')).toContain('((value) as f64).trunc()');
        expect(rust('double d = (double)value;')).toContain('((value) as f64)');
    });

    it('rewrites enum member access with ::', () => {
        expect(rust('var o = OsbOrigin.Centre;')).toContain('OsbOrigin::Centre');
        expect(rust('var e = OsbEasing.OutQuad;')).toContain('Easing::OutQuad');
        expect(rust('var p = ParameterType.AdditiveBlending;')).toContain(
            'ParameterType::Additive'
        );
    });

    it('converts interpolated strings to format!', () => {
        const code = rust('var s = $"bar {i} of {BarCount}";');
        expect(code).toContain('format!("bar {} of {}", i, BarCount)');
    });

    it('maps generator API calls onto the runtime', () => {
        expect(rust('GetLayer(OsbLayer.Background);')).toContain('rt.storyboard.layer');
        expect(rust('var f = GetFft(time, 64, null, OsbEasing.None, 0);')).toContain(
            'rt.context.get_fft'
        );
    });

    it('maps the Beatmap property and GetMapsetBitmap onto the runtime', () => {
        expect(rust('var p = Beatmap.BackgroundPath;')).toContain(
            'rt.context.beatmap().unwrap().background_path'
        );
        expect(rust('var b = GetMapsetBitmap("bg.png");')).toContain(
            'rt.context.get_mapset_bitmap(&"bg.png")'
        );
        expect(rust('var t = AudioDuration;')).toContain('rt.context.audio_duration()');
    });

    it('rewrites LINQ terminal operators and null operators', () => {
        const code = rust('var t = Beatmap.HitObjects.LastOrDefault()?.EndTime ?? AudioDuration;');
        expect(code).toContain('.last()');
        expect(code).toContain('.map(|__osb| __osb.end_time)');
        expect(code).toContain('.unwrap_or(rt.context.audio_duration())');
    });

    it('maps OpenTK Color4 static colors to runtime constants', () => {
        expect(rust('var c = Color4.Beige;')).toContain('Color4::BEIGE');
        expect(rust('var c = Color4.LightGoldenrodYellow;')).toContain(
            'Color4::LIGHT_GOLDENROD_YELLOW'
        );
    });

    it('collapses sprite command overloads and moves the easing last', () => {
        expect(rust('bg.Scale(t, 2);')).toContain('.scale(t, t, 2.0, 2.0, Easing::None)');
        expect(rust('bg.Fade(a, b, 0, 1);')).toContain('.fade(a, b, 0.0, 1.0, Easing::None)');
        expect(rust('bg.Fade(OsbEasing.OutQuad, a, b, 0, 1);')).toContain(
            '.fade(a, b, 0.0, 1.0, Easing::OutQuad)'
        );
        expect(rust('bg.Move(t, 10, 20);')).toContain(
            '.move_(t, t, Vec2::new(10.0, 20.0), Vec2::new(10.0, 20.0), Easing::None)'
        );
        expect(rust('bg.Additive(a, b);')).toContain(
            '.parameter(a, b, ParameterType::Additive, 0.0, 1.0, Easing::None)'
        );
    });

    it('wraps brace-less if bodies in blocks', () => {
        const code = rust('if (x == 1) y = 2;');
        expect(code).toContain('if (x == 1.0) {');
        expect(code).toContain('y = 2.0');
    });

    it('strips a UTF-8 BOM before tokenizing', () => {
        const result = transpileCSharpToRust('\uFEFFvar x = 1;');
        expect(result.code).toContain('let mut x = 1.0;');
        expect(result.diagnostics).toHaveLength(0);
    });

    it('is insensitive to arbitrary C# whitespace and formatting', () => {
        const code = rust('for( var i=0 ;i< 10 ;i++)\n{\nvar x=GetLayer( "" );\n}');
        expect(code).toContain('let mut i=0.0;');
        expect(code).toContain('while i< 10.0');
        expect(code).toContain('.layer');
    });

    it('resolves osuTK/OpenTK and namespaced type aliases', () => {
        expect(rust('var v = new osuTK.Vector2(1, 2);')).toContain('Vec2::new(1.0, 2.0)');
        expect(rust('var v = new OpenTK.Vector3(1, 2, 3);')).toContain('Vec3::new(1.0, 2.0, 3.0)');
        expect(rust('var m = Matrix4.LookAt(a, b, c);')).toContain('Matrix4::look_at(a, b, c)');
    });

    it('rewrites C# for loops to a while loop over an f64 counter', () => {
        const code = rust('for (var i = 0; i < n; i++) { f(i); }');
        expect(code).toContain('let mut i = 0.0;');
        expect(code).toContain('while i < n {');
        expect(code).toContain('i += 1.0;');
    });

    it('converts lambdas to Rust closures', () => {
        expect(rust('list.ForEach((a, b) => { g(a, b); }, 0, s => s);')).toContain('|a, b|');
        expect(rust('list.ForEach(x => x, 0, s => s);')).toContain('|x|');
    });

    it('maps KeyframedValue construction and indexing', () => {
        expect(rust('var k = new KeyframedValue<float>(null);')).toContain(
            'KeyframedValue::<f64>::new()'
        );
        expect(rust('var p = new KeyframedValue<Vector2>[8];')).toContain(
            'vec![KeyframedValue::<Vec2>::new(); (8.0) as usize]'
        );
        expect(rust('var h = fft[i];')).toContain('fft[(i) as usize].clone()');
    });

    it('rewrites null comparisons and is-patterns', () => {
        const nullCheck = rust('var a = list.FirstOrDefault() != null;');
        expect(nullCheck).toContain('.first()');
        expect(nullCheck).toContain('.is_some()');
        expect(rust('if (h is OsuSlider) { }')).toContain('.is_slider()');
    });

    it('reports unsupported generator APIs as diagnostics', () => {
        const result = transpileCSharpToRust('var f = LoadFont("font.ttf");');
        expect(result.diagnostics.some(d => d.code === 'OSB_UNSUPPORTED_API')).toBe(true);
    });

    it('rewrites the ternary operator to an if/else expression', () => {
        expect(rust('var c = flag ? a : b;')).toContain('let mut c = if flag { a } else { b };');
        expect(rust('x = cond ? 1 : 2;')).toContain('x = if cond { 1.0 } else { 2.0 };');
        expect(rust('return flag ? a : b;')).toContain('return if flag { a } else { b };');
    });

    it('handles nested ternaries with C# right-associativity', () => {
        expect(rust('var c = a ? b : c ? d : e;')).toContain(
            'if a { b } else { if c { d } else { e } }'
        );
        expect(rust('var c = a ? b ? c : d : e;')).toContain(
            'if a { if b { c } else { d } } else { e }'
        );
    });

    it('handles ternaries in arguments, indices and interpolations', () => {
        expect(rust('f(a ? b : c, d);')).toContain('f(if a { b } else { c }, d)');
        expect(rust('var c = arr[flag ? 0 : 1];')).toContain(
            'arr[(if flag { 0.0 } else { 1.0 }) as usize]'
        );
        expect(rust('var s = $"{(flag ? a : b)}";')).toContain(
            'format!("{}", (if flag { a } else { b }))'
        );
    });

    it('does not report a diagnostic for a supported ternary', () => {
        expect(transpileCSharpToRust('var c = flag ? a : b;').diagnostics).toHaveLength(0);
    });

    it('routes a held sprite through the storyboard by handle', () => {
        const code = rust('var bg = GetLayer("").CreateSprite(p, o); bg.Fade(0, 100, 0, 1);');
        expect(code).toContain('let mut bg = rt.storyboard.create_sprite("", p, o)');
        expect(code).toContain('rt.storyboard.fade(bg, 0.0, 100.0, 0.0, 1.0, Easing::None)');
    });

    it('routes a held layer and its sprites through the storyboard', () => {
        const code = rust(
            'var layer = GetLayer("x"); var bar = layer.CreateSprite(p, o, pos); bar.Scale(t, 2);'
        );
        expect(code).toContain('let mut layer = rt.storyboard.layer_handle("x")');
        expect(code).toContain('let mut bar = rt.storyboard.create_sprite_at(layer, p, o, pos)');
        expect(code).toContain('rt.storyboard.scale(bar, t, t, 2.0, 2.0, Easing::None)');
    });

    it('routes sprite reads through the shared accessor', () => {
        const code = rust('var bg = GetLayer("").CreateSprite(p, o); var s = bg.ScaleAt(t);');
        expect(code).toContain('rt.storyboard.sprite(bg).scale_at(t)');
    });
});
