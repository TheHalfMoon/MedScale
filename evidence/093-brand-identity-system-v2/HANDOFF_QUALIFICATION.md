# Spec 093 — V0 Handoff Qualification

Date: 2026-09-17
Branch: `design/medscale-identity-v2`
Branch base at worktree creation: `origin/main@ee8daef3a2782bbdbcb3324766a5d6b95c09fa09`

## Qualified setup scope

This packet qualifies the design-system setup and V0 handoff only. It does not qualify final rendered UI, production adoption, exact-head merge readiness, or Spec 093 canonical closure.

Prepared artifacts include:
- founder-approved ScaleFold M gradient/app-icon/monochrome assets;
- light and dark intro reference assets;
- canonical machine-readable design tokens;
- deterministic CSS and Slint token generation;
- typography, icon, product grammar, component, page, and motion contracts;
- V0 master prompt, input manifest, and acceptance checklist.

## Delegation boundary

The founder delegated high-fidelity UI composition to V0 after the identity system was prepared. V0 remains a visual/reference implementation step. Product authority, clinical semantics, privacy/network permissions, model admission, release claims, and MESC scope are unchanged.

## Checks completed

- `python3 scripts/generate-brand-tokens.py --check` — PASS.
- canonical token JSON parsed successfully — PASS.
- eight Spec 093 SVG assets/runtime copies parsed as XML — PASS.
- `cargo check -p medscale-desktop --locked` — PASS on the authorized macOS host; this also compiled the changed runtime SVG assets through the Desktop build path.
- `git diff --check` — PASS after removing Markdown trailing whitespace.

The generated `theme_tokens.slint` is a handoff artifact and is not yet imported into the production theme; final production-port qualification remains pending after V0 visual acceptance.

## Parallel-work warning

Spec 074 is an active independent implementation lane. Latest observed remote Spec 074 head during setup was `f170c8e29cf932211e654b8aa0d180505981d66b`; this is informational only and must be reverified before any reconciliation or merge. No Spec 074 feature code is part of this handoff branch.

## Remaining before canonical closure

V0 must produce the reference UI, the result must be reviewed against the acceptance checklist, accepted visual decisions must be ported only into authorized production surfaces, rendered evidence must be captured, and normal exact-head CI/review/merge/post-main qualification must still pass.

## 2026-09-18 ScaleFold language refinement

Founder review of an Abridge identity-system reference prompted a bounded design-system refinement: the ScaleFold M is now explicitly treated as the seed of an original generative visual language rather than a standalone mark.

Added/updated handoff authority covers:
- Pillar / Fold / Step / Gap / Rhythm primitives;
- ScaleFold supergraphics and pattern grammar;
- ScaleFold Display as a treatment derived from Instrument Sans, not a new font;
- original brand-to-work and mark-assembly motion grammar;
- V0 acceptance criteria requiring recognizability without constant logo repetition;
- explicit prohibition on copying Abridge waveform/masonry language or other competitor-specific identity devices.

This refinement changes design language only. It does not alter MedScale product authority, runtime behavior, clinical semantics, privacy policy, model admission, network policy, or Spec 074 scope.

Validation after the refinement:
- `python3 scripts/generate-brand-tokens.py --check` — PASS.
- `git diff --check` — PASS.
