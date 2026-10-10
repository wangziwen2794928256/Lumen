# glib 0.18.5 security backport

Source: crates.io glib 0.18.5; original MIT license retained.
Upstream fix: https://github.com/gtk-rs/gtk-rs-core/pull/1343
Advisory: https://rustsec.org/advisories/RUSTSEC-2024-0429.html
Only two production lines in src/variant_iter.rs change: the C output pointer is mutable, and g_variant_get_child receives &mut p. No API or version upgrade.
Cargo uses this checked-in source via [patch.crates-io]. Linux optimized regression covers next/next_back/nth/nth_back/last and exhaustion. cargo audit may still identify 0.18.5 by version: this document is a scoped source backport, not a claim that the upstream version is patched. Do not suppress the advisory globally.
Remove this vendor patch only after Tauri/GTK can resolve a supported glib >=0.20 and all platform gates pass.
