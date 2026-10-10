# CROSS_PLATFORM

kernel/platform centralizes Adapter capabilities and host OS selection. Business code calls install_system_events/open_verified_attachment without scattered OS conditions.

Windows retains WTS lock/power handling, subclass cleanup and verified ShellExecute attachment opening. Its native safety test moves with the adapter. macOS defines an NSWorkspace/session boundary; Linux defines logind/portal boundaries. Both report unsupported capabilities and explicit errors until implemented, rather than claiming lock-safe focus support.

Future native builds/tests must cover lock, sleep, shortcuts, attachments and directory permissions on each OS. Portable libraries are not evidence of passed cross-platform integration. Mobile is excluded.

Matching Chinese document：[CROSS_PLATFORM](../zh-CN/CROSS_PLATFORM.md)。


## Phase17 distribution update

Native macOS ARM64/Intel and Linux x64 workflows now live in `.github/workflows/native-build.yml`. Build results must come from the actual run; compilation does not establish OS integration support. See [current handbook](TECHNICAL_HANDBOOK.md) and [installation matrix](INSTALLATION.md) for complete commands, platform gaps, the glib backport, private source publication and retained local evidence. Historical Phase16 reports/build scripts are local acceptance references; they are not all included in the allowlisted GitHub source snapshot.
