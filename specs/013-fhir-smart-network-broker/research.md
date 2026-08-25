# Research: Spec 013 FHIR / SMART / Network Broker

**Date**: 2026-08-25  
**Spec**: `013-fhir-smart-network-broker`

## Decisions

### D1 — Network Broker is the sole product egress abstraction

- **Decision**: All product runtime online I/O goes through Core Host **Network Broker** capability. Request must carry destination, purpose, data-class, authorization. Decision + outcome → durable receipt. Workers, CLI, Desktop, future HF/pack clients, and partner adapters MUST NOT open independent sockets for product purposes.
- **Alternatives**: (a) Per-feature HTTP clients with audit hooks; (b) OS firewall claims only.
- **Rationale**: MASTER_BUILD_PLAN §12; constitution DEFAULT_DENY until broker; bypass tests are an exit gate.

### D2 — Fail-closed allowlist

- **Decision**: Broker consults an explicit **EgressAllowlist**. Miss / disabled / expired / purpose mismatch / data-class mismatch → **Deny** with typed error and receipt. **No transport send on Deny.** Empty allowlist ⇒ all Deny (DEFAULT_DENY-compatible).
- **Alternatives**: Default-allow localhost; warn-only unknown hosts.
- **Rationale**: Fail closed over permissive; privacy minimum disclosure.

### D3 — HTTP client pin (rustls, minimal)

| Crate | Version | Features / notes |
|---|---|---|
| `ureq` | **3.4.0** | `default-features = false`, features = `["rustls", "json"]` |
| `reqwest` | **not admitted** | Would pull tokio/hyper; deferred unless async proven necessary later |
| `native-tls` / OpenSSL product path | **not admitted** | rustls only for product TLS |
| `tokio` | **not admitted in 013** | Avoid async runtime for broker transport |

- **Rationale**: IMPLEMENTATION_DECISION_DEFAULTS — fewest transitive deps, memory-safe TLS, clear exit (swap transport behind broker trait). ureq 3.x default stack is rustls-based; Spec 013 narrows features explicitly.
- **Admission**: `docs/engineering/admissions/013-network-broker-ureq.md` **before** first Cargo dependency line.
- **Freeze**: Exact Cargo.lock hashes at implement; do not float minors without re-admission note.

### D4 — SMART stub / adapter; live partners gated

- **Decision**: Ship `SmartAuthAdapter` trait + fixture-backed stub (discovery / token / refresh envelopes). Ship `FhirPartnerAdapter` trait + fixture retrieve/search. Live partner hosts require EXTERNAL_GATES closeout (not this unit).
- **Alternatives**: Live sandbox SMART app registration in 013.
- **Rationale**: PRODUCTION_CREDENTIALS and PARTNER_EHR_NPHIES_ENDPOINT remain NOT_GRANTED; synthetic/fixture posture continues.

### D5 — Receipts via Spec 002 patterns

- **Decision**:
  - **ActionAuditRecord** (`kind: Audit`, optionally future ExternalActionIntent stubs without full 014 machine): broker allow/deny/attempt; `action` verbs like `network_broker.deny`, `network_broker.attempt`; `payload_digest` binds request body class; `detail` holds destination/purpose/data-class/decision codes (no secrets).
  - **EvaluationRecord** (`evidence_only: true`): FHIR profile / integrity / conformance oracle results; SMART fixture validation notes.
- **Alternatives**: New `NetworkReceipt` durable clinical class.
- **Rationale**: User/roadmap continuity with Spec 002; avoid inventing parallel authority objects.

### D6 — Bypass test strategy

- **Decision**: Combine (1) crate-level architecture/dependency assertions (only `medscale-network` or designated core module depends on `ureq`); (2) capability tests (Unauthorized without broker capability); (3) negative tests attempting direct client construction from CLI/worker test harnesses fail compile or lint policy.
- **Rationale**: Roadmap “bypass tests”; no uncontrolled provider client.

### D7 — FHIR profile / integrity / conformance evidence

- **Decision**: Reuse Spec 003 evidence-only attachment model. Profile/oracle fixture results → EvaluationRecord. Byte integrity via digest compare before accept/send. Inferno core **v1.4.2** / SMART test kit remain EXTERNAL_CONFORMANCE REFERENCE_ONLY; optional offline report digests under `evidence/013-fhir-smart-network-broker/` — not a live CI blocker.
- **Rationale**: SOURCE_ACQUISITION; FHIR never canonical DB.

### D8 — Data-class vocabulary (013 MVP)

```text
SyntheticFixture
PublicMetadata
InterchangeFhirR4
SmartTokenEnvelopeFixture
RealPhi          // always refuse in 013; REAL_PHI gate
SecretsOrCredentials  // never egress as body without future explicit gate
```

Allowlist rows enumerate permitted data-classes per destination. `RealPhi` rows forbidden while REAL_PHI NOT_AUTHORIZED.

### D9 — Purpose vocabulary (013 MVP)

```text
FhirPartnerRead
FhirPartnerSearch
SmartDiscovery
SmartTokenFixture
ConformanceProbe
PackAcquire          // reserved; Spec 015 only — deny unless 015 enables
```

Unknown purpose → Deny.

### D10 — Crate layout

- **Decision**:
  - New `crates/medscale-network` (or `medscale-core` module + thin crate) owning broker, allowlist, ureq transport behind `BrokerTransport` trait.
  - Adapters in `medscale-network` or `medscale-fhir` calling broker only.
  - Contracts types in `medscale-contracts`.
  - Extend doctor aggregation for network_broker axis.
- **Rationale**: Single place for ureq dependency; easy bypass enforcement.

### D11 — Doctor network_broker axis

- **Decision**: Replace Spec 006 `NotImplemented` with structured status: broker present, allowlist entry count (not contents secrets), last receipt freshness optional, posture `DefaultDenyExceptBroker`.
- **Rationale**: Continuity with Spec 006 doctor contract.

### D12 — Anti-scope (binding)

- No REAL_PHI authorization
- No live partner EHR/SMART requirement to close
- No MESC mutation / OpenMed runtime as authority
- No Spec 014 NPHIES / full action state machine
- No Spec 015 HF/pack online client
- No uncontrolled cloud provider SDK
- No reqwest/tokio/native-tls product admission in 013
- No general FHIRPath engine
- No telemetry/crash-upload egress

### D13 — Evidence archive layout

```text
evidence/013-fhir-smart-network-broker/
  BASELINE.md
  BYPASS_TESTS.md
  ALLOWLIST_DENY_MATRIX.md
  RECEIPT_SAMPLES.json
  SMART_FIXTURE_RECEIPTS.json
  FHIR_PROFILE_INTEGRITY.md
  CONFORMANCE_NOTES.md
  limitations.md
```

### D14 — Relationship to Spec 005/006

- Consumes Core Host facade, EncryptedVault durability for receipts, doctor surface.
- Does not reopen CLI wedge or Tauri decisions.
- Updates doctor network_broker placeholder only.

## Alternatives considered (summary)

| Topic | Chosen | Rejected |
|---|---|---|
| HTTP | ureq 3.4.0 + rustls | reqwest + tokio; native-tls |
| Policy | Fail-closed allowlist | Default-allow / warn-only |
| SMART | Stub + fixtures | Live partner in 013 |
| Receipts | ActionAuditRecord + EvaluationRecord | New NetworkReceipt class |
| Retry | No blind retry | Auto-retry on timeout |

## Open residuals (non-blocking)

- Exact allowlist persistence format (JSON policy file vs vault-config object)—choose simplest vault-scoped config at implement.
- Async transport reconsideration if Spec 015 proves need (re-admit behind same broker trait).
- Full ExternalActionIntent effect_state machine (Spec 014).
- Live Inferno runs in CI (optional evidence only).
