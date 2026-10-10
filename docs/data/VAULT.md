# Vault data contract (Phase 11)

Official Module `lumen.vault`, API 1.0. Kernel owns storage, migrations, file selection boundary and verified platform opener. No network or AI calls.

Migration 010 extends the existing artifacts/artifact_files tables; migrations 001–009 remain unchanged. New fields: artifact_date, role, organization, skills/technologies JSON arrays, repository/project/paper URLs, journey/competition foreign keys, revision. File link adds original name; attachments already stores relative path/MIME/size/SHA-256. vault_receipts stores request ID, payload hash, artifact ID and timestamp. Visibility uses existing public_visibility with entity_type=artifact; absent metadata means private. Legacy kinds such as note remain untouched until the user edits and explicitly saves a supported category.

## Commands

- vault_snapshot: latest 1000 artifacts including archive/trash, live reference choices. No files included. Local trusted Official Module only.
- vault_action: strict tagged save/state payload. Save validates canonical date, bounded strings/tags, HTTP(S) URL without credentials, live references and revision. New revision zero; saved revision increments. State=archive/trash/restore; restore clears both tombstones. BEGIN IMMEDIATE + receipt prevents duplicate ACK retries and stale overwrites. Renderer caches one pending payload locally until acknowledgment; this is private local data.
- vault_files: manifest for selected artifact (name, MIME, bytes, SHA-256).
- vault_pick_attachment: native file selection, never accepts renderer filesystem paths. PDF/PNG/JPEG/WebP content signatures, 20 MB cap, exclusive UUID destination and fsync before metadata transaction. Import increments artifact revision. A failure after file creation may leave an unreferenced file; never automatically delete it or replace user files. Do not retry a successful import blindly; refresh first.
- vault_verify: validates owned link, live artifact, controlled relative path and canonical containment, file size/hash; returns verified/missing/changed. Open requires verified and delegates to Kernel platform adapter. PDF/image handling remains the OS viewer's responsibility; signature checks do not scan files for malware.

## Portfolio

Pure local projection of explicitly public, unarchived, undeleted records in the snapshot. Education/Research/Projects/Competition/Awards/Publications/Links groups and Technical Skills use user fields verbatim, with deduplicated tags. No invented biography, degrees, narrative or metrics. Attachment manifest and Journey/competition titles are excluded from this preview. HTTP(S) links open only when clicked. No publishing/export in Phase 11; Phase 12 will add explicit export whitelists.

## Compatibility and limits

1000 newest rows/options; user-visible limit, no promise of full pagination. Attachments are retained during archive/delete/restore; no permanent-delete action. No attachment removal UI, arbitrary file format import, image rendering, Markdown rendering or new Competition editor. Cross-platform storage and manual editor share code; native opener uses existing adapter boundary. Windows shipped; macOS/Linux packaging remains deferred.
