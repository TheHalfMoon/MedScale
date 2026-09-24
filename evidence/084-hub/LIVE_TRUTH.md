# Live Truth — Spec 084 (T084-00)

Observed 2026-09-24 with `git fetch`, `gh pr list`, `gh pr view`, `gh run view`.

```text
BASE (origin/main) = SPEC_083_CLOSURE_MAIN
                     (merge of the Spec 083 closure PR)
SPEC 083           = PR #145 merged as 2892860 (exact-head run 36004914365
                     6/6; post-main run SPEC_083_POST_MAIN)
BRANCH             = spec/084-hub (PR #147, draft until qualified); started
                     from the Spec 083 branch while 083 closed, then
                     forward-merged with main; no rebase, no force-push
OPEN PRS           = #124, #125 (drafts, unrelated; not touched)
STORAGE SCHEMA     = v12 on base; this spec adds v13
DEPENDENCIES       = none added (interprocess, ed25519-dalek, rand, sha2
                     already admitted)
NETWORK            = none; transport is in-process and the Spec 024 local
                     socket
REVIEW POLICY      = FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22 (no external
                     reviewer)
```
