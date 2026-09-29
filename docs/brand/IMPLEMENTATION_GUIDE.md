# MedScale Identity V2 — Implementation Guide

**Status:** CANONICAL_SPEC_093

## Layer 1 — Brand

Owned by `LOGO_SPEC.md`, `BRAND_IDENTITY_SYSTEM.md`, and the assets under `assets/brand/` plus the runtime logo assets under `crates/medscale-desktop/ui/assets/`.

Brand defines recognition. It does not encode product state.

## Layer 2 — Tokens

Owned by `assets/brand/tokens/medscale.tokens.json` and generated outputs. Product code should consume semantic tokens (`canvas`, `surface`, `ink`, `accent`, `success`, and so on) instead of copying raw brand hex values into page code.

## Layer 3 — Components

Reusable components own interaction behavior, focus, accessibility labels/actions, spacing, and state geometry. Pages should compose those primitives instead of reimplementing hover/focus/disabled behavior.

## Layer 4 — Product patterns

Research workspace, evidence panel, model/runtime state, project workspace, analytics, and collaboration patterns may add domain structure but must preserve the same token and component semantics.

## Layer 5 — Pages

A page owns information architecture and task flow. It does not invent a new mini design system.

## Theme rule

There is one component tree and one semantic token model. Light and dark are theme values, not separate page implementations.

## Brand intensity rule

Use the MedScale Spectrum at three intensity levels:
- **Signature:** full gradient mark/app icon/intro;
- **Focal:** one bounded gradient or spectrum accent in a page or major surface;
- **Work:** solid semantic interaction/state colors only.

Dense clinical/research work should spend most of its time at the Work level.

## Parallel implementation rule

Spec 093 is intentionally isolated from active Spec 074 feature work. Shared-file conflicts must be reconciled from current main with a normal merge before exact-head qualification. Never force-push or rebase shared history.

## V0 handoff boundary

The founder delegated high-fidelity UI composition to V0 after the identity system was established. V0 should start from `V0_MASTER_PROMPT.md` and `V0_INPUT_MANIFEST.md`, then validate against `V0_ACCEPTANCE_CHECKLIST.md`.

V0 is a design/reference implementation step. Do not wire production authority or claim a feature exists merely because a visual surface is rendered.

After V0 acceptance, port only the approved visual decisions into whichever production surface is explicitly authorized at that time. Reuse semantic tokens and component contracts rather than copying arbitrary generated styling.
