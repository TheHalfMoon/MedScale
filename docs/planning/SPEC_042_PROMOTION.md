# Spec 042 promotion — delivery-plan scale perf measurement

**Classification:** `EXISTING_Q05_RESIDUAL_ELIGIBLE_FOR_PROMOTION`  
**Not:** deferred advanced product.

## Authority
Specs 027/032 shipped harness with READY_BASE scale (64 events / 128KiB) and `budgets_claimed_met=false`.
Trusted V1 delivery plan targets (10k timeline / 1MiB FHIR) remain unmeasured at that scale.
Lexical 10k records still require corpus work — honestly not claimed in this unit.

## Scope
- Env `MEDSCALE_027_DELIVERY_PLAN_SCALE=1` → FHIR 1 MiB + elevated timeline (CI uses 1000; host may set `MEDSCALE_027_TIMELINE_EVENTS=10000`)
- Evidence under `evidence/042-perf-delivery-plan-scale/`
- Keep `budgets_claimed_met=false`; no RELEASE_READY
- Lexical 10k still limited (builtin corpus)
- Advanced product remains **043+**
