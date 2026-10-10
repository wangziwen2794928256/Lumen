# DATABASE

The authoritative core database remains AppData/lumen.db. No old database relocation or duplicate is introduced. core/ is reserved and empty pending a future explicit migration. Only Kernel and trusted bundled services own SQLx; community guests receive no connection, host path or SQL API.

Migration 008 adds module_preferences, planner_observations and triggers only; 001–007 remain immutable. Registry writes use optimistic revision in a transaction. Observations capture real changes after upgrade: estimates/dates/states, actual/planned focus, planner choices, capacity and preferences. No invented historical backfill or private bodies/titles.

Existing consistent backups, checksummed migration history, transactions and corrupt-storage refusal remain. SQLite owns WAL/SHM; a live raw file copy is not a backup.

Matching Chinese document：[DATABASE](../zh-CN/DATABASE.md)。

Phase12 keeps schema10 and immutable migrations001-010. Versioned settings key profile.v1 stores the private-by-default Profile with optimistic revision. Public export queries only explicit public_visibility artifact rows and never serializes arbitrary database rows. No new migration is required.

Phase13 retains schema10 and immutable migrations001-010. backup.policy.v1 uses existing settings. Portable exchange uses a versioned fixed 24-table allowlist with bound values, dry-run rollback and pre-import complete backup; derived focus metrics rebuild from intervals. No production schema changes. See BACKUP_PRIVACY.md.
