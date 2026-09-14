# Spec 058 — cross-platform portable package qualification driver.
[CmdletBinding()]
param(
    [string]$OutputDir = "target/medscale-release/058",
    [string]$EvidenceDir = "target/medscale-evidence/058-portable-release-package"
)
$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot
function Get-Sha256Hex([string]$Path) { return (Get-FileHash -Algorithm SHA256 -Path $Path).Hash.ToLowerInvariant() }
function Resolve-RepoPath([string]$Path) {
    if ([IO.Path]::IsPathRooted($Path)) { return [IO.Path]::GetFullPath($Path) }
    return [IO.Path]::GetFullPath((Join-Path $repoRoot $Path))
}
function Get-OnlyZip([string]$Dir) {
    $zips = @(Get-ChildItem -File -Path $Dir -Filter '*.zip')
    if ($zips.Count -ne 1) { throw "expected exactly one ZIP in $Dir; found $($zips.Count)" }
    return $zips[0].FullName
}
function Get-ActiveLabel([string]$InstallRoot) {
    $state = Get-Content -Raw (Join-Path $InstallRoot 'state.json') | ConvertFrom-Json
    $manifest = Get-Content -Raw (Join-Path $InstallRoot "slots/$($state.current_slot)/package-manifest.json") | ConvertFrom-Json
    return [string]$manifest.package_label
}
$outFull = Resolve-RepoPath $OutputDir
$evidenceFull = Resolve-RepoPath $EvidenceDir
if (Test-Path $outFull) { Remove-Item -Recurse -Force $outFull }
if (Test-Path $evidenceFull) { Remove-Item -Recurse -Force $evidenceFull }
New-Item -ItemType Directory -Force -Path $outFull,$evidenceFull | Out-Null
$baseA = Join-Path $outFull 'baseline-a'
$baseB = Join-Path $outFull 'baseline-b'
$candidate = Join-Path $outFull 'candidate'
$generatedSbom = Join-Path $outFull 'SBOM.cdx.json'
& (Join-Path $PSScriptRoot 'generate-release-sbom.ps1') -OutPath $generatedSbom
if ($LASTEXITCODE -ne 0) { throw 'fresh release SBOM generation failed' }
& (Join-Path $PSScriptRoot 'build-portable-release-package.ps1') -OutputDir $baseA -PackageLabel 'baseline' -SbomPath $generatedSbom
if ($LASTEXITCODE -ne 0) { throw 'baseline A package build failed' }
& (Join-Path $PSScriptRoot 'build-portable-release-package.ps1') -OutputDir $baseB -PackageLabel 'baseline' -SbomPath $generatedSbom -SkipBuild
if ($LASTEXITCODE -ne 0) { throw 'baseline B package build failed' }
& (Join-Path $PSScriptRoot 'build-portable-release-package.ps1') -OutputDir $candidate -PackageLabel 'candidate' -SbomPath $generatedSbom -SkipBuild
if ($LASTEXITCODE -ne 0) { throw 'candidate package build failed' }
$baseAZip = Get-OnlyZip $baseA
$baseBZip = Get-OnlyZip $baseB
$candidateZip = Get-OnlyZip $candidate
$baseAHash = Get-Sha256Hex $baseAZip
$baseBHash = Get-Sha256Hex $baseBZip
if ($baseAHash -ne $baseBHash) { throw 'deterministic package assembly failed: duplicate baseline ZIP hashes differ' }
& (Join-Path $PSScriptRoot 'verify-portable-release-package.ps1') -PackagePath $baseAZip | Out-Null
if ($LASTEXITCODE -ne 0) { throw 'baseline package verification failed' }
& (Join-Path $PSScriptRoot 'verify-portable-release-package.ps1') -PackagePath $candidateZip | Out-Null
if ($LASTEXITCODE -ne 0) { throw 'candidate package verification failed' }
$installRoot = Join-Path ([IO.Path]::GetTempPath()) ('medscale-058-install-' + [guid]::NewGuid().ToString('N'))
try {
    & (Join-Path $PSScriptRoot 'install-portable-release-package.ps1') -Action install -PackagePath $baseAZip -InstallRoot $installRoot
    if ($LASTEXITCODE -ne 0 -or (Get-ActiveLabel $installRoot) -ne 'baseline') { throw 'baseline install proof failed' }
    & (Join-Path $PSScriptRoot 'install-portable-release-package.ps1') -Action upgrade -PackagePath $candidateZip -InstallRoot $installRoot
    if ($LASTEXITCODE -ne 0 -or (Get-ActiveLabel $installRoot) -ne 'candidate') { throw 'candidate upgrade proof failed' }
    & (Join-Path $PSScriptRoot 'install-portable-release-package.ps1') -Action rollback -InstallRoot $installRoot
    if ($LASTEXITCODE -ne 0 -or (Get-ActiveLabel $installRoot) -ne 'baseline') { throw 'rollback proof failed' }
} finally {
    if (Test-Path $installRoot) { Remove-Item -Recurse -Force $installRoot }
}
$report = [ordered]@{
    schema_version = 1
    spec_id = '058-portable-release-package'
    git_sha = (git rev-parse HEAD).Trim()
    tree_sha = (git log -1 --format=%T).Trim()
    cargo_lock_sha256 = Get-Sha256Hex (Join-Path $repoRoot 'Cargo.lock')
    platform = if ($IsWindows) { 'windows' } elseif ($IsMacOS) { 'macos' } else { 'linux' }
    arch = [Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString().ToLowerInvariant()
    baseline_package_sha256 = $baseAHash
    candidate_package_sha256 = Get-Sha256Hex $candidateZip
    deterministic_package_assembly = $true
    portable_package_qualified = $true
    install_upgrade_rollback_qualified = $true
    signed = $false
    notarized = $false
    reproducible_binary_build = $false
    release_ready = $false
}
$enc = [Text.UTF8Encoding]::new($false)
[IO.File]::WriteAllText((Join-Path $evidenceFull 'qualification.json'), (($report | ConvertTo-Json -Depth 6).TrimEnd() + "`n"), $enc)
Write-Host 'PORTABLE_RELEASE_PACKAGE_QUALIFICATION_OK'
Write-Host "EVIDENCE=$(Join-Path $evidenceFull 'qualification.json')"
Write-Host 'LIMITATION: unsigned portable package only; native installers/signing/notarization/reproducible compiler output remain unqualified.'
