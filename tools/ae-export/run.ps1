# Exports the running After Effects project to JSON via live-dump.jsx.
#
#   .\run.ps1                        # storyboard scope (transform + markers) -> ae-project.json
#   .\run.ps1 -Scope full            # every property (slow: shape/effect trees)
#   .\run.ps1 -Out C:\path\proj.json
#
# After Effects must already be running. The export runs synchronously inside AE,
# so this waits for the output file to appear (up to -TimeoutSec).

param(
    [ValidateSet("storyboard", "full")]
    [string]$Scope = "storyboard",
    [string]$Out = "C:\Users\user\AppData\Local\Temp\opencode\ae-project.json",
    [int]$TimeoutSec = 240
)

$ErrorActionPreference = "Stop"

function Get-RunningAfterFx {
    $running = Get-Process AfterFX -ErrorAction SilentlyContinue |
        Where-Object { $_.Path } | Select-Object -First 1
    if (-not $running) { throw "After Effects is not running; start it first." }
    return $running.Path
}

$scriptPath = (Resolve-Path (Join-Path $PSScriptRoot "live-dump.jsx")).Path -replace '\\', '/'
$outJs = $Out -replace '\\', '/'
$call = if ($Scope -eq "full") { "aeExportFull" } else { "aeExportStoryboard" }

if (Test-Path -LiteralPath $Out) { Remove-Item -LiteralPath $Out -Force }

$afterFx = Get-RunningAfterFx
& $afterFx -s "`$.evalFile('$scriptPath'); $call('$outJs')"

$deadline = (Get-Date).AddSeconds($TimeoutSec)
while ((Get-Date) -lt $deadline) {
    if (Test-Path -LiteralPath $Out) {
        $size = (Get-Item -LiteralPath $Out).Length
        if ($size -gt 0) {
            Write-Output "exported $Scope -> $Out ($size bytes)"
            return
        }
    }
    Start-Sleep -Milliseconds 500
}
throw "timed out waiting for $Out after $TimeoutSec s"
