# Plan: Spec 015 READY_BASE

Contracts + Core Host refuse path + doctor axis + dependency architecture test (no HF SDK). Online bytes never fetched in this unit.

## Decisions
| ID | Decision |
|---|---|
| D1 | READY_BASE closes with deny-by-default online path |
| D2 | No `huggingface_hub` / HF HTTP client dependency |
| D3 | Mobile constraints copied as typed fields from Spec 009 evidence |
| D4 | Future allow path must call Network Broker (not implemented until gate) |
