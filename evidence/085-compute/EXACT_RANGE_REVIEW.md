# Exact-Range Scope Record — Spec 085

A deterministic scope record under
`docs/planning/FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`. Not a semantic
review; no external reviewer ran or is required. The deterministic security
challenge is in `SECURITY.md`.

```text
BASE_SHA   = __BASE__ (origin/main, Spec 084 closure)
RANGE_HEAD = __HEAD__
PLATFORM   = Windows 11 workstation, git for Windows, 2026-09-26
```

| Check | Command (bash) | Result |
|---|---|---|
| Files outside authorized scope | `git diff --name-only $B $H` filtered to `crates/medscale-(contracts\|storage\|core\|cli)/`, `docs/planning/(BUILD_QUEUE\|SPEC_085_PROMOTION).md`, `evidence/085-compute/`, `specs/085-compute/` | __SCOPE__ |
| Network crate | `git diff --name-only $B $H -- crates/medscale-network` | __NET__ |
| Dependency / CI / toolchain manifests | `git diff --name-only $B $H -- '*Cargo.toml' Cargo.lock deny.toml supply-chain .github rust-toolchain.toml scripts` | only `crates/medscale-contracts/Cargo.toml` (two `[[bin]]` targets); no dependency added; `Cargo.lock` unchanged |
| Network APIs in 085 modules | `grep -nE "TcpStream\|TcpListener\|UdpSocket\|ureq\|reqwest\|hyper::\|medscale_network\|std::net"` over the 085 source files | none |
| Process launch | `grep -n "Command::new"` over the 085 source files | one: `compute_supervisor.rs` (the resolved worker only, cleared environment) |
| Worker linkage | `crates/medscale-contracts/src/bin/compute_worker.rs` imports | `medscale_contracts` and `std` only |
| CLI reaching storage, SQLite or process APIs | `grep -nE "medscale_storage\|rusqlite\|std::process"` over `medscale-cli/src/compute.rs` | none |
| `unwrap`/`expect`/`panic!`/`unreachable!` and print calls outside tests (contracts, storage, Core 085 modules, worker) | awk scan before `#[cfg(test)]` | none |
| Added `unsafe` | `git diff $B $H -- 'crates/*.rs' \| grep '^+' \| grep -cE 'unsafe \{\|unsafe fn'` | 0 |
| Whitespace and line endings | `git -c core.whitespace=cr-at-eol diff --check $B $H`; `git diff --numstat $B $H` (no whole-file rewrites) | clean |
| Format / dependency direction | `cargo fmt --all -- --check`; `scripts/check-dependency-direction.ps1` | clean; passed |
| History | forward merges of main only; no rebase, no force-push | verified |

## Defects found and fixed on this branch

| Head | Found by | Defect | Fix |
|---|---|---|---|
| 6b88ad8 | CI (Clippy) | collapsible `if` in manifest validation | `0bbfaed` |
| 0bbfaed | exact-range review | manual loop counter in the consistency check (would fail Clippy) | `7adf90f` |
| 7adf90f | exact-range review | a scripted edit rewrote `medscale-cli/src/main.rs` from CRLF to LF (1,623-line diff) | `e26a773` |
| 7adf90f | CI (Clippy) | assertions on constants in schema tests | `7560ad5` |
