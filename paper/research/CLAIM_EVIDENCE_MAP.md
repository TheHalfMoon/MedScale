# Claim-to-evidence map

Status vocabulary: `SUPPORTED_AT_SNAPSHOT`, `PARTIAL`, `TO_BE_TESTED`, `NOT_CLAIMED`.

| Candidate claim | Status | Primary repository evidence at frozen snapshot | Paper use |
|---|---|---|---|
| One admitted Core authority path is used across qualified surfaces | SUPPORTED_AT_SNAPSHOT | `evidence/092-whole-platform-qualification/MATRIX.md` row 1; subsystem authority qualification packets; `CoreFacade::dispatch` paths | Contribution 1 / architecture |
| Source identity is distinct from content digest; exact source bytes are digest-verifiable | SUPPORTED_AT_SNAPSHOT | `crates/medscale-contracts/src/objects/source.rs`; Specs 003/004 evidence | Contribution 2; Experiment A controls |
| Derived source artifacts bind parent source, transform id/version, loss class, representation, bytes, digest | SUPPORTED_AT_SNAPSHOT | `crates/medscale-contracts/src/objects/source.rs` | Contribution 2; Experiment A |
| Research evidence preserves explicit unknown axes and conservative support semantics | SUPPORTED_AT_SNAPSHOT | `crates/medscale-contracts/src/research_packs.rs`; `evidence/089-research-packs/{QUALIFICATION,SECURITY}.md` | Contribution 3 |
| A universal single enum implements every paper failure label | NOT_CLAIMED | No such evidence established | Paper must distinguish subsystem states from analytical taxonomy |
| Network/institutional paths fail closed in the qualified local synthetic profile; no hidden cloud fallback | SUPPORTED_AT_SNAPSHOT | Spec 092 matrix rows 12, 15, 32; Privacy Gate, institutional adapter/federation evidence | Contribution 4; Experiment B |
| Compute, R, extensions, adapters, and federation are fully sandboxed/production qualified | NOT_CLAIMED | Matrix marks relevant areas PARTIAL/EXTERNAL and `PLATFORM_QUALIFIED=false` | Limitation |
| Whole-platform integrated qualification spans 35 evidence areas and backup/restore/restart | SUPPORTED_AT_SNAPSHOT | `evidence/092-whole-platform-qualification/{MATRIX,CLOSURE}.md` | Contribution 6 / qualification |
| Integrated qualification found a restore ID-sequence defect and a nondeterministic privacy matcher defect, both repaired/regression-tested | SUPPORTED_AT_SNAPSHOT | Spec 092 closure/matrix; final audit | Failure analysis |
| Provenance mutation detection rate | TO_BE_TESTED | Dedicated paper harness not yet executed | Experiment A result |
| Adversarial/failure refusal rate and false-accept count | TO_BE_TESTED | Dedicated paper harness not yet executed | Experiment B result |
| Deterministic repeatability/recovery results beyond existing Spec 092 assertions | TO_BE_TESTED | Dedicated paper harness not yet executed | Experiment C result |
| Provenance/performance/storage cost | TO_BE_TESTED | New measurements required | Experiment D result |
| Clinical validity, outcome benefit, regulatory compliance, real-PHI readiness, universal security | NOT_CLAIMED | Explicitly excluded by final audit | Never state as result |

## Evidence discipline

Every quantitative result added to the manuscript must add a row here with the raw result path, generator path, experiment command, frozen code SHA, and result checksum.
