# Generate a CycloneDX-like SBOM scaffold from cargo metadata (Spec 027).
# This is NOT a full release SBOM: native binaries, model weights, and signed
# package contents are out of scope. Prefer re-running from repo root.
#
# Usage:
#   pwsh ./scripts/generate-sbom-scaffold.ps1
#   pwsh ./scripts/generate-sbom-scaffold.ps1 -OutPath evidence/027-perf-sbom-release-evidence/sbom-scaffold.json

[CmdletBinding()]
param(
    [string]$OutPath = "evidence/027-perf-sbom-release-evidence/sbom-scaffold.json"
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

Write-Host "Generating SBOM scaffold via cargo metadata..."
$metadataJson = cargo metadata --format-version 1 --locked | Out-String
$metadata = $metadataJson | ConvertFrom-Json

$components = @()
foreach ($pkg in $metadata.packages) {
    $licenses = @()
    if ($pkg.license) {
        $licenses = @($pkg.license -split '\s+OR\s+|\s+/\s+' | ForEach-Object { $_.Trim() } | Where-Object { $_ })
    }
    $purl = "pkg:cargo/$($pkg.name)@$($pkg.version)"
    if ($pkg.source -and $pkg.source -like '*crates.io*') {
        $purl = "pkg:cargo/$($pkg.name)@$($pkg.version)"
    }
    $components += [ordered]@{
        type        = 'library'
        'bom-ref'   = $purl
        name        = $pkg.name
        version     = $pkg.version
        purl        = $purl
        licenses    = @($licenses | ForEach-Object { @{ license = @{ id = $_ } } })
        scope       = if ($pkg.source) { 'required' } else { 'required' }
    }
}

$bom = [ordered]@{
    bomFormat     = 'CycloneDX'
    specVersion   = '1.5'
    version       = 1
    serialNumber  = "urn:uuid:medscale-027-sbom-scaffold"
    metadata      = [ordered]@{
        timestamp = (Get-Date).ToUniversalTime().ToString('o')
        tools     = @(
            @{
                vendor  = 'MedScale'
                name    = 'generate-sbom-scaffold.ps1'
                version = '027'
            }
        )
        component = [ordered]@{
            type    = 'application'
            name    = 'MedScale'
            version = '0.0.0-workspace'
        }
        properties = @(
            @{ name = 'medscale:sbom_kind'; value = 'scaffold_cargo_metadata' }
            @{ name = 'medscale:release_ready'; value = 'false' }
            @{ name = 'medscale:includes_native_model_assets'; value = 'false' }
            @{ name = 'medscale:limitations'; value = 'Crates from cargo metadata only; not a signed release SBOM; native/model assets missing' }
        )
    }
    components = $components
}

$outFull = Join-Path $repoRoot $OutPath
$outDir = Split-Path -Parent $outFull
if (-not (Test-Path $outDir)) {
    New-Item -ItemType Directory -Force -Path $outDir | Out-Null
}

($bom | ConvertTo-Json -Depth 12) | Set-Content -Path $outFull -Encoding utf8
Write-Host "Wrote $outFull ($($components.Count) components)"
Write-Host "LIMITATION: not a full RELEASE_READY SBOM (native/model assets missing; unsigned)."
