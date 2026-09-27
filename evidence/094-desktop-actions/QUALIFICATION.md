# Qualification — Spec 094 Research OS Desktop row actions (reversible slice)

`PASS` is recorded only for rows that exact-head CI ran green on Linux,
Windows and macOS (see `EXACT_HEAD_QUALIFICATION.md`, written with the
closure); until then rows are `PENDING`.

| # | Requirement | Tests | Status |
|---|---|---|---|
| Q1 | Only reversible, non-data-moving actions are offered, one per row, derived from plane and state | desktop `only_reversible_actions_are_offered` | PENDING |
| Q2 | Actions go through Core; actions not offered and missing objects change nothing | desktop `actions_go_through_core_and_unknown_actions_change_nothing` | PENDING |
| Q3 | An adapter is suspended and resumed from Desktop with Core receipts | desktop `an_adapter_is_suspended_and_resumed_through_core` | PENDING |
| Q4 | Read view unchanged (Spec 093) | desktop `every_plane_is_listed_and_empty_is_not_unavailable`, `errors_map_to_explicit_states` | PENDING |
| Q5 | Exact-head and post-main CI | - | PENDING |

Not claimed: terminal or data-moving Desktop actions, rendered-UI or assistive-technology qualification, clinical or release claims.
