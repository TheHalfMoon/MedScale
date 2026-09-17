# MedScale Research OS V2 Decision Register

**Status:** Planning decision register — not implementation authority  
**Applies to:** Candidate Specs 075–092 after Program Amendment 001  
**Does not change:** promoted Spec 074 scope

This register prevents implementation-by-preference for the V2 surfaces introduced by Data Source Fabric, R Workspace, and Community Extensions. Live canonical repository authority still wins at promotion time.

## Decision table

| ID | Question | Default decision | Owning candidate | Evidence that may change the default |
|---|---|---|---:|---|
| V2-Q01 | Data ingress model | One MedScale-owned `DataSourceManifest` / `DataSnapshot` / receipt model for local files, DBs, Kaggle, HF and later institutional sources. | 075 | Only a demonstrated semantic mismatch that cannot be represented without weakening provenance or authority. |
| V2-Q02 | Source credentials | Secrets stay in admitted secret store; manifests/receipts carry opaque `CredentialRef` only. | 075 | No weaker alternative permitted. |
| V2-Q03 | Foundation source effects | Read/discover/preview/import/refresh only. No remote writes, deletes, uploads, submissions or DDL/DML. | 075 | Later dedicated effect-authority spec with idempotency/unknown-state proof. |
| V2-Q04 | Canonical analysis input | Prefer immutable `DataSnapshot`; live mutable bindings must be explicitly marked partially reproducible. | 075/082 | Provider offers verifiable transaction/time-travel revision semantics equivalent to immutable snapshot binding. |
| V2-Q05 | Local tabular interchange | Arrow/Parquet first where appropriate; preserve raw bytes separately. | 075/082/086 | Benchmark/compatibility evidence shows another bounded format is materially better for a specific workload. |
| V2-Q06 | Local SQL source | SQLite source support first; DuckDB is evidence-selected, not assumed. | 075 | Workload benchmark shows material analytical/import value without conflicting SQLite/runtime constraints. |
| V2-Q07 | Remote relational DBs | PostgreSQL, MySQL/MariaDB and SQL Server read connectors before generic ODBC. | 075 | Institutional demand/driver portability evidence justifies ODBC earlier. |
| V2-Q08 | DB query safety | Typed schema discovery + parameter binding + read-only/default-deny parser/planner; no string-concatenated arbitrary SQL path. | 075 | No weaker alternative permitted. |
| V2-Q09 | Kaggle integration | Governed adapter around official provider semantics; Python client, if used, is isolated rather than trusted Desktop dependency. | 075 | Native/protocol implementation proves simpler and more maintainable while preserving exact provider semantics. |
| V2-Q10 | Hugging Face dataset integration | Exact dataset/revision/file acquisition; no provider remote code/trust-remote-code in trusted path. | 075 | No weaker alternative permitted. |
| V2-Q11 | Provider-supplied scripts | Never execute merely to acquire/import a dataset in trusted path. | 075 | Dedicated sandboxed transformation job explicitly promoted later. |
| V2-Q12 | Refresh behavior | Refresh creates a new snapshot/revision; it never silently mutates a previously receipted snapshot. | 075 | No weaker alternative permitted. |
| V2-Q13 | Schema drift | Detect via fingerprint; default to explicit `SchemaChanged` and require review/mapping before canonical downstream reuse. | 075 | Provider schema contract proves backward-compatible change and active spec defines deterministic auto-admission. |
| V2-Q14 | Data Source Desktop UX | First-class `Data Sources` workspace, not hidden inside Analytics. | 075 | Product authority explicitly changes navigation. |
| V2-Q15 | Data source extension API | Community connectors implement the same 075 adapter contract; no plugin-specific source model. | 087 | No weaker alternative permitted. |
| V2-Q16 | R placement | Never embed unrestricted R inside trusted Desktop/Core. Use staged external workspace or Compute job. | 086 | No weaker alternative unless a dedicated sandbox proof establishes equivalent isolation. |
| V2-Q17 | R IDE foundation | External RStudio Desktop and Positron launch over a staged workspace. | 086 | Platform availability may change which launcher is exposed, not the trust boundary. |
| V2-Q18 | R environment reproducibility | Capture exact R version + `renv.lock` digest/state + input digests + script digest + runtime facts. | 086 | Equivalent lock/environment mechanism may be added, but not less provenance. |
| V2-Q19 | R data interchange | Arrow/Parquet by default where semantically appropriate; exact snapshots remain authority. | 086 | Type/format fidelity benchmark requires a different explicit export format. |
| V2-Q20 | Automated R execution | `Rscript` runs through MedScale Compute with filesystem/network/resource/secret policy. | 085/086 | No direct Desktop/Core execution path permitted. |
| V2-Q21 | R output admission | Explicit Publish/Import only; no ambient watched-folder auto-import. | 086 | No weaker alternative permitted. |
| V2-Q22 | Posit Workbench | Institutional adapter, not Personal-mode dependency. | 090 | No change unless product topology changes explicitly. |
| V2-Q23 | Extension default authority | Empty capability set. Install does not inherit user/Project privileges. | 087 | No weaker alternative permitted. |
| V2-Q24 | Extension runtime | Prefer evidence-qualified WASM with narrow Host API for suitable workloads; otherwise isolated worker process. | 087 | Benchmark/security/compatibility evidence selects worker as default or another sandbox with equal boundaries. |
| V2-Q25 | Wasmtime | Qualification candidate, not automatic dependency. | 087 | Admit only after startup/memory/host-call/security/update evidence. |
| V2-Q26 | Extism | Reference/benchmark candidate; never authority plane. | 087 | Adopt only if it materially reduces safe host/runtime complexity versus MedScale-owned worker/WASM contracts. |
| V2-Q27 | Native in-process plugins | Denied by default. | 087 | Separate narrow exception spec with ABI, crash, memory-safety, signing, sandbox/containment and rollback proof. |
| V2-Q28 | Plugin UI | Declarative/typed contributions only by default; no arbitrary WebView/native code injection into trusted shell. | 087 | Dedicated UI-extension sandbox proof. |
| V2-Q29 | Registry topology | Community Registry is optional/self-hostable via Hub; offline/manual verified install remains supported. | 084/087 | No mandatory MedScale cloud permitted. |
| V2-Q30 | Registry trust | Listing is discovery, not safety approval. Verify digest/signature/provenance/API compatibility/capabilities locally. | 087 | No weaker alternative permitted. |
| V2-Q31 | Extension signatures | Signed/provenance-bound Extension Packs; unsigned dev mode is local/explicit and visibly non-production. | 087 | No weaker release path permitted. |
| V2-Q32 | Extension update | Capability increase requires visible re-consent; no silent capability widening/self-update. | 087 | No weaker alternative permitted. |
| V2-Q33 | Extension telemetry | No extension telemetry/network by default; network requires explicit destination-scoped capability. | 087 | User/admin explicit grant only. |
| V2-Q34 | Extension secrets | Secrets exposed only as opaque/use-scoped handles; never raw vault/keychain enumeration. | 087 | No weaker alternative permitted. |
| V2-Q35 | Extension persistence | Extension-private state is scoped and quota-bound; canonical artifacts remain Core-owned. | 087 | No alternate authority store. |
| V2-Q36 | Research Packs vs Extensions | Research Packs = domain/declarative semantics; Extensions = executable/integration capability. Installation of one never silently authorizes the other. | 087/089 | No weaker alternative permitted. |
| V2-Q37 | Community source quality | Registry may expose compatibility/security/provenance metadata but must not claim clinical correctness without domain evidence. | 087 | Evidence-backed certification program may add clearly scoped claims later. |
| V2-Q38 | Plugin revocation | Disable execution immediately; preserve canonical artifacts; record revocation reason and installed digest. | 087 | No silent deletion of user research outputs. |
| V2-Q39 | Air-gapped operation | Existing installed local capabilities continue without registry reachability where their own dependencies are local. | 087 | No mandatory online registry check for ordinary offline use. |
| V2-Q40 | Data source network authority | All remote source access uses existing MedScale network/capability boundaries; connectors never open arbitrary hidden clients. | 075/079/087 | No weaker alternative permitted. |
| V2-Q41 | External source licensing | Capture available license/terms metadata and require explicit user/admin acknowledgement where provider restrictions demand it; do not infer usage rights. | 075 | Legal/product policy may add stricter gates. |
| V2-Q42 | Imported file trust | Acquired bytes are hostile until size/type/parser/digest checks complete; archives get traversal/symlink/bomb defenses. | 075 | No weaker alternative permitted. |
| V2-Q43 | Snapshot storage | Do not duplicate bytes needlessly when content-addressed existing artifact storage can own them; preserve immutable digest identity. | 075 | Storage benchmark/format constraints may add chunking/dedup implementation, not weaken identity. |
| V2-Q44 | Live database exploration | Bounded read-only preview allowed; any result used as canonical analysis input must be snapshotted or explicitly marked mutable/partial. | 075/082 | No hidden live-query evidence claims. |
| V2-Q45 | Extension marketplace moderation | Technical admission gates first; community/review tiers are metadata, not authority. | 087 | A later governance program may add publisher trust tiers without bypassing local verification. |
| V2-Q46 | Extension SDK language | Define protocol/IDL/manifest first; language-specific SDKs are adapters generated/built over it. | 087 | Developer adoption evidence may prioritize Rust/TypeScript/Python SDK order, not change Host API authority. |
| V2-Q47 | Data connector SDK | Same extension Host API + 075 source contract; connector SDK cannot bypass source admission/receipts. | 087 | No weaker alternative permitted. |
| V2-Q48 | R package installation network | Restore/install network is explicit Compute/workspace capability, off by default in strict offline mode, and separately receipted. | 086 | Institutional mirror may be allowed by destination policy. |
| V2-Q49 | R secrets | `.Renviron`/environment secrets are not persisted into canonical artifacts; injected secrets are use-scoped and redacted from receipts/logs. | 086 | No weaker alternative permitted. |
| V2-Q50 | Whole-platform claim | Data source, R, and Extensions features are not release-ready until Spec 092 cross-plane qualification proves revocation, migration, recovery, privacy, security, accessibility and scale. | 092 | No shortcut permitted. |

## Implementer rule

When a promoted spec encounters one of these questions, apply the default. Do not reopen architecture merely because a library/framework is convenient. Change a default only when the owning promoted spec records the required evidence and the repository's canonical governance accepts the decision.

## Fail-safe rules

When evidence is insufficient:

- source adapter unavailable -> explicit `Unavailable`, not a hidden fallback;
- exact remote revision unavailable -> partial reproducibility, not a fabricated revision;
- R runtime/lock restore unavailable -> do not execute as reproducible;
- extension sandbox unavailable -> extension does not run;
- signature/provenance invalid -> quarantine/deny;
- capability undecidable -> deny;
- schema drift ambiguous -> `SchemaChanged`/review;
- external effect state unknown -> preserve `Unknown`, never retry blindly.
