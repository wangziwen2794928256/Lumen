# Backup, restore and privacy

Phase13 keeps all operations local in the Kernel. Open Settings → Local backup / Privacy Center. Backups and private exports contain personal data and are separate from the explicit public website projection. No upload or AI request is added.

## Complete backups

Startup and a five-minute timer check while the app is running. Default retention: enabled, 7 daily / 4 weekly / 6 monthly buckets, using local dates and ISO weeks. Retention is the union of the newest snapshot in each bucket. Missed days are not fabricated. Limits: 1–90 daily, 1–52 weekly, 1–60 monthly. An in-flight operation can finish after a policy change. A valid current-day snapshot prevents repeat creation and cleanup; changed retention applies when the next new daily snapshot succeeds. No OS service or scheduled task is installed.

A complete snapshot contains a consistent SQLite snapshot (including settings), app/schema versions, SHA256 manifest, attachment payloads including trash, and COMPLETE containing the manifest hash. All payload sizes/hashes and SQLite integrity, foreign keys, migration checksums and readable samples must validate. Failures leave an incomplete directory without a success receipt. I/O errors, including insufficient storage, propagate before retention; no physical disk-exhaustion experiment was run.

Only verified, canonical direct-child daily-DATE-UUID directories are eligible for retention, after a new snapshot verifies. Manual, migration, malformed, future-dated, incomplete, corrupt and escaped/symlink folders are protected. Manual backups have no automatic retention. Old migration backups remain compatible with their previous verifier, but are not complete recovery sources unless they include full payloads and COMPLETE.

## Portable exchange

JSON format lumen-data-backup/1, schema10, includes Journal, Focus, Tasks, Music, Coding, Artifacts and related entities from a fixed 24-table allowlist. It excludes attachment files/relations, preferences, settings, integration grants, secrets and derived/append-only analytics. It is a data exchange, not a full machine snapshot. CSV and Markdown provide six category files containing structured records; they can be read without SQLite. Export COMPLETE is a completion marker, not a cryptographic signature. Files are plaintext local files under exports/private-UUID.

Choose JSON opens a native file picker. Limit: 64 MiB, 100000 records, 1 million bytes per string. The Kernel validates format, columns, primitive types, canonical IDs, safe URL columns, constraints and foreign keys in a transaction which is then rolled back. Counts show add/update/conflicts/invalid/unchanged. Conflicts or invalid records block the whole import. An existing differing row updates only with a newer updated_at; there is no automatic merge. Finish local focus before import; unfinished source focus rows are rejected.

The reviewed file contents are cached as an immutable in-memory snapshot. Confirmation imports that snapshot, not a subsequent edit to the disk file. A token and fingerprint of incoming/current data protect confirmation; local data changes require another review. Confirmation creates a complete pre-import backup, then commits one transaction. Failures leave the existing database intact. Repeated inserts do not create duplicate IDs; revisions can make a differing repeat require review. Focus aggregates are rebuilt from imported intervals. No integration is enabled by import.

## Isolated recovery

Choose a complete backup folder. The Kernel verifies it, clones into cache/restore-UUID, opens the candidate with the real Database startup path, checks schema/integrity and readable samples, closes it, reseals its database hash and verifies again. RESTORE_REVIEW.json records aggregate results. The candidate can be inspected for a later offline recovery review. This phase does not swap/overwrite the live database; AGENTS.md prohibits production overwrites. Keep the source backup and candidate until the recovery decision is complete.

## Privacy controls and limits

Activity titles, browser URLs, document names and private text tracking are unimplemented and uncollected. Windows media listening defaults off; an existing explicit preference is preserved. Privacy Center shows counts of integration records and credential references, never secret values. Disconnect disables listening, clears allowed source, clears credential references and soft-deletes integration account records atomically. Explicit collection deletion moves only windows-media plays to trash; manual history remains. Revoking remote service tokens still requires the service account; no credential provider integration was added.

Open Data Folder resolves the Kernel-owned root through the platform service. Windows adapter is implemented; macOS/Linux native folder-opening and packaging are not validated here. Complete backups are not encrypted or digitally signed. Physical disk-full, OS power-loss and third-party provider revocation tests were NOT RUN. Native privacy-setting changes/deletion were NOT RUN under computer-use guidance; isolated Rust and E2E tests cover those behaviors.

Verification: scripts/verify/phase13.ps1; docs/testing/phase13-gates.json; docs/testing/phase13/NATIVE_QA.md. Chinese: ../zh-CN/BACKUP_PRIVACY.md.
