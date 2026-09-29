# Dedicated paper evaluation plan

**Snapshot:** `1e2b7d94e970256b38bda15fa91f62bc397e825a`

**Data policy:** synthetic only. No real PHI. No production credentials.

**Interpretation rule:** existing repository qualification is prior evidence. Dedicated paper experiments are new measurements and must be stored separately. Historical repaired defects are not counted as prospective failures.

## RQ1 — Provenance integrity

### Objective
Measure whether invalid provenance relationships are detected/refused without falsely refusing known-valid controls.

### Planned case families

1. source content changed while recorded digest is held constant;
2. digest changed while source bytes are held constant;
3. derived artifact bound to the wrong source identity;
4. transform identity/version mismatch where validation exists;
5. cross-project artifact/source binding;
6. forged or mismatched receipt where a receipt contract exists;
7. malformed lineage / parent reference;
8. evidence assessment with nonexistent citation but supporting relation;
9. retracted citation incorrectly marked as support;
10. tampered recovery material that changes provenance-bearing state;
11. recovery/receipt mismatch after restore;
12. matched valid controls for every mutation class.

### Metrics
- invalid cases tested;
- false accepts (primary safety-oriented error count);
- detected/refused invalid cases by class;
- valid controls tested;
- false refusals;
- exact emitted error/state;
- deterministic repeat agreement.

Do not report a percentage without its numerator/denominator and case-generation rule.

## RQ2 — Adversarial and failure qualification

### Pre-specified classes
Malformed input; forged signatures; replay; revoked identity; cross-project access; corrupt backup; interrupted migration; stale/invalid state; malicious digest; wrong privacy classification; unknown/unavailable external effect; ID-sequence rollback; duplicate submission; unauthorized capability; corrupt event/state chain when the subsystem exposes such a path.

### Result schema

| case_id | subsystem | fault | precondition | expected | observed | evidence | result |
|---|---|---|---|---|---|---|---|

A fail-closed refusal counts as correct only if refusal is the pre-specified expected behavior.

## RQ3 — Reproducibility and recovery

### Determinism
For deterministic operations, run identical inputs in independent fresh synthetic vaults and compare the declared invariant (digest, canonical serialization, query result, or state transition). Avoid comparing timestamps/random identifiers unless the contract claims deterministic identity.

### Recovery chain

```text
seed synthetic state
-> verify
-> backup
-> verify backup
-> restore into fresh destination
-> verify restored state
-> restart/reopen
-> re-verify
-> create valid post-restore objects/actions
-> verify monotonic identity/state invariants
```

### Cross-platform scope
Reuse hosted Linux/Windows/macOS CI only for cases that genuinely execute on each OS. Platform-specific exclusions remain explicit.

## RQ4 — Performance and storage cost

### Scales
100, 1,000, and 10,000 synthetic artifacts initially.

### Candidate measurements
- source ingestion;
- provenance-bearing derived artifact creation;
- provenance/evidence verification;
- selected knowledge retrieval;
- backup and restore;
- deterministic compute startup/execution if safely comparable;
- database/blob storage growth attributable to test corpus.

### Procedure
Record CPU, memory, OS, toolchain, build profile, commit, warm-up rule, repetitions, raw monotonic timings, and output sizes. Prefer repeated trials and report median, p95, and dispersion. Add confidence intervals where sample design supports them.

### Baseline discipline
Use the term `overhead` only for a paired baseline that performs the same semantic operation without the measured provenance work. If no fair baseline exists, report absolute cost rather than manufacturing an ablation.

## Optional Experiment E — ablation

Deferred by default. It may proceed only as an isolated research harness after documenting:
- scientific question;
- identical input corpus;
- what validation is removed/simulated;
- why the comparison is not confounded by unrelated code paths;
- why production MedScale is not weakened.

## Artifact layout to create during implementation

```text
paper/artifact/
  README.md
  manifests/
  fixtures/
  scripts/
  raw/
  generated/
  checksums/
```

Raw results are append-only per experiment run. Generated manuscript tables/figures must be reproducible from `raw/` and never hand-edited as authoritative data.
