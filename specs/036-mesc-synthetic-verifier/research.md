# Research — Spec 036

## Problem
Spec 012 ships fail-closed admit only. Digests on the admit request were never verified against local bytes. `MESC_ARTIFACT_ACCEPTANCE.md` required a versioned manifest verifier with synthetic adversarial fixtures before a real release can qualify.

## Decision
Ship MedScale-owned synthetic verifier READY_BASE independently of upstream MESC assets. Product admit stays gated. Synthetic fixtures are labeled non-MESC and never authorize Pack enablement.

## Non-goals
Publisher signature policy, live GitHub asset fetch, model execution, clearing EXTERNAL_GATES.
