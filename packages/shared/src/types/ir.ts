export type EffectParameterType =
    | 'int'
    | 'long'
    | 'double'
    | 'float'
    | 'bool'
    | 'string'
    | 'enum'
    | 'vec2'
    | 'vec3'
    | 'color'
    | 'origin'
    | 'easing'
    | 'unknown';

export interface EffectEnumValue {
    name: string;
    value?: number;
}

/** The native control a configurable maps to in a host. */
export type ParameterControl =
    | 'slider'
    | 'checkbox'
    | 'color'
    | 'angle'
    | 'popup'
    | 'point'
    | 'text'
    | 'picker';

/** The kind of project asset an asset input resolves to. */
export type AssetKind =
    | 'sprite'
    | 'background'
    | 'audio'
    | 'font'
    | 'subtitle'
    | 'composition'
    | 'layer'
    | 'media'
    | 'file';

export interface EffectParameter {
    name: string;
    csType: string;
    type: EffectParameterType;
    defaultValue?: unknown;
    defaultValueLiteral?: string;
    group?: string;
    description?: string;
    enumValues?: EffectEnumValue[];
    order: number;
    /** The host control this configurable maps to. */
    control?: ParameterControl;
    /** For `picker` controls, the asset kind the picker chooses. */
    assetKind?: AssetKind;
}

export interface EffectField {
    name: string;
    csType: string;
    modifiers: string[];
    initializer?: string;
}

export interface EffectMethod {
    name: string;
    signature: string;
    body: string;
    isOverride: boolean;
}

export type DiagnosticSeverity = 'info' | 'warning' | 'error';

export interface Diagnostic {
    severity: DiagnosticSeverity;
    message: string;
    code?: string;
    line?: number;
    column?: number;
}

export interface EffectScript {
    className: string;
    namespace?: string;
    baseClass: string;
    sourceFile?: string;
    usingDirectives: string[];
    fields: EffectField[];
    parameters: EffectParameter[];
    methods: EffectMethod[];
    generateBody: string;
    generateLine?: number;
    transpiledBody?: string;
    /**
     * Set when the script is an `.osb` importer: its `Generate()` is
     * mapped onto `Runtime::import_osb` and its hand-written parser helpers are
     * not emitted (they are replaced by the runtime's `.osb` parser).
     */
    importOsb?: boolean;
    diagnostics: Diagnostic[];
}

export type OsbOrigin =
    | 'TopLeft'
    | 'TopCentre'
    | 'TopRight'
    | 'CentreLeft'
    | 'Centre'
    | 'CentreRight'
    | 'BottomLeft'
    | 'BottomCentre'
    | 'BottomRight';

export type OsbEasing =
    | 'None'
    | 'Out'
    | 'In'
    | 'InQuad'
    | 'OutQuad'
    | 'InOutQuad'
    | 'InCubic'
    | 'OutCubic'
    | 'InOutCubic'
    | 'InQuart'
    | 'OutQuart'
    | 'InOutQuart'
    | 'InQuint'
    | 'OutQuint'
    | 'InOutQuint'
    | 'InSine'
    | 'OutSine'
    | 'InOutSine'
    | 'InExpo'
    | 'OutExpo'
    | 'InOutExpo'
    | 'InCirc'
    | 'OutCirc'
    | 'InOutCirc'
    | 'InElastic'
    | 'OutElastic'
    | 'OutElasticHalf'
    | 'OutElasticQuarter'
    | 'InOutElastic'
    | 'InBack'
    | 'OutBack'
    | 'InOutBack'
    | 'InBounce'
    | 'OutBounce'
    | 'InOutBounce';

export type OsbLayer = 'Background' | 'Fail' | 'Pass' | 'Foreground' | 'Overlay';

export interface GeneratedFile {
    path: string;
    contents: string;
    kind: 'source' | 'project' | 'build' | 'manifest' | 'runtime' | 'readme' | 'asset';
    executable?: boolean;
}

export interface ConvertOptions {
    target: string;
    pluginName?: string;
    pluginId?: string;
    category?: string;
    outputDir?: string;
    resolution?: { width: number; height: number };
}

export interface ConvertResult {
    target: string;
    effectName: string;
    pluginId: string;
    script: EffectScript;
    files: GeneratedFile[];
    diagnostics: Diagnostic[];
}
