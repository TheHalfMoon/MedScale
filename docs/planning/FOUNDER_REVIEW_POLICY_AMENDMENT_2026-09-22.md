# Founder Review-Policy Amendment (2026-09-22)

**Status:** `ACTIVE`
**Supersedes:** the 2026-09-22 founder directive that named Alibaba Open Code
Review (OCR) as the sole accepted semantic/code reviewer and a required
merge/closure gate.

## Decision

The founder revoked the Alibaba Open Code Review requirement, effective
2026-09-22 (later the same day as the directive it replaces).

```text
EXTERNAL_SEMANTIC_CODE_REVIEW_REQUIRED = NO
OCR_REQUIRED                           = NO (no provider, API key, `ocr llm test`,
                                             `ocr review`, exact-range OCR evidence,
                                             or OCR approval is required)
SUBSTITUTE_REVIEWER                    = NONE (no Cubic, CodeRabbit, Copilot Review,
                                             Jev/TypeSafe, other LLM/bot reviewer, and no
                                             Claude self-review presented as independent review)
MERGE_AND_CLOSURE_GATE                 = deterministic, executable qualification +
                                         exact-head required CI + post-main CI +
                                         requirement-to-evidence traceability
```

No cloud dependency, API key, or local model runtime (for example Ollama) is
introduced to replace the removed review step.

## What the quality gate is now

A merge candidate qualifies through the repository's own deterministic and
executable systems, as applicable to the unit:

- formatting, compilation, dependency-direction checks, Clippy with denied
  warnings, cargo-deny and supply-chain checks, `git diff --check`;
- contract, unit, integration, migration, backup/restore, crash/reopen,
  concurrency, malformed-input, corruption, security, authorization-boundary,
  cross-project isolation, context-boundary, leakage, CLI, Desktop/Core parity
  and workspace tests;
- performance and packaging qualification where the unit touches them;
- exact-head GitHub CI (all required jobs, bound to the final candidate head);
- post-main CI after merge;
- a deterministic exact-range scope record (changed files vs authorized scope,
  dependency manifests, unexpected files) in place of a semantic review;
- requirement-to-evidence traceability: each checked requirement maps to its
  implementation, tests, evidence and qualification.

A green suite does not by itself prove every product claim; each requirement
still maps to the specific evidence that proves it.

## Effect on existing governance text

- `DEFINITION_OF_DONE.md`: "current-head review" and "no unresolved material
  review finding" are satisfied by the deterministic qualification above plus
  any defect found by it or by the implementer being fixed or recorded as an
  explicit residual. They no longer require an external reviewer.
- `EXTERNAL_GATES.md`: row `OCR_ENGINE_LLM_ENDPOINT` is `RETIRED` and is not
  a blocker.
- Spec 078 `plan.md` step 7 and `tasks.md` T078-08: the OCR exact-range review
  item is replaced by a deterministic exact-range scope record.

## Historical records preserved

The OCR directive existed and was applied. These records stay unchanged as
history. They are not current requirements:

- Spec 076 and 077 exact-range reviews run in OCR delegation mode
  (`evidence/076-collaboration-substrate/EXACT_RANGE_REVIEW.md`,
  `evidence/077-medagent-workbench/EXACT_RANGE_REVIEW.md`) and the fixes they
  produced (`04fa4cb`, `90589fc`);
- earlier OCR records for Specs 066-069, 071, 072 and 074;
- `evidence/078-model-fleet-compare/logs/ocr-delegate-preview-547fb41.txt`;
- the live 2026-09-22 check that `ocr` v1.12.7 was installed with no engine
  endpoint configured.
