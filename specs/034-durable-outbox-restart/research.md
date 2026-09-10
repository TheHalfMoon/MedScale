# Research — Spec 034

Outbox is a derived projection of `ActionAuditRecord` (`ExternalActionIntent`). Spec 016 already persists `object_class=audit` when SyntheticVault is open. Spec 034 qualifies restart reload + UNKNOWN reconcile; does not invent a separate outbox table or live transport.
