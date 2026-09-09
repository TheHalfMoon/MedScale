# Plan: Spec 023 Vault Open-Metadata Privacy

1. Spec Kit package (specify/clarify/plan/research/ADR/data-model/contracts/quickstart/checklist/tasks/analyze).
2. Design: **workspace-wide SQLCipher** via rusqlite `bundled-sqlcipher-vendored-openssl` (replaces `bundled`). SyntheticVault / writer-lock remain **plaintext-compatible** (no `PRAGMA key`). EncryptedVault derives SQLCipher key from VaultDek.
3. Migrate EncryptedVault `SqliteMetaStore` open path to apply key before migrate/read/write; keep AES-GCM `meta.sealed` at-rest seal + Spec 017 wipe.
4. Doctor: add `open_work_page_encrypted`; set `sqlcipher_enabled=true`; keep `private_data_ready=false`.
5. Tests: wrong-key fail-closed; round-trip; open-work negative scan; SQLCipher version string; doctor honesty.
6. Update admission `005-sqlcipher-rusqlite.md` with embedded version evidence.
7. Evidence `evidence/023-vault-open-metadata-privacy/{SUMMARY,LIMITATIONS}.md`.
8. BUILD_QUEUE / SPECKIT_MASTER_ROADMAP_V2 / START_HERE / EXTERNAL_GATES (OS key/snapshot note; do not clear REAL_PHI).
9. Gates: `cargo fmt`; clippy `-D warnings`; `cargo test --workspace --locked` with `CARGO_TARGET_DIR=D:\medscale-target`.

## Architecture

```text
VaultDek (32B, ZeroizeOnDrop)
  -> SQLCipher PRAGMA key (hex passphrase material; never logged)
  -> page-encrypted meta.work.sqlite3 while unlocked
  -> on close: seal bytes to meta.sealed (AES-GCM) + wipe work/sidecars (017)
```

SyntheticVault continues as engineering plaintext path (no privacy claim).
