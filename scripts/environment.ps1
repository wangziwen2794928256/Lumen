$ErrorActionPreference = 'Stop'
$toolchainRoot = if ($env:LUMEN_TOOLCHAIN_ROOT) { $env:LUMEN_TOOLCHAIN_ROOT } else { 'D:\LumenToolchain' }
$env:CARGO_HOME = Join-Path $toolchainRoot 'cargo'
$env:RUSTUP_HOME = Join-Path $toolchainRoot 'rustup'
$env:PATH = (Join-Path $env:CARGO_HOME 'bin') + ';' + $env:PATH
$bundledRoot = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies'
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
  $nodeDir = Join-Path $bundledRoot 'node\bin'
  if (Test-Path (Join-Path $nodeDir 'node.exe')) { $env:PATH = "$nodeDir;" + $env:PATH }
}
if (-not (Get-Command pnpm -ErrorAction SilentlyContinue)) {
  $pnpmDir = Join-Path $bundledRoot 'bin\fallback'
  if (Test-Path (Join-Path $pnpmDir 'pnpm.cmd')) { $env:PATH = "$pnpmDir;" + $env:PATH }
}
foreach ($command in 'node','pnpm','cargo') {
  if (-not (Get-Command $command -ErrorAction SilentlyContinue)) { throw "$command unavailable. See docs/environment/dependency_decisions.md" }
}
