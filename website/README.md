# Lumen static portfolio

This directory contains the static renderer only. Production exports are created by Kernel in the user's AppData, never committed here. There is no default personal JSON or database access.

Export via **Profile → Review → Export local website**, then:

```powershell
node scripts/preview-website.mjs 'C:\path\to\exported\website-UUID'
```

The terminal prints the loopback URL. Ctrl+C stops preview. No deployment occurs. Only index.html/site.js/site.css/public-profile.json are served; copying these files is enough to run independently of Lumen and SQLite.

Schema and privacy boundaries: [English](../docs/en/PUBLIC_WEBSITE.md), [中文](../docs/zh-CN/PUBLIC_WEBSITE.md).
