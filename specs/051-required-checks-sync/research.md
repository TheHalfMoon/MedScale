# Spec 051 research

Live `ci.yml` top-level job `name:` values:

1. `rust (ubuntu-latest)` (matrix)
2. `rust (windows-latest)` (matrix)
3. `rust (macos-latest)` (matrix)
4. `perf delivery-plan scale (windows)` (Spec 042)
5. `cargo-deny`
6. `supply-chain policy present`

Spec 037 packet listed 1–3, 5–6 only. Spec 042 added job 4 without refreshing the
owner packet → documentation drift residual.

Note: OS privacy probes run inside the rust matrix tests; there is no separate
`privacy probes (windows)` workflow job.
