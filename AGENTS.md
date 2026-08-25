# AGENTS.md — MedScale Repository Execution Rules

## Authority order

1. Explicit founder instruction for the current task.
2. Live GitHub repository truth for mutable state.
3. This repository's canonical planning artifacts.
4. Frozen MedScale architecture and contracts referenced by the planning artifacts.
5. Primary standards/upstream evidence within their scoped subject.
6. Historical planning/review material.

Never use a stale handoff or remembered SHA over live repository truth.

## Current repository phase

```text
PHASE = PLANNING_CANONICAL_V2
PRODUCT_IMPLEMENTATION = NOT_STARTED
DEFAULT_IMPLEMENTATION_AUTHORITY = NO
REAL_PHI_AUTHORITY = NO
MODEL_EXECUTION_AUTHORITY = NO unless explicitly granted
NETWORK_EGRESS = DEFAULT_DENY
```

The presence of a plan, roadmap, or task does not itself authorize implementation.

## Mandatory build process

MedScale uses GitHub Spec Kit and a Spec-of-Specs approach. Follow `docs/planning/SPECKIT_MASTER_ROADMAP_V2.md` in dependency order.

For each material executable spec, use:

```text
/speckit.specify
/speckit.clarify
/speckit.plan
/speckit.checklist
/speckit.tasks
/speckit.analyze
-> qualification
-> /speckit.implement only with explicit founder implementation authorization
-> /speckit.converge after implementation
```

Repository constitution changes additionally require `/speckit.constitution`.

## Non-negotiable architecture

- Rust is the primary language and trusted-core language.
- Local-first and privacy-first are constitutional.
- Network egress is default-deny.
- Useful core operation requires neither a cloud account nor a remote model.
- No hidden telemetry, remote logging, or remote crash upload is required.
- Source bytes remain distinct from derived representations.
- Source identity is not content-hash identity.
- `Proposal != ClinicalAssertion`.
- Identity merge is explicit; never silently auto-merge.
- Projections are rebuildable and non-authoritative.
- FHIR R4 4.0.1 is the initial interchange baseline, not the canonical database.
- External validator output is evidence; MedScale owns acceptance.
- Workers receive no ambient canonical DB, master keys, unrestricted filesystem, network, secrets, or authority.
- MESC boundary is `ARTIFACT_FIRST + EXCEPTIONAL_SANDBOXED_SERVICE`.
- External action state includes `UNKNOWN`; UNKNOWN is never blindly retried.

## Donor / OpenMed rules

Before any donor code or artifact enters MedScale, record exact upstream source/revision, licence/permission evidence, transitive dependencies, maintenance responsibility, security placement, tests, SBOM/provenance, and exit strategy.

Allowed dispositions:

```text
ABSORB
PORT_TO_RUST
VENDOR
FORK
SIDECAR
REFERENCE
REJECT
```

Do not reject a capability solely because it is non-Rust. Do not allow non-Rust or donor code to own MedScale authority semantics.

## H0 hard boundary

H0-A/H0-B are synthetic-only and LLM-free. They do not include real PHI, OpenMed integration, MESC integration, OCR, ASR, vector search, agents, live SMART/NPHIES, external actions, or model inference.

## Mutation discipline

Before every material mutation, re-read the relevant live branch/head, changed files, active PRs, CI/review state, current spec, and dependency gates.

Do not force-push, rewrite history, bypass failed tests, or claim PASS without exact evidence.

When a unit becomes `CLOSED_CANONICAL`, continue only to the next unit that is genuinely eligible and authorized by both repository ordering and current founder authority.
