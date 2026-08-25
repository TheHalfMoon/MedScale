//! SQLite metadata for source visibility and migration journal.

use std::path::Path;

use medscale_contracts::objects::{AuthorityScopeId, DigestSha256, OpaqueId, RealmId};
use rusqlite::{Connection, params};
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
        if journal.finished_version < 1 {
            self.conn.execute(
                "INSERT OR REPLACE INTO migration_journal(version, state) VALUES (1, 'finished')",
                [],
            )?;
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
        let payload: Vec<_> = sources
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
        Ok(serde_json::to_vec(&payload).unwrap_or_default())
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
