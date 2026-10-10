# Public export and local website

Phase 12 implements Official Profile; the whitelist exporter, preview server and deep-link parser belong to Kernel. No website is deployed. GitHub connectivity is unrelated to public export and does not authorize uploading personal data.

## Use

1. In Vault, explicitly mark selected works public. Archived/deleted works are excluded.
2. In Profile, write an introduction and HTTP/HTTPS contact links. The entire introduction is private until explicitly checked public and saved. The main user record is never inferred or exported.
3. Review the saved public projection. Export/preview require its content fingerprint; if another window changes public content, review again.
4. Export produces a new AppData `exports/website-UUID/` directory containing `index.html`, `site.js`, `site.css`, `public-profile.json` and a final `COMPLETE` hash marker. No SQLite, attachments or private files are copied.
5. Open local preview from the app, or run `node scripts/preview-website.mjs <export-directory>` with Node from the documented toolchain. The latter works independently of the application and database. Stop with Ctrl+C.

## Public schema 1.0

Top-level allowlist: `schema_version`, `exported_at`, `profile`, `artifacts`. Profile may be null. A public Profile contains only display_name, headline, about, contact links, public=true and revision=0 (fixed compatibility value; editing history is not exported). Artifact allowlist: id/title/description/kind/date/role/organization/skills/technologies/links/public=true. The explicitly public description is exported verbatim; review it before exporting. Associations, file manifests, attachment data/paths, local revision/state/timestamps and other database rows are excluded. Unknown schema/fields, private entities and active URL schemes are rejected by the website renderer.

Journal, Mood, Quick Capture, music/listening history, raw Focus and private notes are not supported public export types in v1.0, even if a visibility row exists. Related Journey/Competition private titles are not implicitly included. Competitions on the website are public Vault works of kind competition; projects/research/awards similarly use explicit public works.

## Privacy and transport

Exports are immutable snapshots. Revoking public status changes future projections; it does not erase previously exported files. Manage old exports manually. A partial export without COMPLETE is not a successful export; it may remain after an IO failure. No auto-upload, remote API, CDN, telemetry or third-party scripts.

The live preview binds only 127.0.0.1 on an ephemeral port and serves only five exact GET paths from memory. Host validation rejects DNS rebinding; no CORS/database endpoint/path traversal/query interpretation. Same-machine users/processes can read explicitly public preview content while it runs; it stops when the app exits. JSON uses textContent, never innerHTML, and links use noopener/noreferrer with a restrictive CSP. On-disk exports are validated again by the renderer.

## Deep links

Explicit registration writes `HKCU\Software\Classes\lumen` pointing at the current executable. Moving the executable requires registering again. Exact accepted forms: `lumen://today`, `lumen://calendar`, `lumen://journal/YYYY-MM-DD`, `lumen://artifact/canonical-lowercase-UUID`, `lumen://journey/canonical-lowercase-UUID`. Query/fragment/percent encoding/traversal/other schemes are rejected. Routes select local pages; they do not expose their content to the website or grant file access. Artifact opens the live local editor, Journal filters its date, Journey highlights the matching local card. Module disable guards remain effective.

Windows protocol launching reaches the Kernel single-instance callback, which validates the route and asynchronously restores/focuses the existing window before navigation (Phase14). macOS/Linux registration/browser adapters remain explicit unsupported boundaries. Browser permission prompts are controlled by the browser; no bypass. Exported contact links are user authored and unverified.

## Reproduction

Run the explicit commands in BUILD_FROM_SOURCE.md and TESTING.md. The current60-case E2E suite includes detached website/private sentinel coverage at three viewport sizes, plus migration/backup/rollback Rust tests. Original Phase12 verification/native reports remain local-only. Use isolated QA for synthetic native tests; schema10 and migrations001–010 are unchanged.

Windows Shell may append a single root slash to authority-only links: `lumen://today/` and `lumen://calendar/` are also accepted. Double slashes and all other additional paths remain rejected. Windows 系统可能给无参数入口补一个末尾斜杠；仅 Today、Calendar 兼容这一形式。
