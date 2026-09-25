# Spec 085 Promotion — MedScale Compute (local bounded worker foundation)

**Status:** `PROMOTED_IMPLEMENTATION_AUTHORIZED`
**Promotion date:** 2026-09-26
**Canonical base:** `__BASE__` (Spec 084 closure)
**Target branch:** `spec/085-compute`

## Authority

The founder's standing continuation directive requires promoting the next
dependency-ready Research OS unit after each closure without routine
approval. `IMPLEMENTATION_AUTHORITY.md` remains active
(`DEPENDENCY_INSTALL_AUTHORITY = YES_IF_OWNING_SPEC_ADMITS_AND_LOCKS_IT`;
this spec admits nothing).

Live verification at promotion time:

- Spec 084 is `CLOSED_CANONICAL`: final head `1c98478` passed exact-head
  run `36033616030` (6/6); PR #147 merged as `6caa698`; post-merge main run
  `36193194092` passed 6/6 (`evidence/084-hub/CLOSURE.md`).
- Specs 077 and 079 are `CLOSED_CANONICAL` (see `BUILD_QUEUE.md`).

Dependency proof: `RESEARCH_OS_V2_SPEC_IMPLEMENTATION_CONTRACTS.md` and
`RESEARCH_OS_EXECUTION_ROADMAP.md` number MedScale Compute **085**, hard
dependency **077 + 079** ("Local worker foundation does not require Hub").
Both are closed. 086 (R Workspace) and 087 (Community Extensions) depend on
085, so no other unit is dependency-ready ahead of it.

Sandbox truth (live source, `medscale-contracts/src/os_sandbox`): every
per-OS mechanism is `ReadyBaseMeasured`; `platform_qualified=false`;
`try_apply_os_sandbox` refuses any `PlatformQualified` plan while
`EXTERNAL_GATES` is open, and residuals
`macos_app_sandbox_signed_enforcement` and
`multi_os_platform_qualified_composition` remain open. This spec does not
change that and claims nothing beyond it.

Review policy: `FOUNDER_REVIEW_POLICY_AMENDMENT_2026-09-22.md`.

## What "Compute" means in this foundation

Core admits a typed job, pins its exact input, and runs it in a separate
MedScale-owned worker process. The worker executes only a closed set of
deterministic job kinds implemented in MedScale's own code. It receives
its staged input over a pipe, never a path into the vault; it cannot write
canonical state; it returns a candidate that Core validates before
committing it as a derived output.

```text
source snapshot (075) != staged input (exact canonical bytes, pinned digest)
  != computation (worker process) != candidate output (worker stdout)
  != derived output (Core-validated, committed) != reviewed result (not here)
  != external effect (none)
```

There is no arbitrary code path: no shell, no Python, no R, no user
script, no plugin, no dynamic library loading, and no network code. The
worker applies the one `ReadyBaseMeasured` mechanism its OS has to itself
(Linux Landlock FS allowlist + TCP deny + `RLIMIT_NOFILE`; Windows Job
Object active-process limit 1 + kill on close; macOS Seatbelt network
deny) and reports what it applied. That is defense in depth for MedScale's
own code, not a secure sandbox for untrusted code, and is never reported
as `platform_qualified`.

## Authorized scope

- Contracts (`medscale-contracts/src/compute.rs`): `ComputeJobKind`,
  `ComputeParams`, `ComputeJobRequest`, `StagedInput`, `RuntimeIdentity`,
  `ExecutionPolicy`, `SandboxRequirement`, `ResourceCeilings`,
  `ComputeJob` (the manifest plus state), `ComputeState`,
  `ComputeDenyReason`, `ComputeFailure`, `SandboxReport`,
  `ComputeReceipt`, `ComputeOutput`, and the worker protocol
  (`WorkerRequest`, `WorkerResponse`, `WorkerOutcome`, `WorkerRefusal`).
- Job kinds (closed, versioned, deterministic):
  - `column_profile` v1: per input column, the declared type, non-null
    count, missing count, distinct count, minimum, maximum and (numeric
    only) mean;
  - `sorted_projection` v1: the named columns of every input row, stably
    sorted by up to eight named key columns, ascending or descending.
  Both take exactly one Spec 075 snapshot of the same Project and produce
  one table (`ResultTableDoc`, as in Spec 082).
- A MedScale-owned worker binary `medscale-compute-worker`
  (`medscale-contracts/src/bin/compute_worker.rs`) that links only the
  contracts crate (no storage, no Core, no keys).
- Core authority: submit (admission, pinning, receipts for refusals), run
  (single claim, staging, supervision, validation, atomic commit), cancel,
  recover, status and reads; facade requests; CLI `medscale compute ...`.
- Storage v13 -> v14 (additive), backup/restore with fail-closed parsing,
  consistency checks, restart recovery.

## Frozen contracts

- `ComputeJobId` is a Core-allocated opaque id (`compute-job-N`).
- A job binds exactly one Project, one kind, typed params, one staged
  input (`snapshot_id`, `content_digest`, `schema_fingerprint`,
  `row_count`, `byte_len`), a runtime identity (`worker`, `worker_version`,
  `protocol_version`, `kind_version`), an execution policy, resource
  ceilings, and a `manifest_digest` over all of these. The manifest never
  changes after submission.
- Execution policy fields are fixed values validated on every read:
  `network=denied`, `shell=denied`, `user_code=denied`,
  `environment=cleared`, `inherited_handles=stdio_pipes_only`,
  `filesystem=empty_scratch_only`, plus `sandbox_requirement`
  (`ready_base_required` by default, or `process_isolation_only`).
- Ceilings (all bounded by hard maxima): input bytes, output bytes, output
  rows, stderr bytes, timeout in milliseconds.
- States (closed; never collapsed): `queued`, `running`, `completed`,
  `cancelled`, `timed_out`, `denied`, `resource_exhausted`, `failed`,
  `unavailable`, `corrupt`, `interrupted`. Every state except `queued` and
  `running` is terminal and has exactly one receipt.
- Stdout is the protocol channel only (one JSON response, bounded).
  Stderr is untrusted: only its byte count and the digest of its bounded
  prefix are recorded, never its text.
- Outputs are committed only after Core validates: protocol version, job
  id and manifest digest binding, digest of the exact output bytes, that
  the bytes are the canonical encoding, and the kind's output shape.

## Frozen acceptance requirements

1. A `column_profile` and a `sorted_projection` job over a synthetic
   snapshot complete in a separate worker process; their outputs equal
   hand-computed fixtures and are byte-identical across two runs.
2. Every submission that reaches a Project leaves a job; refused ones are
   terminal `denied` with a receipt: missing or other-Project snapshot,
   unknown column, bad params, input over the byte ceiling.
3. A job runs at most once: a second run of a non-queued job is refused
   and executes nothing; a cancelled job never runs.
4. A staged input whose bytes no longer match the pinned digest is not
   executed and ends `corrupt`; a snapshot removed after submission ends
   `denied` (stale reference).
5. A worker that hangs ends `timed_out`; one that floods stdout ends
   `resource_exhausted`; one that floods stderr is bounded and its text is
   not stored; one that crashes ends `failed`; one that writes malformed,
   partial, mismatched-job or wrong-digest output ends `corrupt`; none of
   these commit an output.
6. Cancelling a running job through its cancel handle kills the worker and
   ends `cancelled` without output; a queued job cancels directly.
7. A missing worker executable ends `unavailable`; when the policy
   requires a READY_BASE sandbox and the worker cannot apply one, the job
   ends `unavailable` and nothing is computed. The receipt reports the
   applied mechanism and `platform_qualified=false`.
8. The worker runs with a cleared environment, an empty private working
   directory, no path into the vault, and stdio pipes only; it reports
   zero environment variables.
9. A job left `running` by a crash is recovered as `interrupted` and never
   re-executed automatically; state survives reopen and backup/restore;
   tampered rows are refused; v13 backups restore with empty Compute
   tables.
10. Nothing crosses Project, realm or authority scope.
11. The CLI reaches Compute only through Core.
12. Exact-head and post-main CI pass.

## Explicitly not authorized

- Arbitrary code of any kind: shell, Python, R, WASM, scripts, plugins,
  user-supplied binaries or expressions.
- Any network path, remote or self-hosted worker, cloud fallback, GPU,
  scheduler or lease-based worker pool (later slices; institutional
  schedulers are 090).
- Any new dependency.
- Claims of `platform_qualified`, of a secure sandbox for untrusted code,
  or of OS-enforced memory or CPU quotas beyond what is listed above.
- Writing to any source snapshot or any Spec 074-084 object.
- Real PHI, production credentials, release claims.

Recorded residuals (expected, non-blocking):

- the worker's OS mechanism is `ReadyBaseMeasured` only; memory is bounded
  by the input ceiling and job design, not an OS quota; the process-count
  limit is OS-enforced on Windows only;
- cancelling a running job needs an in-process handle; the CLI runs jobs
  synchronously;
- the worker binary is not yet in the portable release package, where
  Compute therefore reports `unavailable`;
- no Desktop surface.

## Completion rule

`CLOSED_CANONICAL` only after merge on a green exact head and recorded
post-main verification. Closure of 085 does not authorize 086.
