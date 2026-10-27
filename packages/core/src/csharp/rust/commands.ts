/**
 * Collapses the `OsbSprite` command overloads onto the runtime's
 * fixed-arity methods.
 *
 * C# resolves these by argument count and by an optional leading `OsbEasing`;
 * Rust has no overloads, so the transpiler normalizes every call here to one
 * canonical shape with the easing in the trailing slot (or `Easing::None`).
 */

export interface CommandArg {
    /** The argument, already transpiled to Rust. */
    text: string;
    /** Whether this is the C# leading `OsbEasing` (moved to the trailing slot). */
    easing: boolean;
}

const SCALAR: Record<string, string> = {
    Scale: 'scale',
    Rotate: 'rotate',
    Fade: 'fade',
    MoveX: 'move_x',
    MoveY: 'move_y',
};

const VECTOR: Record<string, string> = { Move: 'move_', ScaleVec: 'scale_vec' };

const COLOR: Record<string, string> = { Color: 'color', ColorHsb: 'color_hsb' };

const PARAMETER_FLAG: Record<string, string> = {
    Additive: 'Additive',
    FlipH: 'FlipHorizontal',
    FlipV: 'FlipVertical',
};

export const COMMAND_METHODS = new Set<string>([
    ...Object.keys(SCALAR),
    ...Object.keys(VECTOR),
    ...Object.keys(COLOR),
    'Parameter',
    ...Object.keys(PARAMETER_FLAG),
]);

/** Returns `.method(args)` for a supported command call, or `null` if unmapped. */
export function normalizeCommandCall(name: string, rawArgs: CommandArg[]): string | null {
    const args = rawArgs.slice();
    let easing = 'Easing::None';
    if (args[0]?.easing) easing = (args.shift() as CommandArg).text;

    if (name in SCALAR) return scalarCall(SCALAR[name] as string, args, easing);
    if (name in VECTOR) return vectorCall(VECTOR[name] as string, args, easing);
    if (name in COLOR) return colorCall(name, args, easing);
    if (name === 'Parameter') {
        if (args.length !== 3) return null;
        return `.parameter(${args[0]?.text}, ${args[1]?.text}, ${args[2]?.text}, 0.0, 1.0, ${easing})`;
    }
    if (name in PARAMETER_FLAG) {
        return parameterFlagCall(PARAMETER_FLAG[name] as string, args);
    }
    return null;
}

function scalarCall(method: string, args: CommandArg[], easing: string): string | null {
    const value = args.map(arg => arg.text);
    if (value.length === 4)
        return `.${method}(${value[0]}, ${value[1]}, ${value[2]}, ${value[3]}, ${easing})`;
    if (value.length === 2)
        return `.${method}(${value[0]}, ${value[0]}, ${value[1]}, ${value[1]}, ${easing})`;
    return null;
}

function vectorCall(method: string, args: CommandArg[], easing: string): string | null {
    const value = args.map(arg => arg.text);
    if (value.length === 4)
        return `.${method}(${value[0]}, ${value[1]}, ${value[2]}, ${value[3]}, ${easing})`;
    if (value.length === 6)
        return `.${method}(${value[0]}, ${value[1]}, Vec2::new(${value[2]}, ${value[3]}), Vec2::new(${value[4]}, ${value[5]}), ${easing})`;
    if (value.length === 2)
        return `.${method}(${value[0]}, ${value[0]}, ${value[1]}, ${value[1]}, ${easing})`;
    if (value.length === 3)
        return `.${method}(${value[0]}, ${value[0]}, Vec2::new(${value[1]}, ${value[2]}), Vec2::new(${value[1]}, ${value[2]}), ${easing})`;
    return null;
}

function colorCall(name: string, args: CommandArg[], easing: string): string | null {
    const value = args.map(arg => arg.text);
    const rgb = (r: string, g: string, b: string) => `Color4::from_rgb(${r}, ${g}, ${b})`;
    const hsb = (h: string, s: string, b: string) => `Color4::from_hsb(${h}, ${s}, ${b}, 1.0)`;

    if (name === 'Color') {
        if (value.length === 4)
            return `.color(${value[0]}, ${value[1]}, ${value[2]}, ${value[3]}, ${easing})`;
        if (value.length === 6)
            return `.color(${value[0]}, ${value[1]}, ${value[2]}, ${rgb(value[3] as string, value[4] as string, value[5] as string)}, ${easing})`;
        if (value.length === 8)
            return `.color(${value[0]}, ${value[1]}, ${rgb(value[2] as string, value[3] as string, value[4] as string)}, ${rgb(value[5] as string, value[6] as string, value[7] as string)}, ${easing})`;
        if (value.length === 2)
            return `.color(${value[0]}, ${value[0]}, ${value[1]}, ${value[1]}, ${easing})`;
        if (value.length === 5)
            return `.color(${value[0]}, ${value[0]}, ${rgb(value[1] as string, value[2] as string, value[3] as string)}, ${rgb(value[1] as string, value[2] as string, value[3] as string)}, ${easing})`;
        return null;
    }

    if (value.length === 6)
        return `.color_hsb(${value[0]}, ${value[1]}, ${value[2]}, ${hsb(value[3] as string, value[4] as string, value[5] as string)}, ${easing})`;
    if (value.length === 9)
        return `.color_hsb(${value[0]}, ${value[1]}, ${hsb(value[2] as string, value[3] as string, value[4] as string)}, ${hsb(value[5] as string, value[6] as string, value[7] as string)}, ${easing})`;
    if (value.length === 4)
        return `.color_hsb(${value[0]}, ${value[0]}, ${hsb(value[1] as string, value[2] as string, value[3] as string)}, ${hsb(value[1] as string, value[2] as string, value[3] as string)}, ${easing})`;
    return null;
}

function parameterFlagCall(parameter: string, args: CommandArg[]): string | null {
    const value = args.map(arg => arg.text);
    const tail = `ParameterType::${parameter}, 0.0, 1.0, Easing::None`;
    if (value.length === 2) return `.parameter(${value[0]}, ${value[1]}, ${tail})`;
    if (value.length === 1) return `.parameter(${value[0]}, ${value[0]}, ${tail})`;
    return null;
}
