# PERMISSION_MODEL

Community modules have zero authority by default. Only storage.own, widget.contribute and plan.propose may be declared; core.sqlite, arbitrary filesystem/network, native execution and Kernel removal are rejected. Declared requests and user grants are distinct. There is no grant UI or execution authorization in this phase.

Future Kernel-issued opaque handles bind plugin identity/version/content hash/API/action scope/quota/expiry. Expanded update permissions require new consent. Disable/uninstall revokes handles and stops the worker before changing state.

Storage proxies own KV/database operations, never guest SQL. Any future network feature must constrain origin/method, redirects, private addresses/DNS rebinding and response sizes, rather than network.any. Self-declared official status grants no trust.

Matching Chinese document：[PERMISSION_MODEL](../zh-CN/PERMISSION_MODEL.md)。
