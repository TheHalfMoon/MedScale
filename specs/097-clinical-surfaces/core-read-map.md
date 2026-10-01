# Core read map — next governed slice

This is an implementation plan, not current frontend data or a new capability.
The current Patients index remains unavailable until the 096 adapter exists.

## Owned subject context

Use an explicitly owned synthetic Host and a fixed Rust-held subject context.
The existing presentation golden has Ada Synthetic (`pres-patient`, identifier
P-004), not Sarah Chen. It is a source fixture, not an ingested record in this app.
Do not change its name or silently reinterpret the Slint view as a Core roster.

Any future fixture import must use governed Core ingest and explicit synthetic
promotion. It must record source/proposal/assertion identities separately; the
frontend cannot auto-promote or construct clinical authority. No automatic
private vault discovery or broad CliSession operator session is acceptable.

## Supported reads and mapping

| Rust contract | Surface mapping | Limitation |
|---|---|---|
| SubjectBriefV1.sections.identity | Compact identity with field coverage and citation refs | Display name may be missing, unknown or conflicting; never synthesize one. |
| SubjectBriefV1.sections.conditions | Supported condition facts with literal coverage | Empty values alone do not establish clinical absence. |
| SubjectTimelineV1.events | Longitudinal section with effective/recorded time and assertion/source refs | Preserve missing time and source identity; do not imply freshness or review. |
| SubjectCoverageV1 | Explicit coverage slots and conflict/unknown/unit distinctions | Evidence quality is distinct from clinical review status. |
| DrillDownPresentation | Bounded provenance inspector | Named Rust operation only, no raw source bytes, paths or generic request JSON. |

There is no inspected general roster contract. One owned subject context can
produce one supported row once the Core reads are real and qualified; it cannot
establish a complete workspace patient count. The UI must label that scope.

## Execution gates and failure semantics

First satisfy specs/096-tauri-foundation/ipc-contract.md, including Rust-owned
session/lease/context, request/schema correlation, bounded fields and denial
tests. Add only explicit domain commands and generated minimum capabilities.
Unavailable, denied, unsupported and corrupt results never become empty success.
Projection data is rebuildable and non-authoritative. No LLM or runtime egress.

Inspect and qualify the resulting actual row before implementing Patient Detail
and Evidence. Keep the current unavailable presentation when a read fails.
