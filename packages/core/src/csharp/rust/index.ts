import type { Diagnostic } from '@tochii/osb-toolkit-shared';

import type { Token, TranspileOptions, TranspileResult } from '../../types/csharp.js';
import { normalizeTypeName } from '../compat.js';
import { TYPE_NAMES } from '../known.js';
import { tokenizeCSharp } from '../tokenizer/index.js';
import { COMMAND_METHODS, type CommandArg, normalizeCommandCall } from './commands.js';
import {
    API_MAP,
    COLOR_NAMES,
    ENUM_MEMBER_MAP,
    ENUM_TYPES,
    IS_TYPE_MAP,
    MATH_HELPER_MAP,
    MATH_MAP,
    MATRIX_STATIC_MAP,
    METHOD_MAP,
    NUMERIC_TYPES,
    PROPERTY_MAP,
    STATEMENT_BOUNDARY,
    TYPE_MAP,
    UNSUPPORTED_API,
} from './maps.js';
import { isAssignmentTarget, rustCastType, rustSnakeCase, screamingSnake } from './names.js';
import { convertString } from './strings.js';

export { rustSnakeCase } from './names.js';

interface SigEntry {
    token: Token;
    index: number;
}

/** A C# ternary expression `cond ? a : b` located in the significant-token array. */
interface TernarySpan {
    /** Significant-token offset of the condition's first token. */
    start: number;
    /** Offset of the `?`. */
    question: number;
    /** Offset of the matching `:`. */
    colon: number;
    /** Offset of the last token of the false branch. */
    end: number;
}

/** Tokens that end a ternary condition when scanning left from the `?`. */
const TERNARY_LEFT_BOUNDARY = new Set(['=', ';', '{', '}', ',', ':', '?', '=>']);

/** Sprite methods that only read state; routed through the shared accessor. */
const SPRITE_READ_METHODS = new Set([
    'PositionAt',
    'ScaleAt',
    'RotationAt',
    'OpacityAt',
    'ColorAt',
    'AdditiveAt',
]);

/**
 * Transpiles a C# effect body to Rust that calls the `osb-runtime` API.
 *
 * The transpiler is a token-rewriting pass over the shared C# tokenizer, so it
 * stays close to the original C#->JS backend and shares its diagnostics model.
 */
export function transpileCSharpToRust(
    source: string,
    options: TranspileOptions = {}
): TranspileResult {
    const tokens = tokenizeCSharp(source, options.file);
    const diagnostics: Diagnostic[] = [];
    const sig: SigEntry[] = tokens
        .map((token, index) => ({ token, index }))
        .filter(
            entry =>
                entry.token.type !== 'whitespace' &&
                entry.token.type !== 'newline' &&
                entry.token.type !== 'comment'
        );

    const emitDeclarations = options.declarations ?? true;
    const ternaries = findTernaries();
    const resources = findResourceVars();
    const spriteVars = new Set<string>([...(options.spriteNames ?? []), ...resources.sprites]);
    const layerVars = new Set<string>([...(options.layerNames ?? []), ...resources.layers]);
    let out = '';
    let i = 0;
    let sigPtr = 0;
    // Set when a sprite-handle variable is seen immediately before a `.`; the
    // following member decides how to route through the storyboard.
    let pendingSpriteReceiver: string | null = null;

    const at = (offset: number): Token | undefined => sig[sigPtr + offset]?.token;
    const prevSigValue = (): string | undefined => sig[sigPtr - 1]?.token.value;

    const consumeThrough = (offset: number): void => {
        const fullIndex = sig[sigPtr + offset]?.index;
        i = fullIndex === undefined ? tokens.length : fullIndex + 1;
    };

    while (i < tokens.length) {
        while (sigPtr < sig.length && (sig[sigPtr] as SigEntry).index < i) sigPtr += 1;
        const token = tokens[i] as Token;

        if (token.type === 'whitespace' || token.type === 'newline') {
            out += token.value;
            i += 1;
            continue;
        }
        if (token.type === 'comment') {
            out += token.value.startsWith('///') ? token.value.replace('///', '//') : token.value;
            i += 1;
            continue;
        }

        // Ternary expressions are buffered: the whole `cond ? a : b` is emitted
        // as an `if/else` expression the moment its condition starts, before any
        // of its tokens reach the streaming rewriter (see `findTernaries`).
        const ternary = ternaries.get(sigPtr);
        if (ternary) {
            out += rewriteTernary(ternary);
            consumeThrough(ternary.end - sigPtr);
            continue;
        }

        const rewritten = tryRewrite();
        if (rewritten) {
            out += rewritten.text;
            if (rewritten.consume !== undefined) {
                consumeThrough(rewritten.consume);
            } else {
                i += 1;
            }
            continue;
        }

        out += emitToken(token);
        i += 1;
    }

    return { code: out, diagnostics };

    function tryRewrite(): { text: string; consume?: number } | null {
        const first = at(0);
        if (!first) return null;
        const value = first.value;

        // A sprite-handle variable's member access: commands and group calls go
        // through `Storyboard` forwarders that take the handle (so a command whose
        // argument reads the same sprite borrows the storyboard only as the
        // receiver), and reads go through the shared `sprite(..)` accessor.
        if (value === '.' && pendingSpriteReceiver !== null) {
            const receiver = pendingSpriteReceiver;
            pendingSpriteReceiver = null;
            const member = at(1)?.value ?? '';
            if (
                (COMMAND_METHODS.has(member) ||
                    member === 'StartLoopGroup' ||
                    member === 'EndGroup' ||
                    member === 'StartTriggerGroup') &&
                at(2)?.value === '('
            ) {
                const close = matchBracket(2);
                if (close >= 0) {
                    const forwarded = rewriteForwardedCommand(receiver, member, close);
                    if (forwarded) return forwarded;
                }
            }
            const accessor = SPRITE_READ_METHODS.has(member) ? 'sprite' : 'sprite_mut';
            const isCall = at(2)?.value === '(';
            const mapped = isCall
                ? (METHOD_MAP[member] ?? member)
                : (PROPERTY_MAP[member] ?? member);
            return { text: `rt.storyboard.${accessor}(${receiver}).${mapped}`, consume: 1 };
        }

        // Shared-ownership handles: a sprite variable's method calls go through
        // `rt.storyboard.sprite_mut(..)` (no borrow held), and a layer variable's
        // `Create*` calls go through the storyboard. This lets a helper hold a
        // sprite handle while it also calls `GetLayer`/`Random` on `rt`.
        if (first.type === 'identifier') {
            if (spriteVars.has(value) && at(1)?.value === '.') {
                pendingSpriteReceiver = value;
                return { text: '', consume: 0 };
            }
            if (
                layerVars.has(value) &&
                at(1)?.value === '.' &&
                at(2)?.type === 'identifier' &&
                at(3)?.value === '('
            ) {
                const rewritten = rewriteCollectionCreate(value, at(2)!.value, 3);
                if (rewritten) return rewritten;
            }
        }

        // External library static call: `HelperClass.Distance(...)` -> `self.distance(...)`.
        if (first.type === 'identifier' && at(1)?.value === '.' && at(2)?.type === 'identifier') {
            const library = options.libraryMethods?.[`${value}.${at(2)!.value}`];
            if (library !== undefined && at(3)?.value === '(') {
                const needsRt =
                    options.rtLibraryMethods?.includes(`${value}.${at(2)!.value}`) ?? false;
                if (!needsRt) return { text: `self.${library}`, consume: 2 };
                const empty = at(4)?.value === ')';
                return { text: empty ? `self.${library}(rt` : `self.${library}(rt, `, consume: 3 };
            }
        }
        // A `[Configurable]` reference becomes `self.<field>`; a call to a user
        // helper method becomes `self.<method>(...)`. `names` (bundling) prefixes
        // both per script and takes precedence over the plain name lists.
        if (first.type === 'identifier') {
            const mapped = options.names?.[value];
            const isHelperCall =
                at(1)?.value === '(' &&
                (mapped !== undefined || options.helperNames?.includes(value));
            if (isHelperCall) {
                const name = `self.${mapped ?? rustSnakeCase(value)}`;
                const needsRt = options.rtHelpers?.includes(value) ?? false;
                if (!needsRt) return { text: name, consume: 0 };
                const empty = at(2)?.value === ')';
                return { text: empty ? `${name}(rt` : `${name}(rt, `, consume: 1 };
            }
            if (mapped !== undefined || options.configurableNames?.includes(value)) {
                const field = `self.${mapped ?? rustSnakeCase(value)}`;
                const next = at(1)?.value;
                return { text: isAssignmentTarget(next) ? field : `${field}.clone()`, consume: 0 };
            }
        }

        if (value === '?.') {
            const conditional = rewriteNullConditional();
            if (conditional) return conditional;
        }
        if (value === '??') {
            const coalesce = rewriteNullCoalesce();
            if (coalesce) return coalesce;
        }
        // `?.`/`??` are separate tokens; a bare `?` is the C# ternary operator.
        if (value === '?') {
            diagnostics.push({
                severity: 'warning',
                message:
                    "C# ternary operator '?:' is not supported yet; rewrite as an if/else expression",
                code: 'OSB_UNSUPPORTED_CONSTRUCT',
                line: first.line,
                column: first.column,
            });
            return null;
        }
        // `x == null` / `x != null` become `Option` predicates.
        if ((value === '==' || value === '!=') && at(1)?.value === 'null') {
            return { text: value === '!=' ? '.is_some()' : '.is_none()', consume: 1 };
        }
        // `x is Type` becomes a runtime predicate on the hit-object kind.
        if (value === 'is' && at(1)?.type === 'identifier' && IS_TYPE_MAP[at(1)!.value]) {
            return { text: `.${IS_TYPE_MAP[at(1)!.value]}()`, consume: 1 };
        }
        if (value === '.' && at(1)?.value === 'CreateSprite' && at(2)?.value === '(') {
            const sprite = rewriteCreateSprite();
            if (sprite) return sprite;
        }
        // `First()` / `Last()` throw on empty in C#; unwrap the Option.
        if (
            value === '.' &&
            (at(1)?.value === 'First' || at(1)?.value === 'Last') &&
            at(2)?.value === '('
        ) {
            const close = matchBracket(2);
            if (close === 3) {
                const method = at(1)!.value === 'First' ? 'first' : 'last';
                return { text: `.${method}().unwrap()`, consume: close };
            }
        }
        if (value === '.' && COMMAND_METHODS.has(at(1)?.value ?? '')) {
            const command = rewriteCommand();
            if (command) return command;
        }
        if (value === 'if') {
            const branch = rewriteIfAt(0);
            if (branch) return { text: branch.text, consume: branch.end };
        }

        if (first.type === 'identifier') {
            if (value === 'GetLayer' && at(1)?.value === '(') {
                const getLayer = rewriteGetLayer();
                if (getLayer) return getLayer;
            }
            if (value === 'GetMapsetBitmap' && at(1)?.value === '(') {
                const bitmap = rewriteMapsetBitmap();
                if (bitmap) return bitmap;
            }
            // `CancellationToken.ThrowIfCancellationRequested()` has no analogue.
            if (
                value === 'CancellationToken' &&
                at(1)?.value === '.' &&
                at(2)?.value === 'ThrowIfCancellationRequested' &&
                at(3)?.value === '('
            ) {
                const close = matchBracket(3);
                if (close >= 0) return { text: '', consume: close };
            }
            if (value === 'Random' && at(1)?.value === '(') {
                const random = rewriteRandom();
                if (random) return random;
            }
            if (
                (value === 'CommandColor' || value === 'Color4') &&
                at(1)?.value === '.' &&
                ['FromHsb', 'FromRgb', 'FromHsl', 'ToHsl'].includes(at(2)?.value ?? '') &&
                at(3)?.value === '('
            ) {
                const factory = rewriteColorFactory(at(2)!.value);
                if (factory) return factory;
            }
            if (
                (value === 'Vector2' || value === 'Vector3' || value === 'Vector4') &&
                at(1)?.value === '.'
            ) {
                const vector = rewriteVectorStatic(value);
                if (vector) return vector;
            }
            if (value === 'OsbSprite' && at(1)?.value === '.') {
                const sprite = rewriteSpriteStatic();
                if (sprite) return sprite;
            }
            if (value === 'Matrix4' && at(1)?.value === '.') {
                const matrix = rewriteMatrixStatic();
                if (matrix) return matrix;
            }
            if (value === 'Beatmap' && at(1)?.value === '.') {
                return { text: 'rt.context.beatmap().unwrap()', consume: 0 };
            }
            if (value === 'AudioDuration') {
                return { text: 'rt.context.audio_duration()', consume: 0 };
            }
            if (value === 'Color4' && at(1)?.value === '.' && COLOR_NAMES.has(at(2)?.value ?? '')) {
                return { text: `Color4::${screamingSnake(at(2)!.value)}`, consume: 2 };
            }
            if (
                value === 'MathHelper' &&
                at(1)?.value === '.' &&
                MATH_HELPER_MAP[at(2)?.value ?? '']
            ) {
                return { text: `osb_runtime::math::${MATH_HELPER_MAP[at(2)!.value]}`, consume: 2 };
            }
        }

        if (value === 'foreach') return rewriteForeach();
        if (value === 'for') {
            const loop = rewriteFor();
            if (loop) return loop;
        }
        if (value === 'this') return { text: 'self', consume: 0 };
        if (value === 'null') return { text: 'None', consume: 0 };
        if (value === 'new') return rewriteNew();
        if (value === 'Math') return rewriteMath();
        if (value === 'string' && at(1)?.value === '.' && at(2)?.value === 'Empty') {
            return { text: 'String::new()', consume: 2 };
        }
        if (value === 'ref' || value === 'out') {
            const next = at(1);
            if (next && (next.type === 'identifier' || next.type === 'keyword')) {
                return { text: '', consume: 0 };
            }
        }
        if (value === '(') {
            const lambda = rewriteLambdaParams();
            if (lambda) return lambda;
            const cast = rewriteCast();
            if (cast) return cast;
        }
        // Indexing: `expr[index]` casts the index to `usize`.
        if (value === '[') {
            const index = rewriteSubscript();
            if (index) return index;
        }
        if (value === 'const' && at(1)?.type === 'keyword' && at(2)?.type === 'identifier') {
            return { text: 'let', consume: 1 };
        }

        if (first.type === 'identifier') {
            const mappedType = TYPE_MAP[value];
            const isEnum = ENUM_TYPES.has(value);
            if (isEnum && at(1)?.value === '.' && at(2)?.type === 'identifier') {
                const member = ENUM_MEMBER_MAP[at(2)!.value] ?? at(2)!.value;
                const typeName = value === 'OsbEasing' || value === 'Easing' ? 'Easing' : value;
                return { text: `${typeName}::${member}`, consume: 2 };
            }
            if (mappedType && at(1)?.value !== '.') {
                return { text: mappedType, consume: 0 };
            }
            if (mappedType && at(1)?.value === '.' && at(2)?.type === 'identifier') {
                const member = ENUM_MEMBER_MAP[at(2)!.value] ?? at(2)!.value;
                return { text: `${mappedType}::${member}`, consume: 2 };
            }
        }

        if (value === '.' && at(1)?.type === 'identifier') {
            const member = at(1)!.value;
            if (at(2)?.value === '(' && METHOD_MAP[member]) {
                return { text: `.${METHOD_MAP[member]}`, consume: 1 };
            }
            if (at(2)?.value !== '(' && PROPERTY_MAP[member]) {
                const property = PROPERTY_MAP[member];
                return { text: property === 'len' ? '.len()' : `.${property}`, consume: 1 };
            }
        }

        // Lambda: `x => ...` and `(a, b) => ...` become Rust closures.
        if (first.type === 'identifier' && at(1)?.value === '=>') {
            return { text: `|${value}| `, consume: 0 };
        }
        if (value === '=>') {
            return { text: ' ', consume: 0 };
        }

        if (first.type === 'identifier' && API_MAP[value]) {
            const next = at(1);
            if (next?.value === '(' || next?.value === '.' || next?.value === ';') {
                return { text: API_MAP[value] as string, consume: 0 };
            }
        }
        if (value === 'GetBeatmap' && at(1)?.value === '(') {
            return { text: 'rt.beatmap().unwrap()', consume: 1 };
        }
        if (first.type === 'identifier' && UNSUPPORTED_API.has(value) && at(1)?.value === '(') {
            diagnostics.push({
                severity: 'warning',
                message: `Generator API '${value}' is not yet mapped to the Rust runtime`,
                code: 'OSB_UNSUPPORTED_API',
                line: first.line,
                column: first.column,
            });
        }

        if (emitDeclarations) {
            const declaration = rewriteDeclaration();
            if (declaration) return declaration;
        }

        if (
            value === 'switch' ||
            value === 'try' ||
            value === 'lock' ||
            value === 'unsafe' ||
            value === 'fixed' ||
            value === 'goto' ||
            value === 'using'
        ) {
            diagnostics.push({
                severity: 'warning',
                message: `Unsupported C# construct '${value}' was passed through unchanged`,
                code: 'OSB_UNSUPPORTED_CONSTRUCT',
                line: first.line,
                column: first.column,
            });
        }

        return null;
    }

    function rewriteForeach(): { text: string; consume: number } | null {
        if (at(1)?.value !== '(') return null;
        const closeOffset = matchBracket(1);
        if (closeOffset < 0) return null;

        let inOffset = -1;
        let depth = 0;
        for (let offset = 2; offset < closeOffset; offset += 1) {
            const v = at(offset)?.value;
            if (v === '(' || v === '[' || v === '<') depth += 1;
            else if (v === ')' || v === ']' || v === '>') depth -= 1;
            else if (v === 'in' && depth === 0) {
                inOffset = offset;
                break;
            }
        }
        if (inOffset === -1) return null;

        const nameToken = at(inOffset - 1);
        if (!nameToken || (nameToken.type !== 'identifier' && nameToken.type !== 'keyword')) {
            return null;
        }
        const exprStartFull = sig[sigPtr + inOffset + 1]?.index ?? 0;
        const exprEndFull = sig[sigPtr + closeOffset - 1]?.index ?? 0;
        const exprText = tokens
            .slice(exprStartFull, exprEndFull + 1)
            .map(item => item.value)
            .join('');
        const exprCode = transpileFragment(exprText);

        const bodyOpen = closeOffset + 1;
        if (at(bodyOpen)?.value !== '{') {
            diagnostics.push({
                severity: 'warning',
                message: 'foreach without a block body was left unchanged',
                code: 'OSB_UNSUPPORTED_CONSTRUCT',
                line: nameToken.line,
            });
            return null;
        }
        const bodyClose = matchBracket(bodyOpen);
        if (bodyClose < 0) return null;

        const openFull = sig[sigPtr + bodyOpen]?.index ?? 0;
        const closeFull = sig[sigPtr + bodyClose]?.index ?? 0;
        const innerText = tokens
            .slice(openFull + 1, closeFull)
            .map(item => item.value)
            .join('');
        const innerCode = transpileFragment(innerText);

        const indented = innerCode
            .split('\n')
            .map(line => (line.trim() ? `    ${line}` : line))
            .join('\n');
        const text = `for ${nameToken.value} in (${exprCode}).iter() {\n${indented}\n}`;
        return { text, consume: bodyClose };
    }

    /** Maps a `KeyframedValue<T>` type argument to the runtime type. */
    function rustKeyframeType(csType: string): string {
        switch (csType) {
            case 'Vector2':
                return 'Vec2';
            case 'Vector3':
                return 'Vec3';
            case 'Color4':
            case 'CommandColor':
                return 'Color4';
            default:
                return 'f64';
        }
    }

    /**
     * Reads a possibly namespace-qualified type name and resolves aliases, so
     * `osuTK.Vector2` and `OpenTK.Vector2` both yield `Vector2`.
     */
    function readQualifiedTypeName(
        startOffset: number
    ): { name: string; endOffset: number } | null {
        const first = at(startOffset);
        if (!first || (first.type !== 'identifier' && first.type !== 'keyword')) return null;
        let offset = startOffset;
        let qualified = first.value;
        while (at(offset + 1)?.value === '.' && at(offset + 2)?.type === 'identifier') {
            qualified += `.${at(offset + 2)!.value}`;
            offset += 2;
        }
        return { name: normalizeTypeName(qualified), endOffset: offset };
    }

    function rewriteNew(): { text: string; consume: number } | null {
        const qualified = readQualifiedTypeName(1);
        if (!qualified) return null;
        const { name: typeName, endOffset } = qualified;
        const mapped = TYPE_MAP[typeName] ?? typeName;
        const after = endOffset + 1;

        if (typeName === 'Vector2' || typeName === 'Vector3' || typeName === 'Vector4') {
            return { text: `${mapped}::new`, consume: endOffset };
        }
        if (typeName === 'Color4' || typeName === 'CommandColor') {
            return { text: 'Color4::new', consume: endOffset };
        }
        if (typeName === 'KeyframedValue' && at(after)?.value === '<') {
            const closeGeneric = matchBracket(after);
            if (closeGeneric < 0) return null;
            const inner = rustKeyframeType(at(after + 1)?.value ?? 'float');
            const tail = at(closeGeneric + 1)?.value;
            // `new KeyframedValue<T>(null)` -> an empty keyframed value.
            if (tail === '(') {
                const close = matchBracket(closeGeneric + 1);
                if (close < 0) return null;
                return { text: `KeyframedValue::<${inner}>::new()`, consume: close };
            }
            // `new KeyframedValue<T>[count]` -> a vector of keyframed values.
            if (tail === '[') {
                const close = matchBracket(closeGeneric + 1);
                if (close < 0) return null;
                const count = transpileRange(closeGeneric + 2, close - 1);
                return {
                    text: `vec![KeyframedValue::<${inner}>::new(); (${count}) as usize]`,
                    consume: close,
                };
            }
            return null;
        }
        if (at(after)?.value === '<') {
            const closeOffset = matchBracket(after);
            if (closeOffset < 0) return null;
            // Generic collections (`List<T>`, `Dictionary<K,V>`) become `Vec`.
            if (at(closeOffset + 1)?.value === '(') {
                return { text: 'Vec::new', consume: closeOffset + 1 };
            }
            return { text: 'Vec::new', consume: closeOffset };
        }
        return null;
    }

    function rewriteMath(): { text: string; consume: number } | null {
        if (at(1)?.value !== '.') return null;
        const member = at(2)?.value;
        if (!member) return null;
        if (member === 'PI') return { text: 'std::f64::consts::PI', consume: 2 };
        // `Math.Round(value, digits)` maps to the runtime's decimal rounding.
        if (member === 'Round' && at(3)?.value === '(') {
            const close = matchBracket(3);
            if (close >= 0) {
                const args = splitArguments(3, close).map(([start, end]) =>
                    transpileRange(start, end)
                );
                if (args.length === 2) {
                    return {
                        text: `osb_runtime::math::round_to(${args[0]}, ${args[1]})`,
                        consume: close,
                    };
                }
            }
        }
        if (MATH_MAP[member]) {
            return { text: `osb_runtime::math::${MATH_MAP[member]}`, consume: 2 };
        }
        return null;
    }

    function rewriteCast(): { text: string; consume: number } | null {
        const typeToken = at(1);
        if (!typeToken || at(2)?.value !== ')') return null;
        const castType = typeToken.value;
        const cast = rustCastType(castType);
        if (!cast) return null;
        const operandStart = 3;
        const operand = at(operandStart);
        if (!operand) return null;
        const endOffset = consumePrimary(operandStart);
        if (endOffset < 0) return null;
        const operandText = rawRange(operandStart, endOffset);
        const inner = transpileFragment(operandText);
        const value = `((${inner}) as ${cast.type})`;
        return { text: cast.truncate ? `${value}.trunc()` : value, consume: endOffset };
    }

    /** Splits a call's arguments at top-level commas, as inclusive `[start, end]` offsets. */
    function splitArguments(openOffset: number, closeOffset: number): Array<[number, number]> {
        const ranges: Array<[number, number]> = [];
        let depth = 0;
        let start = openOffset + 1;
        for (let offset = openOffset + 1; offset < closeOffset; offset += 1) {
            const v = at(offset)?.value;
            if (v === '(' || v === '[') depth += 1;
            else if (v === ')' || v === ']') depth -= 1;
            else if (v === ',' && depth === 0) {
                ranges.push([start, offset - 1]);
                start = offset + 1;
            }
        }
        if (start < closeOffset) ranges.push([start, closeOffset - 1]);
        return ranges;
    }

    function rawRange(startOffset: number, endOffset: number): string {
        const startFull = sig[sigPtr + startOffset]?.index;
        const endFull = sig[sigPtr + endOffset]?.index;
        if (startFull === undefined || endFull === undefined) return '';
        return tokens
            .slice(startFull, endFull + 1)
            .map(item => item.value)
            .join('');
    }

    function transpileRange(startOffset: number, endOffset: number): string {
        return transpileFragment(rawRange(startOffset, endOffset));
    }

    /** Transpiles a fragment, preserving configurable mapping and collecting diagnostics. */
    function transpileFragment(text: string): string {
        const result = transpileCSharpToRust(text, {
            file: options.file,
            configurableNames: options.configurableNames,
            helperNames: options.helperNames,
            names: options.names,
            libraryMethods: options.libraryMethods,
            spriteNames: [...spriteVars],
            layerNames: [...layerVars],
            rtHelpers: options.rtHelpers,
            rtLibraryMethods: options.rtLibraryMethods,
        });
        diagnostics.push(...result.diagnostics);
        return result.code.trim();
    }

    function isEasingArgument(offset: number): boolean {
        const v = at(offset)?.value;
        return v === 'OsbEasing' || v === 'Easing';
    }

    function rewriteCommand(): { text: string; consume: number } | null {
        const nameToken = at(1);
        if (nameToken?.type !== 'identifier') return null;
        if (at(2)?.value !== '(') return null;
        const close = matchBracket(2);
        if (close < 0) return null;
        const args: CommandArg[] = splitArguments(2, close).map(([start, end]) => ({
            text: transpileRange(start, end),
            easing: isEasingArgument(start),
        }));
        const normalized = normalizeCommandCall(nameToken.value, args);
        if (!normalized) {
            diagnostics.push({
                severity: 'warning',
                message: `Unsupported '${nameToken.value}' overload was passed through unchanged`,
                code: 'OSB_UNSUPPORTED_CONSTRUCT',
                line: nameToken.line,
                column: nameToken.column,
            });
            return null;
        }
        return { text: normalized, consume: close };
    }

    /**
     * Emits a sprite command as a `Storyboard` forwarder call taking the handle.
     * [`normalizeCommandCall`] collapses the C# overload and moves the easing
     * last; the handle is then injected as the first argument.
     */
    function rewriteForwardedCommand(
        receiver: string,
        member: string,
        close: number
    ): { text: string; consume: number } | null {
        if (COMMAND_METHODS.has(member)) {
            const args: CommandArg[] = splitArguments(2, close).map(([start, end]) => ({
                text: transpileRange(start, end),
                easing: isEasingArgument(start),
            }));
            const normalized = normalizeCommandCall(member, args);
            if (!normalized) return null;
            const body = normalized.slice(1);
            const open = body.indexOf('(');
            const method = body.slice(0, open);
            const inner = body.slice(open + 1, -1);
            return {
                text: `rt.storyboard.${method}(${receiver}${inner ? `, ${inner}` : ''})`,
                consume: close,
            };
        }
        const method = METHOD_MAP[member];
        if (!method) return null;
        const args = splitArguments(2, close).map(([start, end]) => transpileRange(start, end));
        return {
            text: `rt.storyboard.${method}(${receiver}${args.length ? `, ${args.join(', ')}` : ''})`,
            consume: close,
        };
    }

    /** `CreateSprite(path, origin[, position])` picks the 2- or 3-argument form. */
    function rewriteCreateSprite(): { text: string; consume: number } | null {
        const close = matchBracket(2);
        if (close < 0) return null;
        const args = splitArguments(2, close).map(([start, end]) => transpileRange(start, end));
        if (args.length === 2)
            return { text: `.create_sprite(${args[0]}, ${args[1]})`, consume: close };
        if (args.length === 3)
            return {
                text: `.create_sprite_at(${args[0]}, ${args[1]}, ${args[2]})`,
                consume: close,
            };
        return null;
    }

    /**
     * `GetLayer(name)` becomes a non-borrowing layer handle; when it is
     * immediately followed by `CreateSprite`/`CreateAnimation`, the whole chain
     * becomes a single `Storyboard::create_*` call so the storyboard is not
     * borrowed twice in one expression.
     */
    function rewriteGetLayer(): { text: string; consume: number } | null {
        const close = matchBracket(1);
        if (close < 0) return null;
        const memberOffset = close + 1;
        if (
            at(memberOffset)?.value === '.' &&
            at(memberOffset + 2)?.value === '(' &&
            ['CreateSprite', 'CreateAnimation', 'CreateSample'].includes(
                at(memberOffset + 1)?.value ?? ''
            )
        ) {
            const receiver = transpileRange(2, close - 1);
            const rewritten = rewriteCollectionCreate(
                receiver,
                at(memberOffset + 1)!.value,
                memberOffset + 2
            );
            if (rewritten) return rewritten;
        }
        return { text: 'rt.storyboard.layer_handle', consume: 0 };
    }

    /**
     * Emits a layer's `CreateSprite`/`CreateAnimation`/`CreateSample` as a
     * `Storyboard` call returning a handle. `receiver` is the already-transpiled
     * layer expression (a `LayerHandle` variable or a `GetLayer` argument).
     */
    function rewriteCollectionCreate(
        receiver: string,
        member: string,
        openOffset: number
    ): { text: string; consume: number } | null {
        const close = matchBracket(openOffset);
        if (close < 0) return null;
        const args = splitArguments(openOffset, close).map(([start, end]) =>
            transpileRange(start, end)
        );
        if (member === 'CreateSprite') {
            if (args.length === 2)
                return {
                    text: `rt.storyboard.create_sprite(${receiver}, ${args[0]}, ${args[1]})`,
                    consume: close,
                };
            if (args.length === 3)
                return {
                    text: `rt.storyboard.create_sprite_at(${receiver}, ${args[0]}, ${args[1]}, ${args[2]})`,
                    consume: close,
                };
            return null;
        }
        if (member === 'CreateAnimation' && args.length === 6) {
            return {
                text: `rt.storyboard.create_animation(${receiver}, ${args.join(', ')})`,
                consume: close,
            };
        }
        return null;
    }

    /**
     * Classifies local variables that hold layer/sprite handles, so their method
     * calls can be routed through the storyboard by handle (no long-lived borrow).
     * A variable assigned from a `CreateSprite`/`CreateAnimation` call holds a
     * sprite handle; one assigned directly from `GetLayer` holds a layer handle.
     */
    function findResourceVars(): { sprites: Set<string>; layers: Set<string> } {
        const sprites = new Set<string>();
        const layers = new Set<string>();
        for (let offset = 0; offset < sig.length; offset += 1) {
            if (sig[offset]?.token.value !== '=') continue;
            const prev = sig[offset - 1]?.token;
            if (!prev || (prev.type !== 'identifier' && prev.type !== 'keyword')) continue;

            let end = offset + 1;
            let depth = 0;
            while (end < sig.length) {
                const v = sig[end]?.token.value;
                if (v === '(' || v === '[' || v === '{') depth += 1;
                else if (v === ')' || v === ']' || v === '}') {
                    if (depth === 0) break;
                    depth -= 1;
                } else if (v === ';' && depth === 0) break;
                end += 1;
            }

            let creates = false;
            for (let o = offset + 1; o < end; o += 1) {
                const v = sig[o]?.token.value;
                if (
                    (v === 'CreateSprite' || v === 'CreateAnimation') &&
                    sig[o + 1]?.token.value === '('
                ) {
                    creates = true;
                    break;
                }
            }
            if (creates) {
                sprites.add(prev.value);
            } else if (
                sig[offset + 1]?.token.value === 'GetLayer' &&
                sig[offset + 2]?.token.value === '('
            ) {
                layers.add(prev.value);
            }
        }
        return { sprites, layers };
    }

    function rewriteMapsetBitmap(): { text: string; consume: number } | null {
        const close = matchBracket(1);
        if (close < 0 || close === 2) return null;
        const arg = transpileRange(2, close - 1);
        return { text: `rt.context.get_mapset_bitmap(&${arg})`, consume: close };
    }

    function rewriteColorFactory(method: string): { text: string; consume: number } | null {
        const close = matchBracket(3);
        if (close < 0) return null;
        const args = splitArguments(3, close).map(([start, end]) => transpileRange(start, end));
        if (method === 'FromRgb' && args.length === 3) {
            return { text: `Color4::from_rgb(${args.join(', ')})`, consume: close };
        }
        if (method === 'FromHsb' && args.length === 3) {
            return { text: `Color4::from_hsb(${args.join(', ')}, 1.0)`, consume: close };
        }
        if (method === 'FromHsb' && args.length === 4) {
            return { text: `Color4::from_hsb(${args.join(', ')})`, consume: close };
        }
        if (method === 'FromHsl' && args.length === 1) {
            return { text: `Color4::from_hsl(${args[0]})`, consume: close };
        }
        if (method === 'ToHsl' && args.length === 1) {
            return { text: `Color4::to_hsl(${args[0]})`, consume: close };
        }
        return null;
    }

    /** `Random(max)` is `Random(0, max)`; both forms map to the context RNG. */
    function rewriteRandom(): { text: string; consume: number } | null {
        const close = matchBracket(1);
        if (close < 0) return null;
        const args = splitArguments(1, close).map(([start, end]) => transpileRange(start, end));
        if (args.length === 1)
            return { text: `rt.context.random(0.0, ${args[0]})`, consume: close };
        if (args.length === 2)
            return { text: `rt.context.random(${args[0]}, ${args[1]})`, consume: close };
        return null;
    }

    function rewriteVectorStatic(typeName: string): { text: string; consume: number } | null {
        const member = at(2)?.value;
        if (at(1)?.value !== '.' || !member) return null;
        const mapped = typeName === 'Vector2' ? 'Vec2' : typeName === 'Vector3' ? 'Vec3' : 'Vec4';
        if (member === 'One') return { text: `${mapped}::ONE`, consume: 2 };
        if (member === 'Zero') return { text: `${mapped}::ZERO`, consume: 2 };
        if (member === 'Lerp' && at(3)?.value === '(') {
            const close = matchBracket(3);
            if (close < 0) return null;
            const args = splitArguments(3, close).map(([start, end]) => transpileRange(start, end));
            if (args.length === 3) {
                return { text: `(${args[0]}).lerp(${args[1]}, ${args[2]})`, consume: close };
            }
        }
        return null;
    }

    function rewriteSpriteStatic(): { text: string; consume: number } | null {
        const member = at(2)?.value;
        if (at(1)?.value !== '.' || !member || at(3)?.value !== '(') return null;
        const close = matchBracket(3);
        if (close < 0) return null;
        const args = splitArguments(3, close).map(([start, end]) => transpileRange(start, end));
        if (member === 'GetOriginVector' && args.length === 3) {
            return { text: `OsbSprite::get_origin_vector(${args.join(', ')})`, consume: close };
        }
        if (member === 'InScreenBounds' && args.length === 4) {
            return { text: `OsbSprite::in_screen_bounds(${args.join(', ')})`, consume: close };
        }
        return null;
    }

    function rewriteMatrixStatic(): { text: string; consume: number } | null {
        const member = at(2)?.value;
        if (at(1)?.value !== '.' || !member || !MATRIX_STATIC_MAP[member]) return null;
        if (at(3)?.value !== '(') return null;
        const close = matchBracket(3);
        if (close < 0) return null;
        const args = splitArguments(3, close).map(([start, end]) => transpileRange(start, end));
        return {
            text: `Matrix4::${MATRIX_STATIC_MAP[member]}(${args.join(', ')})`,
            consume: close,
        };
    }

    function rewriteNullConditional(): { text: string; consume: number } | null {
        const endOffset = consumePrimary(1);
        if (endOffset < 0) return null;
        const chain = transpileFragment(`__osb.${rawRange(1, endOffset)}`);
        return { text: `.map(|__osb| ${chain})`, consume: endOffset };
    }

    function rewriteNullCoalesce(): { text: string; consume: number } | null {
        const endOffset = consumePrimary(1);
        if (endOffset < 0) return null;
        const fallback = transpileRange(1, endOffset);
        return { text: `.clone().unwrap_or(${fallback})`, consume: endOffset };
    }

    /** Finds the `;` ending a statement that starts at `startOffset` (depth 0). */
    function findStatementEnd(startOffset: number): number {
        let depth = 0;
        for (let offset = startOffset; offset < sig.length - sigPtr; offset += 1) {
            const v = at(offset)?.value;
            if (v === '(' || v === '[' || v === '{') depth += 1;
            else if (v === ')' || v === ']' || v === '}') depth -= 1;
            else if (v === ';' && depth === 0) return offset;
            else if (depth < 0) return -1;
        }
        return -1;
    }

    /**
     * Rewrites a C# `for (init; cond; step)` into a block with a `let` init and a
     * `while` loop. C# scalars are `f64` in the generated code, so `i++`/`i--`
     * become `i += 1.0` / `i -= 1.0`.
     */
    function rewriteFor(): { text: string; consume: number } | null {
        if (at(1)?.value !== '(') return null;
        const close = matchBracket(1);
        if (close < 0) return null;

        const sections: Array<[number, number]> = [];
        let depth = 0;
        let start = 2;
        for (let offset = 2; offset < close; offset += 1) {
            const v = at(offset)?.value;
            if (v === '(' || v === '[' || v === '{') depth += 1;
            else if (v === ')' || v === ']' || v === '}') depth -= 1;
            else if (v === ';' && depth === 0) {
                sections.push([start, offset - 1]);
                start = offset + 1;
            }
        }
        sections.push([start, close - 1]);
        if (sections.length !== 3) return null;

        const init = transpileRange(sections[0]![0], sections[0]![1]);
        const condition = transpileRange(sections[1]![0], sections[1]![1]);
        const step = rewriteForStep(sections[2]![0], sections[2]![1]);
        if (!step) return null;

        const bodyStart = close + 1;
        let body: string;
        let end: number;
        if (at(bodyStart)?.value === '{') {
            const bodyClose = matchBracket(bodyStart);
            if (bodyClose < 0) return null;
            body = transpileRange(bodyStart + 1, bodyClose - 1);
            end = bodyClose;
        } else {
            const statementEnd = findStatementEnd(bodyStart);
            if (statementEnd < 0) return null;
            body = transpileRange(bodyStart, statementEnd);
            end = statementEnd;
        }

        const indented = indentLines(body, '        ');
        const text = `{\n    ${init};\n    while ${condition} {\n${indented}\n        ${step};\n    }\n}`;
        return { text, consume: end };
    }

    function rewriteForStep(start: number, end: number): string | null {
        const next = at(start + 1)?.value;
        if (start + 1 === end && (next === '++' || next === '--')) {
            const target = transpileRange(start, start);
            return next === '++' ? `${target} += 1.0` : `${target} -= 1.0`;
        }
        return transpileRange(start, end);
    }

    /** `(a, b) =>` lambda parameter lists become Rust closure parameters. */
    function rewriteLambdaParams(): { text: string; consume: number } | null {
        const close = matchBracket(0);
        if (close < 0 || at(close + 1)?.value !== '=>') return null;
        const params = splitArguments(0, close).map(([start, end]) => transpileRange(start, end));
        return { text: `|${params.join(', ')}|`, consume: close };
    }

    /**
     * `expr[index]` casts the index expression to `usize`.
     *
     * C# array elements are references; a value use (not an assignment target and
     * not the receiver of a method/field access) clones, so `var x = arr[i]` gets
     * an owned copy while `arr[i].push(..)` mutates in place.
     */
    function rewriteSubscript(): { text: string; consume: number } | null {
        const previous = sig[sigPtr - 1]?.token;
        if (!previous) return null;
        const isReceiver =
            previous.type === 'identifier' || previous.value === ']' || previous.value === ')';
        if (!isReceiver) return null;
        const close = matchBracket(0);
        if (close < 0) return null;
        const index = transpileRange(1, close - 1);
        const next = at(close + 1)?.value;
        const isTarget = next === '.' || (next !== undefined && isAssignmentTarget(next));
        const suffix = isTarget ? '' : '.clone()';
        return { text: `[(${index}) as usize]${suffix}`, consume: close };
    }

    function indentLines(text: string, prefix: string): string {
        return text
            .split('\n')
            .map(line => (line.trim() ? `${prefix}${line}` : line))
            .join('\n');
    }

    /**
     * Finds every C# ternary `cond ? a : b` in the significant-token array.
     *
     * The `?` may nest, so the matching `:` is found by tracking `?`/`:` nesting
     * in addition to bracket depth, and the false branch runs to the next
     * statement/argument boundary. Spans are keyed by their condition start so
     * the emit loop can replace the whole expression before its tokens stream.
     */
    function findTernaries(): Map<number, TernarySpan> {
        const spans = new Map<number, TernarySpan>();
        for (let offset = 0; offset < sig.length; offset += 1) {
            if (sig[offset]?.token.value !== '?') continue;
            const colon = matchTernaryColon(offset);
            if (colon < 0) continue;
            const start = findTernaryConditionStart(offset);
            const end = findTernaryFalseBranchEnd(colon);
            if (end < colon || spans.has(start)) continue;
            spans.set(start, { start, question: offset, colon, end });
        }
        return spans;
    }

    /** Finds the `:` that matches the ternary `?` at `question`. */
    function matchTernaryColon(question: number): number {
        let depth = 0;
        let level = 0;
        for (let offset = question; offset < sig.length; offset += 1) {
            const value = sig[offset]?.token.value;
            if (value === undefined) break;
            if (value === '(' || value === '[' || value === '{') depth += 1;
            else if (value === ')' || value === ']' || value === '}') {
                if (depth === 0) return -1;
                depth -= 1;
            } else if (depth === 0) {
                if (value === ';') return -1;
                if (value === '?') level += 1;
                else if (value === ':') {
                    level -= 1;
                    if (level === 0) return offset;
                }
            }
        }
        return -1;
    }

    /** Scans left from the `?` to the start of its condition expression. */
    function findTernaryConditionStart(question: number): number {
        let depth = 0;
        for (let offset = question - 1; offset >= 0; offset -= 1) {
            const value = sig[offset]?.token.value;
            if (value === ')' || value === ']' || value === '}') depth += 1;
            else if (value === '(' || value === '[' || value === '{') {
                if (depth === 0) return offset + 1;
                depth -= 1;
            } else if (depth === 0 && value !== undefined) {
                if (TERNARY_LEFT_BOUNDARY.has(value) || value === 'return') return offset + 1;
            }
        }
        return 0;
    }

    /** Scans right from the `:` to the last token of the false branch. */
    function findTernaryFalseBranchEnd(colon: number): number {
        let depth = 0;
        let last = colon;
        for (let offset = colon + 1; offset < sig.length; offset += 1) {
            const value = sig[offset]?.token.value;
            if (value === undefined) break;
            if (value === '(' || value === '[' || value === '{') depth += 1;
            else if (value === ')' || value === ']' || value === '}') {
                if (depth === 0) break;
                depth -= 1;
            } else if (depth === 0 && (value === ';' || value === ',' || value === '=>')) break;
            last = offset;
        }
        return last;
    }

    /** Emits a buffered ternary as a Rust `if/else` expression. */
    function rewriteTernary(span: TernarySpan): string {
        const start = span.start - sigPtr;
        const question = span.question - sigPtr;
        const colon = span.colon - sigPtr;
        const end = span.end - sigPtr;
        const condition = transpileRange(start, question - 1);
        const whenTrue = transpileRange(question + 1, colon - 1);
        const whenFalse = transpileRange(colon + 1, end);
        return `if ${condition} { ${whenTrue} } else { ${whenFalse} }`;
    }

    /**
     * Rewrites an `if`/`else` chain, wrapping brace-less bodies in blocks (Rust
     * requires braces where C# allows a single statement).
     */
    function rewriteIfAt(offset: number): { text: string; end: number } | null {
        if (at(offset)?.value !== 'if' || at(offset + 1)?.value !== '(') return null;
        const condClose = matchBracket(offset + 1);
        if (condClose < 0) return null;
        const condition = transpileRange(offset + 1, condClose);
        const bodyStart = condClose + 1;

        let text: string;
        let end: number;
        if (at(bodyStart)?.value === '{') {
            const close = matchBracket(bodyStart);
            if (close < 0) return null;
            text = `if ${condition} ${transpileRange(bodyStart, close)}`;
            end = close;
        } else {
            const statementEnd = findStatementEnd(bodyStart);
            if (statementEnd < 0) return null;
            text = `if ${condition} {\n    ${transpileRange(bodyStart, statementEnd)}\n}`;
            end = statementEnd;
        }

        if (at(end + 1)?.value === 'else') {
            if (at(end + 2)?.value === 'if') {
                const nested = rewriteIfAt(end + 2);
                if (nested) {
                    text += ` else ${nested.text}`;
                    end = nested.end;
                }
            } else if (at(end + 2)?.value === '{') {
                const close = matchBracket(end + 2);
                if (close >= 0) {
                    text += ` else ${transpileRange(end + 2, close)}`;
                    end = close;
                }
            } else {
                const statementEnd = findStatementEnd(end + 2);
                if (statementEnd >= 0) {
                    text += ` else {\n    ${transpileRange(end + 2, statementEnd)}\n}`;
                    end = statementEnd;
                }
            }
        }
        return { text, end };
    }

    function consumePrimary(startOffset: number): number {
        let offset = startOffset;
        const first = at(offset);
        if (!first) return -1;
        if (first.type === 'number' || first.type === 'string' || first.type === 'char') {
            return offset;
        }
        if (first.value === 'new') {
            offset += 1;
            while (at(offset)?.type === 'identifier' || at(offset)?.value === '.') offset += 1;
        } else if (first.type === 'identifier' || first.type === 'keyword') {
            offset += 1;
        } else if (first.value === '(' || first.value === '[') {
            const close = matchBracket(offset);
            if (close < 0) return -1;
            offset = close + 1;
        } else {
            return -1;
        }
        while (true) {
            const v = at(offset)?.value;
            if (v === '.') {
                offset += 1;
                if (at(offset)?.type === 'identifier' || at(offset)?.type === 'keyword')
                    offset += 1;
                continue;
            }
            if (v === '(' || v === '[') {
                const close = matchBracket(offset);
                if (close < 0) return -1;
                offset = close + 1;
                continue;
            }
            break;
        }
        return offset - 1;
    }

    function matchBracket(startOffset: number): number {
        const open = at(startOffset)?.value;
        const closeChar =
            open === '(' ? ')' : open === '[' ? ']' : open === '<' ? '>' : open === '{' ? '}' : '';
        if (!closeChar) return -1;
        let depth = 0;
        for (let offset = startOffset; offset < sig.length - sigPtr; offset += 1) {
            const v = at(offset)?.value;
            if (v === open) depth += 1;
            else if (v === closeChar) {
                depth -= 1;
                if (depth === 0) return offset;
            }
        }
        return -1;
    }

    function rewriteDeclaration(): { text: string; consume: number } | null {
        const first = at(0);
        if (!first) return null;
        const prev = prevSigValue();
        if (
            prev === 'new' ||
            prev === '.' ||
            prev === 'as' ||
            prev === 'is' ||
            prev === 'return' ||
            prev === 'case'
        ) {
            return null;
        }
        const isTypeKeyword =
            first.type === 'keyword' &&
            (NUMERIC_TYPES.has(first.value) ||
                ['bool', 'string', 'char', 'object', 'var'].includes(first.value));
        const isKnownType = first.type === 'identifier' && TYPE_NAMES.has(first.value);
        if (!isTypeKeyword && !isKnownType) return null;

        const nameOffset = consumeType(0);
        if (nameOffset < 0) return null;
        const nameToken = at(nameOffset);
        if (nameToken?.type !== 'identifier') return null;
        const after = at(nameOffset + 1)?.value;
        if (after !== '=' && after !== ';' && after !== ',' && after !== ')' && after !== 'in') {
            return null;
        }
        if (first.value === 'var' && after === 'in') return null;
        if (!STATEMENT_BOUNDARY.has(prev ?? '') && prev !== undefined) return null;

        return { text: 'let mut', consume: nameOffset - 1 };
    }

    function consumeType(startOffset: number): number {
        let offset = startOffset;
        const first = at(offset);
        if (!first) return -1;
        if (first.value === 'var') return offset + 1;
        offset += 1;
        while (at(offset)?.value === '.') offset += 2;
        if (at(offset)?.value === '<') {
            const close = matchBracket(offset);
            if (close < 0) return -1;
            offset = close + 1;
        }
        while (at(offset)?.value === '[' && at(offset + 1)?.value === ']') offset += 2;
        if (at(offset)?.value === '?') offset += 1;
        return offset;
    }

    function emitToken(token: Token): string {
        if (token.type === 'number') {
            const raw = token.value.replace(/_/g, '').replace(/[fFdDmMlLuU]+$/, '');
            // C# numeric literals are implicitly converted to the surrounding type;
            // Rust is not. Emitting integer literals as float literals keeps them
            // polymorphic (Rust infers f32 or f64 per use), while positions that
            // need an integer are cast by the transpiler.
            return /^[0-9]+$/.test(raw) ? `${raw}.0` : raw;
        }
        if (token.type === 'string') {
            return convertString(token, diagnostics, transpileFragment);
        }
        if (token.type === 'char') {
            const inner = token.value.slice(1, -1).replace(/\\'/g, "'");
            return `'${inner.replace(/\\/g, '\\\\').replace(/'/g, "\\'")}'`;
        }
        if (token.type === 'identifier' && token.value.startsWith('@')) {
            return token.value.slice(1);
        }
        return token.value;
    }
}
