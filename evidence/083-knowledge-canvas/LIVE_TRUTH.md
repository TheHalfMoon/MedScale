# Live Truth — Spec 083 (T083-00)

Observed 2026-09-24 with `git fetch`, `gh pr list`, `gh pr view`, `gh run view`.

```text
BASE (origin/main) = 4b9351a56f58ae23146e6b77db62e941d2863132
                     (merge of closure PR #146; Spec 082 closure bookkeeping)
BRANCH             = spec/083-knowledge-canvas (PR #145, draft until qualified)
RECONCILIATION     = forward merge of main into the branch (9625f04);
                     no rebase, no force-push
OPEN PRS           = #144 (Spec 080/081 restore-hex fix, re-qualifying on
                     main), #124, #125 (drafts, unrelated; not touched)
STORAGE SCHEMA     = v11 on base; this spec adds v12
DEPENDENCIES       = none added; lexical scoring is native Rust
REVIEW POLICY      = FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22 (no external
                     reviewer)
CLOSURE PR #146    = exact-head run 35977276424 (6/6 on 93dfd6f)
POST-MAIN RUN FOR BASE = 35983448840 (push on 4b9351a)
EARLIER 083 HEAD   = 665a161 run 35971707983 (6/6, base 3ea6610; ubuntu
                     832 passed, 0 failed)
```
