# MedScale Research OS Definition of Ready

**Status:** Planning candidate.

A Research OS candidate unit is not ready for canonical promotion merely because the roadmap names it.

Before promotion, the next unit must have:

1. live-main base SHA and clean authority chain;
2. explicit dependency proof;
3. bounded in/out scope;
4. affected crates/files/interfaces identified;
5. threat model delta;
6. data/privacy classification;
7. donor/dependency choices either closed or expressed as measurable alternatives;
8. migration/recovery plan;
9. acceptance tests and evidence artifacts;
10. performance/resource envelope where relevant;
11. CLI/Desktop parity impact;
12. rollback/removal path for experimental infrastructure;
13. no hidden requirement for a MedScale-hosted cloud;
14. no unresolved question that changes the authority or privacy model.

The first promoted unit should be the smallest dependency-ordered foundation needed by the rest of the program, not the most visually exciting feature.
