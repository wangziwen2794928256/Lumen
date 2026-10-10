param(
  [string]$TargetDirectory = 'D:\LumenToolchain\target-release16-rust1981',
  [string]$AdditionalConfig,
  [ValidatePattern('^[A-Fa-f0-9]{1,16}$')][string]$AffinityMask
)
$ErrorActionPreference = 'Stop'
$root = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
Set-Location $root
. scripts/environment.ps1
$prior = @{}
$priorAffinity = (Get-Process -Id $PID).ProcessorAffinity
foreach ($key in 'CARGO_TARGET_DIR','CARGO_BUILD_JOBS','CARGO_INCREMENTAL','CARGO_PROFILE_RELEASE_CODEGEN_UNITS','CARGO_PROFILE_RELEASE_OPT_LEVEL') { $prior[$key] = [Environment]::GetEnvironmentVariable($key) }
try {
  if ($AffinityMask) { (Get-Process -Id $PID).ProcessorAffinity = [IntPtr]::new([Convert]::ToInt64($AffinityMask, 16)) }
  $env:CARGO_TARGET_DIR = $TargetDirectory
  $env:CARGO_BUILD_JOBS = '1'
  $env:CARGO_INCREMENTAL = '0'
  $env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS = '1'
  $env:CARGO_PROFILE_RELEASE_OPT_LEVEL = '2'
  $buildArgs = @('tauri','build','--config','src-tauri/tauri.release.conf.json')
  if ($AdditionalConfig) {
    if (-not (Test-Path -LiteralPath $AdditionalConfig -PathType Leaf)) { throw 'Additional config missing' }
    $buildArgs += @('--config', $AdditionalConfig)
  }
  $buildArgs += @('--bundles','nsis','--','--locked')
  & pnpm @buildArgs
  if ($LASTEXITCODE -ne 0) { throw 'Windows release build failed' }
} finally {
  (Get-Process -Id $PID).ProcessorAffinity = $priorAffinity
  foreach ($key in $prior.Keys) { [Environment]::SetEnvironmentVariable($key, $prior[$key]) }
}
