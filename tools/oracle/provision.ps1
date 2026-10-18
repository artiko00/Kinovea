<#
.SYNOPSIS
  Checks (and optionally installs) the build prerequisites for the Kinovea parity oracle on Windows.
.DESCRIPTION
  Check mode (default): reports MSBuild, C++/CLI support, NuGet and 7-Zip. Prints the exact winget
  commands for anything missing. Never installs unless -Install is passed.
#>
param([switch]$Install)

$ErrorActionPreference = 'Stop'
$vswhere = 'C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe'
$components = @(
    'Microsoft.VisualStudio.Workload.ManagedDesktopBuildTools',
    'Microsoft.Net.Component.4.8.TargetingPack',
    'Microsoft.VisualStudio.Workload.VCTools',
    'Microsoft.VisualStudio.Component.VC.CLI.Support'
)
$override = '--quiet --wait --norestart --nocache --includeRecommended ' + (($components | ForEach-Object { "--add $_" }) -join ' ')
$buildToolsCmd = "winget install --id Microsoft.VisualStudio.2022.BuildTools --exact --silent --accept-package-agreements --accept-source-agreements --override `"$override`""
$nugetCmd = 'winget install --id Microsoft.NuGet --exact --silent --accept-package-agreements --accept-source-agreements'
$sevenZipCmd = 'winget install --id 7zip.7zip --exact --silent --accept-package-agreements --accept-source-agreements'

function Find-MSBuild {
    if (-not (Test-Path $vswhere)) { return $null }
    $p = & $vswhere -products * -requires Microsoft.Component.MSBuild -find 'MSBuild\**\Bin\MSBuild.exe' 2>$null | Select-Object -First 1
    if ($p) { return $p } else { return $null }
}
function Has-Cli {
    if (-not (Test-Path $vswhere)) { return $false }
    $r = & $vswhere -products * -requires Microsoft.VisualStudio.Component.VC.CLI.Support -property installationPath 2>$null
    return [bool]$r
}
function Has-Cmd($name) { return [bool](Get-Command $name -ErrorAction SilentlyContinue) }

$missing = @()
$msbuild = Find-MSBuild
if ($msbuild) { Write-Host "[ok] MSBuild: $msbuild" } else { Write-Host '[missing] MSBuild / Build Tools 2022'; $missing += $buildToolsCmd }
if (Has-Cli) { Write-Host '[ok] C++/CLI support' } elseif ($msbuild) { Write-Host '[missing] C++/CLI component'; $missing += $buildToolsCmd }
if (Has-Cmd nuget) { Write-Host '[ok] nuget' } else { Write-Host '[missing] nuget'; $missing += $nugetCmd }
if ((Has-Cmd 7z) -or (Test-Path 'C:\Program Files\7-Zip\7z.exe')) { Write-Host '[ok] 7z' } else { Write-Host '[missing] 7z'; $missing += $sevenZipCmd }

$missing = $missing | Select-Object -Unique
if (-not $missing) { Write-Host 'All prerequisites present.'; exit 0 }

if (-not $Install) {
    Write-Host ''
    Write-Host 'Run the following to provision (or re-run this script with -Install):'
    $missing | ForEach-Object { Write-Host "  $_" }
    exit 2
}

foreach ($cmd in $missing) {
    Write-Host "[install] $cmd"
    cmd /c $cmd
    if ($LASTEXITCODE -ne 0 -and $LASTEXITCODE -ne 3010) { Write-Host "[error] exit $LASTEXITCODE"; exit $LASTEXITCODE }
}
Write-Host '[done] re-run in check mode to confirm.'
exit 0
