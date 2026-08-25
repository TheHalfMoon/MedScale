# MedScale Standing Implementation Authority

**Date:** 2026-08-25  
**Status:** `FOUNDER_STANDING_AUTHORITY_ACTIVE`

This document supersedes earlier V2 metadata lines that said `Implementation authorization: NO` or that required another founder approval before Spec 001. Those lines described the earlier planning phase and are no longer the active execution gate. Architecture, safety, source, and scope constraints in those documents remain canonical.

```text
CURSOR_IMPLEMENTATION_AUTHORITY = YES_WITHIN_CANONICAL_V2_PLAN
LOCAL_EDIT_AUTHORITY = YES
DEPENDENCY_INSTALL_AUTHORITY = YES_IF_OWNING_SPEC_ADMITS_AND_LOCKS_IT
PUBLIC_SOURCE_RETRIEVAL = YES
TEST_FUZZ_BENCHMARK_AUTHORITY = YES_WITHIN_SPEC
BRANCH_AUTHORITY = YES
COMMIT_AUTHORITY = YES
PUSH_AUTHORITY = YES_IF_AUTHENTICATED
PR_AUTHORITY = YES
MERGE_AUTHORITY = YES_AFTER_REQUIRED_EXACT_HEAD_GATES_PASS
CONTINUE_AUTONOMOUSLY = YES

FORCE_PUSH = NO
DESTRUCTIVE_HISTORY_REWRITE = NO
REPOSITORY_SETTINGS_AUTHORITY = NO
MESC_MUTATION_AUTHORITY = NO
REAL_PHI_AUTHORITY = NO
PRODUCTION_CREDENTIAL_AUTHORITY = NO
LEGAL_OR_GATED_TERMS_ACCEPTANCE_AUTHORITY = NO
PRODUCTION_EXTERNAL_ACTION_AUTHORITY = NO
```

Model execution is spec-scoped. H0 forbids it. Later specs may execute admitted local models using synthetic/permitted non-PHI data when the qualified spec requires it. A gated model requiring the founder to accept terms is an external gate; use another admissible comparator or continue independent work.

Development/build network use is allowed only for public source/documentation/tool/dependency acquisition needed by the owning spec. Runtime product network is separate and remains default-deny until Spec 013.

Cursor must not ask for additional permission for ordinary actions already covered above.