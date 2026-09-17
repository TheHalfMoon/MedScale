# MedScale Research OS Planning Status V2

**Amendment branch:** `plan/research-os-data-extensions-amendment`  
**Base main:** `a80c33307afc4577790282652e5b20911beb4bbe`  
**Original Research OS planning PR:** `#121` — merged to main as `a80c33307afc4577790282652e5b20911beb4bbe`  
**Promoted implementation lane:** Spec 074 only, Draft PR `#122`  
**V2 amendment review lane:** Draft PR `#123`  
**V2 amendment status:** `REVIEW_READY / PLANNING_ONLY`; no 075+ implementation authority.

## Scope of Amendment 001

The founder added three major platform requirements after the original Research OS packet merged:

1. first-class **Data Source Fabric** for local data, databases, Kaggle, Hugging Face datasets and later institutional/object-store sources;
2. first-class **R Workspace** integration with RStudio/Posit and Compute-mediated R execution;
3. **Community Extensions** / Hub registry ecosystem with Obsidian-quality developer/community experience but stronger healthcare/research capability isolation.

These requirements materially change future dependencies and candidate numbering, but they do not expand already-promoted Spec 074.

## V2 candidate sequence

```text
074 Project + Artifact Graph Foundation              [separately promoted]
075 Data Source Fabric                               [candidate]
076 Collaboration Substrate                          [candidate]
077 MedAgent Workbench                               [candidate]
078 Model Fleet + Compare                            [candidate]
079 Privacy Gate                                     [candidate]
080 Governed Browse                                  [candidate]
081 AudioFlow Foundation                             [candidate]
082 Analytics Gate                                   [candidate]
083 Knowledge + Research Canvas                      [candidate]
084 MedScale Hub                                     [candidate]
085 MedScale Compute                                 [candidate]
086 R Workspace                                      [candidate]
087 Community Extensions                             [candidate]
088 AudioFlow Advanced                               [candidate]
089 Research Packs                                   [candidate]
090 Institutional Adapters                           [candidate]
091 Federation                                       [candidate]
092 Whole-Platform Qualification                     [candidate]
```

Every 075+ number remains candidate-only and must be reconciled against live canonical main before promotion.

## V2 amendment artifacts

### Founder/program amendment
- `RESEARCH_OS_PROGRAM_AMENDMENT_001_DATA_EXTENSIONS.md`

### Product plans
- `DATA_SOURCE_FABRIC_PLAN.md`
- `R_WORKSPACE_PRODUCT_PLAN.md`
- `COMMUNITY_EXTENSIONS_PRODUCT_PLAN.md`

### Integrated program contracts
- `RESEARCH_OS_EXECUTION_ROADMAP.md` — V2 074-092 dependency graph;
- `RESEARCH_OS_V2_DECISION_REGISTER.md` — 50 explicit defaults/evidence-selected decisions;
- `RESEARCH_OS_V2_IMPLEMENTATION_CONTRACT_ADDENDUM.md`;
- `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` — per-candidate implementation contracts 075-092;
- `RESEARCH_OS_V2_REPOSITORY_MAP_ADDENDUM.md`;
- `RESEARCH_OS_V2_VERIFICATION_ADDENDUM.md`;
- `RESEARCH_OS_V2_GAP_CLOSURE_REVIEW.md`;
- updated `SOURCE_ADOPTION_MATRIX.md`;
- updated `RESEARCH_OS_PLAN_INDEX.md`;
- updated `RESEARCH_OS_PACKET_VERSION.md`;
- updated `RESEARCH_OS_FINAL_PLANNING_ASSERTIONS.md`.

## Authority truth

The amendment does **not** authorize:

- candidate Spec 075 implementation;
- database network egress in production;
- Kaggle/Hugging Face credentials in production;
- R execution;
- Wasmtime/Extism or any extension runtime;
- Community Registry operation;
- third-party extension installation;
- real PHI;
- MESC mutation;
- automatic implementation of 075 after 074 closes.

Current planning truth:

```text
SPEC_074_IMPLEMENTATION_AUTHORIZED=true
SPEC_075_PLUS_IMPLEMENTATION_AUTHORIZED=false
RESEARCH_OS_V2_AMENDMENT_REVIEW_READY=true
RESEARCH_OS_V2_AMENDMENT_IMPLEMENTED=false
MATERIAL_KNOWN_V2_PLANNING_GAPS=0
REAL_PHI_AUTHORIZED=false
MESC_MUTATION_AUTHORIZED=false
```

`MATERIAL_KNOWN_V2_PLANNING_GAPS=0` means no presently known material founder requirement in Amendment 001 lacks a planned owner, authority boundary, failure model, repository placement or verification path. It does not claim implementation or technology qualification.

## Important supersession rule

The original Research OS packet remains valuable architecture context, but for **candidate 075+ semantic numbering/dependencies**, Program Amendment 001 + V2 Roadmap + V2 Decision Register + V2 addenda/contracts are the current planning truth.

Any future promotion packet must remove ambiguity by naming the semantic unit, not relying on an old integer alone.

Example:

```text
Candidate 075 = Data Source Fabric
not the former V1 alias "Collaboration Substrate"
```

## Completed amendment hardening

- [x] integrate Data Source/R/Extensions into roadmap;
- [x] add product plans for all three founder requirements;
- [x] add shared implementation contracts;
- [x] add per-spec implementation contracts 075-092;
- [x] add repository ownership map;
- [x] add verification campaigns;
- [x] add V2 Decision Register with fail-safe defaults;
- [x] update source-adoption policy;
- [x] update index/status/version/final assertions;
- [x] classify V1 future numbering as historical when conflicting;
- [x] record V2 cross-plane/stale-numbering gap closure;
- [x] preserve 074 scope and PR #122 separation;
- [x] keep amendment documentation-only;
- [x] open planning-only review PR #123;
- [ ] verify exact-head CI state for the final PR #123 head.

## Implementation interaction with Spec 074

Spec 074 remains bounded to Project + Artifact Graph Foundation.

The V2 amendment may be reviewed in parallel with Spec 074 implementation because it changes only future planning. However:

- do not merge amendment content into the Spec 074 implementation branch;
- do not add Data Sources/R/Extensions code to PR #122;
- Muse/implementation agents working Spec 074 should ignore V2 future implementation except where it confirms explicit non-goals;
- after Spec 074 canonical closure, the next candidate is Data Source Fabric only if separately promoted under then-live authority.

## Planning completion result

The amendment now satisfies its planning-completion condition:

- all three founder requirements map to bounded contracts and product surfaces;
- Data Source Fabric has one source/credential/snapshot model across local/DB/Kaggle/HF;
- R has explicit isolation/staging/reproducibility/publication rules;
- Extensions have signing/capability/sandbox/update/rollback/registry rules;
- repository ownership and verification campaigns are explicit;
- V2 decisions cannot be delegated to implementer preference without evidence;
- old future-number references are explicitly superseded for 075+ planning;
- V2 gap review reports no material known cross-plane gap;
- no implementation or release-readiness claim is made.
