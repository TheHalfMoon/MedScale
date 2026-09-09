# Research: Spec 022

## Q05 prior partial

PR #39 pinned GitHub Actions to immutable commit SHAs and set workflow `permissions: contents: read`. That work does not configure branch protection or claim RELEASE_READY.

## Locked builds

Cargo `--locked` fails if `Cargo.lock` is out of date relative to manifests. CI and documented verify paths should use it. `Cargo.lock` is already committed at repository root. Toolchain pin remains `rust-toolchain.toml` channel `1.97.1` (matches CI).

## RELEASE_READY requirements (Trusted V1 delivery plan)

Immutable source/tree + lock; qualified OS matrix; mandatory CI/reviews; reproducible package contents; SBOM (incl. native/model assets); rights/license decision; checksums/provenance/signing; migration/recovery proof; source-linked claims/limitations; no unresolved material findings. Current product state remains FALSE. Spec 022 only hardens prep evidence and honesty.

## OS matrix honesty

Windows + Linux: baseline CI (`ubuntu-latest`, `windows-latest`). macOS: unqualified. iOS/Android: scaffold/doctor axes only (Spec 009 READY_BASE). Host-independent Rust tests never imply app readiness.

## External gate

Branch protection / required status checks need repository owner settings authority. Observed historically as unprotected / empty rulesets. Record in EXTERNAL_GATES; do not mutate via API.
