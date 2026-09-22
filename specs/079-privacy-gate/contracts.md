# Contracts — Spec 079 Privacy Gate

Module: `crates/medscale-contracts/src/privacy_gate.rs`. Every enum has a
closed `as_str()`/`parse()` vocabulary. Every struct is
`#[serde(deny_unknown_fields)]`. No field holds a detected plaintext value.

## 1. Reused primitives

`OpaqueId`, `ObjectHeader`, `DigestSha256` (objects), `ProjectRevision` and
revision checks (Spec 074 `project_graph`), `DerivedSourceArtifact` and
`SourceRecord` (Spec 002), `EgressDataClass` (Spec 013 network; mapped, not
replaced).

## 2. Classification

```text
DataClass = LocalPhi | TeamProtected | ExternalDeidentified | Public
  restrictiveness: LocalPhi > TeamProtected > ExternalDeidentified > Public
ClassificationBasis = Declared | DefaultUnclassified | DeidReceipt
ArtifactClassification {
  header, project_id, artifact_id, data_class, basis,
  deid_receipt_id: Option<OpaqueId>,   // required iff basis == DeidReceipt
  revision }
```

`DefaultUnclassified` is never stored; Core returns it for an artifact with
no row. Declaring a class less restrictive than `LocalPhi` for a source
artifact requires an explicit caller action; a `DeidReceipt`-basis row is
written only by the transform path.

## 3. Detection

```text
SensitiveSpanKind = PersonName | Date | Identifier | Phone | Email |
  PostalAddress | PostalCode | Url | IpAddress | ModelEntity
RecognizerFamily = Deterministic | StructuredFhir | LocalModel
RecognizerIdentity { recognizer_id, version, family,
  model_pack_id: Option<OpaqueId> }        // Some only for LocalModel
SensitiveSpan { kind, start, end, recognizer_id }  // byte offsets, char
  boundaries, start < end; never serialized into receipts
RecognizerStatus = Completed | Unavailable | Failed
RecognizerResult { recognizer: RecognizerIdentity, status, span_count }
```

## 4. Policy and transform

```text
TransformOp = Redact | Tokenize | Generalize | Pseudonymize | Drop
  Redact      -> "[REDACTED:<KIND>]"
  Tokenize    -> "[<KIND>-<n>]"  (n = order of first appearance in this transform)
  Generalize  -> Date -> year only; PostalCode -> first 3 chars + "**";
                 IpAddress -> first octet + ".x.x.x"; other kinds -> Redact
  Pseudonymize-> "PSN-<12 hex>" keyed digest under the map key
  Drop        -> removed
PrivacyProfileStatus = Active | Revoked
PrivacyPolicyProfile { header, project_id, name, target_class
  (ExternalDeidentified | TeamProtected), rules: Vec<(SensitiveSpanKind,
  TransformOp)> (every kind exactly once), use_model_recognizer: bool,
  status, revision }
ResidualScanStatus = NoResidualDetectedByAdmittedRecognizers |
  ResidualDetected | Unavailable
ResidualScanResult { status, residual_span_count, recognizers:
  Vec<RecognizerResult> }
TransformOpCount { kind, op, count }
DeidReceiptStatus = Valid | Revoked
DeidReceipt { header, project_id, source_artifact_id, source_digest,
  output_artifact_id, output_digest, profile_id, profile_revision,
  recognizers: Vec<RecognizerResult>, op_counts: Vec<TransformOpCount>,
  pseudonym_map_id: Option<OpaqueId>, residual: ResidualScanResult,
  limitations: Vec<String> (fixed vocabulary), status, revision }
```

`DeidReceipt` is the persisted `PrivacyTransform` record: one transform, one
receipt, one output artifact. The plan (spans x ops) is computed in memory
and only its counts persist.

## 5. Pseudonym maps

```text
PseudonymMapStatus = Active | Revoked
PseudonymMapRef { header, project_id, key_account, entry_count, status,
  revision }       // key_account names a KeyStore entry; never the key
PseudonymEntry { map_id, pseudonym, sealed_value: WrappedBlob }  // storage-only
ReidentificationAudit { header, map_id, pseudonym, requested_by, reason,
  outcome: Returned | DeniedRevoked | DeniedUnknownPseudonym |
  DeniedKeyUnavailable }  // no plaintext
```

## 6. Egress

```text
EgressBoundary = ModelExternalDelegate | Browse | DataSourceExport |
  DataSourceWrite | Hub | Compute | RWorkspace | Connector | Extension |
  AnalyticsAdapter | NetworkBroker
EgressOutcome = Allow | Deny
EgressReason = AllowedPublic | AllowedDeidentified | AllowedTeamHub |
  DeniedLocalPhi | DeniedUnclassified | DeniedTeamProtectedOffHub |
  DeniedNoReceipt | DeniedReceiptRevoked | DeniedDigestMismatch |
  DeniedResidualDetected | DeniedResidualUnavailable | DeniedProfileRevoked
EgressDecision { header, project_id, artifact_id, boundary, data_class,
  basis, outcome, reason, deid_receipt_id: Option<OpaqueId> }
```

Mapping to Spec 013 `EgressDataClass`: `Public` -> `NonPhiMetadata`,
`ExternalDeidentified` -> `RedactedEvidence`; `TeamProtected` and
`LocalPhi` have no broker class and are refused before any broker call.

## 7. Bounds

```text
PROFILE_NAME_MAX_CHARS = 128
REID_REASON_MAX_CHARS = 512
TRANSFORM_INPUT_MAX_BYTES = 1_048_576
MAX_SPANS_PER_TRANSFORM = 20_000
PSEUDONYM_HEX_CHARS = 12
```

## 8. Freeze record (filled at T079-01)

```text
@@FREEZE@@
```
