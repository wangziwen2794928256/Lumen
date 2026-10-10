---
name: official-app-installer
description: Prepare and verify a Windows application installer from an official vendor website, Microsoft Store, official GitHub repository or vendor CDN, then install only with user approval.
---

# Official app installer

This independent Codex workflow does not add installer automation to Lumen.

Identify the requested product, Windows architecture and intended version. Use current official sources. Verify publisher ownership starting at the vendor's official site; a GitHub release or CDN URL alone does not establish ownership. Follow official links, record the origin and redirects, version and architecture. Reject mirrors, repacks, ads, search-result download buttons and unexplained redirects. Prefer Microsoft Store or vendor-signed stable packages when they meet the request.

Download only the requested installer to a dedicated task folder. Do not execute it while preparing verification. Inspect its file version, SHA256, Authenticode status and publisher with [scripts/inspect-installer.ps1](scripts/inspect-installer.ps1). Supply the expected publisher exactly as shown in the independently verified vendor certificate, and an official checksum when published. A matching self-computed hash proves identity only against that reference, not trust. Unsigned packages, invalid signatures, publisher mismatch or unavailable official hashes must be reported honestly; do not bypass SmartScreen or security controls. Microsoft Store follows its trusted product identity and publisher flow instead of an arbitrary executable script.

Present the concrete product/version/source/publisher/signature/hash, destination, elevation and expected changes. Obtain explicit installation approval before execution, unless the user's existing instruction explicitly approves installing that exact application. Approval to research or download is not installation approval. Respect tool-specific approval rules too. User handles authentication, verification and security/privacy prompts. Do not disable security software, change policy, read credentials, add unrelated software or uninstall anything without explicit authorization.

After approved installation, use the supported official installer and inspect the installed version and launch result. Do not accept bundled extras. Stop on unexpected publisher, changed download, additional product, payment or permissions outside the approved scope. On failure preserve logs without secrets and report it; retry only after diagnosing a concrete cause. Do not turn a failed install into an unapproved uninstall/reinstall.

Report installed location/version and evidence, or the exact remaining approval/blocker. Distinguish preparation from actual successful installation. The helper is read-only and never downloads, executes or installs a package.
