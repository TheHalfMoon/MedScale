# Public source license — counsel / founder decision packet (Spec 037)

**Gate:** `PUBLIC_SOURCE_LICENSE_CHOICE` = `PENDING`  
**Cursor must not** choose the final SPDX identifier.

## What Cursor already completed

- Workspace crates remain `publish = false` / privately `UNLICENSED` until a public choice exists
- `cargo-deny` license policy for dependencies
- Spec 032 NOTICE inventory (`docs/legal/NOTICE_INVENTORY.md`, `evidence/032-privacy-probes-notice-perf/notice-inventory.json`)
- Attribution / third-party inventory preparation (dependency licenses via deny + NOTICE scaffold)

## Smallest explicit founder/legal choice

Choose **one** public SPDX expression for MedScale first-party source distribution, for example:

```text
OPTION_A = Apache-2.0
OPTION_B = MIT
OPTION_C = Apache-2.0 OR MIT
OPTION_D = other SPDX (specify exact expression)
OPTION_E = remain private / no public source release yet
```

Also confirm whether third-party NOTICE aggregation must ship in every binary package (recommended: YES).

## Required inputs from counsel/founder

1. Chosen SPDX expression (or OPTION_E)
2. Whether dual-licensing is intended
3. Any jurisdiction-specific notice text beyond SPDX
4. Confirmation that dependency licenses in the NOTICE inventory are acceptable for that choice

## Expected output

```text
SPDX_LICENSE_IDENTIFIER = <chosen>
NOTICE_REQUIRED_IN_PACKAGE = true|false
EFFECTIVE_DATE = <ISO date>
```

## How Cursor will verify

1. Set workspace `license` / crate metadata to the chosen SPDX (only after this packet is closed)
2. Update `EXTERNAL_GATES.md` row to `DECIDED`
3. Re-run cargo-deny / NOTICE generation
4. Bind choice into release manifest when packaging exists

## What this does **not** clear

- `RELEASE_READY`
- Signing / notarization
- MESC / partner / PHI gates
