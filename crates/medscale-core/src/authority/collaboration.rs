//! Collaboration Substrate Core authority paths (Spec 076).
//!
//! Every mutation flows: actor/session -> realm/scope -> RoomMembership
//! visibility (collaboration-scope, independent of Core `Capability`) ->
//! validation -> expected revision -> transaction (primary row + its
//! `ActivityRecord`, same commit) -> typed result. Surfaces never touch
//! storage.
//!
//! T076-03 scope only: `ParticipantIdentity`, `Room`, `RoomMembership`.
//! Thread/message/task/note/approval/activity-read Core paths land in
//! T076-04 through T076-09.
//!
//! Audit split (see `contracts.md` section 14): `ParticipantIdentity` is
//! scope-level, not room-level, and has no room to chain an `ActivityRecord`
//! into, so participant register/revoke reuse the *existing* in-memory
//! `ActionAuditRecord` trail (same mechanism Spec 074 uses for
//! `project.create`). Every room-scoped mutation (room, membership, and
//! everything built on them in later slices) appends to the new
//! `collab_activity_records` hash chain instead, in the same transaction as
//! its primary row write.

use medscale_contracts::AUTHORITY_SCHEMA_VERSION;
use medscale_contracts::collaboration::{
    COLLAB_SCHEMA_VERSION, MembershipRole, ParticipantIdentity, ParticipantKind, ParticipantStatus,
    Room, RoomMembership, RoomStatus,
};
use medscale_contracts::envelopes::AuthorityError;
use medscale_contracts::objects::{AuthorityScopeId, ObjectHeader, OpaqueId, RealmId, VaultId};
use medscale_storage::{MetaError, SqliteMetaStore};

use super::store::{InMemoryAuthorityStore, StoredObject};
use crate::process::{LeaseRegistry, SessionRegistry};

fn meta_err(err: MetaError) -> AuthorityError {
    match err {
        MetaError::NotFound => AuthorityError::NotFound,
        MetaError::Conflict(message) => AuthorityError::Conflict { message },
        MetaError::UnsupportedSchema(message) => AuthorityError::UnsupportedSchema { message },
        MetaError::CorruptObjectBody(message) => AuthorityError::Corrupt { message },
        other => AuthorityError::Internal {
            message: other.to_string(),
        },
    }
}

fn header(realm: &RealmId, scope: &AuthorityScopeId, id: OpaqueId) -> ObjectHeader {
    ObjectHeader {
        id,
        schema_version: COLLAB_SCHEMA_VERSION,
        realm_id: realm.clone(),
        authority_scope_id: scope.clone(),
    }
}

/// Authenticated authority view for one request.
pub struct Collab<'a> {
    pub store: &'a mut InMemoryAuthorityStore,
    pub meta: &'a SqliteMetaStore,
    pub sessions: &'a SessionRegistry,
    pub leases: &'a LeaseRegistry,
    pub vault_id: &'a VaultId,
    pub realm: RealmId,
    pub scope: AuthorityScopeId,
    pub session_id: Option<OpaqueId>,
}

impl Collab<'_> {
    /// Resolves the request actor: session holder, else lease holder.
    fn actor(&self) -> Result<OpaqueId, AuthorityError> {
        if let Some(holder) = self
            .session_id
            .as_ref()
            .and_then(|session_id| self.sessions.holder_of(session_id))
        {
            return Ok(holder);
        }
        self.leases
            .holder(self.vault_id)
            .ok_or(AuthorityError::LeaseRequired)
    }

    /// Appends one scope-level audit row to the existing trail (participant
    /// lifecycle only; see module docs).
    fn audit(&mut self, action: &str, targets: Vec<OpaqueId>) -> Result<(), AuthorityError> {
        let actor = self.actor()?;
        let id = self.store.alloc_id("audit");
        let record = medscale_contracts::objects::ActionAuditRecord {
            header: ObjectHeader {
                id,
                schema_version: AUTHORITY_SCHEMA_VERSION,
                realm_id: self.realm.clone(),
                authority_scope_id: self.scope.clone(),
            },
            kind: medscale_contracts::objects::ActionAuditKind::Audit,
            actor,
            action: action.to_owned(),
            target_refs: targets,
            effect_state: None,
            payload_digest: None,
            detail: None,
        };
        self.store.insert(StoredObject::Audit(record));
        Ok(())
    }

    fn scoped_room(&self, id: &OpaqueId) -> Result<Room, AuthorityError> {
        let room = self.meta.get_room(id).map_err(meta_err)?;
        if room.header.realm_id != self.realm || room.header.authority_scope_id != self.scope {
            return Err(AuthorityError::WrongScope);
        }
        Ok(room)
    }

    fn scoped_participant(&self, id: &OpaqueId) -> Result<ParticipantIdentity, AuthorityError> {
        let participant = self.meta.get_participant(id).map_err(meta_err)?;
        if participant.header.realm_id != self.realm
            || participant.header.authority_scope_id != self.scope
        {
            return Err(AuthorityError::WrongScope);
        }
        Ok(participant)
    }

    /// Resolves the caller's own `ParticipantIdentity` via their session/
    /// lease holder id. `Unauthorized` when the caller has never registered
    /// a participant in this scope: every collaboration action requires
    /// registration first, including creating the first room.
    fn caller_participant(&self) -> Result<ParticipantIdentity, AuthorityError> {
        let holder_id = self.actor()?;
        self.meta
            .get_participant_by_holder(&self.scope, &holder_id)
            .map_err(meta_err)?
            .ok_or(AuthorityError::Unauthorized)
    }

    /// Resolves the caller's participant identity and proves they hold an
    /// active membership in `room_id`, scoped to this realm/scope. This is
    /// the collaboration-visibility check (`security.md` T3): independent
    /// of, and never a substitute for, the Core `Capability` gate already
    /// enforced by the facade before any `Collab` method runs.
    fn require_membership(
        &self,
        room_id: &OpaqueId,
    ) -> Result<ParticipantIdentity, AuthorityError> {
        self.scoped_room(room_id)?;
        let participant = self.caller_participant()?;
        let membership = self
            .meta
            .find_active_membership(room_id, &participant.header.id)
            .map_err(meta_err)?;
        if membership.is_none() {
            // Denied resolution must not reveal room existence beyond what
            // the scope check above already necessarily reveals via
            // NotFound/WrongScope; a member-less caller gets the same
            // Unauthorized a nonexistent room would not otherwise imply.
            return Err(AuthorityError::Unauthorized);
        }
        Ok(participant)
    }

    // ----- participants -----

    /// Registers a participant for `holder_id`, or returns the existing one
    /// (idempotent on `(scope, holder_id)`). `kind` is immutable once set;
    /// re-registering an existing holder with a different `kind` is a
    /// conflict, not a silent overwrite.
    pub fn register_participant(
        &mut self,
        holder_id: OpaqueId,
        kind: ParticipantKind,
        display_name: String,
        agent_profile_ref: Option<OpaqueId>,
    ) -> Result<ParticipantIdentity, AuthorityError> {
        if let Some(existing) = self
            .meta
            .get_participant_by_holder(&self.scope, &holder_id)
            .map_err(meta_err)?
        {
            if existing.kind != kind {
                return Err(AuthorityError::Conflict {
                    message: "participant kind is immutable once registered".to_owned(),
                });
            }
            return Ok(existing);
        }
        if kind != ParticipantKind::Agent && agent_profile_ref.is_some() {
            return Err(AuthorityError::InvalidArgument {
                message: "agent_profile_ref is only admitted for kind = agent".to_owned(),
            });
        }
        let id = self
            .meta
            .alloc_collab_id("collab-participant", "participant")
            .map_err(meta_err)?;
        let participant = ParticipantIdentity::new(
            header(&self.realm, &self.scope, id.clone()),
            holder_id,
            kind,
            display_name,
        )
        .map_err(|message| AuthorityError::InvalidArgument { message })?;
        self.meta
            .insert_participant(&participant, agent_profile_ref.as_ref())
            .map_err(meta_err)?;
        self.audit("participant.register", vec![id])?;
        Ok(participant)
    }

    /// Scoped participant read.
    pub fn get_participant(&self, id: &OpaqueId) -> Result<ParticipantIdentity, AuthorityError> {
        self.scoped_participant(id)
    }

    /// Revokes a participant (status only; `kind`/`holder_id` never change).
    pub fn revoke_participant(
        &mut self,
        id: &OpaqueId,
        expected: u64,
    ) -> Result<ParticipantIdentity, AuthorityError> {
        let current = self.scoped_participant(id)?;
        if current.status != ParticipantStatus::Active {
            return Err(AuthorityError::Conflict {
                message: "participant is not active".to_owned(),
            });
        }
        let revoked = self
            .meta
            .set_participant_status(id, expected, ParticipantStatus::Revoked)
            .map_err(meta_err)?;
        self.audit("participant.revoke", vec![id.clone()])?;
        Ok(revoked)
    }

    // ----- rooms -----

    /// Creates a Room scoped to an existing Project and grants the caller
    /// `Owner` membership. The caller must already be a registered
    /// participant (`register_participant` first).
    pub fn create_room(
        &mut self,
        project_id: OpaqueId,
        experiment_id: Option<OpaqueId>,
        name: String,
    ) -> Result<Room, AuthorityError> {
        let project = self.meta.get_project(&project_id).map_err(meta_err)?;
        if project.header.realm_id != self.realm || project.header.authority_scope_id != self.scope
        {
            return Err(AuthorityError::WrongScope);
        }
        let creator = self.caller_participant()?;
        let room_id = self
            .meta
            .alloc_collab_id("collab-room", "room")
            .map_err(meta_err)?;
        let room = Room::new(
            header(&self.realm, &self.scope, room_id.clone()),
            project_id,
            experiment_id,
            name,
        )
        .map_err(|message| AuthorityError::InvalidArgument { message })?;
        let actor_id = self.actor()?;
        let activity_id = self
            .meta
            .alloc_collab_id("collab-activity", "activity")
            .map_err(meta_err)?;
        self.meta
            .insert_room_with_activity(
                &room,
                header(&self.realm, &self.scope, activity_id),
                &actor_id,
            )
            .map_err(meta_err)?;
        let membership_id = self
            .meta
            .alloc_collab_id("collab-membership", "membership")
            .map_err(meta_err)?;
        let membership = RoomMembership::new(
            header(&self.realm, &self.scope, membership_id),
            room_id,
            creator.header.id,
            MembershipRole::Owner,
        );
        let owner_activity_id = self
            .meta
            .alloc_collab_id("collab-activity", "activity")
            .map_err(meta_err)?;
        self.meta
            .insert_membership_with_activity(
                &membership,
                header(&self.realm, &self.scope, owner_activity_id),
                &actor_id,
            )
            .map_err(meta_err)?;
        self.meta.get_room(&room.header.id).map_err(meta_err)
    }

    /// Scoped, membership-gated Room read (`security.md` T3).
    pub fn get_room(&self, id: &OpaqueId) -> Result<Room, AuthorityError> {
        self.require_membership(id)?;
        self.scoped_room(id)
    }

    /// Lists rooms in one Project that the caller is an active member of.
    /// Visibility is intersected with membership, not merely Project scope
    /// (`security.md` T3): a caller with Project-level access but no
    /// `RoomMembership` sees nothing.
    pub fn list_rooms(
        &self,
        project_id: &OpaqueId,
        status: Option<RoomStatus>,
        limit: u32,
        after: Option<String>,
    ) -> Result<(Vec<Room>, Option<String>), AuthorityError> {
        let caller = self.caller_participant()?;
        let (rooms, next) = self
            .meta
            .list_rooms(&self.scope, project_id, status, limit, after.as_deref())
            .map_err(meta_err)?;
        let mut visible = Vec::with_capacity(rooms.len());
        for room in rooms {
            let member = self
                .meta
                .find_active_membership(&room.header.id, &caller.header.id)
                .map_err(meta_err)?;
            if member.is_some() {
                visible.push(room);
            }
        }
        Ok((visible, next))
    }

    /// Compare-and-swap room name (membership-gated; no dedicated activity
    /// event exists for rename in the frozen 076 vocabulary).
    pub fn rename_room(
        &self,
        id: &OpaqueId,
        expected: u64,
        name: String,
    ) -> Result<Room, AuthorityError> {
        self.require_membership(id)?;
        self.meta
            .update_room_name(id, expected, &name)
            .map_err(meta_err)
    }

    /// Archives a Room (membership-gated; requires `Owner` role).
    pub fn archive_room(&self, id: &OpaqueId, expected: u64) -> Result<Room, AuthorityError> {
        let caller = self.require_membership(id)?;
        let membership = self
            .meta
            .find_active_membership(id, &caller.header.id)
            .map_err(meta_err)?
            .ok_or(AuthorityError::Unauthorized)?;
        if membership.role != MembershipRole::Owner {
            return Err(AuthorityError::Unauthorized);
        }
        let actor_id = self.actor()?;
        let activity_id = self
            .meta
            .alloc_collab_id("collab-activity", "activity")
            .map_err(meta_err)?;
        let (room, _activity) = self
            .meta
            .archive_room_with_activity(
                id,
                expected,
                header(&self.realm, &self.scope, activity_id),
                &actor_id,
            )
            .map_err(meta_err)?;
        Ok(room)
    }

    // ----- room memberships -----

    /// Adds a member to a room (membership-gated: only an existing member
    /// may add another). Rejects a duplicate *active* `(room, participant)`
    /// pair rather than silently duplicating it.
    pub fn add_membership(
        &self,
        room_id: &OpaqueId,
        participant_id: &OpaqueId,
        role: MembershipRole,
    ) -> Result<RoomMembership, AuthorityError> {
        self.require_membership(room_id)?;
        let target = self.scoped_participant(participant_id)?;
        if target.status != ParticipantStatus::Active {
            return Err(AuthorityError::Conflict {
                message: "participant is not active".to_owned(),
            });
        }
        if self
            .meta
            .find_active_membership(room_id, participant_id)
            .map_err(meta_err)?
            .is_some()
        {
            return Err(AuthorityError::Conflict {
                message: "participant already has an active membership in this room".to_owned(),
            });
        }
        let membership_id = self
            .meta
            .alloc_collab_id("collab-membership", "membership")
            .map_err(meta_err)?;
        let membership = RoomMembership::new(
            header(&self.realm, &self.scope, membership_id),
            room_id.clone(),
            participant_id.clone(),
            role,
        );
        let actor_id = self.actor()?;
        let activity_id = self
            .meta
            .alloc_collab_id("collab-activity", "activity")
            .map_err(meta_err)?;
        self.meta
            .insert_membership_with_activity(
                &membership,
                header(&self.realm, &self.scope, activity_id),
                &actor_id,
            )
            .map_err(meta_err)?;
        self.meta
            .find_active_membership(room_id, participant_id)
            .map_err(meta_err)?
            .ok_or(AuthorityError::Internal {
                message: "membership vanished immediately after insert".to_owned(),
            })
    }

    /// Lists active memberships for one room (membership-gated).
    pub fn list_memberships(
        &self,
        room_id: &OpaqueId,
    ) -> Result<Vec<RoomMembership>, AuthorityError> {
        self.require_membership(room_id)?;
        self.meta.list_room_memberships(room_id).map_err(meta_err)
    }

    /// Removes a member (membership-gated). A member may always remove
    /// themself; removing another participant requires the caller to hold
    /// `Owner` role.
    pub fn remove_membership(
        &self,
        room_id: &OpaqueId,
        membership_id: &OpaqueId,
        expected: u64,
    ) -> Result<RoomMembership, AuthorityError> {
        let caller = self.require_membership(room_id)?;
        let memberships = self.meta.list_room_memberships(room_id).map_err(meta_err)?;
        let target = memberships
            .iter()
            .find(|m| m.header.id == *membership_id)
            .cloned()
            .ok_or(AuthorityError::NotFound)?;
        if target.participant_id != caller.header.id {
            let caller_membership = memberships
                .iter()
                .find(|m| m.participant_id == caller.header.id)
                .ok_or(AuthorityError::Unauthorized)?;
            if caller_membership.role != MembershipRole::Owner {
                return Err(AuthorityError::Unauthorized);
            }
        }
        let actor_id = self.actor()?;
        let activity_id = self
            .meta
            .alloc_collab_id("collab-activity", "activity")
            .map_err(meta_err)?;
        let (membership, _activity) = self
            .meta
            .remove_membership_with_activity(
                membership_id,
                expected,
                header(&self.realm, &self.scope, activity_id),
                &actor_id,
            )
            .map_err(meta_err)?;
        Ok(membership)
    }
}
