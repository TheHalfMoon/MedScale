# Claim-to-evidence map

Status vocabulary: `SUPPORTED_AT_SNAPSHOT`, `SUPPORTED_AT_EXPERIMENTAL_REVISION`, `PARTIAL`, `TO_BE_TESTED`, `NOT_CLAIMED`.

| Candidate claim | Status | Primary repository evidence | Paper use |
|---|---|---|---|
| One admitted Core authority path is used across qualified surfaces | SUPPORTED_AT_SNAPSHOT | `evidence/092-whole-platform-qualification/MATRIX.md` row 1; subsystem authority qualification packets; `CoreFacade::dispatch` paths | Contribution 1 / architecture |
| Source identity is distinct from content digest; exact source bytes are digest-verifiable | SUPPORTED_AT_SNAPSHOT | `crates/medscale-contracts/src/objects/source.rs`; Specs 003/004 evidence | Contribution 2; Experiment A controls |
| Derived source artifacts bind parent source, transform id/version, bytes, and digest on the exercised Core path | SUPPORTED_AT_EXPERIMENTAL_REVISION | A1 harness `crates/medscale-core/tests/paper_provenance_system.rs`; qualifying run `36643514848`; revision `6a04266...` | Contribution 2; Experiment A result |
| Research evidence preserves explicit unknown axes and conservative support semantics | SUPPORTED_AT_SNAPSHOT | `crates/medscale-contracts/src/research_packs.rs`; `evidence/089-research-packs/{QUALIFICATION,SECURITY}.md` | Contribution 3 |
| A universal single enum implements every paper failure label | NOT_CLAIMED | No such evidence established | Paper must distinguish subsystem states from analytical taxonomy |
| Network/institutional paths fail closed in the qualified local synthetic profile; no hidden cloud fallback | SUPPORTED_AT_SNAPSHOT | Spec 092 matrix rows 12, 15, 32; Privacy Gate, institutional adapter/federation evidence | Contribution 4; Experiment B |
| Compute, R, extensions, adapters, and federation are fully sandboxed/production qualified | NOT_CLAIMED | Matrix marks relevant areas PARTIAL/EXTERNAL and `PLATFORM_QUALIFIED=false` | Limitation |
| Whole-platform integrated qualification spans 35 evidence areas and backup/restore/restart | SUPPORTED_AT_SNAPSHOT | `evidence/092-whole-platform-qualification/{MATRIX,CLOSURE}.md`; `crates/medscale-core/tests/whole_platform_092.rs` | Contribution 6 / qualification; C0 |
| Integrated qualification found a restore ID-sequence defect and a nondeterministic privacy matcher defect, both repaired/regression-tested | SUPPORTED_AT_SNAPSHOT | Spec 092 closure/matrix; final audit | Failure analysis |
| The 13 A0/A1 provenance/authority cases pass on hosted Linux, macOS, and Windows at the exact experimental revision | SUPPORTED_AT_EXPERIMENTAL_REVISION | Run `36643514848`; revision `6a04266...`; retained A0/A1 artifacts and SHA-256 checksums | Experiment A result |
| The six B0 authority/effect cases pass on hosted Linux, macOS, and Windows at the exact experimental revision | SUPPORTED_AT_EXPERIMENTAL_REVISION | Run `36643514848`; revision `6a04266...`; retained B0 artifacts and SHA-256 checksums | Experiment B result |
| Across A0/A1/B0, 19 unique cases yield 57/57 successful repeated platform executions | SUPPORTED_AT_EXPERIMENTAL_REVISION | Run `36643514848`; Linux artifact `11067444321`; macOS `11067474345`; Windows `11068681168` | Evaluation summary; must state 19 unique cases |
| Deterministic repeatability/recovery results beyond existing Spec 092 assertions | TO_BE_TESTED | C1 repeated fresh-vault recovery not yet executed | Experiment C result |
| Provenance/performance/storage cost | TO_BE_TESTED | New measurements required | Experiment D result |
| Clinical validity, outcome benefit, regulatory compliance, real-PHI readiness, universal security | NOT_CLAIMED | Explicitly excluded by final audit and paper protocol | Never state as result |

## Quantitative evidence binding

Qualifying A/B result:

- experimental revision: `6a04266ae59bfb75f24d6634becd3bc4d32576c4`
- workflow run: `36643514848`
- command generator/workflow: `.github/workflows/paper-evaluation.yml`
- evidence capture: `paper/artifact/scripts/evidence_io.py`
- Linux artifact: `11067444321`, archive SHA-256 `8575a05792f1b4941d4fa65940da0fb02d38f6328fc0a7325400be2abfdc5112`
- macOS artifact: `11067474345`, archive SHA-256 `64db3cccf604e241632b2ff6894ab67200621466da7036ac1fef1ceb170a8e82`
- Windows artifact: `11068681168`, archive SHA-256 `2a2609a69b28bf1d41c115a043944129da11354b668e8bf80060caafda153c04`

The GitHub event SHA for this PR run is a synthetic merge revision and is retained only as trigger metadata; qualification is bound to the checkout SHA above.

## Evidence discipline

Every later quantitative result added to the manuscript must add a row here with the raw result path/artifact, generator path, experiment command, exact code SHA, and result checksum. A manuscript-only commit does not retroactively change the experimental revision to which these results apply.
