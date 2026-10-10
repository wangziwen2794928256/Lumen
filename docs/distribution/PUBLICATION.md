# Source publication policy

The GitHub repository starts with a source-only allowlisted snapshot, not the owner's complete local phase history. The original history and Phase0–16 acceptance remain intact locally. This avoids uploading personal/native screenshots and machine logs while keeping commit/file provenance in UPSTREAM_CHECKPOINT.json.

Private repository: https://github.com/wangziwen2794928256/Lumen . No Git remote is attached to the original local history because it and the source-only publication have different roots. A dedicated ignored work/phase17-publish checkout owns origin. Do not force-push or merge unrelated histories to hide this distinction.

Maintainers can clone the GitHub source and develop normally from its main branch. To publish subsequent local-owner changes, review the allowlist and source diff, export to the dedicated checkout, commit, verify the committed blobs, then push:

```sh
python scripts/distribution/export-source.py work/phase17-publish
git -C work/phase17-publish diff --stat
git -C work/phase17-publish add .
git -C work/phase17-publish commit -m "Describe reviewed source changes"
python scripts/distribution/verify-source.py work/phase17-publish
git -C work/phase17-publish push origin main
```

The export fails on credential patterns, forbidden data extensions, unexpected existing files or symlink escape. Hashes describe canonical Git blob bytes (LF text, unchanged binary). Review remains necessary: a regex is not proof that arbitrary newly added files contain no private content. Never copy production data into an allowlisted source directory.

The native workflow is manual-only, contents:read, with pinned official actions. It uploads CI artifacts, not release assets. Owner-authorized release upload uses existing Git authentication in memory; no token is stored in source, logs, environment files or workflow secrets. Windows Phase16 assets retain their original provenance; experimental native assets carry their actual BUILD.json commit and SHA256.

PPT and installer binaries are release assets rather than source tree dependencies. Private status is the default staging choice; changing visibility is a separate owner decision. No website deployment occurs.
