# MedScale Research OS Migration Strategy

**Status:** Planning candidate — not implementation authority

## Principle

The Research OS must evolve MedScale without throwing away the current Core, Desktop, CLI, model Pack, Evidence, privacy, and workflow work. This is an additive architectural migration, not a rewrite.

## Preserve

The future program should preserve unless a bounded spec proves replacement is necessary:

- one Rust authority path;
- Desktop/CLI semantic parity;
- signed/admitted Pack model;
- local-first operation;
- Evidence/Provenance concepts;
- explicit authority-changing actions;
- Network Broker and gated external access principles;
- current native Desktop direction and design system;
- synthetic/permitted-data boundary until separately authorized;
- existing release/qualification truth discipline.

## Avoid

Do not introduce:

- a second independent project/database authority;
- a web-app rewrite solely to embed donor UIs;
- mandatory cloud identity or sync;
- hidden online model acquisition;
- direct external-agent access to the vault;
- Superset/OpenRAG/VoiceStudio/Buzz as mandatory whole-platform runtime dependencies;
- new product truth claims merely because planning docs exist.

## Migration stages

### Stage 0 — Planning only

Current branch. Add research/architecture/source/roadmap artifacts only. Do not modify `specs/CURRENT.md` or production behavior.

### Stage 1 — Project/Artifact compatibility layer

When promoted, introduce the minimum generic Project/Artifact contracts around existing MedScale domain objects. Existing routes and CLI commands should continue to work while Project-aware views are added.

### Stage 2 — New work emits Project relations

MedAgent/Audio/Analytics outputs are created as typed Project artifacts from the start. Avoid backfilling legacy state repeatedly through ad-hoc migrations.

### Stage 3 — Collaboration references artifacts

Rooms/tasks/notes/messages reference artifact identities/revisions; they do not become alternate artifact stores.

### Stage 4 — Optional Hub

Move collaboration synchronization behind Hub contracts without changing local artifact semantics. Personal/offline remains a supported topology.

### Stage 5 — Workers and institutional adapters

Externalize heavyweight compute only after job/artifact contracts are stable. Do not make worker protocols responsible for domain semantics.

## UI migration

The existing native Desktop design remains authority. Future planning may adjust information architecture, but must preserve the Spec 073 visual/system constraints until a later explicitly promoted design spec supersedes them.

A likely evolution is to introduce Projects as a stronger organizing surface while retaining existing Patients/Documents/Models/Evidence capability access during migration. Do not delete current routes merely to match the future product map.

## CLI migration

Every new durable object requires script-safe inspection and stable machine output. Candidate additions should follow current MedScale CLI truth/exit-code/evidence discipline rather than inventing a separate Research OS CLI.

## Data migration

Every migration must define:

- old schema/version;
- new schema/version;
- deterministic transform;
- backup/rollback;
- interrupted-migration recovery;
- validation counts/digests where practical;
- no authority duplication;
- compatibility window and eventual removal plan.

## Donor migration

Donor code should never land as a giant import commit. Preferred sequence:

1. define MedScale contract;
2. implement minimal native baseline or adapter stub;
3. qualify donor component against the contract;
4. copy/adapt only bounded winning surface;
5. preserve provenance/NOTICE;
6. test behavior at the MedScale boundary;
7. remove donor-specific assumptions from higher layers.

## Completion truth

Research OS planning does not make current MedScale incomplete in a retroactive sense; it defines a future product program. Conversely, closing the current product sequence does not imply Research OS features exist. Release truth must always state which product generation and scope is being qualified.
