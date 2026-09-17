# MedScale Research OS Acceptance Framework

**Status:** Planning candidate — not implementation authority

## Purpose

The Research OS program is too broad to accept through feature checklists. Every promoted specification should prove a bounded set of invariants using executable evidence where possible.

## Universal gates

### G1 — Authority
- all authority-changing paths are typed and explicit;
- model/agent/event/browser/worker confidence cannot bypass authority;
- approvals identify actor, scope, target revision and expiry where relevant.

### G2 — Provenance
- every derived artifact binds exact inputs/revisions;
- executable model/runtime/tool identity is immutable and inspectable;
- copied/adapted donor components preserve exact provenance.

### G3 — Privacy
- data classes are explicit;
- egress and sharing are policy-checked outside the model;
- sensitive logs/caches are bounded;
- denial paths are tested.

### G4 — Local-first
- the declared local product path works without mandatory MedScale cloud services;
- network dependencies are explicit capabilities;
- offline degradation is honest and recoverable.

### G5 — Reproducibility
- analytics, model, retrieval, Browse, audio and compute runs preserve the parameters/environment/source identities needed to understand or reproduce the result within declared limits.

### G6 — Failure semantics
- cancelled, partial, failed, unavailable, denied, stale and unknown are distinct states;
- no silent fallback creates a false PASS.

### G7 — Recovery
- crash/restart/resume behavior is defined;
- durable state has backup/migration/rollback expectations;
- partially completed workflows do not create ambiguous authority.

### G8 — Performance/resource truth
- supported envelopes are measured on declared hardware;
- latency/memory/storage/network claims include workload and method;
- no competitor/runtime superiority claim without matched evidence.

### G9 — Accessibility and user control
- keyboard/focus/labels are preserved in native UI;
- capture/recording/network/agent/browser activity that matters to user trust is visible;
- cancellation/stop paths are reachable.

### G10 — Security
- capability boundaries are threat-modeled;
- hostile project/web/model content cannot directly grant tools/authority;
- worker/browser/connector isolation is tested according to risk.

## Domain-specific gates

### Project + Artifact Graph
- existing canonical object IDs/provenance are preserved rather than duplicated;
- Project relations are typed/revisioned;
- archive/migration/reopen are tested;
- Project organization never becomes a second authority model.

### Collaboration
- collaboration event != canonical artifact;
- task/note conflict behavior is deterministic or explicitly user-mediated;
- agent membership does not imply authority grants;
- ephemeral presence cannot become durable authority.

### MedAgent/Fleet
- context selection is inspectable;
- tool calls carry receipts;
- multi-lane comparison separates evidence from consensus;
- partial lane failures remain visible;
- external delegates cannot receive sensitive data unless explicitly authorized.

### Privacy Gate
- benchmark representative PII/PHI classes;
- residual risk is reported honestly;
- reversible mappings have separate key authority;
- export/Browse/model/Hub/Compute attempts after denial fail closed.

### Governed Browse
- all product egress uses admitted network/broker authority;
- PUBLIC versus transformed/deidentified versus protected/local data-class behavior is proven;
- redirects/private-network/SSRF abuse fails closed under declared policy;
- deterministic retrieval/browser routes are preferred before agentic browsing;
- hostile pages/snippets/downloads cannot change instructions/capabilities or exfiltrate other project context;
- raw credentials never enter model prompts or normal logs;
- download/upload boundaries are explicit and quarantined where applicable;
- `BrowseReceipt` binds source URL/origin, retrieval time, route/tool identity and evidence span/digest where obtainable;
- side-effecting browser actions remain denied until separately qualified with effect/approval/Unknown semantics.

### AudioFlow
- visible capture state;
- latency and long-session stability;
- Arabic/code-switch and medical-sensitive token evaluation where claimed;
- transcript revisions preserve source mapping;
- speaker identity is opt-in and deletable;
- capture/worker logs do not leak transcript/audio by default.

### Analytics
- generated SQL/plan is inspectable;
- source artifact revisions are bound;
- results are deterministic or uncertainty is documented;
- not-run statistical checks remain not-run;
- AI-generated queries default to safe/read-only behavior.

### Knowledge / RAG / Canvas
- retrieval is permission-filtered before disclosure/cache reuse;
- index/parser/chunker/embedding identity is preserved;
- stale/deleted source state propagates honestly;
- exact source spans/pages/URLs/audio timestamps are linkable where the source modality is integrated;
- no-evidence and conflicting-evidence states are first-class;
- Canvas references do not silently copy or detach provenance.

### Hub / Collaboration Sync
- tenant/project authorization is applied to data, search, caches, pub/sub and events;
- revoke-during-session behavior is tested;
- offline conflict resolution is deterministic or explicitly user-mediated;
- deletion/tombstone propagation cannot silently resurrect data;
- Hub loss does not destroy local project authority;
- backup/restore and supported-version rollback are rehearsed.

### Compute
- job manifests enforce filesystem/network/resource scopes;
- worker crash/OOM/timeout/cancel/lost/partial paths are distinguishable;
- outputs are bound to exact job/runtime/input identities;
- workers cannot enumerate ambient vault contents;
- late output after revocation/cancel is quarantined until Core reevaluation.

### Research Packs
- Pack schema cannot bypass Core authority or privacy;
- import/export contracts are versioned;
- domain validation is bounded and evidence-backed;
- Pack removal/upgrade has migration behavior;
- executable extensions use normal tool/worker boundaries.

### Institutional Adapters
- credentials are scoped and redacted;
- data-class/capability policy is enforced;
- mapping/version mismatch fails explicitly;
- external writes preserve idempotency/confirmation/`Unknown` semantics;
- provider unavailability cannot silently change authority or locality.

### Federation
- unknown cross-site policy denies;
- data-stays-at-site is the default;
- site/input provenance survives aggregation;
- revocation/expiry and network partition behavior are defined;
- statistical/privacy validity is proven for each promoted federated method.

## Definition of done

A candidate specification is complete only when its declared acceptance evidence is present for the exact reviewed revision. Planning text, UI presence, green compilation, or an agent's completion report alone are insufficient.
