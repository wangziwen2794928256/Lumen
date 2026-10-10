param([string]$Executable = (Join-Path (Split-Path $PSScriptRoot -Parent) 'builds\Lumen.exe'))
$ErrorActionPreference = 'Stop'
$shortcutTarget = (Resolve-Path -LiteralPath $Executable).Path
if ([IO.Path]::GetExtension($shortcutTarget) -ne '.exe') { throw 'Expected a Windows executable' }
$desktopDirectory = [Environment]::GetFolderPath('Desktop')
$shortcutPath = Join-Path $desktopDirectory 'Lumen.lnk'
$shortcutShell = New-Object -ComObject WScript.Shell
if (Test-Path -LiteralPath $shortcutPath) {
  $existing = $shortcutShell.CreateShortcut($shortcutPath)
  if ($existing.TargetPath -ne $shortcutTarget) { throw 'Existing Lumen shortcut points elsewhere; preserved it.' }
}
$shortcut = $shortcutShell.CreateShortcut($shortcutPath)
$shortcut.TargetPath = $shortcutTarget
$shortcut.WorkingDirectory = Split-Path $shortcutTarget -Parent
$shortcut.IconLocation = "$shortcutTarget,0"
$shortcut.Description = 'Lumen 本地个人成长操作系统'
$shortcut.Save()
$verified = $shortcutShell.CreateShortcut($shortcutPath)
if ($verified.TargetPath -ne $shortcutTarget -or -not (Test-Path -LiteralPath $shortcutPath)) { throw 'Shortcut verification failed' }
[PSCustomObject]@{shortcut=$shortcutPath;target=$verified.TargetPath;working_directory=$verified.WorkingDirectory;icon=$verified.IconLocation}
