# MedScale Product UI Grammar

**Status:** CANONICAL_DRAFT_SPEC_068

## 1. The work is the interface

Primary screens begin with the clinician/operator task and its evidence, not with product metrics or decorative modules.

Home is a clinical workspace. Patient is a longitudinal evidence workspace. Models is runtime/provenance inspection. Evidence is proof and comparative qualification.

## 2. Surface hierarchy

Use this default hierarchy:
1. current context and task;
2. primary work surface;
3. linked evidence/source state;
4. intelligence/model layer;
5. reviewable action;
6. governance detail.

If a page gives equal visual weight to all six layers, it is not shaped yet.

## 3. Containers

Use cards only when a boundary matters. Prefer whitespace, alignment, and dividers for grouping.

Do not create:
- card grids as the default page structure;
- cards inside cards;
- rounded-square icon tiles above headings;
- decorative panels with no semantic boundary.

Primary reading/editing surfaces should feel continuous and calm.
## 4. Care-flow grammar

Use the sequence `Prepare → Understand → Act` when a clinical workflow spans context, intelligence, and consequence.

- **Prepare:** source freshness, longitudinal history, conflicts, coverage, visit context.
- **Understand:** evidence-linked synthesis, model output, source inspection, uncertainty.
- **Act:** explicit review, payload identity, durable state, reconciliation.

This is a workflow grammar, not a forced three-column layout. Adapt it to the task while preserving the order of authority.

## 5. AI/model grammar

Never present an assistant response without nearby runtime/evidence affordances when the response could affect understanding or action.

Model Center rows expose at minimum:
- task;
- model/repository identifier;
- source;
- runtime;
- device;
- trust/admission state;
- benchmark state;
- provenance or digest where available.

A model logo may identify source/vendor but may not visually dominate or imply that MedScale is part of that company.

## 6. Evidence grammar

Evidence is readable in place. Prefer linked source snippets, timestamps, provenance, and state labels over a generic `Sources` footer.

Comparative evidence uses explicit state labels. Competitor branding is secondary to the capability and evidence itself.
