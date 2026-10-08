# Founder Productization Review-Policy Amendment (2026-10-08)

**Status:** `ACTIVE`
**Authority:** founder decision A, 2026-10-08 (explicit written approval in the founder session).
**Scope:** only the productization stack, PRs #171 → #174 → #175 → #177 → #179 (Specs 095–101).
**Amends:** the scoped independent-review requirement of the 2026-09-30 productization directive, recorded as:
- `docs/planning/TAURI_DESKTOP_MIGRATION_DECISION_2026-09-30.md` ("Jev and Alibaba OCR status" per merge candidate);
- `docs/planning/PRODUCTIZATION_PROGRAM.md` (zero new cost; Jev and Alibaba Open Code Review requested where applicable);
- `docs/planning/EXTERNAL_GATES.md` row `PRODUCTIZATION_ZERO_COST_REVIEW_095` (on the stack branches).

## History preserved

- 2026-09-22: `FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md` removed external semantic review as a general merge gate in favor of deterministic qualification.
- 2026-09-30: the productization directive reinstated Jev independent judgment and Alibaba Open Code Review for this stack, with zero new cost.
- 2026-09-30 to 2026-10-08: no zero-cost execution path existed for either reviewer. The stack was recorded as `JEV_STATUS=BLOCKED_BY_ZERO_COST` and `ALIBABA_OCR_STATUS=BLOCKED_BY_ZERO_COST`. No paid call was made and no review result was fabricated. All five PRs remained draft and unmerged.

## Decision

Engineering qualification and eligible normal merges of the stack may proceed **without** paid independent Jev or Alibaba Open Code Review execution.

```text
JEV_STATUS              = NOT_EXECUTED   (no independent Jev judgment exists for this stack)
ALIBABA_OCR_STATUS      = NOT_EXECUTED   (no Alibaba Open Code Review result exists for this stack)
INDEPENDENT_REVIEW      = NOT_PERFORMED  (never reported as passed or approved)
SUBSTITUTE_REVIEWER     = NONE           (Claude self-review, CodeRabbit, Cubic, Qodo and other bots
                                          are not independent review evidence)
ADDITIONAL_SPEND        = 0              (no new reviewers, model calls, services or infrastructure)
```

## Merge and closure gate for the stack

Each candidate must provide all of the following, bound to its exact head:

1. **Deterministic tests.** Rust fmt, Clippy with denied warnings and tests; frontend tests and build; the contrast guard; routes and IPC tests; bundle-configuration tests.
2. **Exact-head CI.** The six required `main` checks pass on the candidate's exact head, and post-merge CI on `main` passes.
3. **Native qualification.** The `tauri-preview` native build, test and package jobs on Windows, macOS and Ubuntu where the candidate contains the Tauri desktop, plus the Spec 101 installer qualification where it contains packaging.
4. **Independent machine-generated verification evidence.** cargo-deny and supply-chain policy output, the Spec 101 release-set verifier and its negative tests, and installer install/launch/uninstall reports with artifact digests.
5. **Security and architecture analysis.** A written analysis of the candidate's effective diff: trust boundaries, Tauri capabilities and CSP, IPC surface, Core authority, data paths, dependency graph changes. Unresolved security findings are listed explicitly.
6. **Review threads.** All review threads are resolved (enforced by the `main` ruleset).
7. **Merge method.** Normal merge commits only. No squash, rebase, force-push or history rewrite. Branch protection is unchanged.

## What this amendment does not change

This amendment changes only the independent-review requirement for this stack. It does **not** waive or relax:

- dependency-security findings, including F096-T01 (`glib` 0.18.5, RUSTSEC-2024-0429). A failing dependency-security check is not accepted by this amendment. Merging code whose own dependency-security check fails requires a separate, explicit founder risk-acceptance decision. **No such acceptance exists** (see the correction below);
- required CI checks or the `main` ruleset;
- signing (`SIGNING_STATUS=NOT_GRANTED`);
- privacy (F096-T05; `PRIVATE_DATA_READY=false`);
- clinical validation (`NOT_PERFORMED`) and regulatory status (`NOT_PERFORMED`);
- release readiness (`RELEASE_READY=false`, `PLATFORM_QUALIFIED=false`, `MULTI_CLIENT_RELEASE_READY=false`);
- the synthetic-only rule (no real PHI);
- Slint's status as the legacy reference implementation pending a separate retirement decision.

It does not apply to any work outside the five listed PRs.

## Correction (2026-10-08)

An earlier commit on this PR added `FOUNDER_RISK_ACCEPTANCE_RUSTSEC_2024_0429_2026-10-08.md`. It was based on an answer selected in an in-session question prompt. The founder has since stated that the approval covered **only** this review-policy amendment and the F101-02 implementation, and that no separate acceptance of the `glib` advisory was authorized.

That document is **withdrawn** by a forward commit; Git history is preserved and nothing was merged on its basis. Consequences:
- F096-T01 stays `OPEN` with no risk acceptance;
- PRs that bring the Tauri desktop (and its failing `tauri dependency policy` job) into `main` (#174, #175, #177, #179) are **not merge-eligible** until the advisory is genuinely remediated or the founder explicitly accepts it in writing;
- #171 (no Tauri, no `glib`) is unaffected.
