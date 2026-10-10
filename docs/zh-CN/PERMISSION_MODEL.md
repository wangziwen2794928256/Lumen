# PERMISSION_MODEL

社区默认零授权，当前可声明storage.own、widget.contribute、plan.propose；core.sqlite、任意filesystem/network、native.execute、kernel.uninstall一律拒绝。声明与用户授予分开；本阶段没有权限授予UI或执行授权。

未来capability由Kernel生成不可伪造句柄，绑定plugin id/version/content hash/API/操作范围/大小配额/有效期。更新增加权限必须重新同意；disable/uninstall先撤销全部句柄并终止worker，拒绝旧令牌重用。

storage只代理插件自己的KV/数据库，不让插件提交SQL。网络若未来开放，需要精确origin/method、重定向重新校验、禁止localhost/private-IP/DNS重绑定及响应大小限制，不能泛化network.any。插件自报official不能提高权限。

本文件对应英文版本：[PERMISSION_MODEL](../en/PERMISSION_MODEL.md)。
