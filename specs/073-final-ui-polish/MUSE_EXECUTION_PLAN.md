# Muse Execution Plan — Spec 073 Final UI Polish

Status: EXECUTION_HANDOFF_ACTIVE
Executor: Muse
Reviewer / governance owner: supervising agent
Scope: Spec 073 only

## Mission

Finish Spec 073 from exact live repository/GitHub truth through `CLOSED_CANONICAL`, or stop only on a genuinely external blocker proven with exact evidence.

## Immutable constraints

- Work only in `/Users/abdulazizalsh/Projects/MedScale-073` on `spec/073-final-ui-polish` unless live truth proves otherwise.
- Reverify branch, HEAD, `origin/main`, worktree cleanliness, open PRs, required checks, and canonical docs before acting.
- Preserve all valid local Spec 073 work. Do not reset, discard, overwrite, or regenerate evidence blindly.
- Never force-push, rebase, rewrite shared history, bypass branch protection, weaken tests/gates, or fabricate evidence.
- MESC is a separate project. Do not touch MESC code, docs, gates, residuals, or release calculations.
- `RELEASE_READY`, `PRIVATE_DATA_READY`, `MULTI_CLIENT_RELEASE_READY`, and real-PHI authority remain false unless independent real evidence proves otherwise.
- Do not create a production Web runtime. `docs/brand/web-reference/` remains documentation/reference only.
- Preserve JSON/script CLI contracts. Human-facing CLI identity must remain separate from machine-readable output.
- Preserve the founder-approved MedScale mark: black circular field, soft monochrome signature M, MedScale Shelf recognition cue. Do not redesign it.
- Preserve Light mode as the primary default and system-following Dark mode.
- Preserve Instrument Sans, Source Serif 4, platform monospace, custom MedScale icon family, narrow rail + refined sidebar.
- Preserve the evidence-first / authority-safe product semantics and accessibility/focus behavior.

## Current implementation truth to reverify

The working tree already contains substantial Spec 073 implementation and evidence. Do not assume these details are still true without checking them live:

- Desktop identity/system pass implemented across shared shell and product routes.
- Signature `M` with the MedScale Shelf is implemented in canonical SVG assets.
- Light/Dark native renders were regenerated after the latest changes.
- Impeccable Web detector previously returned zero findings using the installed `impeccable` CLI.
- Native screenshots were manually reviewed under the Impeccable critique/audit/distill/typeset/polish/harden discipline.
- Focused Spec 073, historical compatibility, Desktop, CLI, native smoke/perf, Clippy, Rust 1.88, and workspace tests have previously passed after stale test updates.
- Historical compatibility updates exist for Specs 060, 062, 066, and 068.
- `crates/medscale-cli/src/main.rs` has canonical CRLF line endings. Preserve them and avoid workspace-wide write-mode formatting that creates a false 2,900-line diff.
- Runtime/perf tests can rewrite `evidence/027-perf-sbom-release-evidence/perf_harness_latest.json`; never stage such generated churn unless Spec 073 explicitly requires it.

## Phase A — Reconcile and freeze the exact intended diff

1. Fetch `origin/main` and verify current base/head.
2. Inspect every tracked and untracked change.
3. Remove only proven generated/transient churn; preserve intended implementation and evidence.
4. Confirm no unrelated project/worktree files are changed.
5. Confirm all font assets include their license files and no extra font files are introduced beyond the admitted set.
6. Confirm the Web reference is explicitly non-production.
7. Confirm the CLI diff contains only intended human-output identity changes and preserves CRLF.
8. Confirm `BUILD_QUEUE.md`, `PROJECT_COMPLETION_STATUS.md`, Spec 073 docs, and repository closure docs tell one consistent current truth.

## Phase B — Exact-range substantive review

Review the complete `origin/main...HEAD+working-tree` scope, not isolated files.

Review for:

- visual/system consistency and no generic SaaS/card/status-chip regression;
- logo/MedScale Shelf consistency at runtime and reference surfaces;
- Light/Dark contrast and low-fatigue hierarchy;
- keyboard/focus/accessibility semantics;
- no overflow, clipping, tiny functional text, or minimum-window regression;
- CLI machine-readable contract separation;
- Web reference non-production boundary;
- no fabricated clinical metrics, patient authority, model authority, or release claims;
- no MESC coupling;
- no stale compatibility assertions that freeze superseded visual implementation details;
- no generated evidence churn;
- no unnecessary dependency/runtime changes;
- no line-ending-only diff noise.

Resolve every material finding. Do not add speculative polish after review is clean.

## Phase C — Final Impeccable and rendered evidence gate

1. Run `impeccable detect --json docs/brand/web-reference/reference.html docs/brand/web-reference/tokens.css` using the installed `impeccable` binary. Zero non-ignored findings required.
2. Keep the approved Instrument Sans detector exception documented; do not change the founder-approved font merely to silence a detector rule.
3. Rebuild the native Desktop only if the exact reviewed tree changed after the current renders.
4. If native UI changed, regenerate deterministic window-level captures for:
   - Home Light
   - Home Dark
   - Patients Light
   - Models Light
   - Evidence Light
   - Workflows Light
   - Settings Dark
5. Manually inspect the actual native captures and record hashes. Reject captures of Terminal/browser/other foreground apps.
6. Do not claim automated Impeccable native detection for Slint; document native review as platform-specific manual rendered audit.

## Phase D — Final local qualification on one exact tree

Use fail-fast execution and preserve logs for the exact candidate tree:

```text
cargo fmt --all -- --check
git diff --check
cargo test -p medscale-core --test final_ui_polish_073 --locked
cargo test -p medscale-core --test product_differentiation_068 --locked
cargo test -p medscale-core --test desktop_cli_hardening_066 --locked
cargo test -p medscale-core --test native_desktop_ui_060 --locked
cargo test -p medscale-core --test native_population_insights_062 --locked
cargo test -p medscale-desktop --locked
cargo test -p medscale-cli --locked
cargo build -p medscale-desktop --locked
target/debug/medscale-desktop --smoke
target/debug/medscale-desktop --perf-idle-ms 250
impeccable detect --json docs/brand/web-reference/reference.html docs/brand/web-reference/tokens.css
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo +1.88.0 check --workspace --all-targets --locked
cargo test --workspace --locked
```

If any test rewrites historical perf evidence, restore that generated file before candidate freeze and rerun the affected checks.

## Phase E — Canonical candidate and GitHub qualification

1. Update Spec 073 tasks/evidence only for gates actually proven.
2. Keep Spec status `IN_PROGRESS` until exact-head required CI is green and protected merge is complete.
3. Stage only intended files.
4. Perform a staged-diff review and `git diff --cached --check`.
5. Commit in English with a concise Spec 073 message.
6. Push normally to `spec/073-final-ui-polish`.
7. Open or update the Spec 073 PR against `main` with an English body that includes:
   - scope summary;
   - native/CLI/Web-reference boundaries;
   - Impeccable evidence;
   - rendered evidence hashes;
   - local qualification results;
   - explicit release/MESC non-claims.
8. Reverify the exact remote PR head SHA.
9. Wait for every required check applicable to that exact head. Do not reuse stale CI from another SHA.
10. Resolve any material review/thread/check finding with a new forward commit and requalify the new exact head.
11. Merge only through the repository's protected normal merge path after all gates are genuinely green. Never rebase or force-push.

## Phase F — Post-main verification and canonical closure

1. Fetch main and verify the actual merge commit/tree and PR state.
2. Verify all required post-merge workflows triggered for the merge commit complete successfully.
3. Only then update canonical closure truth in a forward closure change if repository governance requires a post-merge closure commit/PR; follow the existing Spec 068–072 pattern exactly.
4. Final canonical truth must state:

```text
SPEC_073 = CLOSED_CANONICAL
MEDSCALE_REPOSITORY_IMPLEMENTATION = COMPLETE_PENDING_EXTERNAL_GATES
PROMOTED_REPOSITORY_OWNED_RESIDUALS = 0
NEXT_PROMOTED_SPEC = NONE
RELEASE_READY = FALSE
PRIVATE_DATA_READY = FALSE
MULTI_CLIENT_RELEASE_READY = FALSE
REAL_PHI_AUTHORIZED = FALSE
MESC = SEPARATE_PROJECT
```

5. Do not promote Spec 074+.

## Completion report

Return exact evidence, not a narrative guess:

```text
SPEC_073_STATUS=
BASE_SHA=
FINAL_HEAD_SHA=
PR_NUMBER=
PR_HEAD_SHA=
MERGE_SHA=
POST_MAIN_SHA=
LOCAL_QUALIFICATION=
IMPECCABLE_FINDINGS=
RENDERED_EVIDENCE=
REQUIRED_CI=
POST_MERGE_CI=
REPOSITORY_IMPLEMENTATION_CLOSURE=
RELEASE_READY=FALSE
MESC=SEPARATE_PROJECT
EXTERNAL_BLOCKERS=
```
