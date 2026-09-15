# Research — Spec 066

The 060–065 product sequence made Patients, Insights, Workflows, Tasks, Messages, Audit, Exports, Settings, Integrations, and CLI product reads real. A hardening audit found three remaining product mismatches: the advertised top-level Documents route still fell through to generic future-slice text, Home synthetic cards used `High risk` despite no qualified clinical risk contract, and PR branches ran both `push` and `pull_request` copies of the same six CI jobs.

The required ruleset checks are job-context names, not branch-push events. Keeping `pull_request` plus `push: main` preserves exact-head PR qualification and post-merge main qualification while removing redundant spec-branch duplicate work. Required job names remain unchanged.

The existing document pipeline separates custody, extraction evidence, and authority. Documents hardening should expose that separation rather than invent OCR/semantic understanding.
