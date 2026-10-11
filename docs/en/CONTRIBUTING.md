# Contribution guide

Users, developers, designers and documentation authors are welcome. No particular editor, Codex account or AI service is required. Start with the [repository contribution guide](../../CONTRIBUTING.md). [Chinese counterpart](../zh-CN/CONTRIBUTING.md).

## Choose a starting point

Keep the default experience simple and approachable. Request additional capabilities or build focused modules yourself, optionally with AI assistance, instead of accumulating core features. Review and validate generated work. A mobile app is an important future direction: contribute needs, prototypes or data contracts, while recognizing that no mobile version or default device sync is available. See [extension ideas](EXTENSION_IDEAS.md).

Report bugs, add examples, correct wording, translate, improve accessibility or contribute modules, theme designs and editable presentations. Exam preparation, languages, research, coding and career development are all relevant. See [EXTENSION_IDEAS](EXTENSION_IDEAS.md).

Read [MODULE_TUTORIAL](MODULE_TUTORIAL.md), [MODULE_SYSTEM](MODULE_SYSTEM.md) and [API_REFERENCE](API_REFERENCE.md) for source integration. Official modules currently use trusted source code. Community WASM execution, a marketplace and runtime theme-pack loading remain deferred. Design contracts do not establish executable plugin support.

## Development and validation

Check existing Issues/PRs first. Discuss substantial features, user flows and data boundaries before implementation. Fork and work on a focused branch, preserving stable code and lockfiles. Pinned tools, native prerequisites, AppData isolation and full commands are in [ENVIRONMENT_AND_CONFIGURATION](ENVIRONMENT_AND_CONFIGURATION.md). Browser previews cannot replace native acceptance.

| Change | Applicable checks |
| --- | --- |
| Documentation, translation, presentations | Links, terminology and bilingual consistency; inspect each slide and distinguish concept art from product evidence |
| Frontend logic and UI | lint, format:check, typecheck, relevant unit tests, build; affected E2E, keyboard and layout checks |
| Kernel / Rust | fmt, relevant unit tests, Clippy; affected IPC, permission and native integration validation |
| Database and backups | New numbered migrations, failure recovery, backup consistency and record preservation using isolated fixtures only |
| Module API or permissions | Update schemas/types/contracts, compatibility checks and permission-denial cases |

Inspect UI changes at 1280×720, 1440×900 and 1920×1080, including light/dark themes, keyboard focus, ARIA, contrast and reduced motion. zh-CN remains the default; new user-facing copy should support English. Respect experimental macOS/Linux boundaries. Mock checks do not establish native platform support.

## What to include in a PR

Explain the problem and reproduction steps, resulting behavior, related Issue, compatibility/data impact and actual validation. Documentation-only changes can explain why business checks are N/A. Mark unrun checks NOT RUN with a reason, never PASS. Use synthetic screenshots. Maintainers review scope, implementation and evidence before deciding whether to merge. Main-repository write access or a particular AI tool are not contributor requirements.

## Privacy, security and licensing

Never upload production SQLite, private bodies, attachments, backups, secrets or private logs. Debug builds do not automatically isolate AppData; verify the development directory. Released migrations are immutable. Never delete or reset real user data. Preserve parameterized SQL, controlled paths, revocation and Kernel authorization. Community code must not gain direct access to lumen.db, arbitrary files or arbitrary networking.

You must have submission rights and retain MIT and third-party license/copyright notices. Forking, adapting and distributing derivative work are welcome. Submitted code and project-created materials should be compatible with the project's MIT license; identify the source and license of third-party material. AI assistance does not remove responsibility for correctness, licensing or privacy. Release signing, package publication and maintainer credentials are not permissions required for ordinary PRs.

Do not put secrets, real data or sensitive exploit details in public security reports. See [Security](../../SECURITY.md).
