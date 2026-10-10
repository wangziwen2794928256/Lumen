# Security

默认 private；生产数据在 AppData。日志不记录 token、密码、日记正文。禁用远程数据库入口，所有 SQL 参数化。依赖审计与凭证扫描在 release 前必须通过。安全问题先停止扩展并记录本地报告，不公开私人信息。
