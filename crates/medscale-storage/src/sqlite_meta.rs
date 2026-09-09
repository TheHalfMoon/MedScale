//! SQLite metadata for source visibility, authority objects, and migration journal.

use std::path::Path;

use medscale_contracts::objects::{AuthorityScopeId, DigestSha256, OpaqueId, RealmId};
use rusqlite::{Connection, OptionalExtension, params};
use thiserror::Error;

use crate::claim::{ClaimError, assert_claim_path};
use crate::migrate::MigrationJournal;

/// Metadata store errors.
#[derive(Debug, Error)]
pub enum MetaError {
    #[error(transparent)]
    Claim(#[from] ClaimError),
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("not found")]
    NotFound,
    #[error("corrupt object body: {0}")]
    CorruptObjectBody(String),
    #[error("migration incomplete at version {0}")]
    MigrationIncomplete(u32),
}

/// Durable authority object row (Spec 016 schema v2).
#[derive(Debug, Clone)]
pub struct AuthorityObjectRow {
    pub object_id: String,
    pub object_class: String,
    pub realm_id: String,
    pub authority_scope_id: String,
    pub body_json: String,
    pub content_digest_hex: Option<String>,
    pub updated_seq: u64,
}

/// Row describing a durable source blob binding.
#[derive(Debug, Clone)]
pub struct SourceMeta {
    pub source_id: OpaqueId,
    pub realm_id: RealmId,
    pub authority_scope_id: AuthorityScopeId,
    pub digest: DigestSha256,
    pub byte_length: u64,
    pub media_type: String,
    pub visible: bool,
    pub resource_type: String,
}

/// SQLite-backed metadata (one connection per vault / process host).
#[derive(Debug)]
pub struct SqliteMetaStore {
    conn: Connection,
}

impl SqliteMetaStore {
    pub fn open(vault_root: &Path) -> Result<Self, MetaError> {
        let root = assert_claim_path(vault_root)?;
        std::fs::create_dir_all(&root)?;
        let db_path = root.join("meta.sqlite3");
        Self::open_at(&db_path)
    }

    /// Open metadata DB at an explicit path (EncryptedVault working copy).
    pub fn open_at(db_path: &Path) -> Result<Self, MetaError> {
        if let Some(parent) = db_path.parent() {
            let _ = assert_claim_path(parent)?;
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(db_path)?;
        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<(), MetaError> {
        self.conn.execute_batch(
            r"
            CREATE TABLE IF NOT EXISTS migration_journal (
              version INTEGER PRIMARY KEY,
              state TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS sources (
              source_id TEXT PRIMARY KEY,
              realm_id TEXT NOT NULL,
              authority_scope_id TEXT NOT NULL,
              digest_hex TEXT NOT NULL,
              byte_length INTEGER NOT NULL,
              media_type TEXT NOT NULL,
              visible INTEGER NOT NULL,
              resource_type TEXT NOT NULL
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_sources_scope_digest
              ON sources(authority_scope_id, digest_hex);
            CREATE TABLE IF NOT EXISTS gc_marks (
              digest_hex TEXT PRIMARY KEY,
              epoch INTEGER NOT NULL
            );
            ",
        )?;
        let journal = self.migration_journal()?;
        if journal.started_version.is_some()
            && journal
                .started_version
                .is_some_and(|v| v > journal.finished_version)
        {
            // Incomplete migration must not silently continue as empty success.
            return Err(MetaError::MigrationIncomplete(
                journal.started_version.unwrap_or(0),
            ));
        }
        if journal.finished_version < 1 {
            self.conn.execute(
                "INSERT OR REPLACE INTO migration_journal(version, state) VALUES (1, 'finished')",
                [],
            )?;
        }
        let journal = self.migration_journal()?;
        if journal.finished_version < 2 {
            self.begin_migration(2)?;
            self.conn.execute_batch(
                r"
                CREATE TABLE IF NOT EXISTS authority_objects (
                  object_id TEXT PRIMARY KEY,
                  object_class TEXT NOT NULL,
                  realm_id TEXT NOT NULL,
                  authority_scope_id TEXT NOT NULL,
                  body_json TEXT NOT NULL,
                  content_digest_hex TEXT,
                  updated_seq INTEGER NOT NULL
                );
                CREATE INDEX IF NOT EXISTS idx_authority_objects_scope_class
                  ON authority_objects(authority_scope_id, object_class);
                CREATE TABLE IF NOT EXISTS store_state (
                  key TEXT PRIMARY KEY,
                  value TEXT NOT NULL
                );
                INSERT OR IGNORE INTO store_state(key, value) VALUES ('next_seq', '0');
                ",
            )?;
            self.finish_migration(2)?;
        }
        Ok(())
    }

    pub fn migration_journal(&self) -> Result<MigrationJournal, MetaError> {
        let mut stmt = self
            .conn
            .prepare("SELECT version, state FROM migration_journal ORDER BY version")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, u32>(0)?, row.get::<_, String>(1)?))
        })?;
        let mut finished = 0_u32;
        let mut started = None;
        for row in rows {
            let (version, state) = row?;
            if state == "finished" {
                finished = finished.max(version);
            } else if state == "started" {
                started = Some(version);
            }
        }
        Ok(MigrationJournal {
            finished_version: finished,
            started_version: started,
        })
    }

    pub fn begin_migration(&self, to: u32) -> Result<(), MetaError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO migration_journal(version, state) VALUES (?1, 'started')",
            params![to],
        )?;
        Ok(())
    }

    pub fn finish_migration(&self, to: u32) -> Result<(), MetaError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO migration_journal(version, state) VALUES (?1, 'finished')",
            params![to],
        )?;
        Ok(())
    }

    pub fn find_by_digest(
        &self,
        scope: &AuthorityScopeId,
        digest: &DigestSha256,
    ) -> Result<Option<SourceMeta>, MetaError> {
        let hex = hex_digest(digest);
        let mut stmt = self.conn.prepare(
            "SELECT source_id, realm_id, authority_scope_id, digest_hex, byte_length, media_type, visible, resource_type
             FROM sources WHERE authority_scope_id = ?1 AND digest_hex = ?2",
        )?;
        let mut rows = stmt.query(params![scope.as_opaque().as_str(), hex])?;
        match rows.next()? {
            Some(row) => Ok(Some(map_source_row(row)?)),
            None => Ok(None),
        }
    }

    pub fn insert_source(&self, meta: &SourceMeta) -> Result<(), MetaError> {
        self.conn.execute(
            "INSERT INTO sources(source_id, realm_id, authority_scope_id, digest_hex, byte_length, media_type, visible, resource_type)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                meta.source_id.as_str(),
                meta.realm_id.as_opaque().as_str(),
                meta.authority_scope_id.as_opaque().as_str(),
                hex_digest(&meta.digest),
                meta.byte_length as i64,
                meta.media_type,
                i64::from(meta.visible),
                meta.resource_type,
            ],
        )?;
        Ok(())
    }

    pub fn upsert_source(&self, meta: &SourceMeta) -> Result<(), MetaError> {
        self.conn.execute(
            "INSERT INTO sources(source_id, realm_id, authority_scope_id, digest_hex, byte_length, media_type, visible, resource_type)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(source_id) DO UPDATE SET
               realm_id=excluded.realm_id,
               authority_scope_id=excluded.authority_scope_id,
               digest_hex=excluded.digest_hex,
               byte_length=excluded.byte_length,
               media_type=excluded.media_type,
               visible=excluded.visible,
               resource_type=excluded.resource_type",
            params![
                meta.source_id.as_str(),
                meta.realm_id.as_opaque().as_str(),
                meta.authority_scope_id.as_opaque().as_str(),
                hex_digest(&meta.digest),
                meta.byte_length as i64,
                meta.media_type,
                i64::from(meta.visible),
                meta.resource_type,
            ],
        )?;
        Ok(())
    }

    pub fn get_source(&self, source_id: &OpaqueId) -> Result<SourceMeta, MetaError> {
        let mut stmt = self.conn.prepare(
            "SELECT source_id, realm_id, authority_scope_id, digest_hex, byte_length, media_type, visible, resource_type
             FROM sources WHERE source_id = ?1",
        )?;
        let mut rows = stmt.query(params![source_id.as_str()])?;
        let row = rows.next()?.ok_or(MetaError::NotFound)?;
        Ok(map_source_row(row)?)
    }

    pub fn list_sources(&self) -> Result<Vec<SourceMeta>, MetaError> {
        let mut stmt = self.conn.prepare(
            "SELECT source_id, realm_id, authority_scope_id, digest_hex, byte_length, media_type, visible, resource_type FROM sources",
        )?;
        let mapped = stmt.query_map([], map_source_row)?;
        let mut out = Vec::new();
        for row in mapped {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn clear_gc_marks(&self) -> Result<(), MetaError> {
        self.conn.execute("DELETE FROM gc_marks", [])?;
        Ok(())
    }

    pub fn mark_digest(&self, digest: &DigestSha256, epoch: u64) -> Result<(), MetaError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO gc_marks(digest_hex, epoch) VALUES (?1, ?2)",
            params![hex_digest(digest), epoch as i64],
        )?;
        Ok(())
    }

    pub fn is_marked(&self, digest: &DigestSha256) -> Result<bool, MetaError> {
        let mut stmt = self
            .conn
            .prepare("SELECT 1 FROM gc_marks WHERE digest_hex = ?1")?;
        let mut rows = stmt.query(params![hex_digest(digest)])?;
        Ok(rows.next()?.is_some())
    }

    pub fn snapshot_bytes(&self) -> Result<Vec<u8>, MetaError> {
        let sources = self.list_sources()?;
        let source_payload: Vec<_> = sources
            .iter()
            .map(|s| {
                serde_json::json!({
                    "source_id": s.source_id.as_str(),
                    "realm_id": s.realm_id.as_opaque().as_str(),
                    "authority_scope_id": s.authority_scope_id.as_opaque().as_str(),
                    "digest_hex": hex_digest(&s.digest),
                    "byte_length": s.byte_length,
                    "media_type": s.media_type,
                    "visible": s.visible,
                    "resource_type": s.resource_type,
                })
            })
            .collect();
        let objects = self.list_authority_objects()?;
        let object_payload: Vec<_> = objects
            .iter()
            .map(|o| {
                serde_json::json!({
                    "object_id": o.object_id,
                    "object_class": o.object_class,
                    "realm_id": o.realm_id,
                    "authority_scope_id": o.authority_scope_id,
                    "body_json": o.body_json,
                    "content_digest_hex": o.content_digest_hex,
                    "updated_seq": o.updated_seq,
                })
            })
            .collect();
        let payload = serde_json::json!({
            "schema_version": 2,
            "next_seq": self.get_next_seq()?,
            "sources": source_payload,
            "objects": object_payload,
        });
        Ok(serde_json::to_vec(&payload).unwrap_or_default())
    }

    pub fn get_next_seq(&self) -> Result<u64, MetaError> {
        let value: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM store_state WHERE key = 'next_seq'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        Ok(value.unwrap_or_else(|| "0".to_owned()).parse().unwrap_or(0))
    }

    pub fn set_next_seq(&self, next_seq: u64) -> Result<(), MetaError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO store_state(key, value) VALUES ('next_seq', ?1)",
            params![next_seq.to_string()],
        )?;
        Ok(())
    }

    pub fn list_authority_objects(&self) -> Result<Vec<AuthorityObjectRow>, MetaError> {
        let mut stmt = self.conn.prepare(
            "SELECT object_id, object_class, realm_id, authority_scope_id, body_json, content_digest_hex, updated_seq
             FROM authority_objects",
        )?;
        let mapped = stmt.query_map([], |row| {
            Ok(AuthorityObjectRow {
                object_id: row.get(0)?,
                object_class: row.get(1)?,
                realm_id: row.get(2)?,
                authority_scope_id: row.get(3)?,
                body_json: row.get(4)?,
                content_digest_hex: row.get(5)?,
                updated_seq: row.get::<_, i64>(6)? as u64,
            })
        })?;
        let mut out = Vec::new();
        for row in mapped {
            out.push(row?);
        }
        Ok(out)
    }

    /// Replace all authority objects and next_seq in one transaction (Spec 016 sync).
    pub fn replace_authority_snapshot(
        &self,
        objects: &[AuthorityObjectRow],
        next_seq: u64,
    ) -> Result<(), MetaError> {
        let tx = self.conn.unchecked_transaction()?;
        tx.execute("DELETE FROM authority_objects", [])?;
        for obj in objects {
            tx.execute(
                "INSERT INTO authority_objects(object_id, object_class, realm_id, authority_scope_id, body_json, content_digest_hex, updated_seq)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    obj.object_id,
                    obj.object_class,
                    obj.realm_id,
                    obj.authority_scope_id,
                    obj.body_json,
                    obj.content_digest_hex,
                    obj.updated_seq as i64,
                ],
            )?;
        }
        tx.execute(
            "INSERT OR REPLACE INTO store_state(key, value) VALUES ('next_seq', ?1)",
            params![next_seq.to_string()],
        )?;
        tx.commit()?;
        Ok(())
    }
}

fn map_source_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SourceMeta> {
    let digest_hex: String = row.get(3)?;
    Ok(SourceMeta {
        source_id: OpaqueId::new(row.get::<_, String>(0)?),
        realm_id: RealmId::new(row.get::<_, String>(1)?),
        authority_scope_id: AuthorityScopeId::new(row.get::<_, String>(2)?),
        digest: parse_hex(&digest_hex),
        byte_length: row.get::<_, i64>(4)? as u64,
        media_type: row.get(5)?,
        visible: row.get::<_, i64>(6)? != 0,
        resource_type: row.get(7)?,
    })
}

fn hex_digest(digest: &DigestSha256) -> String {
    digest
        .as_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn parse_hex(name: &str) -> DigestSha256 {
    let mut bytes = [0_u8; 32];
    for (i, chunk) in name.as_bytes().chunks(2).enumerate() {
        if i >= 32 {
            break;
        }
        let s = std::str::from_utf8(chunk).unwrap_or("00");
        bytes[i] = u8::from_str_radix(s, 16).unwrap_or(0);
    }
    DigestSha256::from_bytes(bytes)
}
