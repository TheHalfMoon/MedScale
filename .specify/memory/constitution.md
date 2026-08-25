<!--
Sync Impact Report
- Version change: (template placeholders) → 1.0.0
- Modified principles: template placeholders → MedScale canonical principles I–VII
- Added sections: Authority & Process Topology; Source, Privacy & External Gates
- Removed sections: none (template structure retained)
- Follow-up TODOs: none
- Source: docs/planning/MASTER_BUILD_PLAN_V2.md, AGENTS.md, CURSOR.md, IMPLEMENTATION_AUTHORITY.md
-->
# MedScale Constitution

## Core Principles

### I. Rust-Owned Trusted Core
Rust is the primary and trusted-core language. One Rust authority path serves CLI,
Desktop, mobile, and SDK surfaces. UI, CLI, workers, models, and donor frameworks
MUST NOT bypass that path or open the canonical store, keys, or unrestricted
filesystem directly. Non-Rust code may be best-in-class for a capability but
NEVER obtains product authority by itself.

### II. Local-First and Privacy-First
Local-first and privacy-first are constitutional. Useful core operation MUST NOT
require a cloud account or remote model. Product runtime network egress is
DEFAULT_DENY until admitted through the Network Broker. Hidden telemetry, remote
logging, or crash upload MUST NOT be required for core use. Real PHI remains
unauthorized until an explicit external gate closes.

### III. Source Identity and Representation Discipline
Source bytes remain distinct from derived representations. Source identity is
NOT content hash. Spans are tagged with representation and coordinate system.
Terminology is a pack/rights/version problem, not a silent string lookup.
Donor permission for code NEVER implies permission for model weights, datasets,
or restricted terminology.

### IV. Authority Classes Remain Distinct
`Proposal != ClinicalAssertion`. AI, model, and worker output have no authority
by themselves. Projections are rebuildable and non-authoritative. Identity merge
is explicit. FHIR R4 4.0.1 is interchange, not the canonical database; external
validator output is evidence only. Durable object classes remain distinct.

### V. Evidence Before Claims
No `PASS`, `PARITY`, `SURPASS`, `PRIVATE`, `OFFLINE`, `CONFORMANT`, or
`CLOSED_CANONICAL` claim is valid without exact-head evidence. Tests, reviews,
and supply-chain gates MUST bind commit/tree/toolchain/lock/platform results.
Never force-push, destructively rewrite history, or bypass failing required checks.

### VI. Fail-Closed Safety
Workers receive no ambient canonical DB, master keys, unrestricted filesystem,
network, secrets, or authority. External-action `UNKNOWN` is NEVER blindly
retried. Hostile inputs are confined by risk class. Prefer fail-closed over
permissive behavior when ambiguity remains.

### VII. Minimal Reversible Architecture
Prefer smallest reversible designs, mature dependencies behind narrow interfaces,
and compile-time ownership boundaries over premature abstraction or crate-per-noun
fragmentation. Never weaken an invariant to ease a dependency, donor, UI
framework, model, or platform integration.

## Authority & Process Topology

One MedScale Core Host owns each desktop/headless vault’s writable canonical
metadata connection and active vault key material. Desktop UI, CLI, and SDK are
local IPC clients. A per-vault single-writer lease prevents independent writers.
Mobile uses one app-owned Rust core instance with native Keychain/Keystore.
MESC is independent and ARTIFACT_FIRST; MedScale MUST NEVER mutate the MESC
repository or import its Python runtime as product authority.

## Source, Privacy & External Gates

Before copying, porting, vendoring, or linking donor code, follow
`docs/planning/SOURCE_ACQUISITION_AND_COPY_PLAN.md` and create a provenance
admission record. OpenMed baseline is v2.2.0 commit
`59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837`. The founder supplies visual UI via
v0; Cursor owns Rust core, contracts, IPC, accessibility, tests, packaging, and
privacy qualification. v0-generated server/database/network code is untrusted
unless an owning MedScale spec admits it. External human gates are recorded in
`docs/planning/EXTERNAL_GATES.md` and block only their path.

## Governance

This constitution supersedes conflicting historical planning authorization
metadata. Architecture and safety constraints in planning documents remain
binding. Ordinary implementation proceeds under
`docs/planning/IMPLEMENTATION_AUTHORITY.md` without asking the founder for
routine decisions. Amendments require documented rationale, version bump, and
synchronization of `AGENTS.md`, Cursor rules, Spec Kit artifacts, and
`BUILD_QUEUE.md`. All PRs MUST verify constitutional compliance for their scope.
Use `docs/planning/IMPLEMENTATION_DECISION_DEFAULTS.md` for ordinary engineering
choices and `docs/planning/DEFINITION_OF_DONE.md` for closure.

**Version**: 1.0.0 | **Ratified**: 2026-08-25 | **Last Amended**: 2026-08-25
