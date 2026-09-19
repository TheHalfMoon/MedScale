# Clinical Tools + Decision Support Plan — 2026-09-19

**Status:** `PLANNING_CANDIDATE_ONLY`  
**Purpose:** add deterministic and evidence-linked clinical tools without turning model output into medical authority.

## 1. Why this exists

Evidence Q&A and ambient documentation are not enough for a complete clinical intelligence workspace. Clinicians also use:
- validated risk scores;
- dosing/renal-adjustment tools;
- unit conversions;
- criteria/checklists;
- guideline pathways;
- interaction/contraindication references;
- order-set and follow-up logic.

MedScale should support these as inspectable, versioned tools.

## 2. Tool classes

### Deterministic calculator
Formula or finite rules with explicit inputs/units.

Examples may include qualified implementations of:
- risk scores;
- corrected values;
- dose calculations;
- renal/hepatic adjustments;
- clinical criteria.

### Guideline pathway
Versioned decision flow based on an admitted guideline Pack.

### Terminology / medication lookup
Versioned reference data with explicit rights/source.

### Evidence-linked checklist
Human-facing checklist tied to evidence/guideline sections.

### Model-assisted proposal
AI can suggest which tool/path may be relevant or prefill inputs from source data, but deterministic execution and source validation remain separate.

## 3. ClinicalToolManifest

Planning shape:

```text
ClinicalToolManifest {
  tool_id
  title
  version
  jurisdiction?
  specialty?
  source_references[]
  source_version
  rights
  input_schema
  unit_rules
  computation_kind
  output_schema
  limitations
  validation_evidence
  supersedes?
}
```

## 4. Input authority

Every input carries:
- value;
- unit;
- source object/span;
- effective time;
- whether extracted, user-entered, inferred or confirmed.

Auto-prefilled values remain visibly sourced and editable before execution where the tool requires confirmation.

Missing input is not zero.

## 5. Output semantics

Tool output distinguishes:
- numeric/result;
- interpretation text;
- source/guideline version;
- limitations;
- patient applicability;
- review state.

A risk score is not a diagnosis. A pathway suggestion is not an order.

## 6. Formula engine

Prefer a bounded declarative representation for simple calculators.

Requirements:
- deterministic;
- no arbitrary code;
- explicit unit conversion;
- finite operations;
- versioned formula;
- test vectors;
- fail closed on invalid/missing/ambiguous inputs.

Complex tools may execute as isolated qualified workers but still expose the same manifest/result contract.

## 7. Medication / interaction content

Medication interaction, dosing and formulary data may be licensed/restricted.

Architecture must support:
- open local data Packs;
- institution-licensed Packs;
- user/institution external adapters;
- explicit source/version/rights.

Never imply complete drug-interaction coverage from a partial open dataset.

## 8. Patient context integration

MedAgent may propose:

> "This case has the required inputs for Tool X."

The user can inspect:
- why the tool was suggested;
- which inputs were found;
- which inputs are missing;
- source/freshness;
- tool source/version.

The model cannot silently run a tool with guessed missing values.

## 9. Evidence integration

Every tool may link:
- original publication;
- guideline;
- validation study;
- institutional policy.

If source guidance changes, the older tool version remains reproducible and may be marked superseded.

## 10. Order-set and pathway proposals

Guideline/tool outputs may generate ActionProposals:
- labs;
- medication candidates;
- imaging;
- referrals;
- follow-up.

No external effect until explicit approval and owning Spec 090 authorization.

## 11. Care Signals

A care/risk signal should bind:
- trigger rule/model;
- source data;
- source time;
- guideline/ruleset version;
- applicability;
- evidence;
- unresolved prerequisites;
- review state.

Revenue-related signals such as HCC or documentation gaps remain separate from clinical diagnosis authority.

## 12. Tool Packs

Candidate Spec 089 Pack class:
`ClinicalToolPack`.

Contains:
- manifests;
- rules/formulas;
- test vectors;
- source references;
- rights;
- version/supersession;
- validation evidence.

Executable native code is not implicitly trusted through a Tool Pack.

## 13. Safety/evaluation

For deterministic tools prove:
- published/independent test vectors;
- unit behavior;
- edge cases;
- missingness;
- rounding;
- version migration.

For model-assisted input extraction measure:
- extraction accuracy;
- source correctness;
- missing-input detection;
- incorrect prefill rate.

## 14. Research OS ownership

- 077 — MedAgent tool discovery/use;
- 079 — privacy/context/egress;
- 083 — patient evidence/source linkage;
- 088 — workflow presentation and action proposals;
- 089 — versioned ClinicalTool Packs;
- 090 — institution-specific formulary/order/effect adapters;
- 092 — integrated safety/validation.

No implementation authority is granted by this document.
