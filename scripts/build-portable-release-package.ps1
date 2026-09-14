# Spec 058 — deterministic unsigned portable release package builder.
[CmdletBinding()]
param(
    [string]$OutputDir = "target/medscale-release/058",
    [string]$PackageLabel = "candidate",
    [string]$BinaryDir = "target/release",
    [string]$SbomPath = "evidence/054-release-sbom/SBOM.cdx.json",
    [switch]$SkipBuild
)
$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

function Get-Sha256Hex([string]$Path) {
    return (Get-FileHash -Algorithm SHA256 -Path $Path).Hash.ToLowerInvariant()
}
function Write-Utf8Lf([string]$Path, [string]$Text) {
    $enc = [System.Text.UTF8Encoding]::new($false)
    [System.IO.File]::WriteAllText($Path, ($Text.TrimEnd() + "`n"), $enc)
}
function Get-PlatformId {
    if ($IsWindows) { return 'windows' }
    if ($IsMacOS) { return 'macos' }
    if ($IsLinux) { return 'linux' }
    throw 'unsupported release platform'
}

if (-not $SkipBuild) {
    cargo build --release --locked -p medscale-cli -p medscale-desktop
    if ($LASTEXITCODE -ne 0) { throw 'release build failed' }
}
$platform = Get-PlatformId
$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString().ToLowerInvariant()
$metadata = cargo metadata --format-version 1 --locked --no-deps | ConvertFrom-Json
$version = [string](($metadata.packages | Where-Object name -eq 'medscale-cli' | Select-Object -First 1).version)
if ([string]::IsNullOrWhiteSpace($version)) { throw 'workspace version unavailable' }
$sourceSha = (git rev-parse HEAD).Trim()
$treeSha = (git log -1 --format=%T).Trim()
$lockSha = Get-Sha256Hex (Join-Path $repoRoot 'Cargo.lock')
$exe = if ($IsWindows) { '.exe' } else { '' }
$cli = Join-Path $repoRoot "$BinaryDir/medscale$exe"
$desktop = Join-Path $repoRoot "$BinaryDir/medscale-desktop$exe"
foreach ($required in @($cli,$desktop,'LICENSE','README.md','docs/legal/NOTICE_INVENTORY.md',$SbomPath)) {
    if (-not (Test-Path $required)) { throw "required package input missing: $required" }
}

$outFull = Join-Path $repoRoot $OutputDir
New-Item -ItemType Directory -Force -Path $outFull | Out-Null
$stage = Join-Path $outFull ("stage-" + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Force -Path (Join-Path $stage 'bin') | Out-Null
try {
    Copy-Item $cli (Join-Path $stage "bin/medscale$exe")
    Copy-Item $desktop (Join-Path $stage "bin/medscale-desktop$exe")
    Copy-Item 'LICENSE' (Join-Path $stage 'LICENSE')
    Copy-Item 'README.md' (Join-Path $stage 'README.md')
    Copy-Item 'docs/legal/NOTICE_INVENTORY.md' (Join-Path $stage 'NOTICE.md')
    Copy-Item $SbomPath (Join-Path $stage 'SBOM.cdx.json')

    $payloadPaths = @("bin/medscale$exe", "bin/medscale-desktop$exe", 'LICENSE', 'NOTICE.md', 'README.md', 'SBOM.cdx.json') | Sort-Object
    $payload = @()
    foreach ($rel in $payloadPaths) {
        $payload += [ordered]@{ path = $rel; sha256 = Get-Sha256Hex (Join-Path $stage $rel) }
    }
    $manifest = [ordered]@{
        schema_version = 1
        spec_id = '058-portable-release-package'
        package_version = $version
        package_label = $PackageLabel
        platform = $platform
        arch = $arch
        source = [ordered]@{ git_sha = $sourceSha; tree_sha = $treeSha; cargo_lock_sha256 = $lockSha }
        signed = $false
        notarized = $false
        release_ready = $false
        reproducible_binary_build = $false
        payload = $payload
    }
    Write-Utf8Lf (Join-Path $stage 'package-manifest.json') ($manifest | ConvertTo-Json -Depth 8)

    Add-Type -AssemblyName System.IO.Compression
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zipName = "medscale-$version-$platform-$arch-$PackageLabel.zip"
    $zipPath = Join-Path $outFull $zipName
    if (Test-Path $zipPath) { Remove-Item -Force $zipPath }
    $fs = [System.IO.File]::Open($zipPath, [System.IO.FileMode]::CreateNew)
    try {
        $zip = [System.IO.Compression.ZipArchive]::new($fs, [System.IO.Compression.ZipArchiveMode]::Create, $false)
        try {
            $entries = @($payloadPaths + 'package-manifest.json') | Sort-Object
            foreach ($rel in $entries) {
                $entry = $zip.CreateEntry($rel, [System.IO.Compression.CompressionLevel]::NoCompression)
                $entry.LastWriteTime = [DateTimeOffset]::Parse('1980-01-01T00:00:00Z')
                $input = [System.IO.File]::OpenRead((Join-Path $stage $rel))
                try { $stream = $entry.Open(); try { $input.CopyTo($stream) } finally { $stream.Dispose() } } finally { $input.Dispose() }
            }
        } finally { $zip.Dispose() }
    } finally { $fs.Dispose() }
    $zipSha = Get-Sha256Hex $zipPath
    Write-Utf8Lf "$zipPath.sha256" "$zipSha  $zipName"
    Write-Host "PACKAGE_ZIP=$zipPath"
    Write-Host "PACKAGE_SHA256=$zipSha"
} finally {
    if (Test-Path $stage) { Remove-Item -Recurse -Force $stage }
}
