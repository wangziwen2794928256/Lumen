# THEME_SDK

Theme Pack是第四层，不是可执行插件。现有AppearanceProvider/tokens.css保持稳定；业务活动强度用语义token，主题颜色入口仍只有设置。

未来包仅允许已知token名称、严格颜色值、配对light/dark、有界字号/间距；拒绝CSS url()、@import、任意选择器/JS/远程字体。主题manifest/version/hash由Kernel校验并存themes/<id>/<version>；解析错误回退内置默认，不更改安全UI。

对比度至少4.5:1、focus可见、reduced-motion、三尺寸及特殊日区分是验收条件。主题安装/加载器本阶段延期，现有官方外观回归必须通过。

本文件对应英文版本：[THEME_SDK](../en/THEME_SDK.md)。
