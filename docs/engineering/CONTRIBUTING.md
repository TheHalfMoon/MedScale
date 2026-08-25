# Contributing to MedScale

## Authority

Read `CURSOR.md`, `AGENTS.md`, and `docs/planning/BUILD_QUEUE.md` before material work.
Ordinary decisions use `docs/planning/IMPLEMENTATION_DECISION_DEFAULTS.md`.

## Branching

Prefer `spec/<id>-<slug>` for material specs. One material spec per PR unless tasks require otherwise.

## Commits

Focused English commits. No force-push or destructive history rewrite.

## Pull requests

Use `.github/PULL_REQUEST_TEMPLATE.md`. Merge only when required exact-head gates pass.

## Dependencies

Complete `docs/engineering/DEPENDENCY_ADMISSION_TEMPLATE.md` before admitting a load-bearing dependency.
Follow `docs/planning/SOURCE_ACQUISITION_AND_COPY_PLAN.md` before donor code.

## Evidence

Do not claim PASS without exact-head evidence under `evidence/`.

## Anti-scope reminders

- Real PHI unauthorized
- MESC mutation unauthorized
- Product runtime egress DEFAULT_DENY until Spec 013
