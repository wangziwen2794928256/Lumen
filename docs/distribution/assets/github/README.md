# GitHub product visuals / 首页视觉

Six bilingual PNGs use real source-rendered UI with synthetic fixtures, never production SQLite. Screenshots are cropped for the compositions. Canvas source is editable; no generated application controls are used.

Reproduce from repository root after installing documented Node/pnpm dependencies and Microsoft Edge:

```powershell
pnpm exec playwright test -c scripts/distribution/readme-capture.config.ts
node scripts/distribution/build-github-visuals.mjs
```

The capture harness mocks native IPC only in its isolated browser context. Community runtime, AI and mobile are future work. PNG dimensions: hero 1920×1540; workflow/modules 1920×1120. MIT project license applies. README links remain text links outside images for keyboard access and narrow screens.
