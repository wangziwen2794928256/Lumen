# Module technical and usage reference

This volume describes current source rather than treating historical phase notes as current release support. Each section covers workflow, service/storage principles, boundaries and extension opportunities. See [API reference](API_REFERENCE.md) and [module tutorial](MODULE_TUTORIAL.md).

## Today and tasks

Purpose: a daily workspace for tasks, capacity and quick notes. Create subjects for entrance examinations, civil-service preparation, language practice or a portfolio sprint. User-defined task names keep the workflow independent of a particular exam.

Workflow: create a title, local calendar date, estimate of 1–1440 minutes and priority. Confirm the task before starting focus. Complete, skip, cancel, move away and trash are different actions. Trash is reversible. Future tasks enter Today on their scheduled date. Over-capacity guidance suggests reducing or moving work rather than increasing commitments.

Mechanism: useToday fetches a typed snapshot. today_action enters TodayService, where a writer lock and transaction coordinate task, session and planning writes. UUID receipts plus payload hashes make identical retries idempotent. Revisions reject stale overwrites. Estimates describe the plan, while actual duration comes from valid focus intervals.

Extension: question-bank queues, paper-writing checklists and reading tasks should reuse task semantics. A Today widget contributes UI and must not mark tasks complete as a render side effect. Keep task UUIDs through relationships rather than using titles as keys.

Verification: duplicate requests, stale revisions, cross-window busy state, trash restoration, capacity and future-date boundaries. Browser preview has no database. Its empty states do not indicate lost desktop records.

源码 / Source: [src/pages/today/TodayPage.tsx](../../src/pages/today/TodayPage.tsx) · [src/features/today/model.ts](../../src/features/today/model.ts) · [docs/data/TODAY_CORE.md](../../docs/data/TODAY_CORE.md)

## Focus and session safety

Workflow: start from an existing task, then pause, resume or interrupt. Completing the session contributes actual progress. Cancelling excludes its duration and reschedules the task. After a crash, resume, finish at the last confirmed activity or cancel. Offline gaps never count.

Mechanism: the session state machine separates running, paused, interrupted and terminal states. The ledger stores confirmed activity intervals and attributes cross-midnight intervals to local dates. Kernel's Windows adapter forwards lock and sleep events into the shared service and stops accumulation. Unlocking does not resume automatically. Checkpoints support restart recovery without fabricating offline effort from wall-clock differences.

Privacy and support: no window-title, browser-address or private-text tracking exists. Paused, sleeping and offline time are excluded. macOS/Linux session adapters remain unimplemented. Native Today initialization returns NATIVE_HOOK_UNAVAILABLE, making native Today/Focus unavailable on those experimental packages. Browser mocks do not establish OS support.

Extension: Pomodoro reminders, revision prompts and experiment timers should consume the existing session API rather than create a competing duration ledger. New OS handling belongs in kernel/platform and needs lock, disconnect, sleep, clock-change and crash-recovery acceptance.

源码 / Source: [src/pages/focus/FocusPage.tsx](../../src/pages/focus/FocusPage.tsx) · [src/features/today/model.ts](../../src/features/today/model.ts) · [docs/data/FOCUS_CALENDAR.md](../../docs/data/FOCUS_CALENDAR.md)

## Calendar and fixed commitments

Workflow: switch day, week, month or agenda and inspect plans separately from actual work. Fixed events may be all-day or timed and cover study, classes, competitions, exams, deadlines, papers and personal events. Timed events last at most seven days and must end after they start. Restore deleted events from event trash.

Mechanism: date-only values represent local calendar days, while timestamps represent actual instants. Dashed plans and solid actual intervals use different sources. Tasks without a precise time remain date-based. The planner splits fixed events by local day, merges overlaps and deducts occupied minutes once. An all-day event conservatively occupies that day's entire capacity.

Extension: exam countdowns, semester schedules and research milestones can reuse fixed events. ICS import is a future opportunity requiring explicit preview, timezone, recurrence and deduplication rules. It is not implemented. After commitments change, generate another proposal. Existing accepted plans stay visible with capacity warnings.

Verification: cross-midnight and daylight-saving boundaries, device timezone changes, nonduplicated overlap deduction, mutually exclusive all-day/timed inputs, date navigation and restoration. Browser validation never replaces Rust validation.

源码 / Source: [src/pages/calendar/CalendarPage.tsx](../../src/pages/calendar/CalendarPage.tsx) · [src/features/calendar/model.ts](../../src/features/calendar/model.ts) · [docs/data/FOCUS_CALENDAR.md](../../docs/data/FOCUS_CALENDAR.md)

## Journeys and deterministic planning

Workflow: create journeys, phases, subjects and side quests with weekly minimum/preferred/maximum minutes, priority, parent relationships and manual or task-ratio progress. Configure weekday/weekend defaults and daily overrides separately. Tasks have earliest start, deadline, remaining estimate, minimum/preferred session, splitting and fixed-time constraints.

Mechanism: allocate feasible minimums, then preferences, then remaining work using deadline, slack and priority. Daily allocations cannot exceed availability after fixed commitments. Task totals cannot exceed remaining work. Journey and side-quest caps apply. An unsplittable task that cannot fit stays explicitly unallocated. The algorithm does not claim global optimality. Energy requirements are manual labels, not inferred personal state.

Confirmation: generation writes a proposed audit only. Accept/Modify revalidate the basis hash and cross-field constraints. Before/after/reason and accept/reject retain adjustment history. Undo applies only to the latest accepted version and cannot overwrite later changes. Capacity changes never silently rewrite accepted plans.

Templates and extension: Fudan 2028 is an explicitly created editable study template, not an admission prediction. Annual rules retain user-supplied official-source text and hash that text. Civil-service subjects, language levels and skill trees are possible templates. External LLM and local-planner interfaces are data exchange contracts only. This version performs no actual model calls or training.

源码 / Source: [src/pages/roadmap/RoadmapPage.tsx](../../src/pages/roadmap/RoadmapPage.tsx) · [src/features/roadmap/model.ts](../../src/features/roadmap/model.ts) · [docs/data/ROADMAP_PLANNER.md](../../docs/data/ROADMAP_PLANNER.md)

## Journal, search and quick capture

Workflow: write Markdown entries with one of eight types, a date, tags and journey/task/music links. The editor shows local autosave status. New entries autosave as drafts and enter the published list only after explicit save. Open Quick Capture using Ctrl+Shift+Space or its button. Shortcut conflicts do not block manual capture. Combine pagination with local date, type, tag and text search.

Mechanism: each edit keeps a renderer localStorage copy immediately. About 600ms after typing stops, a serialized Save follows. journal_recovery persists independently before revision/relationship validation and the entry/tag/receipt transaction. UUID, sequence and hash prevent duplicate retries and stale draft overwrites. Only a successful ACK clears the matching local copy. Conflict recovery creates a new entry while retaining the original.

Search: FTS5 trigram handles Chinese substrings of three or more characters and English phrases. One- or two-character searches, drafts and trash use escaped LIKE. Search terms remain literal phrases. The main index excludes drafts and deleted records. Search runs locally.

Attachments: a Rust file picker imports PNG/JPEG/WebP/PDF after signature checks and a 20 MB bound. UUID filenames and fsync precede metadata publication. Access rechecks path ownership, size and SHA256. Markdown allows neither raw HTML nor automatic remote images.

Extension: mistake reviews, experiment logs, reading excerpts and writing prompts can reuse entry types and relationships. If both disk and local storage fail, unpersisted input cannot be guaranteed recoverable. Errors remain visible instead of reporting a false save.

源码 / Source: [src/pages/journal/JournalPage.tsx](../../src/pages/journal/JournalPage.tsx) · [src/features/journal/model.ts](../../src/features/journal/model.ts) · [docs/data/JOURNAL_CAPTURE.md](../../docs/data/JOURNAL_CAPTURE.md)

## Coding practice and actual time

Workflow: manually add problems, platform/problem IDs, difficulty, algorithm tags and competitions. Each attempt records a date, language, result, notes and additional manual minutes. Link a completed focus session at most once. Platform links open only on a user click and do not scrape account history.

Mechanism: problems and attempts are separate entities. Platform/problem identity detects duplicate problems while attempts retain the process. manual_seconds represents time outside focus. focus_seconds comes from linked actual intervals. The same session cannot contribute to another attempt. Weekly statistics use attempt dates, distinct practiced problems and first-attempt denominators.

Boundaries: unknown difficulty and missing topics remain unknown. The app does not infer problem quality or rank ability. The latest-200-attempt display bound does not truncate statistical totals. Trash/restore uses soft state and pending edits stay local.

Extension: problem lists, algorithm revision prompts, interview plans and linked code excerpts can build on existing records. OJ API imports would need explicit authorization, rate limits and deduplication. No automatic crawler exists. Verify duplicate identity, repeated session links, stale revisions, weekly boundaries and trash restoration.

源码 / Source: [src/pages/coding/CodingPage.tsx](../../src/pages/coding/CodingPage.tsx) · [src/features/coding/model.ts](../../src/features/coding/model.ts) · [docs/data/CODING.md](../../docs/data/CODING.md)

## Insights and review

Workflow: inspect actual focus trends, planned versus actual work, annual heatmaps, words, weekly reviews and monthly snapshots. Mark rest, illness, exams or travel without erasing records. Personal reflections and next-week focus remain user-authored and separate from system facts.

Mechanism: six real signals multiply configurable weights. Four increasing thresholds map their sum to intensity 0–4. Special days leave the 7/28-day consistency denominator without changing planning capacity. Coding includes linked focus and overlaps deep focus, so the totals must not be added. Task completion describes current scheduled-day state rather than a historical completion event.

Words: jieba-rs uses a local dictionary and Chinese/English stop words. Body hashes maintain rebuildable caches in batches of 250. The word cloud shows at most 50 words and 20 sources per word. Song/Artist/Album music clouds remain distinct from journal text.

Snapshots: completed months retain facts at generation time. The current month updates on read. Legacy non-JSON snapshots stay intact. On-this-day uses the exact month/day and never substitutes for leap day. Private JSON/PNG exports exclude bodies and reflections, but statistical words can still be sensitive.

Architecture: Insights is the first Official Module. A trusted static descriptor contributes its on-this-day widget to Today. Extensions can add transparent review or reading statistics, with explicit denominators, overlap and sources rather than moral scores or unsupported predictions.

源码 / Source: [src/pages/insights/InsightsPage.tsx](../../src/pages/insights/InsightsPage.tsx) · [src/features/insights/model.ts](../../src/features/insights/model.ts) · [docs/data/INSIGHTS.md](../../docs/data/INSIGHTS.md)

## Music and optional media observation

Workflow: manually enter song, artist, album, played-at time, actual listening seconds and a journal link. Save, edit and trash work offline. Observation defaults to off. Windows users may explicitly enable an exact source-app ID and a counting threshold of 5–600 seconds, defaulting to 30.

Mechanism: manual edits create an independent track so other playback metadata stays intact. Automatic events use identity/event keys for deduplication, count a play once at the threshold, then increase duration monotonically. Paused playback does not count. Missing metadata falls back to manual use. Observation neither controls playback nor calls the network. There is no universal OS advertisement flag, so unrecognized ads may require manual removal.

Queries: dates use device localtime with a maximum 366-day span. Lists show 100 recent records and clouds group up to 500 with 50 displayed. Totals cover the full selected range. Music stays outside public website exports. Spotify is a disabled future integration with no OAuth flow.

Extension: playlist notes, artist reviews, manual record collections and language-learning annotations. Online metadata, lyrics and playback controls require separate permission and licensing decisions and do not exist now. macOS/Linux observation returns MEDIA_UNSUPPORTED. Manual entry remains the fallback.

源码 / Source: [src/pages/music/MusicPage.tsx](../../src/pages/music/MusicPage.tsx) · [src/features/music/model.ts](../../src/features/music/model.ts) · [docs/data/MUSIC.md](../../docs/data/MUSIC.md)

## Vault and attachments

Workflow: add projects, awards, certificates, course projects, open-source work, research or demos. Record dates, role, organization, skills, technologies and repository/project/paper URLs. Entries default to private. Explicit public visibility permits profile export. Archive, trash and restore retain attachments.

Mechanism: migration 010 extends artifacts/artifact_files. public_visibility owns sharing state and absent visibility rows remain private. Save/state operations combine BEGIN IMMEDIATE, revisions and request receipts. The newest 1000 items feed lists and previews. Unlimited pagination is not claimed.

Attachments: save a work first, then use the native picker for images/PDFs under 20 MB. Managed attachments storage keeps UUID files with SHA256 and metadata for original name, size and MIME. Opening rechecks ownership and integrity. Signature validation is not malware scanning. A file created before a failed metadata transaction may remain unlinked and is conservatively retained.

Extension: exam milestones, course designs, research outputs and personal projects can share this model. Competition/journey relationships remain private and public previews omit raw attachments and related names. Custom categories need synchronized Rust enums, frontend models, validation and migration compatibility rather than label changes alone.

源码 / Source: [src/pages/vault/VaultPage.tsx](../../src/pages/vault/VaultPage.tsx) · [src/features/vault/model.ts](../../src/features/vault/model.ts) · [docs/data/VAULT.md](../../docs/data/VAULT.md)

## Profile and deliberate sharing

Workflow: enter an introduction and contact links, choose visibility, save, inspect the public projection, then manually export a static website and open local preview. Set work visibility in Vault. After changing sharing choices, export again and manage prior snapshots yourself.

Mechanism: Kernel public_export reads an allowlist and selects only explicitly public, non-archived and non-deleted works. It excludes journals, moods, music history, raw focus intervals, attachments and private relationships. The renderer cannot request an unrestricted export to bypass this boundary. Websites are independent snapshots with no automatic deployment.

Links: HTTP(S) validation and rendering restrictions reject dangerous URLs. lumen:// accepts predefined routes rather than arbitrary files or SQL. On Windows, Kernel validates repeated-launch forwarding and focuses the existing window. macOS/Linux registration and browser-opening helpers currently return unsupported errors.

Extension: fork MIT code into your own portfolio, preserve the license and attribution and describe your modifications honestly. Biographies must come from real user content without invented qualifications or scores. Explicit export is intentional sharing, so local-first is never a promise that a user cannot disclose their own data.

源码 / Source: [src/pages/profile/ProfilePage.tsx](../../src/pages/profile/ProfilePage.tsx) · [src/features/profile/model.ts](../../src/features/profile/model.ts) · [docs/en/PUBLIC_WEBSITE.md](../../docs/en/PUBLIC_WEBSITE.md)

## Privacy, backup and recovery

Workflow: use Privacy to inspect the data directory, disconnect integrations, export private JSON/CSV/Markdown, review imports and rehearse an isolated restore. Full backups include database, settings and attachments. Portable JSON excludes attachment files, settings, credentials and grants. Rehearsal never overwrites the current database.

Mechanism: SQLite uses WAL, synchronous FULL, foreign keys and parameterized queries. Migration history checksums verify released SQL and consistent backups precede upgrades. The daily scheduler checks roughly every five minutes while running, creating one valid daily snapshot. Daily/weekly/monthly retention deduplicates buckets and prunes old daily copies only after a new snapshot verifies. Manual, migration and exception backups are not automatically deleted.

Import: parse and validate within a rolled-back transaction, show new/updated/conflicting/invalid counts, then require explicit confirmation of the same file snapshot and unchanged local basis. Backup precedes writing. Conflicts and invalid records block confirmation. Restore rehearsal verifies COMPLETE, checksums, schema, foreign keys and samples, then actually opens the isolated copy.

Boundary: Kernel security cannot be disabled or uninstalled. OS accounts, disk encryption, user sharing and other local applications remain trust boundaries. SQLite is not advertised as application-layer encrypted. Plugins must never read lumen.db, backups or exports directly. Sensitive context would require a separately designed minimal capability.

源码 / Source: [src/features/privacy/PrivacyCenter.tsx](../../src/features/privacy/PrivacyCenter.tsx) · [src/features/privacy/centerText.ts](../../src/features/privacy/centerText.ts) · [docs/en/BACKUP_PRIVACY.md](../../docs/en/BACKUP_PRIVACY.md)

## Desktop, appearance and language

Workflow: Settings groups general, appearance, study, timer, calendar, music, privacy, backup, data, shortcuts, advanced and about. Choose light/dark/system mode and manage accent colors centrally. Modules support enable, hide, pin, drag and keyboard move buttons. Disabling keeps data.

Language: select English or Simplified Chinese in Appearance, save edits, then apply and reload. lumen.locale.v1 stores the interface preference only. Missing or invalid values fall back to Chinese. The English pack maps 905 existing frontend strings. Navigation reads bilingual manifest names and dates use the selected locale. User bodies, goals, problems and filenames are never translated. New copy needs language resources and regression. Native OS dialogs follow the OS. Existing Rust tray/notification copy remains Chinese, with complete native-shell localization deferred.

Mechanism: appearance persists in localStorage. HTML data-theme/data-accent attributes drive CSS tokens. Module settings use database revisions, reorder within pinned groups and provide keyboard alternatives to dragging. Tauri manages the tray, single instance, autostart, notifications and capture window. Notifications default to off and omit private bodies.

Extension: theme authors update tokens.css and appearance types, then verify both modes, focus, contrast, reduced motion and three window sizes. No runtime currently loads arbitrary Theme Pack directories. The themes directory is not an open CSS plugin loader.

源码 / Source: [src/pages/settings/SettingsPage.tsx](../../src/pages/settings/SettingsPage.tsx) · [src/features/desktop/model.ts](../../src/features/desktop/model.ts) · [docs/en/DESKTOP.md](../../docs/en/DESKTOP.md)

## Bundled module inventory and status

These are trusted features bundled with source, not downloadable third-party plugins. Manifest status describes migration state, not native acceptance on every platform.

| ID | Route | Layer | Status |
| --- | --- | --- | --- |
| `lumen.today` | `/` | kernel | stable |
| `lumen.focus` | `/focus` | official | legacy-bridge |
| `lumen.calendar` | `/calendar` | official | legacy-bridge |
| `lumen.roadmap` | `/roadmap` | official | legacy-bridge |
| `lumen.journal` | `/journal` | official | legacy-bridge |
| `lumen.coding` | `/coding` | official | legacy-bridge |
| `lumen.insights` | `/insights` | official | module |
| `lumen.music` | `/music` | official | module |
| `lumen.vault` | `/vault` | official | module |
| `lumen.profile` | `/profile` | official | module |

Dedicated official entrypoints exist for insights, music, vault and profile. Other official legacy bridges reuse stable pages/services and can migrate incrementally. Kernel Today remains required. Security has no uninstallable plugin entrypoint.
