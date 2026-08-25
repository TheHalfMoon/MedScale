# Check MedScale crate dependency direction.
# Allowed:
#   cli -> core -> contracts
#   core -> storage -> contracts
#   core -> fhir -> contracts
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
$storage = Get-Package 'medscale-storage'
$fhir = Get-Package 'medscale-fhir'

$failures = @()

foreach ($name in @('medscale-contracts')) {
    $pkg = Get-Package $name
    foreach ($forbidden in @('medscale-core', 'medscale-cli', 'medscale-storage', 'medscale-fhir')) {
        if (Test-DependsOn -Package $pkg -DepName $forbidden) {
            $failures += "$name must not depend on $forbidden"
        }
    }
}

if (Test-DependsOn -Package $core -DepName 'medscale-cli') {
    $failures += 'medscale-core must not depend on medscale-cli'
}
if (-not (Test-DependsOn -Package $core -DepName 'medscale-contracts')) {
    $failures += 'medscale-core must depend on medscale-contracts'
}
if (-not (Test-DependsOn -Package $core -DepName 'medscale-storage')) {
    $failures += 'medscale-core must depend on medscale-storage'
}
if (-not (Test-DependsOn -Package $core -DepName 'medscale-fhir')) {
    $failures += 'medscale-core must depend on medscale-fhir'
}
if (-not (Test-DependsOn -Package $cli -DepName 'medscale-core')) {
    $failures += 'medscale-cli must depend on medscale-core'
}
if (-not (Test-DependsOn -Package $storage -DepName 'medscale-contracts')) {
    $failures += 'medscale-storage must depend on medscale-contracts'
}
if (-not (Test-DependsOn -Package $fhir -DepName 'medscale-contracts')) {
    $failures += 'medscale-fhir must depend on medscale-contracts'
}
if (Test-DependsOn -Package $storage -DepName 'medscale-core') {
    $failures += 'medscale-storage must not depend on medscale-core'
}
if (Test-DependsOn -Package $fhir -DepName 'medscale-core') {
    $failures += 'medscale-fhir must not depend on medscale-core'
}

if ($failures.Count -gt 0) {
    Write-Error ("Dependency direction check failed:`n - " + ($failures -join "`n - "))
    exit 1
}

Write-Host 'Dependency direction check passed (cli -> core -> {storage,fhir} -> contracts).'
