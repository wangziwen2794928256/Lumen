# THREAT_MODEL

Trusted principals: the user, Kernel and compiled official code. Untrusted inputs include community packages, self-declared manifests, themes, LLM output and remote responses. Assets include core records, attachments, secrets, planning decisions, grants and availability.

Mitigations: no guest host paths; canonical namespace checks reject escape (future proxies still need race-resistant handles); compiled catalogue prevents official impersonation; version/hash/revocation prevents grant replay; declarative widgets prevent script injection; future worker limits contain resource exhaustion; proposals remain data, never automatic execution/acceptance.

Updates/rollback require package hashes, new consent, isolated DB backup and compatibility checks. Reject binary rollback after incompatible data upgrade. A compromised OS, malicious official code or authorized publication cannot be solved by the Wasm sandbox. The runtime is closed now; designs are not claimed as an implemented sandbox.

Matching Chinese document：[THREAT_MODEL](../zh-CN/THREAT_MODEL.md)。
