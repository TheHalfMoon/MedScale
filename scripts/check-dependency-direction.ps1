# Check MedScale crate dependency direction (Spec 001).
# Allowed: cli -> core -> contracts
$ErrorActionPreference = 'Stop'

$metadataJson = cargo metadata --format-version 1 --no-deps | Out-String
$metadata = $metadataJson | ConvertFrom-Json

function Get-Package {
    param([string]$Name)
    $pkg = $metadata.packages | Where-Object { $_.name -eq $Name } | Select-Object -First 1
    if (-not $pkg) {
        throw "Missing package $Name in workspace metadata"
    }
    return $pkg
}

function Test-DependsOn {
    param(
        [Parameter(Mandatory = $true)]$Package,
        [Parameter(Mandatory = $true)][string]$DepName
    )
    return [bool]($Package.dependencies | Where-Object { $_.name -eq $DepName })
}

$contracts = Get-Package 'medscale-contracts'
$core = Get-Package 'medscale-core'
$cli = Get-Package 'medscale-cli'

$failures = @()

if (Test-DependsOn -Package $contracts -DepName 'medscale-core') {
    $failures += 'medscale-contracts must not depend on medscale-core'
}
if (Test-DependsOn -Package $contracts -DepName 'medscale-cli') {
    $failures += 'medscale-contracts must not depend on medscale-cli'
}
if (Test-DependsOn -Package $core -DepName 'medscale-cli') {
    $failures += 'medscale-core must not depend on medscale-cli'
}
if (-not (Test-DependsOn -Package $core -DepName 'medscale-contracts')) {
    $failures += 'medscale-core must depend on medscale-contracts'
}
if (-not (Test-DependsOn -Package $cli -DepName 'medscale-core')) {
    $failures += 'medscale-cli must depend on medscale-core'
}

if ($failures.Count -gt 0) {
    Write-Error ("Dependency direction check failed:`n - " + ($failures -join "`n - "))
    exit 1
}

Write-Host 'Dependency direction check passed (cli -> core -> contracts).'
