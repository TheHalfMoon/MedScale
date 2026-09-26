# MedScale Research OS V2 — Orca Donor Addendum

**Status:** Planning addendum — not implementation authority  
**Applies to:** candidate Specs 077 MedAgent Workbench, 078 Model Fleet + Compare, and 080 Governed Browse  
**Does not change:** promoted Spec 074 scope, authority, contracts, or implementation sequence  
**MedScale planning base:** `ee8daef3a2782bbdbcb3324766a5d6b95c09fa09`  
**Upstream donor:** `stablyai/orca`  
**Planning pin:** `de15227a1d321840ea35c6bb2d0cc01e3409e5f1` (2026-09-17 live upstream `main` at planning review time)  
**Public license observed:** MIT  
**Founder permission record:** on 2026-09-17 the founder stated that MedScale has permission to use the whole Orca source code.

This addendum makes Orca a first-class qualified donor candidate for the Research OS V2 agent/workbench program. Permission to use the full codebase expands the available implementation options; it does **not** transfer Orca architecture, trust assumptions, network behavior, identity, persistence, telemetry, cloud/mobile, shell, browser, or git semantics into MedScale automatically.

The canonical rule remains:

> **MedScale owns authority, privacy, provenance, project identity, artifact identity, network policy, tool admission, model admission and receipts. Orca may donate implementation patterns and bounded components behind those contracts.**

---

## 1. Why Orca is relevant

Orca is an AI orchestrator/editor that already demonstrates several interaction patterns directly aligned with the planned Research OS:

- multiple CLI agents running side-by-side;
- isolated git worktrees for parallel agent tasks;
- terminal split/pane management;
- persisted terminal/session UX;
- editor/file/markdown/browser tab surfaces;
- embedded browser control and design-mode-style element inspection;
- AI-generated diff review and inline annotation;
- drag/drop files and images into an agent context;
- CLI automation of worktrees, terminals, browser actions and editor state;
- GitHub/project-task integration;
- remote/SSH worktree support;
- mobile monitoring/steering patterns;
- agent completion/unread/notification state.

These are useful product and implementation references for MedAgent, Model Fleet + Compare and Governed Browse. They are **not** proof that the same trust model is suitable for medical/research data.

---

## 2. Adoption posture

### 2.1 Canonical posture

```text
DONOR = stablyai/orca
POSTURE = COPY_SELECTIVE / ADAPT / REFERENCE
WHOLESALE_ARCHITECTURE_ADOPTION = NO
FULL_SOURCE_PERMISSION_AVAILABLE = YES
FLOATING_UPSTREAM_DEPENDENCY = NO
MEDSCALE_AUTHORITY_TRANSFER = NO
SPEC_074_CHANGE = NO
```

Direct code transfer is permitted only after the normal MedScale donor/source procedure is completed for the exact files/components being transferred:

1. pin exact upstream repository and commit;
2. identify exact files/components/functions;
3. record the founder permission basis;
4. record MIT license plus embedded/transitive third-party obligations;
5. justify why reuse is better than native implementation or a stable dependency;
6. inspect security, IPC, shell, browser, child-process, filesystem, network, telemetry, persistence and transitive-dependency implications;
7. adapt behind MedScale-owned contracts;
8. add MedScale behavior/security tests independent of Orca tests;
9. record modifications, provenance and update/maintenance strategy.

No implementation spec may write “copy Orca” as a task. It must name the bounded component and the MedScale contract it will satisfy.

---

## 3. Candidate Spec 077 — MedAgent Workbench

Orca is a **primary donor candidate for workbench interaction and orchestration mechanics**, not for intelligence authority.

### 3.1 High-value donor areas

Study, adapt or selectively transfer:

- agent adapter/process lifecycle patterns;
- multi-pane/split workspace composition;
- terminal session creation, restore, focus and status state;
- editor/file/markdown/browser tab coordination;
- agent run status, unread state and completion notifications;
- prompt-to-agent interaction shell;
- file/image attachment UX;
- session/workspace snapshot and restore patterns;
- diff review and annotation UX for agent-produced code/text artifacts;
- CLI-driven control of the workbench;
- cancellation/interruption/follow-up interaction patterns;
- crash/restart and durable UI-state techniques where compatible.

### 3.2 MedScale-owned replacements/boundaries

Orca concepts must map into MedScale contracts rather than become new authorities:

```text
Orca agent/session concept
    -> AgentIdentity / AgentProfile / AgentRun / AgentTurn

Orca terminal/process concept
    -> typed AgentRuntimeAdapter + capability-bounded executor

Orca worktree/workspace concept
    -> Project + ContextManifest + LaneWorkspace

Orca file attachment concept
    -> exact ProjectArtifactRef / DataSnapshot / selected staged file reference

Orca browser tab concept
    -> BrowseSession / BrowseRoute only after Spec 080

Orca run completion/status
    -> AgentRunState + RunReceipt

Orca tool/action bridge
    -> ToolManifest + ToolInvocation + ToolReceipt
```

### 3.3 Required Workbench rules

- No agent gets ambient vault access.
- No shell process receives the vault master key.
- A terminal pane is an execution surface, not an authority boundary.
- Local CLI agents are not trusted merely because they execute locally.
- Every agent/runtime declares filesystem, network, process, model and data-class capabilities.
- Project context is an explicit `ContextManifest`, never “current working directory = authorized context.”
- Agent output remains proposal/evidence unless a separate Core command admits it.
- External providers/browser use remain denied until owning specs authorize them.
- The native MedScale UI may reuse Orca interaction patterns without adopting Electron as an authority layer.

### 3.4 Suggested implementation slices for 077

```text
077-A Agent adapter contract and local process lifecycle
077-B Workbench pane/session model
077-C Project ContextManifest binding
077-D Terminal/editor/artifact surfaces
077-E Run history, unread/completion state and receipts
077-F Diff/review/annotation surface for produced artifacts
077-G CLI parity and deterministic session restoration
077-H Security/recovery/performance qualification
```

The exact promoted spec may renumber slices, but these concerns must be owned explicitly.

---

## 4. Candidate Spec 078 — Model Fleet + Compare

Orca's parallel worktree orchestration is a strong implementation/product reference for **lane isolation and side-by-side execution**.

### 4.1 Reusable patterns

Study/adapt:

- fan-out of one task/prompt to multiple independent agent sessions;
- per-lane workspace/session identity;
- independent process lifecycle and cancellation;
- side-by-side result surfaces;
- status/progress aggregation without hiding partial failure;
- per-lane terminal/editor state;
- repeatable workspace creation/cleanup;
- review of differences between independent outputs.

### 4.2 Critical semantic difference

A git worktree is useful for code-agent isolation but is **not** MedScale's canonical research isolation or security boundary.

MedScale must define:

```text
LaneWorkspace {
    lane_id,
    project_id,
    context_manifest_ref,
    staged_inputs,
    writable_output_root,
    filesystem_policy,
    network_policy,
    tool_policy,
    model_runtime,
    data_class,
    lifecycle,
}
```

For code-oriented tasks, a git worktree may be one `LaneWorkspace` backend. For clinical/research/data tasks, the backend may instead be a staged filesystem/worker sandbox with no git repository at all.

### 4.3 Compare rules

Orca-style side-by-side UX may be reused, but MedScale comparison semantics remain those of Spec 078:

- no automatic winner;
- no majority = truth rule;
- no permission union across lanes;
- no hidden context difference;
- partial lane failure remains visible;
- compare exact model/runtime/context/tool/data/network facts;
- surface evidence/citation overlap, contradiction candidates, unsupported claims, abstention and schema validity;
- unknown remains unknown;
- any manual “choose/use this result” action is explicit and receipted.

### 4.4 Suggested 078 implementation slices

```text
078-A LaneWorkspace abstraction and sandbox backend
078-B Parallel AgentLane scheduler
078-C Fan-out / cancel / retry state machine
078-D Side-by-side lane result workspace
078-E Structured ComparisonReport
078-F Resource/runtime/evidence comparison
078-G Partial-failure and recovery qualification
078-H Code-worktree optional backend qualification
```

---

## 5. Candidate Spec 080 — Governed Browse

Orca contains valuable embedded-browser, browser-tab, CLI automation and design-mode interaction patterns. These are donor candidates for **browser surface mechanics only**.

### 5.1 Reusable patterns

Study/adapt selectively:

- browser tab/session lifecycle;
- deterministic element addressing;
- snapshot/screenshot interaction patterns;
- click/fill/hover/navigation command surfaces;
- browser-to-agent context handoff;
- element inspection and DOM/CSS capture patterns;
- browser state visible beside agent/editor surfaces;
- timeout/cancellation and session cleanup patterns.

### 5.2 MedScale security replacement

The Orca browser path must not become a bypass around MedScale's network/privacy authority.

Every MedScale browser action must satisfy:

```text
Agent/CLI/Desktop
  -> BrowseRequest
  -> Privacy Gate / EgressDecision
  -> Network Broker policy
  -> BrowseRoute
  -> isolated browser worker/session
  -> bounded result/download candidate
  -> BrowseEvidenceItem + BrowseReceipt
```

Required constraints:

- no ambient browser profile import;
- no direct model access to cookies/tokens/secrets;
- credential handles stay outside prompts;
- no raw sensitive Project context egress without explicit policy;
- prompt injection/web instructions never expand capabilities;
- SSRF/private-network rules remain enforced;
- downloads are quarantined candidates and return through Data Source Fabric admission;
- side-effecting browser actions remain outside 080 foundation unless a later explicit contract admits them;
- browser UI code does not own canonical evidence or project state.

Orca's Design Mode may inform future research/web artifact capture, but MedScale must generate its own evidence/provenance receipts and must not treat DOM content as trusted instruction.

---

## 6. Secondary references outside 077/078/080

Orca may inform, but does not directly expand, these candidate areas:

### 076 Collaboration Substrate

Reference only:

- task-linked agent sessions;
- unread/notification state;
- project-board/task launch UX;
- review comments anchored to produced artifacts.

Canonical room/task/note/activity semantics remain MedScale-owned.

### 087 Community Extensions

Reference only:

- agent integration/adapters;
- command/CLI extension ergonomics;
- skill-sharing UX patterns.

Orca plugin/agent extensibility must not weaken MedScale Extension Pack capability isolation.

### 085 Compute / 090 Institutional Adapters

Remote SSH/worktree ideas may be reconsidered only through the Compute/institutional execution contracts. They are not admitted into local MedAgent merely because Orca supports SSH.

---

## 7. Explicitly non-adopted assumptions

The following Orca concepts are **not** inherited by this planning amendment:

- Electron as a trusted authority layer;
- git worktree = security isolation;
- arbitrary shell = permitted research tool;
- arbitrary CLI agent = admitted model/runtime;
- unrestricted filesystem inheritance from the app process;
- unrestricted network inheritance;
- direct browser networking outside Network Broker;
- cloud/mobile companion infrastructure as a product dependency;
- SSH remote execution as a 077 requirement;
- GitHub/Linear/Jira/GitLab identity as MedScale participant identity;
- external issue/PR/task systems as canonical Project state;
- automatic merge/apply of an agent result;
- telemetry or remote usage tracking by default;
- account switching semantics that expose provider credentials to MedScale agents;
- Orca persistence databases as MedScale canonical storage.

---

## 8. Donor qualification packet required before code transfer

Before the first Orca-derived code enters a promoted implementation branch, create an evidence packet such as:

```text
evidence/<owning-spec>/donors/orca/
  SOURCE.md
  PINNED_REVISION.md
  FILE_TRANSFER_MANIFEST.md
  LICENSE_AND_NOTICE.md
  THIRD_PARTY_DEPENDENCIES.md
  SECURITY_REVIEW.md
  ADAPTATION_MAP.md
  MEDSCALE_BEHAVIOR_TEST_MAP.md
  UPDATE_AND_EXIT_STRATEGY.md
```

Minimum `SOURCE.md` facts:

```text
UPSTREAM=https://github.com/stablyai/orca
PLANNING_PIN=de15227a1d321840ea35c6bb2d0cc01e3409e5f1
IMPLEMENTATION_PIN=<exact reviewed commit; never floating main>
PUBLIC_LICENSE=MIT
FOUNDER_PERMISSION=full-source reuse permission stated 2026-09-17
OWNING_SPEC=<077|078|080|later>
EXACT_TRANSFER_PATHS=<list>
MEDSCALE_CONTRACTS_SATISFIED=<list>
```

The implementation pin may differ from the planning pin if Orca advances. The implementer must re-review the exact selected revision rather than assuming current `main` remains equivalent.

---

## 9. Security review checklist for Orca-derived components

For every transferred/adapted component, explicitly inspect:

- child-process launch arguments and environment inheritance;
- shell interpolation/injection;
- filesystem root/path traversal/symlink handling;
- git/worktree path assumptions;
- IPC message validation and privilege boundaries;
- browser process/profile/cookie/session ownership;
- network calls, proxies, local/private-network access and redirects;
- telemetry/analytics/crash-reporting endpoints;
- persisted terminal/session scrollback and sensitive-data retention;
- clipboard and drag/drop data paths;
- file/image preview parsers;
- remote/SSH helpers;
- credentials/provider account handling;
- automatic updates/downloads;
- executable discovery and PATH resolution;
- environment-variable leakage;
- logs containing prompts, file contents, tokens, paths or sensitive metadata;
- mobile/cloud synchronization code that must be excluded from local-only scope;
- native dependencies and platform-specific binaries;
- dependency licenses, NOTICE files and generated assets.

A component may be technically reusable but still be rejected if removing unsafe assumptions costs more than a MedScale-native implementation.

---

## 10. Testing requirements for adapted components

Orca tests are useful donor evidence but never sufficient for MedScale acceptance.

MedScale must independently test at least:

### Workbench
- agent process spawn/cancel/crash/restart;
- exact context binding;
- denied filesystem/network/tool access;
- no ambient vault/key inheritance;
- deterministic pane/session restoration;
- bounded/scrubbed persisted terminal state;
- file/drop context cannot bypass Project authorization.

### Fleet
- two or more independent lanes;
- no permission union;
- lane-specific staged inputs;
- partial failure/cancel/retry;
- output provenance remains lane-specific;
- workspace cleanup after crash;
- no cross-lane mutable-state leakage.

### Browse
- Network Broker is mandatory;
- privacy denial prevents navigation/request;
- SSRF/private-network denial;
- prompt-injection content cannot grant tools/capabilities;
- credentials absent from agent-visible context;
- browser restart/session cleanup;
- malicious download remains quarantined;
- evidence receipt binds URL/time/content hash/tool/session facts.

---

## 11. Product design guidance

MedScale should take the best part of Orca's product idea — **one coherent workspace where agents, terminals, files, browser views and review surfaces remain visible together** — and apply it to research/healthcare workflows rather than making a generic code editor.

The planned MedAgent experience should therefore converge toward:

```text
Project sidebar
  + Agent/Fleet lanes
  + central conversation/editor surface
  + optional terminal/runtime pane
  + evidence/output pane
  + browser pane when governed browse is admitted
  + artifact/context inspector
  + run/model/privacy/network posture
  + comparison/review surface
```

When a run emits structured outputs, charts, tables, images, documents, diffs or evidence, the workspace may split automatically to show the result beside the conversation while preserving the exact originating `RunReceipt` and Project artifact references.

This interaction model is consistent with the Research OS north star:

> **One workspace. Many engines. One authority. User-owned data. Local by default. Evidence everywhere. Extensible by capability, never by ambient trust.**

---

## 12. Planning conclusion

```text
ORCA_RESEARCHED_AT_LIVE_UPSTREAM = TRUE
ORCA_PLANNING_PIN = de15227a1d321840ea35c6bb2d0cc01e3409e5f1
ORCA_PUBLIC_LICENSE_OBSERVED = MIT
ORCA_FULL_SOURCE_PERMISSION_RECORDED = TRUE
ORCA_PRIMARY_CANDIDATES = 077,078,080
ORCA_SECONDARY_REFERENCE = 076,085,087,090
ORCA_WHOLESALE_ARCHITECTURE_ADOPTION = FALSE
ORCA_DIRECT_AUTHORITY = FALSE
ORCA_CODE_COPIED_BY_THIS_PLANNING_CHANGE = FALSE
SPEC_074_SCOPE_CHANGED = FALSE
IMPLEMENTATION_AUTHORITY_GRANTED = FALSE
```

Orca is now an explicit Research OS V2 donor candidate. A future promoted owning spec may selectively transfer any useful Orca component because full-source permission is available, but only after exact-path qualification and adaptation behind MedScale contracts.