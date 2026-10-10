# 安装、升级与支持范围

| 平台 | 文件 | 当前支持级别 |
| --- | --- | --- |
| Windows x64 | Lumen_0.1.0_x64-setup.exe / Lumen.exe | Phase16 本机验收通过的未签名候选 |
| macOS Apple Silicon | aarch64 DMG / app | 实验目标；以实际 CI 结果为准 |
| macOS Intel | x86_64 DMG / app | 实验目标；以实际 CI 结果为准 |
| Linux x64 | deb / AppImage | 实验目标；以实际 CI 结果为准 |

GitHub 仓库：https://github.com/wangziwen2794928256/Lumen 。当前为私有发行准备仓库，需要账号访问。Actions 中手动运行 Native experimental packages，下载与目标架构一致的 artifact。每个成功产物目录带 BUILD.json 和 SHA256SUMS；保存文件后核对 SHA256。Artifact 保留 30 天，最终交付另有本地副本。

Windows 安装为 currentUser，无需为 Lumen 请求管理员权限；缺少 WebView2 时按微软官方安装流程处理。升级前在设置创建一致性备份。卸载不应删除 AppData，Phase16 隔离验收已验证；不要在未备份情况下手动删除目录。桌面快捷方式的固定 Windows 候选位于此前交付目录，Phase17 不替换已验收二进制。

macOS 最低实验构建配置为 14.0；Apple Silicon 与 Intel 包不能混淆。包尚未签名/公证；若系统阻止运行，请等待签名候选或使用系统支持的审批方式，不关闭系统安全机制。Linux 使用 Ubuntu 22.04 原生构建，deb 需要匹配的 WebKitGTK 4.1 等运行依赖；AppImage 仍依赖目标系统/桌面环境，不能承诺所有发行版兼容。

macOS/Linux 尚无锁屏/睡眠监听与部分平台打开能力，Today/focus 原生命令可能返回 NATIVE_HOOK_UNAVAILABLE。不要将这些实验包用于唯一生产数据；优先独立 OS 用户与测试记录。编译通过不等于完整跨平台验收。详细行为见 [技术手册](TECHNICAL_HANDBOOK.md)。

校验示例：PowerShell `Get-FileHash ./安装包 -Algorithm SHA256`；macOS `shasum -a 256 文件`；Linux `sha256sum -c SHA256SUMS`。只有与可信交付 manifest 一致的文件才用于测试。
