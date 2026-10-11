# Extension directions and contribution entrypoints

Everything is a Module lets different growth workflows share a safe foundation. These are design opportunities, not shipped features. Prefer a small offline, manual and read-only module first.

## Simple core, optional extensions

Lumen prioritizes a simple, approachable growth-recording experience. Keep daily workflows clear instead of competing on feature count or putting every request into the default interface. Users can choose modules and hide or disable entrypoints they do not need.

For additional needs, open a GitHub Issue describing the situation, friction and smallest useful solution. You can also fork the source, follow the [module tutorial](MODULE_TUTORIAL.md) and build an extension yourself, with optional help from AI or coding assistants. AI is a development option, not a requirement for using Lumen. Use synthetic fixtures rather than sending real tasks, journals, databases or secrets to external services. Review and validate generated changes. Reusable small modules are welcome as PRs; inclusion in the default core depends on necessity and maintenance cost.

## Extension directions

| Direction | First useful increment | Reuse | Acceptance boundary |
| --- | --- | --- | --- |
| Mobile app (major outlook) | Offline Today overview, quick capture and reviewed manual-exchange prototype | Domain models, module contracts, Kernel exchange design | Separate data/platform boundaries, conflict recovery, no default cloud sync; not implemented |
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

## Major outlook: a mobile app

A future lightweight iOS / Android experience could let users check today's plan and capture a thought away from their computer, then return to desktop for deeper organization. It should preserve simplicity, local-first storage and manual operation, rather than replicate every desktop screen or module.

Start by exploring an offline Today overview and quick capture, followed by explicit import/export. Any device synchronization needs a separate design for identity, minimal exchange scope, encryption, version compatibility, conflicts and backup recovery. Do not upload the core database by default. Mobile background timing, lock and sleep behavior require explicit platform-adapter support; desktop focus behavior cannot be assumed to work unchanged on a phone.

This is an important future direction, not an available mobile package, synchronization service or promised release date. Contributions can begin with user needs, interaction prototypes, shared data contracts and isolated fixtures. The implementation stack and scope remain to be evaluated.

## Choosing a first contribution

Write the user problem, minimum required data and undo path. Choose one entrypoint and one observable outcome. Demonstrate utility with synthetic fixtures before adding storage or capabilities. A read-only page/widget should not begin with a migration.

## Official source and future community plugins

Today, fork trusted source and follow the [module tutorial](MODULE_TUTORIAL.md). Future untrusted distribution follows the [SDK contract](PLUGIN_SDK.md). Dropping React code, a DLL or a Node package into extensions cannot execute it. Runtime contributions must cover budgets, revocation, update rollback and adversarial tests rather than merely running a Wasm function.

## Your derivative work and attribution

MIT permits modification, redistribution and commercial use with retained license/copyright notices and dependency compliance. Present your fork and modules as your work while explaining inherited foundations and your own changes. Editable documentation and slides welcome more tutorials, translations, demonstrations and real examples. A Star supports the creator. Reproducible issues and focused PRs help directly.
