# THEME_SDK

Theme Packs form the fourth layer and contain no executable plugin. Existing AppearanceProvider and tokens.css remain stable, with one appearance settings entry. Activity intensity uses semantic tokens.

Future packs allow known token names, strict colors, paired light/dark modes and bounded metrics. Reject CSS URLs/imports, arbitrary selectors/scripts and remote fonts. Kernel validates manifest/version/hash under themes/<id>/<version>; invalid packs fall back to built-in defaults without altering security UI.

Acceptance requires 4.5:1 contrast, visible focus, reduced motion, three viewports and distinguishable protected days. Installation/loading is deferred; built-in themes remain regression-tested.

Matching Chinese document：[THEME_SDK](../zh-CN/THEME_SDK.md)。
