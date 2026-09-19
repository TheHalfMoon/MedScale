# Research OS V2 Source Qualification Ledger — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** record exact research pins and initial adoption posture for new sources introduced by the Clinical + Research Intelligence expansion.  
**Rule:** a research pin is not implementation admission. Any direct dependency, copied file, adapted implementation, model, dataset or asset still requires owning-spec review, transitive rights/security review, and exact target-path provenance.

## 1. New source pins

| Source | Research pin | Observed license posture | Candidate posture | Intended value |
|---|---|---|---|---|
| Graphify-Labs/graphify | `v8@b9cd9570728a5ff3485d2a1e36fe9a1272a368ae` | Apache-2.0 at repository root | `REFERENCE / ADAPT / COPY_SELECTIVE after path review` | explained graph edges, path/query/explain, community discovery, local-first graph concepts |
| toeverything/AFFiNE | `canary@368e62d895f3f4c76dfb4f28c79268260e04bcc6` | mixed; root describes MIT for content outside restricted areas, backend/server carries separate Enterprise terms and CE/client portions may use MPL-2.0 | `REFERENCE / ADAPT` by default; direct transfer only after exact path license closure | docs/canvas/table/block UX, local/self-host workspace patterns |
| gristlabs/grist-core | `main@c1f3a697120832db4622b02157996d6ac4e38a6d` | Apache-2.0 (`LICENSE.txt`) | `REFERENCE / ADAPT / COPY_SELECTIVE after path review` | relational spreadsheet, formulas, tables/forms, local data-workbench patterns |
| baserow/baserow | `develop@81e094a1f4b3a62625c218d78fe319ba44098617` | Baserow OSE MIT Expat outside premium/enterprise/docs restrictions; docs CC BY-SA; premium/enterprise separate | `REFERENCE / ADAPT / COPY_SELECTIVE from OSE-only paths after review` | no-code relational database, views, field types, API-first patterns |
| nocodb/nocodb | `develop@28ecd7c239181111dd2f37778d48ed7996624e5a` | Sustainable Use License, updated 2026-01-29; internal/non-commercial restrictions | `REFERENCE_ONLY` unless separately licensed/authorized | Airtable-style feature research, schema/view UX |
| teableio/teable | `develop@5ef2238883cad7c3980084de9a9031135fb9734f` | core apps AGPL-3.0 plus additional brand terms; packages directory MIT | `REFERENCE_ONLY` for core app; exact MIT-package study only | spreadsheet-database UX and implementation comparisons |
| maziyarpanahi/openmed | `master@c741089ee55a96013a3d1876bf7ef03713fbe43b` | Apache-2.0 repository; individual model/data rights remain separate | `REFERENCE / DEPEND / ADAPT / COPY_SELECTIVE` per qualified subsystem | local clinical NER, de-identification, model ecosystem, multilingual/on-device patterns |

## 2. Graphify adoption rules

The useful concepts are:

- explicit graph rather than vector-only retrieval;
- `EXTRACTED` versus `INFERRED`/ambiguous relationship classes;
- query/path/explain operations;
- community/subgraph discovery;
- graph artifact output;
- local-first extraction where feasible;
- cross-document concept linking.

MedScale MUST NOT:

- reuse Graphify's code-domain ontology as clinical semantics;
- make Graphify-generated inference a canonical patient fact;
- allow Graphify/graph workers ambient vault access;
- require a remote Graphify service;
- assume a graph database server is necessary.

Before copying any implementation:

1. pin exact file path(s) at the research pin or a newly reviewed pin;
2. inspect embedded/transitive dependencies and NOTICE obligations;
3. define MedScale-owned graph contracts;
4. prove no hidden egress for the selected path;
5. add independent clinical-graph behavior tests;
6. document exit/replacement strategy.

## 3. AFFiNE adoption rules

AFFiNE licensing is path-sensitive. The repository cannot be treated as a single permissive-license donor.

Use AFFiNE primarily to study:

- composable block model;
- document + edgeless/canvas experience;
- table/database views;
- linked workspace navigation;
- local-first/self-hosted interaction patterns;
- collaboration UI.

Direct source transfer is denied until:

- exact file is classified by the license governing that path;
- any MPL/Enterprise/other obligations are reconciled with MedScale's Apache-2.0 distribution strategy;
- third-party assets/fonts/icons are separately cleared;
- MedScale storage/authority contracts remain primary.

Preferred default: reimplement the product concept natively rather than absorb AFFiNE wholesale.

## 4. Grist adoption rules

Grist Community is the preferred first donor for relational-spreadsheet mechanics because the observed community source is Apache-2.0.

Study/adapt:

- column/field type systems;
- formulas;
- linked/reference records;
- filtered/saved views;
- forms;
- card/detail layouts;
- permission/access-rule concepts;
- offline/self-host patterns.

Do not adopt:

- Grist document identity as MedScale dataset identity;
- application/server authority;
- telemetry/account assumptions;
- arbitrary formula execution without MedScale sandbox/reproducibility review.

Exact copied code, if any, still requires path-level provenance and independent security review.

## 5. Baserow OSE adoption rules

Baserow OSE is a strong donor for:

- field/view architecture;
- database builder UX;
- API-first patterns;
- relation/link fields;
- forms;
- extension/plugin ideas.

Only OSE-compatible paths may be considered. Exclude premium/enterprise material unless separately licensed. Documentation content has separate CC BY-SA terms and should not be copied into MedScale implementation docs merely because code is MIT.

## 6. NocoDB posture

The observed current `develop` branch is under Sustainable Use License with restrictions inconsistent with treating it as a normal permissive donor for an Apache-2.0 product.

Therefore:

```text
NOCODB_CORE_SOURCE_TRANSFER = DENIED_BY_DEFAULT
NOCODB_FEATURE_RESEARCH = ALLOWED
NOCODB_REIMPLEMENT_GENERAL_IDEAS = SUBJECT_TO_NORMAL_CLEAN_DESIGN
```

Revisit only if:

- upstream licensing changes; or
- MedScale receives separately documented rights.

## 7. Teable posture

The observed core applications are AGPL-3.0 with additional brand terms; utility packages are MIT.

Therefore:

- use core application as product/UX research only;
- do not wholesale copy core application code into MedScale;
- exact MIT package reuse may be evaluated independently;
- brand assets are never reused.

## 8. OpenMed posture

OpenMed remains the primary external capability-floor donor for local medical AI.

Candidate areas:

- clinical NER;
- PII/PHI detection/de-identification;
- biomedical entity extraction;
- local model catalog patterns;
- MLX/mobile/on-device patterns;
- multilingual medical model selection;
- local model benchmarks/tests;
- multimodal/vision-language medical model integration where rights permit.

OpenMed repository license does not grant rights to every model weight, dataset, terminology asset or upstream artifact it references.

Every admitted model must separately bind:

- model repository/revision;
- weight license/terms;
- tokenizer/config;
- checksum;
- training/use restrictions known from published terms;
- task/language profile;
- hardware/runtime;
- benchmark evidence;
- update/revocation strategy.

## 9. Dynamic commercial/closed competitor sources

Abridge, OpenEvidence, Suki, Microsoft Dragon Copilot and Freed are reference products, not source-code donors.

Their public product pages/docs are used only to establish feature/category expectations.

Before any future parity claim:

- recheck current public capability;
- define a measurable MedScale equivalent;
- run matched evidence;
- report limitations.

No proprietary assets, prompts, templates, typography, private APIs or trade-secret behavior are to be copied.

## 10. Source transfer record template

Any future source transfer should add a record containing:

```text
source_repository
source_revision
source_path
source_license
source_notice
founder_permission_if_relevant
target_path
adoption_mode
modifications
transitive_dependencies
security_review
behavior_tests
provenance_record
update_strategy
rollback_strategy
exit_strategy
```

No code transfer is complete until this record and owning-spec evidence exist.
