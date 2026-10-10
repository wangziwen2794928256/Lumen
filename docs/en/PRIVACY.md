# PRIVACY

Private by default, offline usable, with no telemetry or automatic plugin networking. Kernel never gives community modules lumen.db, host paths or file handles. Official modules share a trusted process and are not claimed to be mutually sandboxed.

Future plugin data.db/KV lives under AppData/plugins/<id> and is host-managed. Uninstall retains isolated data by default; deletion requires a separate explicit request. Logs/cache must exclude bodies. Core/attachment backup exists; future plugin backup needs per-plugin consent and version metadata.

Local SQLite/backups are currently unencrypted. OS account controls, permissions and disk encryption remain host protections. Private exports may still contain sensitive words/labels. Export is explicit; no AI sharing is introduced.

Matching Chinese document：[PRIVACY](../zh-CN/PRIVACY.md)。
