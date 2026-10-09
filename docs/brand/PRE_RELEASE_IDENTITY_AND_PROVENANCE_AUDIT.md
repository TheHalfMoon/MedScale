# Pre-release identity and provenance audit (issue #182)

**Rule:** product-facing identity is MedScale's own. Technical source provenance, licenses and history are kept, accurate and separate. No history rewrite. No removal of required notices.

**Scope of this record:** `main` as of 2026-10-09 (README, CLI, brand assets, required records). The Tauri desktop surfaces (window title, bundle metadata, icons, in-app copy, About page, release bundles) are audited on the productization stack (PR #179, `apps/desktop-tauri/src/identity.test.ts`). They land on `main` only with that chain, which is blocked by F096-T01 (see [external gates](../planning/EXTERNAL_GATES.md)). This record will be merged with that audit when the chain lands.

## 1. Product-facing surfaces on `main`

| Surface | Finding | Disposition |
|---|---|---|
| `README.md` introduction | Opened with a governance note, then agent instructions and a status block containing `UI_VISUAL_SOURCE = v0`. Described OpenMed as the "capability floor/strategic donor" in product prose | **Changed**:<br>• a MedScale-first product description, with an explicit engineering-preview / synthetic-only status;<br>• the surfaces stated as they are on `main`: the CLI ships, the Tauri desktop is in review, and Slint is a legacy reference;<br>• the engineering and agent sections moved below the introduction, with their content unchanged;<br>• `UI_VISUAL_SOURCE = v0` removed (superseded by Specs 095–101);<br>• the v0 contract link labelled historical;<br>• OpenMed described as a **pinned comparator** under "Comparisons and provenance" |
| CLI (`medscale --help`) | "MedScale · evidence-first local clinical intelligence CLI" | Kept; guarded by test |
| Brand assets (`assets/brand`) | Approved paired-triangle M (Spec 095), first-party SVGs; black/white with sky blue `#70B8C7` and orange `#FB905A` accents | Kept. The superseded identity of PR #125 is not present |
| Slint desktop (`crates/medscale-desktop`) | `LEGACY_REFERENCE_IMPLEMENTATION`; not a release surface | Not rebranded (retired surface); unchanged |

## 2. Provenance retained (not product branding)

| Record | Status |
|---|---|
| `LICENSE` (Apache-2.0) | Retained |
| `third_party/provenance/*` (identity, typography, component template) | Retained |
| `assets/brand/fonts/*-OFL.txt`, `FONT_NOTICE.md` | Retained (license requirement) |
| Planning records (`OSS_CODE_ABSORPTION_MATRIX_V2`, `OPENMED_PARITY_SURPASS_MATRIX_V2`, `SOURCE_ACQUISITION_AND_COPY_PLAN`, `V0_UI_INTEGRATION_CONTRACT`) | Retained as engineering provenance. The README labels the v0 contract historical |
| Code adapted from OpenMed v3.0.0 (Spec 103: catalog parsing, reproducibility hash, residency policy, document windowing and entity decoding) | Each module names the exact upstream file and commit in its header. OpenMed is Apache-2.0; attribution is kept in source |
| Model rights (Spec 103) | [`evidence/103-local-model-catalog/RIGHTS_LEDGER.md`](../../evidence/103-local-model-catalog/RIGHTS_LEDGER.md): card-level licenses and lineage; dataset terms are recorded as unverified |
| Specs, evidence, captures, PR descriptions and bot summaries | Historical records. **Not altered**, and exempt from the identity guard |

The founder states that permission exists to reuse or adapt donor source code. That permission does not remove license or notice obligations. Source-specific licenses remain in the records above. Any separately granted written permission must be added to `third_party/provenance` before it is relied on for an attribution exception. No such exception is applied in this audit.

## 3. Automated guard on `main`

`crates/medscale-core/tests/identity_guard_182.rs` runs in every `rust` CI job (Ubuntu, macOS, Windows):

1. The README introduction and the CLI description contain no:
   - stale `v0` visual-source marker;
   - "strategic donor" or "capability floor" positioning;
   - generator boilerplate or placeholder text.

   The introduction keeps the engineering-preview / synthetic-only / not-clinically-validated status.
2. The README introduction makes no bare model-count claim (a catalog listing is not an executable model).
3. Required records exist: `LICENSE`, `third_party/provenance`, the font license texts and this audit.

## 4. Open items

- Desktop surfaces, release-bundle NOTICE/SBOM checks and screenshots arrive with the Tauri chain (PR #174 → #179), which is blocked by F096-T01.
- No zero-cost tool is admitted for a formal similarity scan of UI code against donor templates, so none was performed.
- Release remains blocked by its gates (`RELEASE_READY=false`).
