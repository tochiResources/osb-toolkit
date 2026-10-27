import type { Diagnostic } from '@tochii/osb-toolkit-shared';

export type TokenType =
    | 'identifier'
    | 'keyword'
    | 'number'
    | 'string'
    | 'char'
    | 'operator'
    | 'punctuation'
    | 'whitespace'
    | 'newline'
    | 'comment'
    | 'eof';

export interface Token {
    type: TokenType;
    value: string;
    line: number;
    column: number;
    /** For strings: whether the literal is interpolated ($"..." / $@"..."). */
    interpolated?: boolean;
    /** For strings: whether the literal is verbatim (@"..."). */
    verbatim?: boolean;
}

export interface ParseOptions {
    file?: string;
    /**
     * Whether a `Generate()` method is required. Effect scripts require it;
     * library classes compiled alongside a script do not.
     */
    requireGenerate?: boolean;
}

export interface TranspileOptions {
    file?: string;
    /** Emit `let`/`const` for C# locals (default true). */
    declarations?: boolean;
    /** `[Configurable]` field names, mapped to `self.<snake_case>` in Rust. */
    configurableNames?: string[];
    /** User helper method names, called as `self.<snake_case>(...)`. */
    helperNames?: string[];
    /**
     * Explicit C# identifier -> Rust field/method name map. Used when bundling,
     * where names are prefixed per script to avoid collisions. Takes precedence
     * over `configurableNames`/`helperNames` and covers both.
     */
    names?: Record<string, string>;
    /**
     * External library static methods, keyed `ClassName.Method` and mapped to the
     * generated Rust method name (e.g. `HelperClass.Distance` -> `distance`).
     */
    libraryMethods?: Record<string, string>;
    /**
     * In-scope variables holding sprite handles (`CreateSprite`/`CreateAnimation`
     * results and sprite-typed helper parameters). Their method calls are routed
     * through `rt.storyboard.sprite_mut(...)`.
     */
    spriteNames?: string[];
    /** In-scope variables holding layer handles (`GetLayer` results). */
    layerNames?: string[];
    /**
     * C# helper names whose emitted Rust method takes `rt`. Only those helpers
     * receive `rt` at call sites, so passing `rt` never conflicts with a sprite
     * command's receiver borrow.
     */
    rtHelpers?: string[];
    /** External library methods (`ClassName.Method`) whose Rust form takes `rt`. */
    rtLibraryMethods?: string[];
}

export interface TranspileResult {
    code: string;
    diagnostics: Diagnostic[];
}
