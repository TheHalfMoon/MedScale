# Spec 079 Promotion — Privacy Gate

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Promotion date:** 2026-09-23
**Canonical base:** `cf8731e497273633efe2968ade4cbdd6523e5595`
**Target branch:** `spec/079-privacy-gate`

## Authority

The founder's standing continuation directive (reaffirmed 2026-09-22 with the
review-policy amendment) requires recomputing the dependency graph after each
closure and promoting the next dependency-ready Research OS unit without
routine per-spec approval. `IMPLEMENTATION_AUTHORITY.md` remains active.

Live verification at promotion time:

- Spec 078 is `CLOSED_CANONICAL`: final head `bd06f6d` passed exact-head run
  `35777530037` (6/6); PR #135 merged as `bf400e8`; post-merge main run
  `35793707525` passed 6/6. Closure bookkeeping PR #136 passed exact-head run
  `35796883084` (6/6 on `9188efb`) and merged as `cf8731e`; its post-merge
  main run `35799605433` passed 6/6.
- Spec 077 is `CLOSED_CANONICAL` (PR #133 merged as `ce8a40d`; closure PR #134
  merged as `ff2e677`).
- Spec 074 is `CLOSED_CANONICAL` (see `BUILD_QUEUE.md`).
- All ids above were read live with `gh pr view` / `gh run view` on
  2026-09-23, not carried from memory.

Dependency proof: `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` and
`RESEARCH_OS_EXECUTION_ROADMAP.md` both number Privacy Gate **079**, with hard
dependency **074 + 077** (both `CLOSED_CANONICAL`); 078 is not a hard
prerequisite and is also closed. Every later candidate (080 Governed Browse,
081 AudioFlow, 082 Analytics Gate, 083 Canvas, 084 Hub, 085 Compute, 086+)
depends on 079, so 079 is the only dependency-ready candidate.

Numbering proof: no `specs/079-*` package, no `SPEC_079_PROMOTION.md` and no
079 code exist on the base (`git ls-tree -r --name-only origin/main | grep -i
079` returns only historical evidence paths unrelated to this unit).

Review policy: `FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md` applies. There
is no external reviewer; deterministic qualification, the exact-range scope
record, exact-head CI and post-main CI are the gate.

## Authorized scope

The minimum governed, local-first privacy boundary described in
`specs/079-privacy-gate/spec.md`:

- `DataClass` {`LocalPhi`, `TeamProtected`, `ExternalDeidentified`, `Public`}
  with per-artifact classification inside a Project. An artifact without a
  classification is treated as `LocalPhi` (fail closed), and that basis is
  reported explicitly, never hidden;
- `PrivacyPolicyProfile`: an explicit, versioned, revocable rule set mapping
  every `SensitiveSpanKind` to one `TransformOp` (redact, tokenize,
  generalize, pseudonymize, drop). A profile that leaves a kind unmapped is
  invalid;
- three recognizer families, all local:
  1. deterministic pattern recognizers (email, phone, date, identifier,
     URL, IP address, labelled/honorific person names, postal code, street
     address), covering English, Spanish, French and German value formats
     (dates, identifiers, addresses, honorifics) inside English-language
     synthetic notes;
  2. a structured FHIR-aware recognizer (Patient name, birthDate,
     identifier, telecom, address; other resources' free-text fields fall
     back to the deterministic recognizer);
  3. the already-admitted local ONNX token-classifier Pack (Spec 069) as a
     model recognizer whose spans carry kind `ModelEntity`. The fixture Pack
     is **not** a qualified PHI recognizer and is recorded as such;
- `PrivacyTransform` + `DeidReceipt`: an immutable transform that reads a
  source artifact, writes a **new** derived artifact (the source is never
  overwritten), and records source/output digests, profile revision,
  recognizer identities and versions, model Pack identity when used,
  per-kind/per-op counts, pseudonym-map reference, residual scan and fixed
  limitations. Receipts never contain the detected plaintext;
- `ResidualScanResult`: all recognizers re-run on the output. Outcomes are
  `NoResidualDetectedByAdmittedRecognizers`, `ResidualDetected`, or
  `Unavailable`. No outcome claims PHI/PII is absent;
- reversible pseudonymization with key separation: pseudonyms are keyed
  digests; the reverse map is sealed (AEAD, `medscale-keys`) under a
  per-map key held in the `KeyStore` under its own account, never in the
  vault metadata. Re-identification is a separate capability that writes an
  audit row before returning anything. Revoking a map destroys its key;
- `EgressDecision`: a single Core policy function that every current and
  future boundary (model external delegate, Browse, data-source export and
  write, Hub, Compute, R workspace, connector, extension, analytics adapter,
  network broker) must call. It fails closed and persists every decision;
- revocation of profiles and receipts; a revoked receipt makes later egress
  of its output deny;
- CLI vertical slice and a native Desktop Privacy route, both Core-backed;
- storage schema v7 -> v8 (additive), backup/restore, consistency checks;
- synthetic multi-locale benchmark corpus with class-wise counts (found,
  missed and extra detections per kind), not a safety claim.

### Language decision for fixtures (recorded, binding on T079-03)

The roadmap asks for "synthetic multilingual" privacy fixtures. The founder
directive of 2026-09-22 requires English for all repository content without
exception. The two are reconciled as follows: fixtures are English-language
synthetic notes that carry non-English value formats (for example "12 de
marzo de 2024", "5. März 2022", "Calle Mayor 12, 28013 Madrid"), and the
recognizers hold the month names, labels and honorifics they need as data.
Full non-English narrative fixtures are a recorded residual that needs an
explicit founder decision; they are not built in this spec.

### Key custody decision

Pseudonym map keys live only in a `medscale_keys::KeyStore`. `CoreFacade`
defaults to a process-memory store (fail closed: a lost key only blocks
re-identification). `CliSession` installs `medscale_keys::select_keystore()`
(OS keyring when available) before its first key-using call, because each
CLI command is its own process. Key account names carry a random suffix so
two vaults that reuse a vault id and map id cannot share or overwrite keys.

## Explicitly not authorized

- real PHI or any claim that automation proves PHI/PII absence;
- HIPAA/GDPR/PDPL de-identification compliance claims;
- any live network egress; 079 decides, it never sends. The existing Spec 013
  broker keeps its own allowlist; 079 adds the data-class decision that later
  specs must call before any broker call;
- Specs 080-092 (Browse, AudioFlow, Analytics, Canvas, Hub, Compute, R,
  extensions, adapters, federation);
- admitting new model Packs or a new inference engine;
- new third-party dependencies (hand-written recognizers; HMAC-SHA256 built
  on the workspace `sha2`; sealing via existing `medscale-keys`);
- jurisdiction-specific legal profiles hard-coded in Core types
  (`RESEARCH_OS_DECISION_RESOLUTION_REGISTER.md` Q15);
- non-English narrative fixtures and non-Latin-script fixtures (limitation
  recorded; see the language decision above);
- modification of any Spec 074-078 contract, table or Core function beyond
  additive wiring;
- MESC work.

## Mandatory architecture constraints

1. Core owns every classification, transform, receipt, pseudonym and egress
   decision. CLI/Desktop call Core only.
2. Source artifacts are immutable; outputs are new `DerivedSourceArtifact`s.
3. Unknown/unclassified is `LocalPhi`. A recognizer that is unavailable or
   fails makes the residual scan `Unavailable`, and egress then denies.
4. Egress to any external boundary requires `Public`, or
   `ExternalDeidentified` backed by a non-revoked receipt whose residual
   scan is `NoResidualDetectedByAdmittedRecognizers` and whose output digest
   still matches. `TeamProtected` may go only to the `Hub` boundary.
   `LocalPhi` never leaves.
5. No plaintext detected value is written to receipts, decisions, audit
   rows, logs or errors.
6. The pseudonym key never enters vault metadata, backups or logs.
7. Additive migration only; pre-079 identities are preserved.
8. Every workflow works with the network disabled.

## Implementation order

```text
T079-00 Live truth and baseline
T079-01 Contracts freeze + invariant tests
T079-02 Storage schema v8, migration, backup/restore, consistency
T079-03 Recognizers (deterministic, FHIR-aware, model) + synthetic corpus
T079-04 Classification + policy profiles through Core, envelope, CLI
T079-05 Transform + DeidReceipt + residual scan through Core
T079-06 Pseudonym maps, key separation, re-identification audit, revocation
T079-07 EgressDecision policy + boundary hooks
T079-08 Native Desktop Privacy route + CLI parity
T079-09 Qualification and closure
```

## Frozen acceptance requirements

Spec 079 closes only when these are proven on the exact candidate head:

1. Classification survives reopen; unclassified reports `LocalPhi` with basis
   `DefaultUnclassified`.
2. A profile mapping every kind is accepted; an incomplete one is refused
   before any write; revoked profiles cannot be used.
3. On the synthetic multi-locale corpus, the transform writes a new derived
   artifact, leaves the source bytes and digest unchanged, and the receipt
   binds both digests, the profile revision and recognizer versions.
4. The receipt, decisions and audit rows contain none of the corpus's
   sensitive values (automated scan in tests).
5. Residual scan re-runs every recognizer; an unavailable recognizer yields
   `Unavailable`, never "clean".
6. Pseudonyms are stable within a map and differ across maps;
   re-identification needs its own capability, writes an audit row first,
   and fails after map revocation. The key is absent from metadata and
   backups.
7. Egress: `LocalPhi` always denies; `ExternalDeidentified` allows only with
   a valid receipt; revoked receipt, tampered output, residual detected or
   unavailable all deny; every decision is persisted.
8. Pre-079 vaults migrate v7 -> v8, reopen and restore; tampered 079 backup
   rows are refused.
9. CLI and Desktop results come from Core.
10. Dependency direction holds; no new dependency; no network.
11. Exact-head required CI and post-main CI pass.

## Completion rule

`CLOSED_CANONICAL` only after merge on a green exact head and recorded
post-main verification. Closure of 079 does not authorize 080; the queue is
recomputed and the next unit promoted separately.
