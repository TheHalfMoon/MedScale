# Clarifications: Spec 058

- Portable ZIP is a real distributable package format already admitted by the signing-prep packet; native OS installer formats remain unqualified.
- Reproducibility means deterministic package assembly from identical input binaries/head/platform/revision. It does not claim reproducible Rust compiler output.
- Lifecycle proof uses real packaged MedScale binaries in versioned temp installation slots; it is separate from Spec 048 vault migration/recovery proof.
- Baseline and candidate package revisions may contain identical binaries; their distinct manifest revisions exercise installer state transitions without fabricating application changes.
- Package verification must fail closed on missing, unexpected, or hash-mismatched files.
- `signed=false`, `notarized=false`, and `release_ready=false` remain mandatory.
- MESC is not involved.
