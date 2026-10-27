# Compatibility

This table is also available at runtime from
`@tochii/osb-toolkit-shared` → `COMPATIBILITY` / `getCompatibility(target)`.

## After Effects

- **SDK:** After Effects SDK 26.5
- **API:** `PF_PLUG_IN_VERSION 13.28`
- **Platforms:** Windows, macOS (x64, arm64)
- **Artifact:** `.aex` (Windows) / `.plugin` bundle (macOS)
- **Host:** CC 2014 (13.0)+
- **Install:**
    - Windows: `%APPDATA%\Adobe\After Effects\<version>\Support Files\Plug-ins`
    - macOS: `/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore`

## DaVinci Resolve

- **SDK:** OpenFX 1.4 (`AcademySoftwareFoundation/openfx`)
- **API:** `kOfxImageEffectPluginApiVersion = 1`
- **Platforms:** Windows, macOS, Linux (x64; Apple Silicon universal)
- **Artifact:** `.ofx.bundle`
- **Host:** Resolve 14+
- **Install:** `%COMMONPROGRAMFILES%\OFX\Plugins` / `/Library/OFX/Plugins` / `/usr/OFX/Plugins`, or `OFX_PLUGIN_PATH`

## Premiere Pro

- **SDK:** Premiere Pro SDK 26.0 + After Effects Effect SDK
- **API:** `PF_` ABI (`appl_id = 'PrMr'`)
- **Platforms:** Windows, macOS (x64; macOS universal)
- **Artifact:** `.aex` (AE-style, loaded from MediaCore)
- **Host:** CC 2015 (9.0)+, 64-bit only
- **Install:** `C:\Program Files\Adobe\Common\Plugins\7.0\MediaCore` / `/Library/Application Support/Adobe/Common/Plugins/7.0/MediaCore`
