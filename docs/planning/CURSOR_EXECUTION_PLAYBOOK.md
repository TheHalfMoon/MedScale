# Cursor Autonomous Execution Playbook

## Objective

Execute MedScale from current live repository truth through the furthest genuinely completable V2 state without asking the founder routine questions and without stopping after one task/PR/spec.

## Loop

For every iteration:

1. **Refresh truth:** `git fetch --all --prune`; inspect `git status`, current branch/head/tree, remote `main`, branches, PRs, CI/reviews, current `BUILD_QUEUE.md`, and current spec/task state.
2. **Select work:** choose the first genuinely eligible unit by roadmap dependencies. Never skip ahead because a later feature is more interesting.
3. **Read authority:** `AGENTS.md`, `CURSOR.md`, owning spec, relevant planning/source/donor docs.
4. **Spec Kit:** complete/repair `spec.md`, clarification closeout, `research.md`, `plan.md`, `data-model.md` where applicable, `contracts/`, `quickstart.md`, `checklists/`, `tasks.md`, implementation handoff; run `/speckit.analyze` and resolve contradictions.
5. **Branch:** use `spec/<id>-<slug>` for a material spec unless repository truth already has the eligible branch/PR.
6. **Implement:** execute tasks in dependency order. Install only admitted/pinned dependencies. Follow source acquisition plan before donor use.
7. **Qualify continuously:** focused tests first, then required full gates; add negative/fault/fuzz/conformance/benchmark evidence where the spec requires it.
8. **Review exact head:** resolve material findings; never infer PASS from stale head or expected failure.
9. **Close:** converge docs/evidence/tasks, merge only if required exact-head gates pass, verify post-merge `main`, mark `CLOSED_CANONICAL`.
10. **Continue:** update `BUILD_QUEUE.md`, immediately begin the next eligible unit.

## Git discipline

- No force push or destructive rebase/history rewrite.
- Keep commits scoped and descriptive in English.
- Prefer one PR per material spec or independently reviewable sub-slice defined by `tasks.md`.
- Do not merge a failing or materially unresolved PR.
- If GitHub authentication is unavailable, keep a clean local branch/commits and continue work that does not require remote mutation; record the remote gate rather than asking a routine question.

## Dependency and source discipline

Every dependency must be owned by a spec, exactly pinned/locked as appropriate, justified against alternatives, checked for license/security/transitives, and have an exit strategy when load-bearing. Never copy donor code before the source acquisition record is complete.

## Network distinction

Development-time retrieval of public docs/source/dependencies is permitted. Product runtime egress is not. Before Spec 013, runtime tests must prove default-deny/no hidden telemetry for the claimed scope.

## External gates

Do not ask the founder to unblock a path while independent work remains. Add/update `EXTERNAL_GATES.md`, use mocks/synthetic fixtures/public alternatives, and continue.

## Final state

Stop autonomous implementation only when `DEFINITION_OF_DONE.md` says the current V2 program is complete or when literally every remaining eligible path is blocked by recorded external gates. Report exact evidence and remaining gates, never an unsupported success claim.