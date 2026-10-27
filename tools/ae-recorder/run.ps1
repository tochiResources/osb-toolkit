# Drives the AE event recorder (recorder.jsx) against the running After Effects
# instance. Because `app.scheduleTask` is not pumped in the command-line engine,
# this script polls AE by sending a tiny `aeRecTick()` script on a timer.
#
#   .\run.ps1                        # load, start a session, and poll until Ctrl+C
#   .\run.ps1 -Action start -IntervalMs 200
#   .\run.ps1 -Action mark -Label "selected layer 3"
#   .\run.ps1 -Action snapshot       # one full snapshot right now
#   .\run.ps1 -Action stop           # append a stop line (for a session started elsewhere)
#   .\run.ps1 -Action tail           # just follow the log
#
# After Effects must already be running; the script will not launch a new one.

param(
    [ValidateSet("start", "stop", "mark", "snapshot", "tail")]
    [string]$Action = "start",
    [string]$Label = "",
    [string]$LogPath = "C:\Users\user\AppData\Local\Temp\opencode\ae-events.jsonl",
    [int]$IntervalMs = 500
)

$ErrorActionPreference = "Stop"

function Get-RunningAfterFx {
    $running = Get-Process AfterFX -ErrorAction SilentlyContinue |
        Where-Object { $_.Path } | Select-Object -First 1
    if (-not $running) {
        throw "After Effects is not running; start it first (the recorder reads the live instance)."
    }
    return $running.Path
}

function Invoke-Jsx {
    param([string]$Code)
    & (Get-RunningAfterFx) -s $Code | Out-Null
}

function Write-NewLogLines {
    if (-not (Test-Path -LiteralPath $LogPath)) { return }
    $lines = @(Get-Content -LiteralPath $LogPath -ErrorAction SilentlyContinue)
    if ($lines.Count -le $script:seen) { return }
    for ($i = $script:seen; $i -lt $lines.Count; $i++) { Write-Host $lines[$i] }
    $script:seen = $lines.Count
}

$script:seen = 0

if ($Action -eq "tail") {
    if (-not (Test-Path -LiteralPath $LogPath)) { throw "Log not found: $LogPath" }
    Get-Content -LiteralPath $LogPath -Wait -Tail 20
    return
}

$scriptPath = (Resolve-Path (Join-Path $PSScriptRoot "recorder.jsx")).Path -replace '\\', '/'

switch ($Action) {
    "start" {
        Invoke-Jsx "`$.evalFile('$scriptPath')"
        Invoke-Jsx "aeRecStart()"
        Write-Host "recording to $LogPath (Ctrl+C to stop)"
        try {
            while ($true) {
                Invoke-Jsx "aeRecTick()"
                Write-NewLogLines
                Start-Sleep -Milliseconds $IntervalMs
            }
        } finally {
            Invoke-Jsx "aeRecStop()"
            Write-NewLogLines
            Write-Host "stopped"
        }
    }
    "stop"     { Invoke-Jsx "`$.evalFile('$scriptPath')"; Invoke-Jsx "aeRecStop()" }
    "snapshot" { Invoke-Jsx "`$.evalFile('$scriptPath')"; Invoke-Jsx "aeRecSnapshot()" }
    "mark"     {
        $escaped = $Label -replace "'", "\'"
        Invoke-Jsx "`$.evalFile('$scriptPath')"
        Invoke-Jsx "aeRecMark('$escaped')"
    }
}

if ($Action -ne "start") {
    Write-Output "sent '$Action' to After Effects; log: $LogPath"
}
