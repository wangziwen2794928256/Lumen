# Requirements coverage and UI audit

This review revisits all 117 numbered original sections, the architecture evolution and later open-source, localization and simple-core requirements. See the [section-level matrix](../data/REQUIREMENTS_MATRIX.md). A working core is not full acceptance of every original detail. Visual quality and interaction are assessed, not certified as universally best.

## Delivered in this revision

Daily fortune is available in the sidebar and the Today secondary column. Disable it in Settings → Advanced → Module management. Hiding only removes navigation; disabling removes its widget and guards the route. The trusted official module uses `lumen.fortune`, the registry and `today.sidebar`. It adds no database tables, permissions or network calls.

Twelve bilingual signs rotate by the device’s local calendar date, stay stable during a day and refresh across midnight, focus/resume and clock changes. This is entertainment and inspiration, not prediction, scoring or planner input. No birthday or account is collected.

Module settings action buttons now have a 12px wrapping gap. Navigation scrolls independently, leaving settings reachable in short windows. A synchronous browser Coding API error now becomes the expected handled error. The stale Phase14 footer and pre-feature diagnostics/about copy were corrected.

## Remaining gaps

| Priority | Finding | Practical boundary |
| --- | --- | --- |
| P1 | Unified global search is incomplete | Generic FTS covers tasks/journal; other modules have separate search. There is no complete all-module entry or date/type/tag global filtering. A future incremental migration is needed. |
| P1 | macOS/Linux lack feature parity | Missing session-event adapters make Today/Focus unavailable. Media, opening helpers and deep links remain limited. Safety must not be removed to hide this. |
| P2 | Native English localization is partial | Rust tray, notifications and some diagnostics still use Chinese. |
| P2 | Today music widget is absent | Journal preview exists; live song card and full greeting experience do not. Music is available in its own module. |
| P2 | Monthly snapshots are generated on access | No unattended monthly scheduler. Review adjustment-history presentation and snapshot photo galleries remain limited. |
| P2 | Large first queries have latency | Isolated ten-year fixture: cold 6217ms, warm 2694ms, seven-day 679ms on this workstation during other build work. Not a controlled hardware benchmark. Native <=2.5s cold start was not revalidated. |
| P2 | Portable export is allowlisted | JSON/CSV/Markdown cover main data, not every internal/preferences/grants table. Use verified complete backups for full recovery. |
| P3 | Spotify/ActivityWatch are incomplete | Spotify is an interface; ActivityWatch has no actual integration. Both are off and unnecessary for manual use. |
| P3 | Some pages remain large | InsightsPage exceeds the suggested component size. Further incremental refactoring is preferable to a wholesale rewrite. |
| P3 | Stack choices differ | React hooks/context and semantic DOM/SVG are used instead of the specified Zustand/TanStack Query/ECharts. The lightweight calendar has ADR-004; state simplification is discussed in ADR-006. Exact stack conformance is not claimed. |

Community execution, full WASM, marketplace, AI Planner and mobile apps are intentionally deferred. Theme packs and third-party lifecycle are contract preparation, not an installed third-party runtime.

## Verification and limits

Lint, formatting, typecheck, 39 frontend unit tests, frontend build and 81 Edge E2E tests pass. Tests cover existing workflows, fortune enable/hide persistence, route/widget guards, button spacing, and 14 routes in Chinese/English, both themes and 1280×720, 1440×900, 1920×1080. Existing populated synthetic flows were repeated.

The audit checks horizontal overflow/clipped controls, reachable settings, persistent shell, keyboard/dialog focus, contrast and reduced motion, with visual inspection of empty and synthetic populated screenshots. It is not a golden pixel comparison or a real assistive-technology user study. No production records were used in screenshots. The muted style and clear primary-task hierarchy are retained.

All 72 Rust tests pass, including migration/rollback/checksum, backups, restore and security boundaries. The ignored ten-year fixture was explicitly run separately. Schema stays at 10; published migrations are untouched. The local Phase22 report records isolated native smoke and final Windows artifacts. Browser mocks do not prove native timer behavior. Installer/upgrade/uninstall and lock-screen acceptance were not repeated for this revision.
