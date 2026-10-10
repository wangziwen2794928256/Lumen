# REPRODUCE_LUMEN

本地 Phase16 checkpoint bcee0483ae5c072e49307dada2bee479a830ef99 对应 Windows 候选。GitHub 是另一个起点的白名单源码历史，通过 publication commit 和 UPSTREAM_CHECKPOINT.json 追溯，不应查找本地专用 checkpoint。按 BUILD_MANIFEST.json/SHA256SUMS 核对源码和包，再执行 BUILD_FROM_SOURCE、TESTING 中命令。本地原验收脚本/报告不全部上传；浏览器 fixture 区别于原生。

性能大数据仅在 TempDir 自动生成，不使用生产 AppData。十年基准在 Phase16 独立运行，不自动算作所有 Phase17 runner 已运行。本阶段不训练、不下载模型；UTC/本地日期和时区见技术手册。

发布包提供完整commit/tag、SHA256清单、优化安装包/便携程序与证据。可复现指源码/依赖/命令/fixture可追溯；当前不是跨机器位级相同二进制承诺，编译器/SDK/linker时间戳等还未固定。

本文件对应英文版本：[REPRODUCE_LUMEN](../en/REPRODUCE_LUMEN.md)。

原生 CI 保留工具版本、构建提交、日志和 BUILD.json；verify-packages.yml 接收分别的 macOS/Linux run ID，要求完整镜像/包/处理器架构覆盖。scripts/distribution/assemble-delivery.py 根据已下载核验的 receipt 和 publication checkout 汇总源码、双语文档、CI 证据 ZIP 与 manifest/checksums，普通源码构建不依赖它。参考配置 opt2/codegen1。CI artifact 保留30天，发行/本地副本长期交付；SHA256 验证文件身份，不宣称跨机器逐字节一致。


## Phase17 发行更新

已新增 `.github/workflows/native-build.yml`，覆盖 macOS ARM64/Intel 与 Linux x64 原生目标。构建结果以实际 run 为准，编译不能代替系统集成验收。最新完整命令、平台缺口、glib 安全回补、私有源码发行与本地证据策略见 [技术手册](TECHNICAL_HANDBOOK.md) 和 [安装矩阵](INSTALLATION.md)。Phase16 历史报告/脚本属于本地验收参考，不全部进入 GitHub 白名单源码快照。
