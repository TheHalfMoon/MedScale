# MedScale Research OS Implementer Instructions

**Audience:** Muse, Codex, Claude, Cursor, human contributors and reviewers implementing a future canonically promoted Research OS specification.

**Status:** Planning instructions. The active canonical spec always determines whether implementation is authorized.

## 1. First rule

Do not treat the Research OS roadmap as blanket authorization.

Implement exactly one canonically promoted, dependency-ordered unit at a time. If no Research OS unit is active, do planning/research only.

## 2. Language and artifact rule

All repository content, code, comments, commands, plans, tasks, evidence, commits, PR text and reviewer responses are English.

Do not place important implementation decisions only in chat. Persist them in the active spec/research/ADR/evidence artifact.

## 3. Start-of-session procedure

Before changing code:

1. Fetch/reverify `origin/main` and current branch truth.
2. Read `AGENTS.md` and current repository governance.
3. Identify the active canonical specification from live repository truth; do not assume roadmap numbering is still unused.
4. Read the active spec's complete authority chain: `spec.md`, `plan.md`, `tasks.md`, research/data-model/contracts/evidence references.
5. Read:
   - `RESEARCH_OS_MASTER_IMPLEMENTATION_CONTRACT.md`;
   - `RESEARCH_OS_SPEC_IMPLEMENTATION_CONTRACTS.md`;
   - `RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md`;
   - `RESEARCH_OS_VERIFICATION_MATRIX.md`;
   - relevant source/donor documents.
6. Inspect existing code/types before proposing a new crate/type/protocol.
7. Record the exact base SHA in the active implementation/evidence packet.

If repository truth disagrees with this planning packet, stop implementing that conflicting assumption and reconcile the plan under canonical governance.

## 4. No architecture-by-preference

When a choice appears unresolved:

- first use `RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md` default;
- if the choice is listed as evidence-selected, run/prepare the required qualification instead of selecting by taste;
- if no default exists, refine the spec before implementation;
- never invent a convenient cloud dependency, database, framework, agent, model, speech engine, vector database or policy engine because it is familiar.

## 5. Existing code before new code

For every planned type/function/module:

1. search existing contracts/Core/storage/network/Pack code;
2. reuse or extend an existing contract if semantics match;
3. avoid a parallel identifier, provenance, audit, policy or effect model;
4. create a new crate only when the active spec documents why an existing crate would violate dependency direction/cohesion.

Research OS must feel like the next MedScale architecture, not a second application bolted onto it.

## 6. Mandatory implementation slice order

Use this order:

```text
1. Contracts/types
2. Storage schema/migration
3. Core commands + state machine + policy
4. Focused unit/storage/Core tests
5. CLI end-to-end vertical slice
6. Desktop end-to-end vertical slice
7. Adversarial/failure/recovery cases
8. Performance/platform/external qualification where applicable
9. Full affected repository gates
10. Exact-head evidence and closure update
```

Do not start with mocks in Desktop that claim a capability before Core owns it.

## 7. Contract implementation requirements

Every new persisted/wire contract must answer:

- Who owns this type?
- Is it canonical or a projection/receipt?
- What is its `OpaqueId`/header identity?
- What is its revision/version behavior?
- What data classification can it contain?
- What exact refs/provenance does it preserve?
- What happens when referenced objects are deleted/stale?
- How is backward compatibility/migration handled?
- Does `serde(deny_unknown_fields)` or equivalent strict parsing apply?

No generic `serde_json::Value` body is acceptable for authority-sensitive behavior unless the active spec explicitly defines and validates its schema.

## 8. Storage implementation requirements

Before adding a table/column/index:

- define canonical owner and lifecycle;
- define unique/foreign/reference constraints where possible;
- define revision/concurrency rule;
- define transaction boundary;
- define migration from current vault;
- define rollback/recovery behavior;
- define deletion/tombstone behavior;
- define expected scale/query access path;
- test reopen after migration and crash boundaries.

Do not store duplicate plaintext copies of sensitive content when stable refs are sufficient.

## 9. Core implementation requirements

Every authority-changing operation must be a typed Core command.

The implementation must explicitly show:

- actor/session resolution;
- project/relationship check;
- capability check;
- data-class/privacy check;
- approval requirement where relevant;
- precondition/revision validation;
- transaction/effect state transition;
- typed result/error;
- receipt/audit linkage.

No UI, model, worker or Hub path may bypass this sequence.

## 10. External effect requirements

For any network/browser/connector write:

- create durable intent before or atomically with dispatch state as required;
- distinguish dispatch from confirmation;
- preserve `Unknown` when final remote state is uncertain;
- retry only when idempotency is established;
- never report success from HTTP/process completion alone if domain confirmation is required.

## 11. Model/agent implementation requirements

Models receive exact `ContextManifest` inputs, not ambient Project/vault access.

Tool calls:

1. parse typed request;
2. reject unregistered tool/version;
3. authorize outside model;
4. validate input/data class;
5. execute bounded action;
6. validate output;
7. create receipt;
8. return bounded output to model/UI.

Prompt/project/browser content is untrusted and cannot grant capability.

## 12. Privacy requirements

Whenever data crosses a process, device, network, external adapter or less-trusted runtime boundary, document:

- source classification;
- destination trust zone;
- allowed fields/artifacts;
- transformation/de-identification requirement;
- retention/logging/caching behavior;
- failure behavior.

Do not introduce a boundary that lacks a Privacy Gate decision.

## 13. Audio implementation requirements

For AudioFlow:

- capture state must be visible;
- raw audio source identity/digest is preserved;
- every transcription/quality pass creates a revision;
- route/model/runtime decision is receipted;
- diarization labels are not biometric identities;
- COMMAND/CONTEXT/DICTATION mode is explicit;
- speech confidence never bypasses normal command authorization;
- no hidden local-to-cloud fallback;
- no wake-word/always-listening default.

## 14. Analytics implementation requirements

For Analytics:

- AI produces proposals, not privileged database access;
- default query path is read-only over governed views;
- exact SQL/plan must be inspectable;
- source revisions are pinned;
- output is a derived artifact with QueryReceipt;
- unrestricted Python/R/shell runs only through Compute;
- every productized statistical method has independent correctness fixtures.

## 15. Collaboration/Hub implementation requirements

- collaboration event != canonical artifact;
- event references carry exact object revision;
- task/note conflicts follow declared rules, never accidental last-write-wins;
- tenant/project scope applies to DB/search/cache/pubsub/object storage/workflows;
- Hub-supplied canonical artifacts are revalidated locally;
- single-user offline mode remains functional.

## 16. Worker/Compute implementation requirements

Worker gets staged exact inputs only.

Never:

- mount full vault;
- pass ambient host secrets;
- allow unrestricted network by default;
- trust output because process returned zero;
- admit late result after revocation without reevaluation.

All outputs are candidates until Core validates/adopts them.

## 17. Donor/source adoption procedure

Before copying/adapting code from Buzz, VoiceStudio or any source:

1. pin repository + exact commit/tag;
2. identify exact files/components/functions;
3. record user/private permission basis where relevant;
4. record public license/NOTICE and embedded third-party obligations;
5. record why reuse beats native implementation/stable dependency;
6. inspect security/runtime/transitive dependency implications;
7. adapt behind MedScale contract;
8. add MedScale behavior tests independent of donor tests;
9. record modifications and maintenance/update strategy.

Permission to copy does not authorize wholesale architecture adoption.

## 18. Testing procedure

For each task:

- add focused tests with the implementation, not afterward;
- run smallest focused command first;
- then crate/integration tests;
- then current repository baseline gates applicable to the change;
- run platform/external qualification only when required and record NOT_RUN/UNAVAILABLE honestly otherwise.

Never delete or weaken a failing test simply to make a gate green unless canonical behavior intentionally changed and the spec/evidence demonstrates the replacement.

## 19. Evidence procedure

The final evidence packet for a spec must map every acceptance criterion to:

```text
exact head SHA
command/test/evidence artifact
result state
platform/workload where relevant
known limitation/blocker
```

A reviewer should not need the agent chat to understand why the spec is considered complete.

## 20. Git/governance behavior

- no force push;
- no rebase of shared review branches;
- no bypass of required gates;
- do not merge your own planning/implementation PR merely because code compiles;
- preserve unrelated user changes;
- do not consume/rerun one-shot external attempts unless canonical governance permits it;
- use small, reviewable commits or a clearly structured bounded patch.

## 21. Stop conditions

Stop implementation and record the blocker when:

- active canonical authority does not permit the next step;
- a required source/model/license/permission cannot be proven;
- migration/recovery cannot be made unambiguous;
- external effect state is unknown and retry may duplicate harm;
- privacy policy does not authorize required data movement;
- a dependency contract from a predecessor spec is absent;
- real platform/external evidence is required but unavailable.

Do not replace these blockers with fabricated evidence or scope expansion.

## 22. End-of-session report

Every implementation session should report:

```text
BASE_SHA=
HEAD_SHA=
ACTIVE_SPEC=
AUTHORIZED_SCOPE=
CHANGED_PATHS=
FOCUSED_TESTS=
FULL_GATES=
PLATFORM_EXTERNAL_EVIDENCE=
DONOR_PROVENANCE_CHANGES=
KNOWN_BLOCKERS=
NEXT_MINIMUM_ACTION=
COMPLETION_CLAIM=
```

`COMPLETION_CLAIM` must be bounded to the evidence. Never state whole-project completion when only a spec/task is complete.
