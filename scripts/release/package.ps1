param(
  [Parameter(Mandatory = $true)][string]$Destination,
  [Parameter(Mandatory = $true)][string]$Installer,
  [Parameter(Mandatory = $true)][string]$Portable
)
$ErrorActionPreference = 'Stop'
$root = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
Set-Location $root
if (git status --porcelain) { throw 'Commit the verified release first so source archive and reports match' }
if (Test-Path -LiteralPath $Destination) { throw 'Choose a new empty release destination; existing output is preserved' }
$installerFile = Get-Item -LiteralPath $Installer
$portableFile = Get-Item -LiteralPath $Portable
if ($installerFile.Extension -ne '.exe' -or $portableFile.Extension -ne '.exe') { throw 'Expected Windows executables' }
$destinationRoot = [IO.Path]::GetFullPath($Destination)
foreach ($folder in 'installer','portable','screenshots','database-schema','test-reports','docs','checksums','source') {
  New-Item -ItemType Directory -Path (Join-Path $destinationRoot $folder) -Force | Out-Null
}
Copy-Item -LiteralPath $installerFile.FullName -Destination (Join-Path $destinationRoot 'installer')
Copy-Item -LiteralPath $portableFile.FullName -Destination (Join-Path $destinationRoot 'portable/Lumen.exe')
Copy-Item -Path 'src-tauri/migrations/*.sql' -Destination (Join-Path $destinationRoot 'database-schema')
Copy-Item -LiteralPath 'docs/en','docs/zh-CN','docs/reports','skills' -Destination (Join-Path $destinationRoot 'docs') -Recurse
Copy-Item -LiteralPath 'docs/testing/phase16' -Destination (Join-Path $destinationRoot 'test-reports') -Recurse
Copy-Item -LiteralPath 'docs/testing/phase16-gates.json','LUMEN_V1_FINAL_REPORT.md','LICENSE','README.md' -Destination $destinationRoot
Get-ChildItem 'tests/screenshots/phase16-*' -File | Copy-Item -Destination (Join-Path $destinationRoot 'screenshots')
$commit = (git rev-parse HEAD).Trim()
$archivePath = Join-Path $destinationRoot 'source/Lumen-source.zip'
git archive --format=zip "--output=$archivePath" $commit
if ($LASTEXITCODE -ne 0) { throw 'Git source archive failed' }
@{commit=$commit;branch=(git branch --show-current).Trim();version='0.1.0';kind='unsigned release candidate';created=(Get-Date).ToUniversalTime().ToString('o');productionDataIncluded=$false} | ConvertTo-Json | Set-Content (Join-Path $destinationRoot 'CHECKPOINT.json')
$checksums = Get-ChildItem -LiteralPath $destinationRoot -File -Recurse | Sort-Object FullName | ForEach-Object {
  $relative = [IO.Path]::GetRelativePath($destinationRoot, $_.FullName).Replace('\','/')
  '{0}  {1}' -f (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant(), $relative
}
$manifest = Join-Path $destinationRoot 'checksums/SHA256SUMS'
$checksums | Set-Content -LiteralPath $manifest -Encoding utf8
foreach ($line in Get-Content -LiteralPath $manifest) {
  $expected, $relative = $line -split '  ', 2
  $actual = (Get-FileHash -LiteralPath (Join-Path $destinationRoot $relative) -Algorithm SHA256).Hash
  if ($actual -ne $expected) { throw "Checksum verification failed: $relative" }
}
Write-Output "Verified $($checksums.Count) release files at $destinationRoot"
