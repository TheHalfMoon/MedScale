# MedScale V2 Source Additions

These sources extend the original 185-URL corpus; they do not replace or silently delete original pointers.

| Domain | Source class | URL | Role / qualification note |
|---|---|---|---|
| OPENMED | EVIDENCE | https://github.com/maziyarpanahi/openmed/releases/tag/v2.2.0 | current pinned competitor release; verify exact tag commit 59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837 |
| OPENMED | EVIDENCE | https://github.com/maziyarpanahi/openmed/blob/v2.2.0/LICENSE | current release licence evidence |
| OPENMED | EVIDENCE | https://github.com/maziyarpanahi/openmed/commit/59d9cb0a2e0ccbba8fa3d891a66d83ffaf45e837 | exact v2.2.0 parity baseline commit |
| TERMINOLOGY | AUTHORITATIVE | https://www.snomed.org/ | SNOMED CT governance/licensing source; rights gate required |
| TERMINOLOGY | CANDIDATE | https://github.com/IHTSDO/snowstorm | terminology server/reference implementation; not required local runtime |
| TERMINOLOGY | AUTHORITATIVE | https://loinc.org/ | LOINC terminology source; rights/version gate |
| TERMINOLOGY | AUTHORITATIVE | https://ucum.org/ | UCUM unit-system source |
| TERMINOLOGY | AUTHORITATIVE | https://icd.who.int/ | WHO ICD source; licensing/version gate |
| TERMINOLOGY | AUTHORITATIVE | https://www.whocc.no/atc_ddd_index/ | ATC/DDD source; rights/version gate |
| TERMINOLOGY | AUTHORITATIVE | https://www.nlm.nih.gov/research/umls/ | UMLS licensing/distribution source; no bundled assumption |
| UNICODE | CANDIDATE | https://github.com/unicode-org/icu4x | Unicode/locale implementation candidate; text semantics remain MedScale-owned |
| CLINICAL_NLP | CANDIDATE | https://github.com/medspacy/medspacy | reference/selected deterministic context rules for Proposals; not H0 authority |
| CLINICAL_NLP | CANDIDATE | https://github.com/allenai/scispacy | clinical/scientific NLP comparator/reference |
| CLINICAL_NLP | CANDIDATE | https://github.com/Georgetown-IR-Lab/QuickUMLS | terminology matching comparator/reference; licence/admission required |
| INTEROP_RESEARCH | AUTHORITATIVE | https://specifications.openehr.org/ | OpenEHR standard source for later parity/adapters |
| INTEROP_RESEARCH | AUTHORITATIVE | https://ohdsi.github.io/CommonDataModel/ | OMOP CDM source for later research projection/parity |
| RUNTIME | CANDIDATE | https://github.com/huggingface/tokenizers | Rust tokenizer candidate; pin/version/regex limits/admission required |
| RUNTIME | CANDIDATE | https://github.com/pykeio/ort | Rust ONNX Runtime binding candidate; runtime remains isolated on desktop |
| RUNTIME | CANDIDATE | https://github.com/tracel-ai/burn | Rust-native runtime benchmark candidate; no preferred status |
| KEYS | CANDIDATE | https://github.com/open-source-cooperative/keyring-core | current keyring API layer; combine with explicit platform stores |
| SUPPLY_CHAIN | CANDIDATE | https://github.com/mozilla/cargo-vet | dependency review/admission tooling candidate |
| SUPPLY_CHAIN | CANDIDATE | https://github.com/rust-secure-code/cargo-auditable | binary dependency metadata/audit evidence tooling candidate |
| SUPPLY_CHAIN | CANDIDATE | https://github.com/rust-fuzz/cargo-fuzz | Rust fuzzing tool for hostile-input surfaces; native donors need equivalent upstream fuzzing |
| PLATFORM_PRIVACY | AUTHORITATIVE | https://developer.apple.com/documentation/security/ksecattrsynchronizable | Apple Keychain synchronization semantics; sensitive key material must not silently sync |
| PLATFORM_MOBILE | AUTHORITATIVE | https://developer.android.com/guide/practices/page-sizes | Android 16 KB page-size compatibility requirement; mobile CI input |
