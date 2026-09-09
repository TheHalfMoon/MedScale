# Quickstart: Spec 021 synthetic journey

Synthetic-only. No real PHI. Product runtime egress remains DEFAULT_DENY.

```powershell
$env:CARGO_TARGET_DIR = "D:\medscale-target"
$root = "D:\medscale-tmp\021-demo"
New-Item -ItemType Directory -Force -Path "$root\vault","$root\backup","$root\restore" | Out-Null

cargo run -p medscale-cli -- journey run `
  --vault-root "$root\vault" `
  --fixture fixtures/synthetic/fhir/r4/presentation/patient-golden.json `
  --backup-dir "$root\backup" `
  --restore-dir "$root\restore" `
  --json

cargo run -p medscale-cli -- doctor --json
```

Expect: journey report with `workflow_ready_base=true`, `release_ready=false`, and doctor `workflow` axis matching.
