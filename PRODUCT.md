# MedScale Product Truth

## Product
MedScale is a local-first healthcare intelligence workspace for clinicians, care teams, operators, and technical builders. Desktop and CLI are the launch surfaces. Mobile applications come only after Desktop + CLI launch quality is established.

## Core promise
MedScale turns trusted longitudinal health data into understandable, source-aware workflows without weakening authority, privacy, or evidence boundaries.

## Launch surfaces
- **Desktop**: the primary visual product for daily clinical and operational work.
- **CLI**: a first-class, scriptable product with the same authority and capability model as Desktop.
- **Mobile**: deferred until Desktop + CLI are launched; no mobile work is part of Specs 060-067 unless explicitly promoted later.

## Product principles
1. Local-first and private by default.
2. One Rust authority path for Desktop, CLI, and future clients.
3. Evidence and provenance are visible product concepts, not hidden implementation detail.
4. AI may assist, summarize, and propose; authority-changing actions remain explicit.
5. Fast enough to feel immediate, calm enough to support consequential work.
6. Advanced power is discoverable through command palette, keyboard, and CLI rather than visual clutter.
7. Synthetic/permitted fixtures only until REAL_PHI authorization is explicitly granted.

## Experience target
The product should feel like a precise, quiet, premium, inspectable clinical intelligence workspace. The founder-approved paired-M board and `docs/planning/PRODUCTIZATION_PROGRAM.md` supersede the older MedScale Signal visuals. Current identity uses black and white, layered neutral light/dark surfaces, Inter and JetBrains Mono NL, compact information density, and visible evidence boundaries. Focus and selection stay monochrome; any functional state accent is supplemental and never becomes the brand. The approved black Patients panel is the dark-mode visual north star, and the light board is the equal-quality light-mode reference.

Models are a first-class product concept. Users must be able to see what model is installed/admitted, where it came from, what task it serves, what runtime/device executes it, its provenance/trust state, benchmark state, and promotion/rollback state. Hugging Face may be a governed model source, but never becomes an authority plane.

Competitive evidence is also a first-class product concept. The product may show where MedScale has a proven advantage over OpenMed and must equally show where OpenMed remains ahead. “MedScale beats OpenMed” is forbidden unless the exact capability has a bound comparative result.

MedScale is a **Clinical Intelligence OS** guided by **Evidence first. Action second.** The work is the interface: clinical context and evidence precede decorative chrome or product metrics. The approved logo construction, type and neutral system live under `docs/brand/` and `DESIGN.md`; route and review truth live in the current Spec 095–100 program. An existing local workspace or vault capability is not user sign-in authority.

## User jobs
Desktop and CLI must ultimately cover the same trusted product capabilities: status/privacy inspection, vault/open lifecycle, ingest/import, patient/subject search, longitudinal timeline, brief/summary, coverage, documents/labs, evidence/insights, care-plan workflows, tasks/messages where authorized, audit, exports, backup/recovery, integrations, packs, model/runtime inspection, comparative evidence, and operator diagnostics.

## Non-goals for this phase
- No real PHI authorization.
- No product runtime cloud dependency.
- No Tauri/WebView.
- No mobile app implementation before Desktop + CLI launch.
- No claim of WCAG conformance until final product qualification evidence exists.
- No production signing/notarization claims without real credentials.

## Success definition
The Desktop and CLI should feel like two expressions of one product: same data semantics, same authority rules, same vocabulary, same privacy posture, and equivalent power appropriate to each interaction model.

## Current productization sequence
Specs 068–094 are closed in their bounded historical scope. Specs 095–100 own the current founder-authorized identity and Desktop productization sequence. Spec 095 is not canonically closed while required zero-cost review remains blocked; Spec 096 implementation remains isolated and unqualified. `RELEASE_READY=false`, `PRIVATE_DATA_READY=false`, and `MULTI_CLIENT_RELEASE_READY=false`.
