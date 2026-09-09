# Source review, 2026-09-09

This is a bounded admission review, not a claim that every historical source or dependency
has been requalified. No donor code, model, dataset or executable was imported or run.
License observations require asset-level review before redistribution.

| Source / immutable revision | Observed value | Disposition and admission prerequisites |
|---|---|---|
| [Magika](https://github.com/google/magika/tree/3ea86d209b7ec70bc372b6bc50f610b55932988b) | Apache-2.0 repository; Rust library 2.0.0-dev, model/runtime dependency, content classification | REFERENCE_ONLY now; OPTIONAL_DEPENDENCY candidate for quarantine classification after Q09. Compare synthetic ambiguous/truncated/polyglot inputs, CPU memory/latency and abstention. Classification never establishes parser safety, validity or authority. Pin model hash, complete transitive/native licenses, offline packaging and sandbox evidence first. |
| [HPC-Ops](https://github.com/Tencent/hpc-ops/tree/2a2e26562433a8ba4b504858f1c938eb7612c901) | MIT LICENSE.txt with third-party exceptions; Torch/CMake/CUDA/CUTLASS native build | BENCHMARK_REFERENCE / FUTURE_RESEARCH. No V1 dependency or portability assumption. A later accelerator proposal must demonstrate end-to-end benefit over portable CPU baseline on owned hardware, with numerical tolerances, fallback, asset rights and confined execution. |
| [AICGSecEval](https://github.com/Tencent/AICGSecEval/tree/94428ebf45141bf4ecd365a51d596dcd51caa690) | Apache-2.0 LICENSE; project-level static/dynamic evaluation methodology and substantial Python/Docker/provider dependencies | REFERENCE_ONLY methodology. Adapt evidence separation and reproducibility concepts into owned synthetic regressions. Do not import or execute exploit corpora, generated projects, credentials or remote evaluation workflows. No medical quality claim follows from security benchmark performance. |

Reviewed donor artifacts: README, repository metadata, license and principal build manifests.
This is not a complete transitive dependency audit, independent benchmark or model-card audit.
Magika's manifest uses magika-tract-runtime; HPC-Ops uses native CUDA extensions; AICGSecEval
requirements include Docker, GitPython, OpenAI and Transformers. Those costs are reasons to
require narrow admission, not evidence of a vulnerability.

## Standards and product references

These primary pages were observed on 2026-09-09. Mutable pages are references, not frozen
implementation inputs. Record exact version/hash in the owning spec before adoption.

| Source | Use / explicit boundary |
|---|---|
| [FHIR R4 security](https://hl7.org/fhir/R4/security.html), [Provenance](https://hl7.org/fhir/R4/provenance.html) | Interchange and provenance design. FHIR does not supply application authorization; selected resources/profiles require explicit coverage and validator evidence. |
| [SMART App Launch](https://hl7.org/fhir/smart-app-launch/) (2.2.0 observed) | Future authorization adapter specification; no live adapter readiness inferred. |
| [openEHR reference model](https://specifications.openehr.org/releases/RM/latest/ehr.html) | Reference for versioned clinical record semantics; do not replace existing canonical objects or add a second database authority. |
| [OMOP CDM 5.4](https://ohdsi.github.io/CommonDataModel/cdm54.html) | Future analytical export reference, not source-custody or operational canonical schema. |
| [SQLite atomic commit](https://sqlite.org/atomiccommit.html), [WAL](https://sqlite.org/wal.html), [temporary files](https://sqlite.org/tempfiles.html) | Storage qualification must include working, journal, WAL and temporary files plus interrupted commits. Documentation does not prove this repository's privacy behavior. |
| [Tauri capabilities](https://tauri.app/security/capabilities/) | Conditional shell reference; merged capabilities and Rust-side authority need explicit review. No unconditional shell adoption. |
| [TUF specification](https://theupdateframework.io/spec/) | Reference for signed metadata, expiry and rollback-resistant update admission; choose a frozen released version before implementation. |
| [OWASP mobile security](https://owasp.org/www-project-mobile-app-security/) | Device qualification checklist reference; no current mobile apps or certification. |
| [Zotero](https://www.zotero.org/support/) | Evidence library usability reference: inspectable sources and organization. No clinical authority inference. |
| [OpenEMR](https://www.open-emr.org/) | Workflow/ecosystem reference; not proof of MedScale EHR parity. |

## Competitive claims

Use the existing pinned OpenMed absorption/parity matrix as the starting inventory. This
review does not replace it with a feature-count claim. “Better than OpenMed” must become
matched-task evidence: installation success, offline behavior, source fidelity, extraction
quality on rights-cleared held-out fixtures, supported languages, latency/memory, recovery,
documentation and task completion. Record both versions, hardware, configuration, dataset
hashes, failures and confidence intervals. Current result: NOT_MEASURED; no SURPASS claim.

Historical source registers remain inventory, not approved dependencies. Every new admission
needs provenance, immutable revision, rights, dependency closure, maintenance/exit costs,
bounded interface and qualification evidence. Prefer reference-only reuse until measurable
product value justifies operating another component.
