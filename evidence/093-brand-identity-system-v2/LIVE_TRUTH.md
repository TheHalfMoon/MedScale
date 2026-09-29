# Spec 093 Live Truth

Date: 2026-09-17

- Base: `origin/main` at `ee8daef3a2782bbdbcb3324766a5d6b95c09fa09` when the worktree was created.
- Branch: `design/medscale-identity-v2`.
- Active parallel implementation lane: `spec/074-project-artifact-graph-foundation`.
- Spec 074 overlap observed at branch start: `crates/medscale-desktop/src/main.rs`, `crates/medscale-desktop/src/project_workspace.rs`, and `crates/medscale-desktop/ui/app.slint`.
- Spec 093 intentionally avoids Spec 074 feature authority. Shared visual work is isolated and must reconcile current main before merge.
- Founder instruction on 2026-09-17 explicitly supersedes the Spec 073 visual identity restrictions for this identity implementation.
- No real PHI, backend authority, model admission, network authority, or MESC work is authorized here.
- Latest observed remote Spec 074 head during handoff preparation: `f170c8e29cf932211e654b8aa0d180505981d66b` (the exact head must be reverified before any merge/reconciliation).
- Founder subsequently delegated the high-fidelity UI composition to V0. Spec 093 setup work therefore stops at a complete design-system and V0 handoff contract; production UI adoption remains a later qualified step.
