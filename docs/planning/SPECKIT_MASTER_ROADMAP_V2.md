# MedScale Spec Kit Master Roadmap V2

**Date:** 2026-08-25  
**Status:** `CANONICAL_PLANNING_V2`  
**Development process:** GitHub Spec Kit  
**Runtime/trusted-core language:** Rust  
**Implementation authorization:** NO

## 1. Spec-of-Specs rule

MedScale stays one platform, but each slice must be independently specifiable and testable. The roadmap is shallow; detailed behavior lives in each Spec Kit package. Spec preparation/research may be parallelized when it does not mutate the repository or cross an implementation gate.

Required lifecycle for each material executable spec:

```text
/speckit.constitution (repository-level / when constitution changes)
/speckit.specify
/speckit.clarify
/speckit.plan
/speckit.checklist
/speckit.tasks
/speckit.analyze
-> qualification
-> /speckit.implement  # only with explicit founder implementation authorization
-> /speckit.converge
```

## 2. Canonical V2 roadmap

| ID | Spec | Intent | Entry gate | V2 additions / exit gate |
|---|---|---|---|---|
| 000 | `medscale-constitution-source-authority` | founder decisions, frozen architecture provenance, source authority | external planning package | captures V2 source/placement/privacy rules; no product code |
| 001 | `rust-repository-speckit-bootstrap` | initialize Rust workspace, Spec Kit, governance, CI/evidence/supply-chain skeleton | explicit founder bootstrap/implementation authorization | cargo-deny/RustSec/OSV; cargo-vet/auditable/fuzz policy; dependency-direction lint; no medical functionality |
| 002 | `trusted-object-source-authority-foundation` | object classes, identity, realm/scope, proposal/promotion, effects/retry, time, text spans, process topology, IPC contracts | 001 | **single-writer Core Host decision**, tagged text coordinate systems, native/FFI hardening contract, worker supervision policy; property/serialization tests pass |
| 003 | `h0a-trusted-ingest-durability` | synthetic FHIR R4 trusted custody and canonical visibility | 002 | lexical/duplicate-key/decimal/version/validator/identity tests; blob read integrity + GC race/restore invariants; **synthetic-scope backup/restore interface proof**; no real PHI/model/network |
| 004 | `h0b-trusted-presentation-coverage` | deterministic timeline + narrow LLM-free Brief + coverage/drill-down | 003 | bounded typed resource extractors; **no general FHIRPath in H0-B**; UCUM/unit semantics where required; negative/absence/conflict/golden rebuild tests |
| 005 | `local-private-vault-encryption-recovery` | pre-PHI production encryption/key/recovery/backup/migration/log/privacy | 004 | vault defaults to local app-data, sync/remote filesystem detection, single-writer/key lifecycle, snapshot/retention/key-loss/restore proof; REAL_PHI remains separately unauthorized |
| 006 | `cli-desktop-foundation` | first useful product surface using same Rust core | 005 + product-wedge evidence | CLI authority contract; Desktop shell prototype; WebView PHI containment or reject WebView; `medscale doctor`; `PRIVACY_PROOF`; no direct UI/CLI DB bypass |
| 007 | `openmed-capability-absorption-parity` | pin OpenMed v2.2 baseline; define parity corpora; donor absorption; terminology/licensing; Saudi/Arabic benchmark program | 004 for research/spec work; capability admission depends on 005/006 as needed | parity matrix is complete/phase-scoped; v2.2 exact baseline; donor deltas all dispositioned; terminology rights track started; **no runtime required by this spec itself** |
| 008 | `local-ai-capability-fabric` | model/NER/PII/de-ID runtime fabric, signed Pack v0, offline import, worker confinement | 005 + 006 + qualified 007 benchmark/admission contracts | `008A` offline pack admission (`medscale packs install <local-path>`), `008S` per-OS worker confinement, workload-specific runtime qualification; no online download path |
| 009 | `mobile-ios-android` | native mobile surfaces over shared Rust semantics | 005 + 006 + mobile FFI qualification; AI pack features additionally require 008 | native Keychain/Keystore; no silent key sync; 16KB Android compatibility; Files/SAF sideload; pack chunk/arch requirements fed back to 008/015; privacy/platform tests |
| 010 | `documents-ocr-voice` | hostile documents, OCR, ASR/diarization, source spans | 005 + 008 | P1 worker isolation, MIME/quarantine, document breadth parity, Arabic/code-switch benchmarks, critical-number source alignment |
| 011 | `evidence-retrieval-medical-intelligence` | structured/lexical/semantic retrieval, terminology grounding, evidence sets, conflict/freshness, research privacy controls | 004 + 008 | RELEVANCE != AUTHORITY; terminology/SDC parity where scoped; reproducible retrieval/evidence benchmark |
| 012 | `mesc-artifact-integration` | admit immutable MESC artifacts; exceptional service only if justified | 008 + explicit MESC artifact release | exact hashes/rights/SBOM/evaluation; no direct Python import/shared DB/key; service path remains separately sandbox-qualified |
| 013 | `fhir-smart-network-broker` | partner FHIR/SMART adapters and sole controlled online egress abstraction | 005 + 006 | Network Broker receipts/allowlist/capability rules; bypass tests; FHIR profile/integrity/conformance evidence; no uncontrolled provider client |
| 014 | `controlled-actions-nphies` | durable action intent/outbox/reconciliation; NPHIES only if product evidence selects it | 013 + workflow evidence | PENDING/SENT/CONFIRMED/FAILED/UNKNOWN; no blind retry; approval binds exact payload; terminology/profile gates for selected NPHIES workflows |
| 015 | `hf-online-pack-ecosystem` | online pack acquisition/publishing and Hugging Face distribution | 008 + 013 + mobile-format constraints from 009 planning | online path only through Network Broker; TUF/offline root + optional Sigstore attestation; chunked/per-arch/resumable packs; HF never runtime requirement |
| 016 | `durable-trusted-record` | persist full trusted-record authority object graph across process restart (Q02) | 002–006 closed; Trusted V1 review | two-process restart + failure/backup qualification; not PRIVATE_DATA_READY |
| 017 | `vault-privacy-qualification` | open-vault work/WAL wipe and privacy honesty (Q03) | 016 closed; 005 EncryptedVault | lifecycle wipe/detect; PRIVATE_DATA_READY only with measured evidence |
| 018 | `host-client-authority` | authenticated local host/client sessions and scoped capabilities (Q04) | 016–017 | sessions/leases/revocation; not remote multi-tenant |
| 019 | `record-semantics` | precision-aware time, append-only amendments, explicit identity reconciliation, missingness (Q06) | 016 + 004 contracts | READY_BASE synthetic; RELEASE_READY false |
| 020 | `fhir-interchange-qualification` | FHIR support matrix, validator evidence, loss-aware export/provenance (Q08) | 019 | CLOSED_CANONICAL READY_BASE; bounded interchange; no false conformance |
| 021 | `minimum-lovable-workflow` | restartable import-review-timeline-export-backup journey (Q07) | 016–020 | CLOSED_CANONICAL READY_BASE; disclosure append; RELEASE_READY=false |
| 022 | `release-qualification-prep` | Q05 locked builds, evidence binding, doctor honesty (prep) | 016–021; Q05 partial pins | CLOSED_CANONICAL READY_BASE prep; RELEASE_READY=false; branch protection EXTERNAL_GATES |
| 023 | `vault-open-metadata-privacy` | SQLCipher page-encrypted EncryptedVault open work (Q03 residual) | 017 closed; admission 005-sqlcipher | CLOSED_CANONICAL READY_BASE; PRIVATE_DATA_READY=false (OS key/swap/snapshot) |
| 024 | `host-os-ipc-authority` | Localhost OS IPC + strict mutating sessions (Q04 residual) | 018 closed; Spec 016 WriterLock | CLOSED_CANONICAL READY_BASE; os_ipc_qualified=true; MULTI_CLIENT_RELEASE_READY=false |
| 025 | `evidence-corpus-lifecycle` | Source-versioned local lexical evidence corpus (Q10) | 011 + 016/019 | CLOSED_CANONICAL READY_BASE; synthetic-owned; clinical quality / RELEASE_READY=false |
| 026+ | `research-site-imaging-genomics-advanced` | deferred site/research/collab/imaging/genomics/plugin/browser/watch/vision expansion | evidence-driven later gates | no Trusted V1 implementation commitment |

## 3. Corrected critical path

```text
FOUNDATION / TRUST
000 -> 001 -> 002 -> 003 -> 004 -> 005 -> 006

EARLY PARALLEL PLANNING/RESEARCH (no product mutation before authorization)
after 004: 007 OpenMed v2.2 parity corpus + Saudi/Arabic corpus design + terminology rights track

CAPABILITY
005 + 006 + qualified 007 -> 008
008 -> 010 / 011
008 + explicit MESC artifact release -> 012

MOBILE
005 + 006 -> 009 base mobile surface
008 -> model-pack capability inside 009

NETWORK / ACTIONS
005 + 006 -> 013 -> 014

ONLINE ECOSYSTEM
008 + 013 (+ 009 pack-format constraints) -> 015

016 durable trusted record (Q02) after foundation
017 vault privacy (Q03) lifecycle wipe
018 host/client authority (Q04)
019 record semantics (Q06)
020 FHIR interchange qualification (Q08) CLOSED_CANONICAL READY_BASE
021 minimum lovable workflow (Q07) CLOSED_CANONICAL READY_BASE
022 release-qualification prep (Q05) CLOSED_CANONICAL READY_BASE prep; RELEASE_READY=false
023 vault open-metadata privacy (Q03 residual) CLOSED_CANONICAL READY_BASE; PRIVATE_DATA_READY=false
024 host OS IPC authority (Q04 residual) CLOSED_CANONICAL READY_BASE; os_ipc_qualified=true; MULTI_CLIENT_RELEASE_READY=false
025 evidence corpus lifecycle (Q10) CLOSED_CANONICAL READY_BASE; synthetic-lexical@version; clinical quality / RELEASE_READY=false
Next: 012 MESC-blocked; OS key/swap/snapshot still block PRIVATE_DATA_READY; 026+ deferred
```

## 4. Core process topology — decided for downstream specs

Desktop/headless uses one Rust **MedScale Core Host** per open vault. It alone owns the writable canonical metadata connection and active vault key material. Desktop UI, CLI and SDK are local IPC clients. When no host exists, a CLI command may spawn/own a transient host under the same authorization path. A per-vault single-writer lease prevents two independent owners. Direct UI/CLI multi-process database opening is forbidden.

IPC uses OS-local transports and peer identity where available (named pipe on Windows, Unix-domain/local equivalent on Unix platforms), versioned messages, request capabilities, explicit deadlines/limits and no DB/key handle transfer. Mobile uses one app-owned Rust core instance; background/platform services route through the same authority facade.

## 5. Candidate initial crate topology

Avoid crate-per-noun fragmentation. Start with dependency/unsafe-boundary driven crates and promote modules only when compile-time ownership pressure justifies it:

```text
crates/
  medscale-contracts/
  medscale-core/          # authority + time + object semantics as modules initially
  medscale-storage/
  medscale-keys/
  medscale-fhir/
  medscale-projections/
  medscale-worker-protocol/
  medscale-worker-host/
  medscale-cli/

apps/
  desktop/
  ios/
  android/
workers/
  models/
  documents/
  voice/
```

`medscale-network`, `medscale-pack`, and `medscale-ffi` become separate crates when their owning specs make the boundary real. Crate promotion is by dependency/security/unsafe boundary, not by domain-name aesthetics.

## 6. Hard anti-scope rules

H0 contains no OpenMed/MESC/model/OCR/ASR/vector/agent/live network/NPHIES/action/real-PHI/browser/imaging/genomics functionality. No REST/gRPC/GraphQL service plane, plugin ecosystem, general FHIRPath engine, custom cryptography, or cloud-provider adapter is required by V2 core. No feature may bypass the Rust authority path because it is “just CLI,” “just a model,” or “just a worker.”


## 2026-09-09 evidence-backed refinement

Read [whole-product review](WHOLE_PRODUCT_REVIEW_2026-09-09.md) and
[trusted V1 delivery priorities](TRUSTED_V1_DELIVERY_PLAN.md). Historical scope closures
remain intact; next bounded planning work is tracked in BUILD_QUEUE.md. Runtime promotion
requires the owning Spec Kit lifecycle. No blanket advanced-capability promotion is implied.
