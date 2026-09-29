# Spec 093 — Brand Identity System V2

Status: DESIGN_SYSTEM_HANDOFF_READY

## Founder promotion

On 2026-09-17 the founder explicitly approved a new MedScale identity direction and asked for implementation, not another concept pass. The approved direction uses the distinctive geometric `M` selected in founder review, a blue-to-violet-to-pink signature spectrum for the mark and intro moments, and a calm dual-theme product system that combines high recognizability with clinical/research restraint.

This instruction is higher authority than the visual restrictions frozen by Spec 073. Spec 073 remains historical evidence for the previous identity and its product-safety work. This spec supersedes only the visual identity, token, typography hierarchy, icon grammar, and presentation rules; it does not reopen clinical authority, privacy, model, network, release, or evidence claims.

The active Spec 074 Project + Artifact Graph work remains independent. Spec 093 is implemented on a separate branch and must not change Spec 074 feature scope or authority. Before merge, this branch must reconcile current `main` normally, without rebase or history rewrite, and rerun affected qualification.

## Goal

Ship a durable MedScale identity system that is recognizable at app-icon size, coherent in light and dark themes, reusable across native Desktop and future surfaces, and technically governed from one token source rather than page-by-page styling. The visual UI implementation is delegated to V0; this repository branch prepares the identity, tokens, assets, component grammar, page blueprints, and acceptance contract V0 must follow.

## Acceptance criteria

1. Freeze one original MedScale `ScaleFold M` geometry with gradient, dark-monochrome, light-monochrome, and app-icon variants.
2. Keep the signature spectrum primarily in the logo, intro/onboarding, selected brand moments, and bounded focal accents; product state remains semantic and never depends on gradient color.
3. Establish one canonical machine-readable token source for brand colors, light/dark semantic surfaces, geometry, and type scale.
4. Generate CSS and Slint token artifacts deterministically from that source and provide a drift check.
5. Preserve a calm light-first research workspace and a true layered dark theme using the same semantic component grammar.
6. Preserve Instrument Sans for product/UI, Source Serif 4 for selective editorial/long-form material, and platform monospace for machine-readable values.
7. Define an original icon language with simple distinguishable silhouettes, rounded joins, balanced optical weight, and no dependency on competitor assets.
8. Apply the new token system and mark to the native Desktop theme without introducing backend authority or changing truth semantics.
9. Provide a polished non-production Web reference showing logo, typography, color, icons, components, light mode, and dark mode.
10. Preserve keyboard focus, labels/actions, status text, local-first posture, evidence provenance, release non-claims, and MESC separation.
11. Pass token-generation checks, Desktop compile/tests/Clippy, diff checks, rendered review, exact-head CI, normal merge, and post-main verification before canonical closure.

## Non-goals

No backend feature, clinical decision authority, provider integration, network permission, model admission, real PHI, mobile runtime, production Web application, release-readiness claim, WCAG certification claim, or MESC work is authorized by this spec.
