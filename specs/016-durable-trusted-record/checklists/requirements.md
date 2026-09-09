# Requirements checklist: Spec 016

- [x] User stories prioritize restart durability and atomic promotion
- [x] FR/SC cover full object graph, not sources-only
- [x] Invariants listed and mapped to protocol
- [x] ADR selects SQLite extension; encryption privacy deferred honestly to Q03
- [x] Field-by-field mapping from `InMemoryAuthorityStore` complete
- [x] Blob/metadata interruption matrix specified
- [x] Writer ownership distinct from lock-file presence
- [x] Backup/restore closure includes objects
- [x] Out of scope excludes MESC/PHI/Q04 sessions/UI
- [x] No second authority service
- [x] Synthetic-only fixtures required
- [ ] Implementation evidence linked per FR after T04–T07
- [ ] Two-process restart PASS recorded
- [ ] Failure suite PASS recorded

**Checklist result**: READY for analyze / T04 implementation. NOT READY to claim CLOSED_CANONICAL.
