# MedScale Research OS Planning Index

**Status:** Planning candidate — not implementation authority

This index is the entry point for the Research OS expansion proposal.

## Read in this order

1. [`RESEARCH_OS_VISION.md`](./RESEARCH_OS_VISION.md) — product thesis, target users, deployment ladder, Project Graph and Research Packs.
2. [`RESEARCH_OS_DECISIONS.md`](./RESEARCH_OS_DECISIONS.md) — candidate architectural decisions and boundaries.
3. [`RESEARCH_OS_ARCHITECTURE.md`](./RESEARCH_OS_ARCHITECTURE.md) — planes, authority model, capabilities, Hub, Compute, privacy and governed browsing.
4. [`RESEARCH_OS_PRODUCT_MAP.md`](./RESEARCH_OS_PRODUCT_MAP.md) — product surfaces and Project workspace composition.
5. [`AUDIOFLOW_PRODUCT_PLAN.md`](./AUDIOFLOW_PRODUCT_PLAN.md) — audio/voice subsystem, huddles, speech routing and evidence model.
6. [`SOURCE_ADOPTION_MATRIX.md`](./SOURCE_ADOPTION_MATRIX.md) — donor/source roles and qualification rules.
7. [`RESEARCH_OS_THREAT_AND_SCALE_MODEL.md`](./RESEARCH_OS_THREAT_AND_SCALE_MODEL.md) — trust zones, threats, scale tiers and failure scenarios.
8. [`RESEARCH_OS_EXECUTION_ROADMAP.md`](./RESEARCH_OS_EXECUTION_ROADMAP.md) — dependency-ordered candidate Specs 074-088, subject to live frontier reconciliation.

## Governance boundary

These documents intentionally live under `docs/planning/`. They do not alter `specs/CURRENT.md`, do not declare any candidate spec executable, and do not supersede existing MedScale Product/Design authority.

Before promotion, canonical governance must:

1. reverify the live repository frontier;
2. reconcile numbering/dependencies with any specs promoted after this planning branch was created;
3. challenge architecture against the current implementation and external evidence;
4. refine the next candidate unit to bounded spec/plan/tasks/evidence form;
5. authorize only the minimum dependency-ordered unit required.

## North-star statement

> **One workspace. Many engines. One authority. User-owned data. Local by default. Evidence everywhere.**

The expansion succeeds only if MedScale becomes more capable without weakening the properties that make it trustworthy: explicit authority, provenance, privacy boundaries, reproducibility, user-controlled infrastructure, and honest evidence.
