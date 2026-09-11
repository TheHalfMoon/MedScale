# MedScale � Start Here

## 1. Current gate

```text
PLAN = CANONICAL_V2
REPOSITORY_PLANNING_FINALIZATION = COMPLETE
SPEC_000 = CLOSED_CANONICAL
SPEC_001 = CLOSED_CANONICAL (see BUILD_QUEUE.md for live states)
FOUNDER_STANDING_CURSOR_IMPLEMENTATION_AUTHORITY = ACTIVE
REAL_PHI = NOT_AUTHORIZED
MESC_MUTATION = NOT_AUTHORIZED
```

The earlier planning-only gate has been superseded by `IMPLEMENTATION_AUTHORITY.md`. Cursor must follow live BUILD_QUEUE.md; Specs 001�011 and 013�015 are CLOSED_CANONICAL; Spec 012 remains MESC-artifact gated. It must not ask the founder for ordinary engineering decisions already governed by the plan.

## 2. Mandatory read order

1. `/CURSOR.md`
2. `/AGENTS.md`
3. `IMPLEMENTATION_AUTHORITY.md`
4. `BUILD_QUEUE.md`
5. `MASTER_BUILD_PLAN_V2.md`
6. `SPECKIT_MASTER_ROADMAP_V2.md`
7. `IMPLEMENTATION_DECISION_DEFAULTS.md`
8. `SOURCE_ACQUISITION_AND_COPY_PLAN.md`
9. `V0_UI_INTEGRATION_CONTRACT.md`
10. relevant source/OSS/OpenMed matrices and current spec package

## 3. Build order

```text
000 CLOSED_CANONICAL
  -> 001 Rust Repository + Spec Kit Bootstrap
  -> 002 Trusted Object / Source / Authority + Process/Text Foundation
  -> 003 H0-A Trusted Ingest + Durability
  -> 004 H0-B Trusted Presentation + Coverage
  -> 005 Local Private Vault + Encryption + Recovery
  -> 006 CLI + Desktop Foundation
```

Then dependency-controlled tracks:

```text
004 -> 007 OpenMed parity/absorption research
005 + 006 + qualified 007 -> 008 Local AI Capability Fabric
005 + 006 -> 009 Mobile base; AI features also require 008
008 -> 010 Documents/OCR/Voice
004 + 008 -> 011 Evidence/Retrieval/Medical Intelligence
008 + released MESC artifact -> 012 MESC Artifact Integration
005 + 006 -> 013 FHIR/SMART/Network Broker -> 014 Controlled Actions/NPHIES
008 + 013 (+ mobile pack constraints) -> 015 Online Pack/HF Ecosystem
016+ remains deferred until canonically promoted
```

## 4. Current follow-on work

Read [whole-product review](WHOLE_PRODUCT_REVIEW_2026-09-09.md),
[delivery plan](TRUSTED_V1_DELIVERY_PLAN.md), and live [BUILD_QUEUE.md](BUILD_QUEUE.md).
Specs 016�023 are `CLOSED_CANONICAL` READY_BASE (022 = Q05 release-qualification **prep** only;
023 = Q03 open-metadata SQLCipher; `PRIVATE_DATA_READY` still FALSE).
Spec **024** is `CLOSED_CANONICAL` READY_BASE (Q04 host OS IPC; `MULTI_CLIENT_RELEASE_READY` still FALSE).
Spec **025** is `CLOSED_CANONICAL` READY_BASE (Q10 versioned synthetic evidence corpus; clinical quality / `RELEASE_READY` still FALSE).
Spec **026** is `CLOSED_CANONICAL` READY_BASE (Q09 pack signer/anti-rollback + Linux Landlock measured; `platform_qualified=false`; `WORKER_OS_SANDBOX_PLATFORM_QUALIFIED` still OPEN).
Spec **027** is `CLOSED_CANONICAL` READY_BASE (Q05 perf harness + SBOM/checksum scaffolds; budgets not claimed; `RELEASE_READY` still FALSE).
Spec **028** is `CLOSED_CANONICAL` READY_BASE (Q03 OS keyring custody; `os_keyring_available`/`os_keyring_used`; `PRIVATE_DATA_READY` still FALSE � swap/snapshot residual).
Spec **029** is `CLOSED_CANONICAL` READY_BASE (macOS CI matrix + CLI/fixture accessibility honesty; `macos_ci_present=true`; `macos_qualified=false`; no WCAG / `RELEASE_READY` claim).
Spec **030** is `CLOSED_CANONICAL` READY_BASE (Q09 Windows Job Object measured; `windows_measured=true`; AppContainer scaffold; `platform_qualified=false`; sandbox gate still OPEN).
Spec **031** is `CLOSED_CANONICAL` READY_BASE (Q09 macOS Seatbelt measured; `macos_measured=true`; App Sandbox entitlements scaffold; `platform_qualified=false`; sandbox gate still OPEN).
Spec **032** is `CLOSED_CANONICAL` READY_BASE (Q03 privacy probes + Q05 NOTICE inventory + perf binding; `probes_present=true`; residual classes open; `notice_inventory_present=true`; `rights_license_decision=false`; `budgets_claimed_met=false`; `PRIVATE_DATA_READY` / `RELEASE_READY` still FALSE).
Spec **033** is `CLOSED_CANONICAL` READY_BASE (Q09 Windows AppContainer FS measured; `windows_appcontainer_fs_measured=true`; `platform_qualified=false`).
Spec **034** is `CLOSED_CANONICAL` READY_BASE (Q12 durable outbox restart; `outbox_restart_qualified=true`; NPHIES still gated).
Spec **035** is `CLOSED_CANONICAL` READY_BASE (EncryptedVault authority sync; `encrypted_authority_sync_qualified=true`; `PRIVATE_DATA_READY` still FALSE).
Spec **036** is `CLOSED_CANONICAL` READY_BASE (MESC synthetic verifier; `verifier_ready_base=true`; Spec 012 / `MESC_RELEASED_ARTIFACT` still NOT_AVAILABLE).
Spec **037** is `CLOSED_CANONICAL` READY_BASE (required-checks packet + checksum verify + license counsel packet + broker transport-fail fixtures; `RELEASE_READY` still FALSE).
Spec **038** is `CLOSED_CANONICAL` READY_BASE (Q09 AppContainer network measured; `windows_appcontainer_network_measured=true`; LPAC scaffold; sandbox gate still OPEN).
Spec **039** is `CLOSED_CANONICAL` READY_BASE (entry-doc honesty + PHI readiness checklist + signing/provenance prep packets; no RELEASE_READY claim).
Spec **040** is `CLOSED_CANONICAL` READY_BASE (Windows AppContainer LPAC ReadyBaseMeasured; `windows_appcontainer_lpac_measured`; `platform_qualified=false`).
Spec **041** is `CLOSED_CANONICAL` READY_BASE (macOS App Sandbox entitlements artifact/probe; enforcement_measured=false; see `SPEC_041_PROMOTION.md`).
Spec **042** is `CLOSED_CANONICAL` READY_BASE (delivery-plan scale perf harness; `budgets_claimed_met=false`).
Spec **046** is `CLOSED_CANONICAL` READY_BASE (SBOM Cargo.lock binding; `sbom_lock_bound`; not full release SBOM).
Spec **047** is `CLOSED_CANONICAL` READY_BASE (release dry-run + cross-verifier; `release_dry_run_verifier_present`; RELEASE_READY still FALSE; see `SPEC_047_PROMOTION.md`).
Spec **048** is `CLOSED_CANONICAL` READY_BASE (migration/recovery release-bar; `migration_recovery_ready_base`; RELEASE_READY still FALSE; see `SPEC_048_PROMOTION.md`).
Spec **049** is `CLOSED_CANONICAL` READY_BASE (package upgrade/rollback dry-run scaffold; `package_upgrade_rollback_scaffold_present`; RELEASE_READY still FALSE; see `SPEC_049_PROMOTION.md`).
Spec **050** is `CLOSED_CANONICAL` READY_BASE (host perf measurement path; `host_perf_measurement_path_present`; budgets still not claimed; see `SPEC_050_PROMOTION.md`).
Spec **051** is `CLOSED_CANONICAL` READY_BASE (REQUIRED_CHECKS live CI sync; `required_checks_packet_synced`; branch protection still `NOT_CONFIGURED_OWNER_SETTINGS`; see `SPEC_051_PROMOTION.md`).
Spec **052** is `CLOSED_CANONICAL` READY_BASE (Linux Landlock FS+TCP+rlimit composition; `linux_landlock_composition_measured`; seccomp still open; `platform_qualified=false`; see `SPEC_052_PROMOTION.md`).
Spec 012 remains MESC-blocked. Deferred advanced work is **053+**.
Do **not** claim `RELEASE_READY`, `PRIVATE_DATA_READY`, or `MULTI_CLIENT_RELEASE_READY`.
Q03 residual page encryption landed in Spec 023; OS keyring READY_BASE in Spec 028; Spec 032/043 probes classify residuals but do not clear swap/snapshot (still block PRIVATE_DATA_READY).
Foundation and prep closure is not product or privacy release readiness.

### Historical bootstrap requirements (already closed)

Spec 001 must:

- establish the Rust workspace and minimal dependency-directed crate skeleton;
- install/bootstrap GitHub Spec Kit and materialize the already-canonical constitution/source authority without reopening founder decisions;
- create CI, formatting/lint/test/evidence skeleton and supply-chain gates;
- establish repository contribution/review/evidence conventions;
- create the next complete Spec 002 package;
- add no medical functionality beyond bootstrap contracts required by 001.

When 001 closes, Cursor immediately starts 002.

## 5. First product wedge

The first useful product outcome is the **Trusted Local Longitudinal Record**: exact source custody, explicit identity/time, deterministic timeline and narrow LLM-free Brief, coverage/conflict/unknown-vs-absence accounting, provenance drill-down, and one Rust authority path shared by CLI/Desktop. It remains valuable with no model installed.

## 6. UI

The founder uses **v0** for visual UI. Cursor continues backend/core/integration work without waiting for final visual polish. v0 output enters through `imports/v0/` and the `V0_UI_INTEGRATION_CONTRACT.md`; generated server/database/network shortcuts never become MedScale authority automatically.

## 7. Evidence and completion

No `PASS`, `PARITY`, `SURPASS`, `PRIVATE`, `OFFLINE`, `CONFORMANT`, or `CLOSED_CANONICAL` claim is valid without exact evidence. Follow `DEFINITION_OF_DONE.md`. An external gate blocks only its own path; record it in `EXTERNAL_GATES.md` and continue.