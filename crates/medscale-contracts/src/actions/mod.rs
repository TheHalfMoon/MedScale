//! Controlled external-action contracts (Spec 014 READY_BASE).

use serde::{Deserialize, Serialize};

use crate::objects::{DigestSha256, EffectState, OpaqueId};

/// Doctor axis for controlled actions / outbox.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlledActionsDoctorStatus {
    pub present: bool,
    pub outbox_enabled: bool,
    pub unknown_blind_retry: bool,
    pub nphies_authorized: bool,
    pub payload_digest_required: bool,
}

impl ControlledActionsDoctorStatus {
    /// Spec 014 READY_BASE defaults.
    #[must_use]
    pub fn ready_base() -> Self {
        Self {
            present: true,
            outbox_enabled: true,
            unknown_blind_retry: false,
            nphies_authorized: false,
            payload_digest_required: true,
        }
    }
}

/// Create request for a durable external-action intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateExternalActionIntentRequest {
    pub actor: OpaqueId,
    pub action: String,
    pub target_refs: Vec<OpaqueId>,
    /// Exact approved payload identity; required for approval binding.
    pub payload_digest: DigestSha256,
}

/// One outbox row (rebuildable projection of intent state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutboxEntry {
    pub action_id: OpaqueId,
    pub action: String,
    pub effect_state: EffectState,
    pub payload_digest: DigestSha256,
}

/// NPHIES adapter posture under READY_BASE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NphiesAdapterPosture {
    DeferredPendingWorkflowEvidence,
}

/// NPHIES invoke request (always gated in READY_BASE).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NphiesInvokeRequest {
    pub workflow_id: String,
    pub payload_digest: DigestSha256,
}
