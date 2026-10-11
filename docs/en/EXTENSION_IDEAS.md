# Extension directions and contribution entrypoints

Everything is a Module lets different growth workflows share a safe foundation. These are design opportunities, not shipped features. Prefer a small offline, manual and read-only module first.

| Direction | First useful increment | Reuse | Acceptance boundary |
| --- | --- | --- | --- |
| Mistake review | Revision list from manually tagged entries | Journal tags, Today widget | Stable entry IDs, no bodies in logs |
| Civil-service practice | Topic timing and essay-review templates | Tasks, Focus, Journal | Reuse the actual ledger, no double counting |
| Exam journeys | Editable subjects and study phases | Roadmap entities | Separate annual sources, no admission prediction |
| Reading | Manual bibliography, progress, excerpts and tasks | Journal, Roadmap | Copyright, page/progress semantics |
| Language practice | Manual skill logs and revision prompts | Tasks, Journal | No automatic audio upload, new format design |
| Research | Experiment templates, milestones and output links | Calendar, Vault | Traceable sources, bounded large files |
| Skills | Manual skill trees with practice evidence | Roadmap, Coding, Vault | Activity is not an ability ranking |
| Portfolio versions | Project change logs and contributions | Vault, Profile | Public allowlists and old snapshot management |
| Accessibility | High-contrast/color-vision-friendly built-ins | Tokens, Appearance | Both themes, focus and reduced motion |
| More languages | New locales and page translations | src/locales, locale.ts | Preserve user text and test labels/date layouts |
| Interoperability | Reviewed ICS/Markdown import | Kernel data exchange | Deduplication, timezones, rehearsal and rollback |
| Local planning | Explainable proposal review | Plan Exchange | Minimal context, explicit acceptance, no automatic training |
| OS support | macOS/Linux session adapters | kernel/platform | Lock/sleep/recovery without invented actual time |
| Security tooling | Manifest/lifecycle static review | Kernel contracts | No native-execution bypass |

## Choosing a first contribution

Write the user problem, minimum required data and undo path. Choose one entrypoint and one observable outcome. Demonstrate utility with synthetic fixtures before adding storage or capabilities. A read-only page/widget should not begin with a migration.

## Official source and future community plugins

Today, fork trusted source and follow the [module tutorial](MODULE_TUTORIAL.md). Future untrusted distribution follows the [SDK contract](PLUGIN_SDK.md). Dropping React code, a DLL or a Node package into extensions cannot execute it. Runtime contributions must cover budgets, revocation, update rollback and adversarial tests rather than merely running a Wasm function.

## Your derivative work and attribution

MIT permits modification, redistribution and commercial use with retained license/copyright notices and dependency compliance. Present your fork and modules as your work while explaining inherited foundations and your own changes. Editable documentation and slides welcome more tutorials, translations, demonstrations and real examples. A Star supports the creator. Reproducible issues and focused PRs help directly.
