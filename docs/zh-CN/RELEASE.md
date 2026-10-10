# RELEASE

Phase17 在已验收 Phase16 Windows0.1.0 候选之上追加私有 prerelease：保持原 hash 的安装/便携程序、实验 macOS ARM64/Intel DMG、Linux x64 deb/AppImage、双语文档、可编辑中文 PPT、源码 ZIP、CI 证据和 SHA256 manifest。BUILD_MANIFEST.json 分别记录原生构建提交与最终 publication commit；见 [构建结果](../distribution/BUILD_RESULTS.md)。原 Windows 验收报告留在本地；失败/待运行不得标 PASS。

未来插件install：离线校验manifest/API/hash/签名策略→staged→disabled；enable需授权和runtime预检；disable撤销停止；update事务化包+独立DB备份、新权限同意→disabled；rollback仅兼容快照，失败保留旧包；uninstall停用后移除包，默认保留用户数据。纯状态机可测，安装器未实现。

scripts/release/build-windows.ps1生成锁定依赖的优化版NSIS当前用户安装包；已记录编译器崩溃后固定jobs1、incremental0、codegen1、优化级别2。缺少WebView2时使用微软bootstrapper，不绕过安全策略。签名需用户证书，未签名候选明确标记。生命周期测试使用专用QA产品/identifier/数据/安装路径，不卸载正式版；路径隔离不能声称全新Windows虚拟机验收。默认卸载保留AppData。

原 scripts/release/package.ps1 保留 Phase16 本地包；Phase17 scripts/distribution/assemble-delivery.py 核验已下载 CI 包/receipt、原 Windows/PPT hash 和源码 blob，生成源码/文档/证据 ZIP、BUILD_MANIFEST.json/SHA256SUMS。按授权上传私有 prerelease、比对 GitHub digest、核验 tag，并保存外部本地副本。不上传生产 SQLite/日记/音乐历史/私人原生截图/凭证/本地历史。便携版仍使用 AppData。原生 workflow 仅读取仓库，发行上传独立执行。不部署网站、不更改可见性、不自动更新；完整市场/runtime/AI/移动端延期。

本文件对应英文版本：[RELEASE](../en/RELEASE.md)。


## Phase17 发行更新

已新增 `.github/workflows/native-build.yml`，覆盖 macOS ARM64/Intel 与 Linux x64 原生目标。构建结果以实际 run 为准，编译不能代替系统集成验收。最新完整命令、平台缺口、glib 安全回补、私有源码发行与本地证据策略见 [技术手册](TECHNICAL_HANDBOOK.md) 和 [安装矩阵](INSTALLATION.md)。Phase16 历史报告/脚本属于本地验收参考，不全部进入 GitHub 白名单源码快照。
