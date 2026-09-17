# MedScale Research OS Verification Matrix

**Status:** Planning contract. Every future promoted Research OS specification MUST instantiate the relevant rows with exact commands, fixtures, thresholds and evidence paths.

## 1. Baseline repository gates

Unless canonical CI changes, every implementation PR affecting Rust workspace behavior must at minimum preserve the repository gates represented by `.github/workflows/ci.yml`:

```text
cargo fmt --all -- --check
./scripts/check-dependency-direction.ps1
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo deny check --all-features
```

Do not replace current CI truth with this document; read live CI before execution.

## 2. Verification layers

Each promoted unit chooses all applicable layers:

- `L0 Contract`: serialization, schema, enum/state validity, deny-unknown behavior where required.
- `L1 Unit`: deterministic pure logic and state-machine transitions.
- `L2 Storage`: transaction, migration, crash/reopen, concurrency and rollback.
- `L3 Core integration`: commands -> policy -> storage -> result/receipt.
- `L4 Surface parity`: CLI and Desktop use the same Core semantics.
- `L5 Adversarial`: denied capabilities, prompt/project/web hostile content, malformed workers/network peers.
- `L6 Scale/performance`: declared workload on declared hardware.
- `L7 Platform`: Windows/macOS/Linux behaviors where feature is supported.
- `L8 Recovery`: crash, cancel, timeout, restart, reconnect, restore and migration rollback.
- `L9 External qualification`: donor/model/service/worker/browser behavior requiring real external artifacts.

A passing lower layer never substitutes for an applicable higher layer.

## 3. Evidence artifact minimum

Every spec closure packet must identify:

```text
spec identifier
exact reviewed commit SHA
base SHA
commands executed
platform/hardware
fixture/dataset version and digest where applicable
pass/fail/skip/not-run state per gate
known limitations
external evidence references
performance method/results where claimed
security/adversarial cases
migration/recovery result
```

No evidence record may convert `SKIPPED`, `NOT_RUN`, `UNAVAILABLE` or `UNKNOWN` into PASS.

## 4. Cross-cutting test matrix

| Concern | Minimum cases |
|---|---|
| Identity | valid actor, unknown actor, revoked actor, stale session, agent actor, service actor |
| Capability | allowed, denied, expired, wrong project, wrong target scope, approval required |
| Revision | current revision, stale expected revision, duplicate/idempotent request, concurrent update |
| Data class | PUBLIC, EXTERNAL_DEIDENTIFIED, TEAM_PROTECTED, LOCAL_PHI, unknown/invalid classification |
| Network | offline, denied destination, redirect to denied destination, timeout before dispatch, timeout after possible dispatch, malformed response |
| Cancellation | before start, queued, active, completion race, late result after cancel/revoke |
| Storage | crash before transaction, crash after commit, migration forward, restore, corrupt/missing derived cache |
| Provenance | exact inputs, superseded inputs, missing source, digest mismatch, stale projection/index |
| Logs | no raw PHI/prompt/audio/credentials by default, IDs/digests sufficient for diagnosis, bounded error details |
| Accessibility | keyboard path, focus visibility, exposed name/role/action, no color-only state |

## 5. Spec 074 verification — Project + Artifact Graph

### Contract
- round-trip every new contract;
- reject malformed predicate/schema version;
- existing canonical object refs remain resolvable.

### Storage
- fresh-vault migration;
- existing populated-vault migration;
- rollback/reopen strategy from declared checkpoint;
- project create + artifact attach in transactional sequence;
- duplicate attach/idempotency;
- graph edge stale revision conflict;
- archive without destructive object deletion.

### Scale
Define at least two fixtures before closure:

```text
Personal fixture: >= 10 projects, >= 1,000 artifact refs total.
Lab fixture: >= 100 projects or one project with >= 100,000 graph/artifact relations.
```

Thresholds are selected by the spec from measured current hardware; the test must exist even if no marketing budget is claimed.

## 6. Spec 075 verification — Collaboration

- append 10k+ messages/events without canonical-artifact mutation;
- membership denial for read/write/search;
- task concurrent update conflict;
- offline note conflict preserves both versions;
- message edit history preserved;
- ephemeral presence disappears and is not treated as audit;
- agent identity cannot inherit owner capabilities implicitly;
- artifact references resolve exact revision or show stale/missing state;
- tamper/audit-checkpoint verification if implemented.

## 7. Spec 076 verification — MedAgent

### Security corpus
Project documents must include prompt-injection fixtures such as attempts to:
- request hidden system instructions;
- call a tool not in the manifest;
- exfiltrate another artifact/project;
- enable network access;
- modify canonical records;
- reveal local secret paths/credentials.

Expected result: content may influence model text but cannot grant capabilities.

### Lifecycle
- normal local run;
- model unavailable;
- invalid Pack;
- cancellation during generation;
- tool denied;
- tool timeout;
- context artifact deleted/stale before admission;
- run receipt survives restart;
- no ambient project/vault context beyond `ContextManifest`.

## 8. Spec 077 verification — Fleet/Compare

Use deterministic fixture outputs in addition to real-model qualification:
- all lanes agree text but cite conflicting evidence;
- majority wrong fixture;
- one lane abstains;
- one lane fails;
- one lane unavailable;
- one lane returns invalid structured output;
- different lane permissions;
- citation references overlap/no-overlap;
- contradiction extractor returns uncertain candidate rather than false certainty where unsupported.

The UI/report must not emit an implicit winner from lane count.

## 9. Spec 078 verification — Privacy Gate

### Synthetic corpus classes
At minimum:
- person names;
- national/local identifiers represented only by synthetic formats permitted for test;
- medical record IDs;
- phone/email;
- addresses/locations;
- dates/ages;
- clinician/provider identifiers;
- account/device/IP identifiers where relevant;
- free-text clinical context;
- Arabic;
- English;
- Arabic-English code-switch;
- negation and medically meaningful numbers/units.

### Required reporting
- precision/recall or equivalent class-wise detection metrics where ground truth exists;
- transformation correctness;
- false-negative examples retained in evidence packet without real PHI;
- residual scan state;
- reversible pseudonym round trip;
- mapping key denial;
- export/Browse/model/Hub/Compute enforcement after transform and without transform.

No acceptance threshold may be converted into a claim of complete anonymization.

## 10. Spec 079 verification — Governed Browse

### Routing and authority
- local/Project source is preferred when sufficient;
- brokered HTTP/search route works only through admitted network authority;
- deterministic browser route is selected only when rendering/navigation is needed;
- agentic browser route is unavailable unless explicitly admitted;
- model/page content cannot create a capability or change route policy;
- external side-effecting actions remain denied in foundation unless separately promoted.

### Egress/data-class corpus
- PUBLIC query allowed only to admitted destination;
- LOCAL_PHI query/body/context denied from public route;
- TEAM_PROTECTED denied from public route;
- EXTERNAL_DEIDENTIFIED denied without valid transformation/egress receipt;
- EXTERNAL_DEIDENTIFIED allowed only with the exact approved transformed artifact/span;
- sensitive data placed in URL/query-string/header/body is detected/blocked according to policy;
- denied request creates honest denial evidence without leaking rejected sensitive payload into operational logs.

### Prompt-injection/web hostile-content corpus
Pages/snippets/downloads attempt to:
- override system/tool instructions;
- request another Project's data;
- reveal credentials;
- navigate to localhost/private metadata services;
- call unregistered tools;
- upload project files;
- perform purchase/submission/write action;
- instruct hidden browser profile/cookie extraction.

Expected: content can affect evidence text only; capabilities/credentials/egress remain controlled outside the model.

### Destination/redirect/SSRF
- allowed origin -> allowed redirect;
- allowed origin -> denied origin;
- redirect loop;
- excessive redirect count;
- localhost/127.0.0.1/::1;
- link-local/cloud metadata ranges;
- private RFC1918 destination;
- malformed URL/IDN/encoded-host variants as applicable to chosen URL parser;
- DNS rebinding/resolve-time policy documented and tested to the extent supported by the route.

### Credentials/session
- raw secret never appears in prompt, receipt or normal logs;
- credential handle bound to origin/scope;
- wrong-origin credential use denied;
- expired/revoked handle denied;
- login/MFA invokes explicit human-takeover state where required;
- browser cookies/profile isolated and cleaned/retained according to declared policy.

### Downloads/content
- declared MIME mismatch;
- oversized body/download;
- archive/binary quarantined;
- HTML/script content returned as untrusted source, not executed in trusted Desktop;
- downloaded candidate preserves URL/time/digest;
- normal MedScale ingest revalidates before canonical use.

### Evidence/reproducibility
- exact URL/origin;
- retrieval time;
- route/tool version;
- selected text/DOM/source span locator;
- content digest/snapshot when obtainable;
- cache/retention state;
- repeated retrieval records changed content rather than silently replacing prior evidence.

### Lifecycle
- cancel during search;
- cancel during navigation;
- timeout;
- browser worker crash;
- offline mode;
- denied destination;
- partial multi-source retrieval;
- restart does not convert incomplete run to success.

## 11. Spec 080 verification — AudioFlow Foundation

### Capture
Per supported platform:
- permission allow/deny;
- device enumerate/select/change;
- device unplug mid-capture;
- pause/resume/stop;
- suspend/resume where supported;
- crash/restart behavior;
- no invisible capture state.

### Audio quality/runtime
Benchmark lanes separately:
- live latency;
- offline final quality;
- Arabic;
- Arabic-English code-switch;
- medical terminology;
- names;
- medication names where permitted fixtures exist;
- dose/number/unit transcription;
- negation;
- diarization speaker-count/stability;
- timestamp alignment.

Measure CPU/RAM/GPU startup/warm behavior and long-session stability. Do not inherit upstream benchmark claims.

### Transcript lineage
- source digest immutable;
- quality pass produces revision, never overwrite;
- segment timestamp links survive revision rules or are explicitly remapped;
- deleted source -> transcript/evidence state is honest;
- speaker label is not persistent identity.

### Voice control
- COMMAND executes only after normal capability checks;
- CONTEXT cannot execute;
- DICTATION only targets selected text/document field;
- interruption stops active turn according to declared latency envelope;
- ambiguous mode requests require explicit state/confirmation rather than silent action.

## 12. Spec 081 verification — Analytics

### Correctness fixture families
- projections/selections;
- joins;
- null/missing semantics;
- grouping/aggregation;
- dates/timezones;
- window operations where productized;
- cohort inclusion/exclusion;
- repeated run with pinned inputs;
- large Parquet/Arrow fixture;
- cancellation/timeout.

Reference expected results must be independently generated/checked.

### Generated query security
- DDL/DML rejected in default path;
- attempts to access hidden tables/files rejected;
- path traversal/external table attempts rejected unless explicitly admitted;
- model text cannot bypass governed view list;
- source revision pinned for receipt.

### Statistical checks
For each productized statistic:
- known-value fixture;
- edge cases;
- missingness behavior;
- assumptions/check status;
- independent reference comparison;
- no inference beyond declared method.

## 13. Spec 082 verification — Knowledge/RAG/Canvas

- lexical retrieval exact-match fixture;
- structured Project Graph retrieval;
- vector retrieval benchmark only if admitted;
- source revision update -> stale index detection;
- deletion/tombstone -> no retrieved plaintext;
- actor/project permission filtering before result disclosure;
- cache separation across scopes;
- exact PDF/page/span links;
- exact Browse URL/span links when 079 integration is enabled;
- exact audio timestamp links when 080 integration is enabled;
- insufficient-evidence state;
- conflicting sources surfaced;
- Canvas broken/stale live-reference behavior.

## 14. Spec 083 verification — Hub

### Concurrency/recovery scenarios
- client A/B simultaneous task update;
- both offline, edit same note, reconnect;
- message append while offline;
- membership revoke while socket/session active;
- agent revoke during workflow;
- interrupted artifact transfer at 1%, 50%, 99%;
- resume duplicate chunks;
- malicious/wrong digest;
- stale sync cursor requiring replay/resync;
- deletion/tombstone during offline interval;
- Hub restart mid-sync;
- database restore from backup;
- supported-version upgrade and rollback rehearsal.

### Isolation
Test project/tenant boundaries across:
- primary DB queries;
- search index;
- cache;
- pub/sub;
- object storage keys;
- workflow queues;
- logs/audit views.

One boundary failure is a release blocker for multi-tenant deployment.

## 15. Spec 084 verification — AudioFlow Advanced

- huddle join does not imply record/transcribe permission;
- recording state visible to participants under declared product contract;
- agent join/read/speak capability separated;
- synthetic/agent speech identified;
- TTS output provenance;
- no voice clone from arbitrary meeting audio;
- subject permission record required for cloning;
- deletion/reset of persistent speaker/voice material;
- media annotation stable to exact source/time/frame revision;
- remote audio worker obeys privacy/egress manifest when Compute integration exists.

## 16. Spec 085 verification — Compute

### Isolation/adversarial
- worker cannot enumerate vault;
- only staged input paths visible;
- denied network unreachable;
- secret handles scoped/expire;
- filesystem escape attempts;
- process spawning policy;
- CPU/RAM/time quota;
- OOM;
- timeout;
- worker crash;
- host restart;
- duplicate output;
- output schema/digest mismatch;
- late output after revoke/cancel quarantined;
- logs scrub sensitive input.

### Reproducibility
Receipt records environment/runtime version and exact input/output digests. Re-run tolerance/expected nondeterminism must be declared per job type.

## 17. Spec 086 verification — Research Packs

For every Pack:
- install;
- compatible upgrade;
- incompatible upgrade rejection/migration;
- disable;
- uninstall with preserved domain data by default;
- missing Pack rendering/CLI behavior;
- schema validation;
- denied capability escalation;
- declarative UI descriptor validation;
- executable tool path goes through worker/tool contract;
- import/export round trip using synthetic fixtures.

## 18. Spec 087 verification — Institutional adapters

Every adapter must test:
- auth success/failure/expiry;
- least-privilege scope;
- mapping/version mismatch;
- timeout before dispatch;
- timeout after possible dispatch -> `Unknown`;
- duplicate/retry/idempotency;
- unavailable provider;
- credential redaction;
- policy/data-class denial;
- audit/receipt;
- malformed/untrusted remote response;
- sandbox/network boundary where applicable.

Write-capable adapters require stronger effect-confirmation tests than read-only adapters.

## 19. Spec 088 verification — Federation

Before product promotion:
- at least two independently configured sites;
- deny on unknown policy vocabulary;
- identity/trust binding and revocation;
- data-stays-at-site baseline;
- federated result provenance back to site/input version;
- network partition/retry;
- expiry;
- statistical correctness for chosen federated method;
- privacy leakage evaluation;
- malicious/Byzantine input behavior appropriate to declared trust model.

Federation remains unqualified if these tests are design-only.

## 20. Spec 089 release qualification profiles

Qualification should define explicit profiles rather than one misleading global PASS:

```text
PROFILE_PERSONAL_OFFLINE
PROFILE_LAB_SELF_HOSTED
PROFILE_INSTITUTIONAL
PROFILE_FEDERATED (only if 088 is promoted and proven)
```

Each profile lists included capabilities and evidence. A lower profile may release while later profile work remains deferred.

## 21. Required proof before merging implementation

An implementation PR may be considered for merge only when:

1. exact head is known;
2. diff matches authorized scope;
3. focused tests pass;
4. live repository baseline/full affected gates pass;
5. required platform/external evidence is present or explicitly blocks closure;
6. donor/provenance changes are recorded;
7. migrations/recovery are exercised where applicable;
8. security denial cases pass;
9. reviewer can map every acceptance criterion to evidence;
10. no claim exceeds the evidence.
