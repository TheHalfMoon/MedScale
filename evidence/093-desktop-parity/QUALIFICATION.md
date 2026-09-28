# Qualification — Spec 093 Research OS Desktop parity (read-only slice)

`PASS` is recorded only for rows that exact-head CI ran green on Linux,
Windows and macOS (see `EXACT_HEAD_QUALIFICATION.md`, written with the
closure); until then rows are `PENDING`.

| # | Requirement | Tests | Status |
|---|---|---|---|
| Q1 | Every Research OS plane is listed for the active project through Core; a fresh vault reports empty or unavailable, never data | desktop `every_plane_is_listed_and_empty_is_not_unavailable` | PENDING |
| Q2 | Core errors map to explicit states (denied, none, corrupt, unavailable) | desktop `errors_map_to_explicit_states` | PENDING |
| Q3 | Core read for adapters used by Desktop | core `a_write_is_intended_sent_and_confirmed_with_receipts` (adapter list assertion) | PENDING |
| Q4 | Exact-head and post-main CI | - | PENDING |

Not claimed: Desktop actions for Research OS planes, rendered-UI or assistive-technology qualification, clinical or release claims.
