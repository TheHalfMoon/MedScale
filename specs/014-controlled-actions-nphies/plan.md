# Plan: Spec 014 READY_BASE

Contracts + Core Host capabilities + doctor axis + fixture tests. No live partner egress.

## Decisions
| ID | Decision |
|---|---|
| D1 | READY_BASE closes without NPHIES workflow evidence |
| D2 | Payload digest required at intent create; immutable thereafter |
| D3 | NPHIES capability always ExternalGateRequired in this unit |
| D4 | Reuse EffectState + effects::transition; harden CreateExternalActionIntent + ListOutbox |
