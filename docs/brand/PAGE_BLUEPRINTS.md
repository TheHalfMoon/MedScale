# MedScale Page Blueprints

**Status:** FOUNDER_APPROVED_HANDOFF — Spec 093

## Global shell

Desktop target: 1440×900. Minimum reference: 1100×720.

Left to right:
1. 56–60 px obsidian global rail.
2. 200–240 px adaptive route sidebar.
3. flexible primary workspace.
4. optional 280–360 px inspector only when the task benefits.

Top context bar contains project/workspace context, search/command access, bounded runtime/privacy posture, and account/theme controls. Avoid a marketing-style product header inside the work canvas.

## Home / Research Command Center

Purpose: orient the user to active work, not show vanity KPIs.

Primary zones:
- continue active project or start a project;
- recent research artifacts and evidence needing review;
- model/runtime availability summary;
- quick access to Data, MedAgent, Compare, Analytics, AudioFlow, and Browse;
- local/privacy posture kept visible but quiet.

Use one focal brand moment at most. The rest is Work mode.

## Projects

Purpose: organize durable research work without replacing underlying authority.

Project list uses clear rows/cards with title, owner/team, updated time, evidence posture, and current stage. Project workspace opens into a tabbed or pane-based environment for Overview, Data, Experiments, Artifacts, Notes, Evidence, and Activity.

Project context remains persistent while moving into Agent, Compare, Analytics, or Browse.

## MedAgent

Purpose: use one or more medical/research models as inspectable assistants.

Default composition:
- left conversation/task lane;
- center artifact/outcome pane that appears only when output exists;
- optional right context inspector for model, evidence, sources, and run settings.

Support parallel lanes without implying that consensus equals correctness. Each lane must identify model/runtime/provenance separately.

## Model Center + Compare

Model Center emphasizes task fit, provenance, admission/runtime state, device, rights, benchmark evidence, and availability.

Compare uses symmetric lanes with shared prompt/task context and clearly separate outputs. Differences are inspectable; no automatic winner badge unless backed by an explicit user-defined metric and evidence.

## Data / Source Fabric

Purpose: connect, inspect, and prepare data while preserving source identity and custody boundaries.

Use a source list + detail inspector pattern. Show connection/state, schema/format, freshness, local/remote posture, permissions, and derived-artifact relationships. Keep destructive or irreversible actions explicit.

## Analytics Gate

Purpose: move from selected data to transparent analysis and reproducible outputs.

Use a three-stage grammar: Define → Run → Inspect. The analysis canvas may combine configuration, code/notebook-like detail, charts, tables, and evidence. Results must identify input snapshot, method, model/tool, parameters, and run identity where available.

## Knowledge / Research Canvas

Purpose: connect papers, notes, evidence, hypotheses, artifacts, and decisions.

Favor a flexible canvas + outline + inspector rather than a decorative node graph by default. Graph views are optional lenses, not the only representation.

## AudioFlow

Purpose: record/import, transcribe, inspect, segment, summarize, and attach audio evidence to research work.

Use waveform/timeline only where it helps actual audio work. Keep source file, transcript state, speaker/segment confidence, model/runtime provenance, and export boundaries visible.

## Governed Browse

Purpose: research the web without hiding network use, source boundaries, or capture provenance.

Use browser content + research sidecar. Captured sources become explicit artifacts with URL, timestamp, title, snapshot/citation state, and project attachment. Network/privacy posture remains visible without dominating the page.

## Team

Purpose: collaborate around projects, evidence, tasks, notes, and review.

Use people/presence sparingly. The important unit is shared work: comments, assignments, review requests, decisions, and activity. Do not turn the product into a chat-first social feed.

## Evidence

Purpose: inspect source, derivation, claim, limitation, and authority state.

Use a readable evidence ledger with source identity, claim text, supporting/contradicting material, freshness, provenance, and limitations. The inspector should make uncertainty easier to understand, not merely expose raw metadata.

## Settings / Privacy / Integrations

Purpose: make local-first behavior, permissions, storage, model/runtime boundaries, network policy, integrations, and account/team settings explicit.

Group by user intent rather than implementation module names. Destructive actions require clear consequences and confirmation. Never use dark patterns to enable network access or data sharing.
