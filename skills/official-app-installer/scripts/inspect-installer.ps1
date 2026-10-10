param(
  [Parameter(Mandatory = $true)][string]$Path,
  [Parameter(Mandatory = $true)][string]$ExpectedPublisher,
  [ValidatePattern('^[A-Fa-f0-9]{64}$')][string]$ExpectedSha256
)
$ErrorActionPreference = 'Stop'
$installer = Get-Item -LiteralPath $Path
if ($installer.PSIsContainer -or $installer.Extension -notin @('.exe', '.msi', '.msix', '.msixbundle')) { throw 'Expected an installer file' }
$hash = (Get-FileHash -LiteralPath $installer.FullName -Algorithm SHA256).Hash
$signature = Get-AuthenticodeSignature -LiteralPath $installer.FullName
$publisher = if ($signature.SignerCertificate) { $signature.SignerCertificate.Subject } else { $null }
$publisherMatches = $publisher -and [string]::Equals($publisher, $ExpectedPublisher, [StringComparison]::OrdinalIgnoreCase)
$hashMatches = if ($ExpectedSha256) { $hash -eq $ExpectedSha256 } else { $null }
$verified = $signature.Status -eq 'Valid' -and $publisherMatches -and ($null -eq $hashMatches -or $hashMatches)
[ordered]@{
  path = $installer.FullName
  version = $installer.VersionInfo.FileVersion
  sha256 = $hash
  signature = [string]$signature.Status
  publisher = $publisher
  publisherMatches = [bool]$publisherMatches
  officialHashProvided = [bool]$ExpectedSha256
  hashMatches = $hashMatches
  verified = [bool]$verified
  sourceOwnership = 'Must be independently verified from official vendor links'
  executed = $false
} | ConvertTo-Json
if (-not $verified) { exit 2 }
