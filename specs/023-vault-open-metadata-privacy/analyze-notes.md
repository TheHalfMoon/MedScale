# Analyze notes: Spec 023

## Coverage

FR-001–FR-008 mapped to tasks T02–T08. Aligns with constitution: local-first, fail-closed keys, no PRIVATE_DATA_READY inflation.

## Risks checked

- Feature collision `bundled` vs sqlcipher: resolved by workspace-wide switch (ADR-023-001).
- False PRIVATE_DATA_READY: doctor defaults + LIMITATIONS + EXTERNAL_GATES note.
- Key leakage: VaultDek ZeroizeOnDrop; pragma_update for key; no logging.
- SyntheticVault breakage: unkeyed plaintext-compatible open retained.

## Qualification

Exact-head `cargo fmt`, `clippy -D warnings`, `cargo test --workspace --locked` required before converge.
