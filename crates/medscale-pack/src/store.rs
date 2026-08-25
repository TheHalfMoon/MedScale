//! In-memory / path-backed admitted pack store.

use medscale_contracts::objects::OpaqueId;
use medscale_contracts::packs::{PackManifestV0, PackPromotionState};

/// Admitted pack registry (Core Host owned).
#[derive(Debug, Default, Clone)]
pub struct PackStore {
    packs: Vec<PackManifestV0>,
}

impl PackStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, manifest: PackManifestV0) {
        if let Some(existing) = self
            .packs
            .iter_mut()
            .find(|p| p.pack_id == manifest.pack_id)
        {
            *existing = manifest;
        } else {
            self.packs.push(manifest);
        }
    }

    #[must_use]
    pub fn list(&self) -> Vec<PackManifestV0> {
        self.packs.clone()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.packs.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.packs.is_empty()
    }

    #[must_use]
    pub fn current_pack_id(&self) -> Option<OpaqueId> {
        self.packs
            .iter()
            .find(|p| p.promotion_state == PackPromotionState::Current)
            .map(|p| p.pack_id.clone())
    }

    /// Promote pack; fail closed if target invalid.
    pub fn promote(
        &mut self,
        pack_id: &OpaqueId,
        to: PackPromotionState,
    ) -> Result<PackManifestV0, &'static str> {
        let idx = self
            .packs
            .iter()
            .position(|p| &p.pack_id == pack_id)
            .ok_or("pack not found")?;
        match to {
            PackPromotionState::Current => {
                if self.packs[idx].promotion_state != PackPromotionState::Candidate
                    && self.packs[idx].promotion_state != PackPromotionState::Canary
                {
                    return Err("current requires candidate or canary");
                }
                for p in &mut self.packs {
                    if p.promotion_state == PackPromotionState::Current {
                        p.promotion_state = PackPromotionState::LastGreen;
                    }
                }
                self.packs[idx].promotion_state = PackPromotionState::Current;
            }
            PackPromotionState::Canary
            | PackPromotionState::LastGreen
            | PackPromotionState::Candidate => {
                self.packs[idx].promotion_state = to;
            }
        }
        Ok(self.packs[idx].clone())
    }
}
