# Installation, upgrade and support

| Platform | Package | Support level |
| --- | --- | --- |
| Windows x64 | NSIS setup / portable exe | Unsigned Phase16 candidate with local acceptance |
| macOS Apple Silicon | aarch64 DMG / app | Experimental; check actual CI result |
| macOS Intel | x86_64 DMG / app | Experimental; check actual CI result |
| Linux x64 | deb / AppImage | Experimental; check actual CI result |

Repository: https://github.com/wangziwen2794928256/Lumen . It is private by default and requires authorized account access. Manually run Native experimental packages under Actions and select the correct architecture artifact. Successful outputs include BUILD.json and SHA256SUMS. CI artifacts expire after 30 days; retain a verified local copy.

Windows uses current-user installation. Follow Microsoft's WebView2 installation process if required. Create an application-consistent backup before upgrading. Phase16 isolated acceptance verified that uninstall retained AppData; do not manually delete unbacked-up data. Phase17 preserves the already accepted Windows binary and shortcut.

The experimental macOS build minimum is 14.0. Choose Apple Silicon or Intel correctly. Candidates have no signing/notarization; use OS-supported approval or wait for signing, never disable system protections. Linux builds run on Ubuntu 22.04. deb needs compatible WebKitGTK 4.1/runtime libraries; AppImage is not proof of compatibility with every distribution or desktop.

macOS/Linux session-event monitoring and several open/deep-link adapters remain unavailable. Today/focus native commands can return NATIVE_HOOK_UNAVAILABLE. Use an isolated OS user and test records; never put the only production data at risk. See the [handbook](TECHNICAL_HANDBOOK.md).

Verify files with PowerShell Get-FileHash -Algorithm SHA256, macOS shasum -a 256, or Linux sha256sum -c SHA256SUMS against the trusted delivery manifest.
