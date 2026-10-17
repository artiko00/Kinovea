<#
.SYNOPSIS
  Builds the original C# Kinovea solution on the Windows oracle host (Release x64).
.PARAMETER Commit   Git commit to build. Defaults to origin/master.
.PARAMETER Bundle   Optional git bundle (copied by fixtures.sh) that carries commits not yet on origin.
.PARAMETER Dest     Working clone. Default C:\oracle\Kinovea.
.PARAMETER Repo     Remote used for the initial clone.
#>
param(
    [string]$Commit = '',
    [string]$Bundle = '',
    [string]$Dest = 'C:\oracle\Kinovea',
    [string]$Repo = 'https://github.com/artiko00/Kinovea.git'
)
$ErrorActionPreference = 'Stop'
function Step($m) { Write-Host "[build] $m" }
function Fail($m, $code = 1) { Write-Host "[build] ERROR: $m"; exit $code }

# 1. Prerequisites (never install here; provision.ps1 owns that).
$vswhere = 'C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe'
if (-not (Test-Path $vswhere)) { Fail "Build Tools not found. Run: powershell -File C:\oracle\provision.ps1 -Install" 2 }
$msbuild = & $vswhere -products * -requires Microsoft.Component.MSBuild -find 'MSBuild\**\Bin\MSBuild.exe' | Select-Object -First 1
if (-not $msbuild) { Fail "MSBuild not found. Run: powershell -File C:\oracle\provision.ps1 -Install" 2 }
$cli = & $vswhere -products * -requires Microsoft.VisualStudio.Component.VC.CLI.Support -property installationPath
if (-not $cli) { Fail "C++/CLI component missing. Run: powershell -File C:\oracle\provision.ps1 -Install" 2 }
if (-not (Get-Command nuget -ErrorAction SilentlyContinue)) { Fail "nuget not found. Run: powershell -File C:\oracle\provision.ps1 -Install" 2 }
if (-not (Get-Command 7z -ErrorAction SilentlyContinue)) { $env:Path += ';C:\Program Files\7-Zip' }
Step "MSBuild: $msbuild"

# 2. Source checkout.
if (-not (Test-Path (Join-Path $Dest '.git'))) {
    Step "cloning $Repo"
    New-Item -ItemType Directory -Force -Path (Split-Path $Dest) | Out-Null
    git clone --quiet $Repo $Dest
    if ($LASTEXITCODE -ne 0) { Fail 'clone failed' }
}
Set-Location $Dest
git fetch --quiet origin
if ($Bundle) {
    Step "fetching bundle $Bundle"
    git fetch --quiet $Bundle HEAD
    if ($LASTEXITCODE -ne 0) { Fail 'bundle fetch failed' }
}
if (-not $Commit) { $Commit = 'origin/master' }
git checkout --quiet --detach $Commit
if ($LASTEXITCODE -ne 0) { Fail "cannot checkout $Commit" }
Step "at $(git rev-parse HEAD)"

# 3. Dependencies.
Step 'nuget restore'
nuget restore Kinovea.VS2019.sln -NonInteractive | Out-Null
if ($LASTEXITCODE -ne 0) { Fail 'nuget restore failed' }
$ffmpeg = Join-Path $Dest 'Refs\FFmpeg'
if (Test-Path (Join-Path $ffmpeg 'ffmpeg-*')) {
    Step 'FFmpeg cached, skipping download'
} else {
    Step 'downloading FFmpeg'
    & (Join-Path $Dest 'Refs\get-dependencies.ps1')
}

# 4. Build.
Step 'msbuild Release x64'
& $msbuild Kinovea.VS2019.sln /nologo /m /v:minimal /p:Platform=x64 /p:Configuration=Release /p:AllowedReferenceRelatedFileExtensions=none
if ($LASTEXITCODE -ne 0) { Fail 'build failed' }
Step 'build OK'
exit 0
