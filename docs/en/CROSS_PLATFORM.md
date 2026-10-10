# CROSS_PLATFORM

kernel/platform centralizes Adapter capabilities and host OS selection. Business code calls install_system_events/open_verified_attachment without scattered OS conditions.

Windows retains WTS lock/power handling, subclass cleanup and verified ShellExecute attachment opening. Its native safety test moves with the adapter. macOS defines an NSWorkspace/session boundary; Linux defines logind/portal boundaries. Both report unsupported capabilities and explicit errors until implemented, rather than claiming lock-safe focus support.

Future native builds/tests must cover lock, sleep, shortcuts, attachments and directory permissions on each OS. Portable libraries are not evidence of passed cross-platform integration. Mobile is excluded.

Matching Chinese document：[CROSS_PLATFORM](../zh-CN/CROSS_PLATFORM.md)。
