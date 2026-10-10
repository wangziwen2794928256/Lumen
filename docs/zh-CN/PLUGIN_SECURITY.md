# PLUGIN_SECURITY

Kernel在编译期拥有安全和权限，不允许社区模块改变安全策略。validate_manifest通过只代表格式/权限合法；authorize_community_execution始终返回PLUGIN_RUNTIME_DEFERRED。宿主renderer不加载社区JS/native DLL。

候选方案：Wasmtime独立worker进程 + Wasm Component Model + 显式capability host imports，默认不链接WASI filesystem/network。实例内存上限建议32MiB、调用返回64KiB、opaque value单项64KiB、CPU fuel与wall-time双限、阻塞host API独立超时；这些是未来设计值，本阶段未执行或测量。

Wasmtime支持WASI capability文件访问，但错误宿主授权仍可泄露数据；fuel/epoch限制不代替host I/O超时。未来每插件、版本与内容hash绑定独立store、撤销令牌、资源配额；先验证负面沙箱用例再允许启用。参考：[Wasmtime Security](https://docs.wasmtime.dev/security.html)、[Interrupting Execution](https://docs.wasmtime.dev/examples-interrupting-wasm.html)、[Component Worlds](https://component-model.bytecodealliance.org/design/worlds.html)。

本文件对应英文版本：[PLUGIN_SECURITY](../en/PLUGIN_SECURITY.md)。
