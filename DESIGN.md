# MedScale design system

**Current authority:** Spec 095 and the founder-approved identity supplied 2026-09-29. See [productization](docs/planning/PRODUCTIZATION_PROGRAM.md); launch UI is not frozen.

## Identity

The locked mark derives from paired equilateral triangles forming an abstract M, with two peaks, center contact and lower negative space. Only pure black #000000 or white #FFFFFF. No stroke, rounding, circle, gradient or healthcare/AI decoration. assets/brand/mark-black.svg is the master; inverse/native variants share its path. docs/brand/LOGO_SPEC.md defines size, clear space and lockups.

MedScale is a **Clinical Intelligence OS**. **Evidence first. Action second.** is an operating principle, not a readiness claim. Typography and clear relationships carry the identity; triangles belong primarily to the logo.

## Tokens and typography

crates/medscale-desktop/ui/theme.slint is the native source. Inter handles UI/wordmark; JetBrains Mono NL handles exact IDs, hashes, timestamps and scientific operators without code ligatures. Minimal admitted OFL assets are embedded for deterministic offline rendering and distributed with their licenses. Full roles: docs/brand/TYPOGRAPHY_SYSTEM.md.

Light uses white/near-white; dark uses black/near-black layers. Neutrals express hierarchy. Focus/selection/interaction are monochrome. Optional functional accents supplement explicit labels and never become branding.

## Product grammar

Prefer panes, readable tables, lists, inspectors, timelines and thin separators. Cards require a grouping purpose. No marketing hero, invented metrics, decorative tiles or perpetual animation. Target 1440×900, minimum 1100×720. Compact changes spacing/control geometry while retaining readable type. Navigation scrolls independently and retains all 25 real routes.

Home's Command Center shows existing synthetic patient context, coverage uncertainty, actual runtime truth and clear links to clinical/evidence/research work. It is not a live roster or clinical risk ranking. Navigation and route qualification proceed through Specs 095–100.

## States and evidence

Current, reviewed, unknown, stale, conflicting, partial, denied, unavailable, unsupported, unmeasured and corrupt retain distinct text and accessible meaning. Unknown never defaults to success. StateBadge supplies a literal state, explanation and distinct monochrome marker. Evidence discloses available source/revision, relationships, uncertainty, freshness/review and lineage. The existing Evidence route is a pinned comparative proof ledger, with no implied live literature service.

## Interaction and qualification

Controls retain Enter/Space, visible focus and exposed role/label/action. Targets are at least 36px normally and 32px compact. Default motion/elevation is zero; future transitions must explain state and respect reduced motion. Measure contrast from actual tokens; inspect actual native pixels. Record keyboard, assistive technology and WCAG claims separately. Compilation/source assertions do not qualify visuals.

Rust Core retains all authority. PHI is unauthorized, egress defaults deny, MESC is separate, and release/privacy/multi-client readiness stays false. CLI machine contracts remain stable. References and figure styles do not create capabilities.

## History

- Pre-068 identity: SUPERSEDED_BY_SPEC_068.
- Spec 068 active visuals: SUPERSEDED_BY_SPEC_073.
- Spec 073 circular Shelf-M, Instrument Sans/Source Serif and Mist Blue active visuals: SUPERSEDED_BY_SPEC_095.
- Closed Specs 068–094 and their evidence remain intact; new identity replaces forward.
