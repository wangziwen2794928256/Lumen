# PLUGIN_SECURITY

Kernel owns security at compile time. Manifest validation is not execution authorization: authorize_community_execution always returns PLUGIN_RUNTIME_DEFERRED. No community JavaScript or native DLL is loaded into the application renderer.

Preferred future design: a separate Wasmtime worker, Component Model and explicit capability imports; no ambient WASI filesystem/network. Proposed limits: 32MiB memory, 64KiB result/value, fuel plus wall-time limits and independently bounded host I/O. These are design values, not implemented or benchmarked limits.

Wasmtime capability APIs do not correct overly broad host grants. Fuel/epoch interruption cannot replace blocking I/O timeouts. Future stores, revoked grants and quotas bind plugin identity/version/content hash. Negative sandbox tests are required before enabling guests. Sources: [Wasmtime Security](https://docs.wasmtime.dev/security.html), [Execution Limits](https://docs.wasmtime.dev/examples-interrupting-wasm.html), [Component Worlds](https://component-model.bytecodealliance.org/design/worlds.html).

Matching Chinese document：[PLUGIN_SECURITY](../zh-CN/PLUGIN_SECURITY.md)。
