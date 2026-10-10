# RELEASE

Phase16在Phase0–15已验证checkpoint之上准备Windows0.1.0发布候选。实际验收见docs/reports/PHASE_16_REPORT.md与LUMEN_V1_FINAL_REPORT.md；未运行、失败、待完成项目不得称为正式发布成功。

未来插件install：离线校验manifest/API/hash/签名策略→staged→disabled；enable需授权和runtime预检；disable撤销停止；update事务化包+独立DB备份、新权限同意→disabled；rollback仅兼容快照，失败保留旧包；uninstall停用后移除包，默认保留用户数据。纯状态机可测，安装器未实现。

scripts/release/build-windows.ps1生成锁定依赖的优化版NSIS当前用户安装包；已记录编译器崩溃后固定jobs1、incremental0、codegen1、优化级别2。缺少WebView2时使用微软bootstrapper，不绕过安全策略。签名需用户证书，未签名候选明确标记。生命周期测试使用专用QA产品/identifier/数据/安装路径，不卸载正式版；路径隔离不能声称全新Windows虚拟机验收。默认卸载保留AppData。

验收提交后scripts/release/package.ps1生成installer、portable、screenshots、database-schema、test-reports、docs、source、checksums，归档准确Git源码并逐文件验证SHA256。明确包含名单排除生产数据。便携程序仍使用Tauri AppData保存数据。公开发布、自动更新、完整插件市场、Wasm runtime、实际AI、移动端继续延期；不能在同一核心库交替使用不兼容版本。

本文件对应英文版本：[RELEASE](../en/RELEASE.md)。


## Phase17 发行更新

已新增 `.github/workflows/native-build.yml`，覆盖 macOS ARM64/Intel 与 Linux x64 原生目标。构建结果以实际 run 为准，编译不能代替系统集成验收。最新完整命令、平台缺口、glib 安全回补、私有源码发行与本地证据策略见 [技术手册](TECHNICAL_HANDBOOK.md) 和 [安装矩阵](INSTALLATION.md)。Phase16 历史报告/脚本属于本地验收参考，不全部进入 GitHub 白名单源码快照。
