# Builds and runs the header-only C++ binding and SDK tests against the
# osb-native cdylib. Locates MSVC with vswhere (Windows) or falls back to a
# system C++ compiler.
$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$out = Join-Path $PSScriptRoot "test\build"
New-Item -ItemType Directory -Force $out | Out-Null

$dll = Join-Path $root "target\release\osb_native.dll"
if (-not (Test-Path $dll)) {
    Push-Location $root
    try { cargo build -p osb-native --release } finally { Pop-Location }
}

$isWindows = $IsWindows -or $env:OS -eq "Windows_NT"
if ($isWindows) {
    $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    $vs = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if (-not $vs) { throw "MSVC (VC.Tools.x86.x64) not found via vswhere" }
    $devcmd = Join-Path $vs "Common7\Tools\VsDevCmd.bat"
}

foreach ($name in @("main", "sdk", "clean")) {
    $source = Join-Path $PSScriptRoot "test\$name.cpp"
    $exe = Join-Path $out "osbtk_cpp_$name.exe"
    if ($isWindows) {
        $command = "call `"$devcmd`" -arch=x64 -no_logo >nul && cl /nologo /std:c++17 /EHsc /I `"$PSScriptRoot`" `"$source`" /Fe:`"$exe`""
        cmd /c $command
    } else {
        c++ -std=c++17 -I "$PSScriptRoot" "$source" -o "$exe"
    }
    Push-Location $root
    try {
        & $exe
        if ($LASTEXITCODE -ne 0) {
            throw "C++ test '$name' failed with exit code $LASTEXITCODE"
        }
    } finally { Pop-Location }
}
