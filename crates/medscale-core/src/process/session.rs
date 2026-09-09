//! In-process client session registry (Spec 018 READY_BASE).

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use medscale_contracts::envelopes::{AuthorityError, Capability};
use medscale_contracts::objects::{OpaqueId, VaultId};

#[derive(Debug, Clone)]
struct ClientSession {
    vault_id: VaultId,
    /// Lease holder that opened the session (FR-001; retained for audit identity).
    #[allow(dead_code)]
    holder_id: OpaqueId,
    granted: Vec<Capability>,
    expires_at_tick: u64,
    revoked: bool,
}

/// Monotonic-tick session store bound to vault lease holders.
#[derive(Debug, Default)]
pub struct SessionRegistry {
    inner: Mutex<SessionState>,
}

#[derive(Debug, Default)]
struct SessionState {
    clock: u64,
    next_seq: u64,
    sessions: HashMap<String, ClientSession>,
}

impl SessionRegistry {
    /// Creates an empty registry at tick 0.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn lock(&self) -> MutexGuard<'_, SessionState> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Opens a session; returns `(session_id, expires_at_tick)`.
    pub fn open(
        &self,
        vault_id: &VaultId,
        holder_id: OpaqueId,
        granted: Vec<Capability>,
        ttl_ticks: u64,
    ) -> (OpaqueId, u64) {
        let mut state = self.lock();
        state.next_seq = state.next_seq.saturating_add(1);
        let session_id = OpaqueId::new(format!("session-{}", state.next_seq));
        let expires_at_tick = state.clock.saturating_add(ttl_ticks);
        state.sessions.insert(
            session_id.as_str().to_owned(),
            ClientSession {
                vault_id: vault_id.clone(),
                holder_id,
                granted,
                expires_at_tick,
                revoked: false,
            },
        );
        (session_id, expires_at_tick)
    }

    /// Marks a session revoked (idempotent if unknown).
    pub fn revoke(&self, session_id: &OpaqueId) {
        let mut state = self.lock();
        if let Some(session) = state.sessions.get_mut(session_id.as_str()) {
            session.revoked = true;
        }
    }

    /// Validates a live session for vault + capability.
    pub fn validate(
        &self,
        session_id: &OpaqueId,
        vault_id: &VaultId,
        capability: Capability,
    ) -> Result<(), AuthorityError> {
        let state = self.lock();
        let Some(session) = state.sessions.get(session_id.as_str()) else {
            return Err(AuthorityError::SessionDenied);
        };
        if session.revoked {
            return Err(AuthorityError::SessionRevoked);
        }
        if state.clock >= session.expires_at_tick {
            return Err(AuthorityError::SessionExpired);
        }
        if &session.vault_id != vault_id {
            return Err(AuthorityError::SessionDenied);
        }
        if !session.granted.contains(&capability) {
            return Err(AuthorityError::SessionDenied);
        }
        Ok(())
    }

    /// Advances the monotonic clock by one tick (tests).
    pub fn tick(&self) {
        self.advance_ticks(1);
    }

    /// Advances the monotonic clock by `n` ticks (tests).
    pub fn advance_ticks(&self, n: u64) {
        let mut state = self.lock();
        state.clock = state.clock.saturating_add(n);
    }

    /// Current monotonic tick (tests / diagnostics).
    #[must_use]
    pub fn current_tick(&self) -> u64 {
        self.lock().clock
    }
}
