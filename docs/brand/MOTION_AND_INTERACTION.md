# MedScale Motion and Interaction

**Status:** FOUNDER_APPROVED_HANDOFF — Spec 093

## Principle

Motion explains a change in state, preserves spatial understanding, or confirms an action. It is not decoration and must never compete with research or clinical content.

## Timing

Reference durations:
- hover/focus color response: 80–120 ms;
- button/menu state: 120–160 ms;
- pane/tab transition: 160–220 ms;
- dialog/sheet transition: 180–240 ms;
- intro/onboarding brand motion: 400–900 ms, only when it does not delay useful startup.

Prefer ease-out for entering content and ease-in-out for position changes. Avoid springy/bouncy motion in dense professional workflows.

## Reduced motion

Honor reduced-motion preferences. Replace large transforms with opacity or immediate state changes. Do not autoplay decorative loops.

## Pane behavior

Split panes should resize directly and predictably. New outcome panes may reveal with a subtle width/opacity transition. Preserve user-resized widths within the current workspace/session where practical.

## Feedback

Actions should acknowledge immediately through control state, inline progress, or local status. Do not use global toasts for every successful interaction.

## ScaleFold brand motion

Brand motion may use the ScaleFold sequence defined in `SCALEFOLD_LANGUAGE.md`: Pillars establish rhythm, Folds connect them, the full Spectrum appears at the identity resolution, and the geometry recedes as the product enters Work mode.

Preferred brand assembly is 450–800 ms. A full intro should not exceed actual initialization and should preferably resolve within 1200 ms.

The Source → Evidence → Action sequence may be used as an abstract workflow metaphor, but never as a claim of clinical correctness or authority.

Never imitate speech waveforms, use perpetual gradient loops, or turn product loading into decorative theater.
