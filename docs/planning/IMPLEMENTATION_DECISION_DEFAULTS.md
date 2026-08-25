# Implementation Decision Defaults

Use this precedence whenever a current spec leaves an ordinary engineering choice open. Do not ask the founder merely because multiple reasonable implementations exist.

1. Live repository truth and current qualified spec/tasks.
2. Frozen constitutional invariants.
3. Fail closed over permissive behavior.
4. Local/offline over cloud/network dependence.
5. Privacy/minimum disclosure over convenience.
6. Rust-owned authority semantics over donor/framework semantics.
7. Memory-safe/bounded implementation over unsafe/native code when capability is equivalent.
8. Mature dependency behind a narrow interface over unnecessary rewrite.
9. Smallest reversible architecture over premature abstraction.
10. Deterministic behavior/evidence over hidden heuristics.
11. Explicit provenance/rights/version pinning over convenience downloads.
12. Measured benchmark result over brand preference.

If two options remain equivalent, choose the simpler option with fewer transitive dependencies and a clearer exit path. Record the decision in the owning `research.md`, `plan.md`, or ADR and continue.

## Escalate to an external gate, not a question

Only external facts that Cursor cannot legitimately decide become gates: legal/counsel determination, acceptance of gated terms, production credentials, real PHI authorization, partner contracts/endpoints, app-store signing/account actions, or founder-supplied final v0 design when a final visual release literally depends on it.

Record the exact blocker and continue independent work.