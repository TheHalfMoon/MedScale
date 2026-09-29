# Research results ledger

This ledger records successful **and failed** paper-evaluation attempts. A failed or methodologically non-qualifying run is retained rather than rewritten into a successful result.

## Pilot P0 — GitHub Actions run 36640758384

- Research branch head associated with the PR: `3484d242a1fc505e1f10372914b0e0636ce30702`
- Scientific base: `1e2b7d94e970256b38bda15fa91f62bc397e825a`
- Workflow: `paper-evaluation`
- Overall result: `FAILED / NON-QUALIFYING PILOT`
- Qualification warning: the initial PR workflow used the default `actions/checkout` behavior and therefore executed GitHub's synthetic PR merge revision (`cba417220d9c160e68f22a07515700bc405403a2`), not the research branch head directly. Because the paper requires exact-revision binding, P0 is retained as pilot evidence only.

### Observed outcomes

| Experiment | Ubuntu | macOS | Windows | Interpretation |
|---|---|---|---|---|
| A0 contract provenance controls | PASS | PASS | PASS | Seven tests completed successfully on each hosted OS in P0, but the run is not promoted to final paper evidence because checkout was the synthetic merge ref. |
| A1 Core provenance binding | PASS | PASS | INFRASTRUCTURE FAILURE BEFORE TEST EXECUTION | Six tests completed successfully on Ubuntu and macOS. Windows failed during native dependency build because the workflow forced Git Bash, which selected an incompatible Perl path for the vendored OpenSSL build. This is not counted as an A1 assertion failure. |
| B0 authority/effect conformance | 4/6, HARNESS FAILURE | 4/6, HARNESS FAILURE | NOT EXECUTED | The two failures were caused by the research harness omitting `Capability::ListOutbox` in sessions that subsequently attempted `ListOutbox`. The other four B0 cases passed on Ubuntu/macOS. The missing grant is a harness setup defect, not evidence of a MedScale product defect. |

### Method defects discovered by P0

1. **Synthetic merge checkout:** default PR checkout did not satisfy the paper's exact-head requirement.
2. **Windows shell contamination:** forcing `shell: bash` bypassed the canonical Windows toolchain environment and caused a Perl/OpenSSL infrastructure failure.
3. **B0 harness capability omission:** two observation steps lacked the observation capability needed by the test itself.
4. **Evidence loss on failure:** checksum and artifact-upload steps were skipped after an experiment step failed.

### Repairs applied after P0

- explicitly checkout `github.event.pull_request.head.sha` for pull-request research runs;
- use a cross-platform Python evidence runner instead of forcing Bash for Cargo experiment steps;
- grant `ListOutbox` only to the B0 cases that need to inspect the outbox, without granting the promotion capability under test;
- run each experiment, hashing, and artifact upload with failure-preserving workflow semantics so partial raw evidence survives future failures;
- record both actual checkout SHA and GitHub event SHA in the environment manifest.

### Claim boundary

P0 may be cited in the research process as evidence that the evaluation harness itself was falsifiable and repaired. It must **not** be used as the manuscript's final cross-platform conformance result. Final result rows require a later exact-head run with retained raw artifacts and checksums.
