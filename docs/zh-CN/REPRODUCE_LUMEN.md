# REPRODUCE_LUMEN

从checkpoint/15-release取候选版确定源码与锁文件；git archive源码不含个人数据。运行scripts/verify/phase16.ps1生成phase16-gates.json与按门禁日志，E2E覆盖三尺寸，浏览器fixture明确区别原生。

性能大数据仅TempDir自动生成，Phase9复现命令和短正文限制保留在INSIGHTS.md；本阶段不训练、不下载模型。UTC记录与本地日期按数据契约处理，不把机器时区隐藏。

发布包提供完整commit/tag、SHA256清单、优化安装包/便携程序与证据。可复现指源码/依赖/命令/fixture可追溯；当前不是跨机器位级相同二进制承诺，编译器/SDK/linker时间戳等还未固定。

本文件对应英文版本：[REPRODUCE_LUMEN](../en/REPRODUCE_LUMEN.md)。

Phase16证据：最终报告/checkpoint记录准确提交；scripts/verify/phase16.ps1与scripts/release/build-windows.ps1定义当前复现命令。参考发行使用opt-level2/codegen1，编译器失败日志保留；SHA256验证交付文件身份，不宣称跨机器逐字节一致。
