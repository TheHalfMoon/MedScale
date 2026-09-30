# Impeccable design skill — Spec 096 development tool

- Upstream: https://github.com/pbakaus/impeccable
- Exact revision: `0d6b47ea19b63afe15e3f93a44d5d9fbbc6fd275`.
- Original bounded path: `.agents/skills/impeccable/`; upstream Git tree identity `98dc8642f86c9778ef343627045c50c9032f305c`.
- Upstream engine/launcher version: `0.1.8` (`ENGINE_VERSION` and skill `scripts/VERSION` at the pinned revision).
- Operation and target: bounded development-skill snapshot at `.agents/skills/impeccable/`. It is not linked to the MedScale Rust runtime, and no detector hook is installed.
- Rights: upstream `LICENSE` is Apache License 2.0, copyright 2025 Paul Bakaus. Preserve upstream notices and do not redistribute an engine binary or download one as part of the product. Upstream `NOTICE.md` and `LICENSE` remain the source of rights evidence; copy them with this snapshot if it is retained.
- Modifications: whitespace-only cleanup in `reference/extract.md`, `reference/harden.md` and `reference/optimize.md` to satisfy repository `git diff --check`; no instruction or executable behavior changes. MedScale's approved board, founder direction, product truth and local design system outrank generic recommendations. Inter, JetBrains Mono NL and monochrome identity are deliberately retained even where generic Impeccable anti-patterns disagree.
- Transitive execution: the bundled launcher can download a self-contained engine on first use. No hook, launcher or engine executes by installing this snapshot. Any future run must be explicit, cost-free, local to development, and reviewed before enabling an automatic hook.
- Security placement: agent guidance only, outside Core, workers, vault, clinical data and the shipped Desktop package. It receives no PHI, production credentials, network authority or canonical data.
- Qualification: verify the selected files against the pinned subtree, record a local digest, inspect the installed skill and its notices, then run only applicable zero-cost design critique/detection. Native Slint frames and source-bound evidence remain necessary for visual acceptance.
- Update/exit: update only by selecting a new exact upstream revision and repeating rights/content review. Remove the project-local skill and provenance to exit; no product migration is required.
