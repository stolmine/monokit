# Monokit Windows Bundle Script
# Creates a self-contained bundle with scsynth and required plugins
# Usage: .\scripts\bundle-windows.ps1 [version]

param(
    [string]$Version = "dev"
)

$ErrorActionPreference = "Stop"

$Arch = "x86_64-pc-windows-msvc"
$Name = "monokit-$Version-$Arch"

# SuperCollider paths - check common install locations
$ScPaths = @(
    "C:\SuperCollider\SuperCollider",
    "C:\Program Files\SuperCollider",
    "C:\Program Files (x86)\SuperCollider"
)

$ScDir = $null
foreach ($path in $ScPaths) {
    if (Test-Path "$path\scsynth.exe") {
        $ScDir = $path
        break
    }
}

if (-not $ScDir) {
    Write-Error "ERROR: SuperCollider not found. Checked: $($ScPaths -join ', ')"
    exit 1
}

$ScSynth = "$ScDir\scsynth.exe"
$ScPlugins = "$ScDir\plugins"
$ScLang = "$ScDir\sclang.exe"

Write-Host "=== Creating monokit bundle v$Version ===" -ForegroundColor Cyan
Write-Host "SuperCollider found at: $ScDir"

# User extensions path
$UserExtensions = "$env:LOCALAPPDATA\SuperCollider\Extensions"
$Sc3Plugins = "$UserExtensions\SC3plugins"
$MiUgens = "$UserExtensions\mi-UGens"

# Distribution directories
$DistDir = "dist\bundle"
$BundleDir = "$DistDir\$Name"

# Build monokit
Write-Host "Building monokit with scsynth-direct feature..."
cargo build --release --features scsynth-direct
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed"
    exit 1
}

# Compile SynthDefs
Write-Host "Compiling SynthDefs..."
if (-not (Test-Path $ScLang)) {
    Write-Error "ERROR: sclang not found at $ScLang"
    exit 1
}

# Kill any lingering sclang/scsynth processes
Get-Process sclang, scsynth -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2

$RepoRoot = Get-Location
$SynthDefOutput = "$RepoRoot\sc\synthdefs"

Write-Host "  Running sclang..."
Push-Location $ScDir
try {
    & $ScLang "$RepoRoot\build_scripts\compile_synthdefs.scd"
} finally {
    Pop-Location
}

# Wait for sclang to exit and clean up
Start-Sleep -Seconds 2
Get-Process sclang -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue

# Verify synthdefs were created
$SynthDefs = Get-ChildItem "$SynthDefOutput\*.scsyndef" -ErrorAction SilentlyContinue
if (-not $SynthDefs -or $SynthDefs.Count -eq 0) {
    Write-Error "ERROR: Compiled SynthDefs not found at $SynthDefOutput"
    exit 1
}
Write-Host "  SynthDefs compiled: $($SynthDefs.Count) files"

# Create bundle directory structure
Write-Host "Creating bundle directory structure..."
if (Test-Path $BundleDir) {
    Remove-Item -Recurse -Force $BundleDir
}
New-Item -ItemType Directory -Force -Path $BundleDir | Out-Null
New-Item -ItemType Directory -Force -Path "$BundleDir\Resources" | Out-Null
New-Item -ItemType Directory -Force -Path "$BundleDir\Resources\plugins" | Out-Null
New-Item -ItemType Directory -Force -Path "$BundleDir\Resources\synthdefs" | Out-Null

# Copy monokit binary
Write-Host "Copying monokit binary..."
Copy-Item "target\release\monokit.exe" "$BundleDir\"

# Copy scsynth
Write-Host "Copying scsynth..."
Copy-Item $ScSynth "$BundleDir\Resources\"

# Copy scsynth dependencies (DLLs in the SC directory)
Write-Host "Copying scsynth dependencies..."
$ScDlls = Get-ChildItem "$ScDir\*.dll" -ErrorAction SilentlyContinue
foreach ($dll in $ScDlls) {
    Copy-Item $dll.FullName "$BundleDir\Resources\"
    Write-Host "    - $($dll.Name)"
}

# Copy SynthDefs
Write-Host "Copying SynthDefs..."
Copy-Item "$SynthDefOutput\*.scsyndef" "$BundleDir\Resources\synthdefs\"

# Copy required plugins
Write-Host "Copying required plugins..."

# Core UGen plugins required by monokit
# Note: BufIOUGens is built into scsynth/IOUGens on Windows (not a separate .scx)
$CorePlugins = @(
    "BinaryOpUGens.scx",
    "UnaryOpUGens.scx",
    "IOUGens.scx",
    "DelayUGens.scx",
    "FilterUGens.scx",
    "DynNoiseUGens.scx",
    "NoiseUGens.scx",
    "PanUGens.scx",
    "TriggerUGens.scx",
    "OscUGens.scx",
    "GrainUGens.scx",
    "FFT_UGens.scx",
    "PV_ThirdParty.scx",
    "DemandUGens.scx",
    "PhysicalModelingUGens.scx",
    "MulAddUGens.scx",
    "DiskIO_UGens.scx",
    "ReverbUGens.scx",
    "LFUGens.scx"
)

Write-Host "  Core plugins:"
foreach ($plugin in $CorePlugins) {
    $pluginPath = "$ScPlugins\$plugin"
    if (Test-Path $pluginPath) {
        Copy-Item $pluginPath "$BundleDir\Resources\plugins\"
        Write-Host "    - $plugin"
    } else {
        Write-Host "    WARNING: $plugin not found" -ForegroundColor Yellow
    }
}

# sc3-plugins (BlackrainUGens for SVF/BMoog, TJUGens for DFM1)
Write-Host "  sc3-plugins:"
if (Test-Path $Sc3Plugins) {
    # On Windows, .scx files are at the root of SC3plugins
    $Sc3Required = @(
        "BlackrainUGens.scx",
        "TJUGens.scx"
    )
    foreach ($plugin in $Sc3Required) {
        $pluginPath = "$Sc3Plugins\$plugin"
        if (Test-Path $pluginPath) {
            Copy-Item $pluginPath "$BundleDir\Resources\plugins\"
            Write-Host "    - $plugin"
        } else {
            Write-Host "    WARNING: $plugin not found" -ForegroundColor Yellow
        }
    }
} else {
    Write-Host "    WARNING: SC3plugins not found at $Sc3Plugins" -ForegroundColor Yellow
}

# mi-UGens (MiPlaits, MiClouds, MiRings)
Write-Host "  mi-UGens:"
if (Test-Path $MiUgens) {
    $MiRequired = @(
        "MiPlaits.scx",
        "MiClouds.scx",
        "MiRings.scx"
    )
    foreach ($plugin in $MiRequired) {
        $pluginPath = "$MiUgens\$plugin"
        if (Test-Path $pluginPath) {
            Copy-Item $pluginPath "$BundleDir\Resources\plugins\"
            Write-Host "    - $plugin"
        } else {
            Write-Host "    WARNING: $plugin not found" -ForegroundColor Yellow
        }
    }
} else {
    Write-Host "    WARNING: mi-UGens not found at $MiUgens" -ForegroundColor Yellow
}

# Summary
Write-Host ""
Write-Host "Bundle structure:" -ForegroundColor Cyan
Get-ChildItem $BundleDir | Format-Table Name, Length -AutoSize

$PluginCount = (Get-ChildItem "$BundleDir\Resources\plugins\*.scx").Count
$SynthDefCount = (Get-ChildItem "$BundleDir\Resources\synthdefs\*.scsyndef").Count
$BundleSize = "{0:N2} MB" -f ((Get-ChildItem $BundleDir -Recurse | Measure-Object -Property Length -Sum).Sum / 1MB)

Write-Host "  Plugin count: $PluginCount"
Write-Host "  SynthDef count: $SynthDefCount"
Write-Host "  Bundle size: $BundleSize"

Write-Host ""
Write-Host "=== Bundle complete ===" -ForegroundColor Green
Write-Host "Location: $BundleDir"
Write-Host ""
Write-Host "To test the bundle:"
Write-Host "  cd $BundleDir"
Write-Host "  .\monokit.exe"
Write-Host ""
Write-Host "To create zip archive:"
Write-Host "  Compress-Archive -Path '$BundleDir' -DestinationPath '$DistDir\$Name.zip'"
